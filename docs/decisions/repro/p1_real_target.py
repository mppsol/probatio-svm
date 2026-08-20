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
# Pubkey-shaped fields inside an agent record (Borsh, fixed order, no padding).
# 8 = registry (constant), 40/72 = owner/authority, 104 = agent asset id, 136 = signer.
AGENT_PUBKEY_OFFSETS = (40, 72, 104, 136)
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


def registry_keys(url):
    res = rpc(url, "getProgramAccounts",
              [REGISTRY, {"encoding": "base64", "withContext": True}])
    slot = res["context"]["slot"]
    records = [a for a in res["value"]
               if len(base64.b64decode(a["account"]["data"][0])) == AGENT_RECORD_LEN]
    keys = set()
    for a in records:
        raw = base64.b64decode(a["account"]["data"][0])
        for off in AGENT_PUBKEY_OFFSETS:
            keys.add(b58enc(raw[off:off + 32]))
    return slot, len(records), keys


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
                    continue
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
    reg_slot, n_records, keys = registry_keys(url)
    print(f"      slot {reg_slot}: {n_records} agent records, "
          f"{len(keys)} distinct referenced pubkeys")

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
    print(f"      nonexistent accounts                : {len(gone)} holding "
          f"${sum(gross[k] for k in gone):,}")
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
