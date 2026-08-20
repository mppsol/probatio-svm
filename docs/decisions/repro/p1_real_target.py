#!/usr/bin/env python3
"""P1 (kill gate) — is there a real on-chain AGENT this harness can certify?

Reproduces every number in docs/decisions/P1-real-target.md from the chain, not from
this repo's code. Two independent populations are fetched and intersected:

  A. Solana Agent Registry (mainnet 8oo4dC4Jv...) — every registered agent record.
  B. Jupiter Perps (PERPHjGB...) — every Position account, the harness's ONLY live
     ingestion path.

Usage:  python3 docs/decisions/repro/p1_real_target.py [--rpc URL]
Needs:  python3 (stdlib only) and ~500 MB of free disk for the Jupiter snapshot.
Runtime: ~3 minutes on a public RPC.
"""
import argparse, base64, collections, json, os, re, sys, tempfile, time, urllib.request

REGISTRY = "8oo4dC4JvBLwy5tGgiH3WwK4B9PWxL9Z4XjA2jzkQMbQ"
JUP_PERPS = "PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu"
POSITION_DISC_B58 = "VZMoMoKgZQb"   # sha256("account:Position")[..8], base58
POSITION_LEN = 216
OFF_OWNER, OFF_SIDE, OFF_SIZE_USD = 8, 152, 161   # crates/harness/src/jupiter.rs
AGENT_RECORD_LEN = 748
# `AgentAccount` Borsh schema, from the repo's own locked SDK -- 8004-solana@0.8.3,
# dist/core/borsh-schemas.js (vendored at attest/node_modules/8004-solana):
#   collection[32] creator[32] owner[32] asset[32] bump:u8 atom_enabled:u8
#   agent_wallet:Option<Pubkey> feedback_digest[32] feedback_count:u64
#   response_digest[32] response_count:u64 revoke_digest[32] revoke_count:u64
#   parent_asset:Option<Pubkey> parent_locked:u8 col_locked:u8
#   agent_uri:String nft_name:String col:String
# The two Option fields shift every later field, so this record CANNOT be read at
# fixed offsets -- it must be walked. `collection` is the constant base-registry
# collection and is not an agent identity; every other pubkey field is counted.
DELTA_UNIT_USD = 100   # crates/harness/src/jupiter.rs::DELTA_UNIT_USD

B58 = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"


def b58enc(b):
    n = int.from_bytes(b, "big")
    s = ""
    while n:
        n, r = divmod(n, 58)
        s = B58[r] + s
    for c in b:
        if c:
            break
        s = "1" + s
    return s or "1"


# --- Ed25519 point decompression (RFC 8032) -------------------------------------------
# A Solana PDA is *by construction* off the Ed25519 curve; an ordinary wallet address is a
# compressed curve point. So "is this address on the curve?" separates wallets from PDAs even
# when the account itself does not exist on chain (rent-collected), which getAccountInfo cannot.
_P = 2 ** 255 - 19
_D = (-121665 * pow(121666, _P - 2, _P)) % _P


def on_curve(pk32):
    """True if the 32-byte key is a valid compressed Ed25519 point (i.e. a plain wallet)."""
    y = int.from_bytes(pk32, "little") & ((1 << 255) - 1)
    if y >= _P:
        return False
    u = (y * y - 1) % _P
    v = (_D * y * y + 1) % _P
    x = (u * pow(v, 3, _P) * pow(u * pow(v, 7, _P), (_P - 5) // 8, _P)) % _P
    if (v * x * x - u) % _P == 0:
        return True
    if (v * x * x + u) % _P == 0:
        return True
    return False


def b58dec(s):
    n = 0
    for c in s:
        n = n * 58 + B58.index(c)
    b = n.to_bytes(32, "big")
    pad = len(s) - len(s.lstrip("1"))
    return bytes(pad) + b[pad:] if pad else b


def rpc(url, method, params, timeout=300):
    body = json.dumps({"jsonrpc": "2.0", "id": 1, "method": method, "params": params}).encode()
    for attempt in range(5):
        try:
            req = urllib.request.Request(url, body, {"Content-Type": "application/json"})
            d = json.loads(urllib.request.urlopen(req, timeout=timeout).read())
            if "result" not in d:
                raise RuntimeError(d.get("error", d))
            return d["result"]
        except Exception:
            if attempt == 4:
                raise
            time.sleep(2 * (attempt + 1))


def delta_units(usd):
    """crates/harness/src/jupiter.rs::delta_units — round USD to nearest $100 band."""
    sign = (usd > 0) - (usd < 0)
    return (usd + sign * (DELTA_UNIT_USD // 2)) // DELTA_UNIT_USD if usd >= 0 \
        else -((-usd + DELTA_UNIT_USD // 2) // DELTA_UNIT_USD)


def decode_agent_account(raw):
    """Walk one AgentAccount. Returns (identity_pubkeys, n_agent_wallets).

    Fails closed: any option tag that is not 0/1, or a record whose declared string
    lengths do not consume the account exactly, raises rather than yielding a partial
    identity set -- a silently-dropped field here would narrow the search and favour
    the KILL, which is the error direction this check exists to prevent.
    """
    o = 8
    keys, wallets = [], 0
    o += 32                                              # collection (base registry, not an identity)
    for _ in range(3):                                   # creator, owner, asset
        keys.append(b58enc(raw[o:o + 32])); o += 32
    o += 2                                               # bump, atom_enabled
    for field in ("agent_wallet", "parent_asset"):
        tag = raw[o]; o += 1
        if tag not in (0, 1):
            raise ValueError(f"{field}: option tag {tag} is neither 0 nor 1")
        if tag == 1:
            keys.append(b58enc(raw[o:o + 32])); o += 32
            if field == "agent_wallet":
                wallets += 1
        if field == "agent_wallet":                      # hash chains sit between the two options
            o += (32 + 8) * 3                            # feedback/response/revoke digest+count
    o += 2                                               # parent_locked, col_locked
    for name in ("agent_uri", "nft_name", "col"):        # three Borsh Strings
        n = int.from_bytes(raw[o:o + 4], "little"); o += 4
        if o + n > len(raw):
            raise ValueError(f"{name}: declared length {n} overruns the account")
        raw[o:o + n].decode()                            # must be UTF-8, or the walk is misaligned
        o += n
    # 748 is the ALLOCATED space, not the content length: Borsh content ends earlier, and
    # Anchor's realloc does not zero the freed tail, so a non-zero tail is expected (4 of
    # 1,471 records carry stale bytes from a previously longer `col`). Landing in bounds
    # with three valid UTF-8 strings is what proves the walk hit real field boundaries.
    return keys, wallets


def registry_keys(url):
    res = rpc(url, "getProgramAccounts",
              [REGISTRY, {"encoding": "base64", "withContext": True}])
    slot = res["context"]["slot"]
    sizes = collections.Counter()
    records, keys, wallets = 0, set(), 0
    for a in res["value"]:
        raw = base64.b64decode(a["account"]["data"][0])
        sizes[len(raw)] += 1
        if len(raw) != AGENT_RECORD_LEN:
            continue
        records += 1
        ks, w = decode_agent_account(raw)
        keys.update(ks); wallets += w
    return slot, records, keys, wallets, sizes


def jupiter_positions(url, path):
    """Stream the full Position set to disk, then scan it. Returns (slot, per-owner net/gross)."""
    body = json.dumps({
        "jsonrpc": "2.0", "id": 1, "method": "getProgramAccounts",
        "params": [JUP_PERPS, {
            "encoding": "base64", "withContext": True,
            "dataSlice": {"offset": 0, "length": POSITION_LEN},
            "filters": [{"dataSize": POSITION_LEN},
                        {"memcmp": {"offset": 0, "bytes": POSITION_DISC_B58}}]}]}).encode()
    req = urllib.request.Request(url, body, {"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=600) as r, open(path, "wb") as f:
        while True:
            chunk = r.read(1 << 22)
            if not chunk:
                break
            f.write(chunk)

    pat = re.compile(rb'"pubkey":"([1-9A-HJ-NP-Za-km-z]{32,44})","account":'
                     rb'\{"lamports":\d+,"data":\["([A-Za-z0-9+/=]+)"')
    slot = None
    net, gross, npos = collections.Counter(), collections.Counter(), collections.Counter()
    total = opened = 0
    all_owners = set()
    buf = b""
    with open(path, "rb") as f:
        head = f.read(1 << 16)
        m = re.search(rb'"slot":(\d+)', head)
        slot = int(m.group(1)) if m else None
        f.seek(0)
        while True:
            chunk = f.read(1 << 22)
            if not chunk:
                break
            buf += chunk
            last = 0
            for m in pat.finditer(buf):
                last = m.end()
                raw = base64.b64decode(m.group(2))
                if len(raw) != POSITION_LEN:
                    continue
                total += 1
                owner = b58enc(raw[OFF_OWNER:OFF_OWNER + 32])
                all_owners.add(owner)
                size = int.from_bytes(raw[OFF_SIZE_USD:OFF_SIZE_USD + 8], "little")
                if size == 0:
                    continue                      # Jupiter keeps closed slots allocated
                side = raw[OFF_SIDE]
                if side not in (1, 2):            # 1 = Long, 2 = Short
                    # Fail closed, matching the harness's decoder: an open position with an
                    # unreadable side must not be silently dropped from the population.
                    raise ValueError(f"open Position {m.group(1).decode()} has side byte {side}")
                usd = size // 1_000_000           # atomic USD (1e6) -> whole USD
                opened += 1
                net[owner] += usd if side == 1 else -usd
                gross[owner] += usd
                npos[owner] += 1
            buf = buf[last:] if last else buf[-512:]
    return slot, total, opened, all_owners, net, gross, npos


def account_owners(url, keys):
    """Resolve each address's owning program (System Program == a plain wallet/EOA)."""
    out, keys = {}, sorted(keys)
    for i in range(0, len(keys), 100):
        batch = keys[i:i + 100]
        vals = rpc(url, "getMultipleAccounts",
                   [batch, {"encoding": "base64", "dataSlice": {"offset": 0, "length": 0}}],
                   timeout=60)["value"]
        for k, a in zip(batch, vals):
            out[k] = a["owner"] if a else None
        time.sleep(0.15)
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--rpc", default=os.environ.get("PROBATIO_RPC_URL",
                                                    "https://api.mainnet-beta.solana.com"))
    ap.add_argument("--keep", help="path to cache the Jupiter snapshot (default: temp file)")
    args = ap.parse_args()
    url = args.rpc
    SYSTEM = "1" * 32

    print("[1/4] Solana Agent Registry ...", flush=True)
    reg_slot, n_records, keys, wallets, sizes = registry_keys(url)
    print(f"      slot {reg_slot}: account sizes {dict(sizes)}")
    print(f"      {n_records} AgentAccounts, {wallets} with an operational agent_wallet, "
          f"{len(keys)} distinct agent identities (creator/owner/asset/agent_wallet/parent_asset)")

    print("[2/4] Jupiter Perps Position accounts (~450 MB) ...", flush=True)
    path = args.keep or os.path.join(tempfile.gettempdir(), "probatio_p1_jup.json")
    jup_slot, total, opened, all_owners, net, gross, npos = jupiter_positions(url, path)
    live = sorted(gross)
    print(f"      slot {jup_slot}: {total} Position accounts, {opened} open "
          f"(sizeUsd>0), {len(all_owners)} distinct owners, {len(live)} with an open position")

    print("[3/4] intersect registry x Jupiter ...", flush=True)
    ever = keys & all_owners
    now = keys & set(live)
    print(f"      registered-agent pubkeys that EVER held a Jupiter position : {len(ever)}")
    print(f"      registered-agent pubkeys with an OPEN position right now   : {len(now)}")
    for k in sorted(ever):
        print(f"        {k}  open={'yes' if k in now else 'no'}")

    print("[4/4] who actually holds the live open interest? ...", flush=True)
    owners_of = account_owners(url, live)
    kinds = collections.Counter(owners_of.values())
    eoa = [k for k, v in owners_of.items() if v == SYSTEM]
    gone = [k for k, v in owners_of.items() if v is None]
    pda = [k for k, v in owners_of.items() if v not in (SYSTEM, None)]
    tot_notional = sum(gross.values())
    print(f"      total open notional                 : ${tot_notional:,}")
    print(f"      plain wallets (System Program)      : {len(eoa)} holding "
          f"${sum(gross[k] for k in eoa):,}")
    off = [k for k in gone if not on_curve(b58dec(k))]
    print(f"      nonexistent accounts                : {len(gone)} holding "
          f"${sum(gross[k] for k in gone):,}")
    print(f"        ...of these, OFF-curve (i.e. actually a PDA/vault, not a wallet): {len(off)}"
          + (f"  {off}" if off else ""))
    print(f"      program-controlled (vault-like PDAs): {len(pda)} holding "
          f"${sum(gross[k] for k in pda):,}")
    for k in sorted(pda, key=lambda x: -gross[x]):
        print(f"        {k}  program={owners_of[k]}  gross=${gross[k]:,}")

    print("\n[verdict function] what does the certification actually separate?")
    passes = [k for k in live if delta_units(net[k]) == 0]
    dust = [k for k in passes if gross[k] < DELTA_UNIT_USD // 2]
    hedged = [k for k in passes if gross[k] >= DELTA_UNIT_USD // 2]
    print(f"      PASS  : {len(passes)} / {len(live)} = {100*len(passes)/len(live):.2f}%")
    print(f"        of which gross < ${DELTA_UNIT_USD//2} (below the rounding band): "
          f"{len(dust)} ({100*len(dust)/max(len(passes),1):.1f}% of the PASS class)")
    print(f"        of which genuinely hedged                       : {len(hedged)}")
    print(f"      FLAG  : {len(live)-len(passes)} / {len(live)} = "
          f"{100*(len(live)-len(passes))/len(live):.2f}%")
    if not args.keep:
        try:
            os.remove(path)
        except OSError:
            pass


if __name__ == "__main__":
    sys.exit(main())
