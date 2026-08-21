//! H4's deliberately narrow, offline Klend binary re-execution measurement.
//!
//! It has no protocol-independent abstraction: all account names, offsets, and
//! instruction construction below refer only to the one operation frozen in
//! `docs/H4-GATE.md`.

use std::{
    collections::BTreeMap,
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
};

use base64::Engine;
use litesvm::LiteSVM;
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use solana_account::Account;
use solana_instruction::{error::InstructionError, AccountMeta, Instruction};
use solana_keypair::Keypair;
use solana_message::{v0, VersionedMessage};
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;
use solana_transaction_error::TransactionError;

const KLEND: &str = "KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD";
const FARMS: &str = "FarmsPZpWu9i7Kky8tPN37rs2TpmMrAZrC7S7vJa91Hr";
const TOKEN_PROGRAM: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const INSTRUCTIONS_SYSVAR: &str = "Sysvar1nstructions1111111111111111111111111";
const COMPUTE_BUDGET: &str = "ComputeBudget111111111111111111111111111111";
const COMPUTE_UNIT_LIMIT: u32 = 1_400_000;
// The H4 manifest copied the account/program payloads but omitted the source
// capture's Clock timestamp.  This is the exact value in Solvo's immutable G1
// source manifest for slot 440,477,781; it is compiled in so a completed H4
// run remains offline and does not read ../solvo.
const FIXTURE_UNIX_TIMESTAMP: i64 = 1_787_230_883;
const OLD_HASH: &str = "8eab9f858da01fee962452ad3838570b3340cd43b80a236de8fe5297063d1cda";
const NEW_HASH: &str = "b1344d1979daec34bea862a3ed5c44ca5dc8b8e72ec32f1a90ac5150229c22d9";
const ACCOUNT_NAMES: [&str; 17] = [
    "obligation",
    "reserve_sol",
    "reserve_usdc",
    "lending_market",
    "scope_prices",
    "usdc_liquidity_mint",
    "usdc_liquidity_supply_vault",
    "usdc_collateral_mint",
    "usdc_collateral_supply_vault",
    "usdc_reserve_farm_state",
    "usdc_obligation_farm_user_state",
    "sol_liquidity_mint",
    "sol_liquidity_supply_vault",
    "sol_collateral_mint",
    "sol_collateral_supply_vault",
    "sol_reserve_farm_state",
    "sol_obligation_farm_user_state",
];

#[derive(Clone)]
struct ClonedAccount {
    pubkey: Pubkey,
    owner: Pubkey,
    lamports: u64,
    data: Vec<u8>,
}

struct Fixtures {
    slot: u64,
    unix_timestamp: i64,
    manifest_hash: String,
    accounts: BTreeMap<String, ClonedAccount>,
    derived: BTreeMap<String, Value>,
    farms: ProgramFixture,
    old: ProgramFixture,
    new: ProgramFixture,
}

#[derive(Clone)]
struct ProgramFixture {
    label: &'static str,
    path: String,
    stored_bytes: usize,
    elf: Vec<u8>,
    code_hash: String,
}

#[derive(Serialize)]
struct ProgramIdentity {
    path: String,
    stored_bytes: usize,
    elf_len: usize,
    code_hash: String,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
struct Mutation {
    account: String,
    pubkey: String,
    offset: usize,
    len: usize,
    before: String,
    after: String,
    why: String,
}

#[derive(Serialize, Clone, PartialEq, Eq)]
struct TokenDelta {
    account: String,
    pubkey: String,
    before: u64,
    after: u64,
}

#[derive(Serialize, Clone, PartialEq, Eq)]
struct ByteRange {
    offset: usize,
    len: usize,
}

#[derive(Serialize, Clone, PartialEq, Eq)]
struct PostAccount {
    account: String,
    pubkey: String,
    sha256: String,
    differing_ranges_from_fixture: Vec<ByteRange>,
}

#[derive(Serialize, Clone, PartialEq, Eq)]
struct ResultRecord {
    status: String,
    error: Option<String>,
    custom_code: Option<u32>,
}

#[derive(Serialize, Clone, PartialEq, Eq)]
struct Auxiliary {
    preamble_result: ResultRecord,
    preamble_logs: Vec<String>,
    preamble_compute_units: u64,
    logs: Vec<String>,
    compute_units: u64,
    return_data_hex: String,
}

#[derive(Serialize, Clone, PartialEq, Eq)]
struct Execution {
    binary: String,
    executed: bool,
    disclosed_mutations: Vec<Mutation>,
    result: ResultRecord,
    token_deltas: Vec<TokenDelta>,
    post_state: Vec<PostAccount>,
    auxiliary: Auxiliary,
}

#[derive(Serialize)]
struct CaseEvidence {
    id: String,
    collateral_amount: u64,
    old: Execution,
    new: Execution,
    verdict: String,
}

#[derive(Serialize)]
struct Evidence {
    schema: String,
    fixture_manifest_sha256: String,
    fixture_slot: u64,
    deposited_amount_d: u64,
    binaries: BTreeMap<String, ProgramIdentity>,
    disclosed_mutations: Vec<Mutation>,
    cases: Vec<CaseEvidence>,
}

struct Keys {
    klend: Pubkey,
    farms: Pubkey,
    token_program: Pubkey,
    instructions_sysvar: Pubkey,
    compute_budget: Pubkey,
    obligation: Pubkey,
    reserve_sol: Pubkey,
    reserve_usdc: Pubkey,
    lending_market: Pubkey,
    lending_market_authority: Pubkey,
    scope_prices: Pubkey,
    usdc_liquidity_mint: Pubkey,
    usdc_liquidity_supply_vault: Pubkey,
    usdc_collateral_mint: Pubkey,
    usdc_collateral_supply_vault: Pubkey,
    usdc_reserve_farm_state: Pubkey,
    usdc_obligation_farm_user_state: Pubkey,
    destination_usdc: Pubkey,
}

struct Harness {
    svm: LiteSVM,
    payer: Keypair,
    mutations: Vec<Mutation>,
}

struct RunReport {
    result: ResultRecord,
    logs: Vec<String>,
    compute_units: u64,
    return_data: Vec<u8>,
}

fn main() {
    let fixtures = load_fixtures();
    let keys = Keys::from_fixtures(&fixtures);
    let d = deposited_amount(&fixtures.accounts["obligation"].data, &keys.reserve_usdc);
    println!(
        "H4 sentinel: fixture slot {}; D (USDC deposited_amount) = {d}",
        fixtures.slot
    );

    let mut identities = BTreeMap::new();
    for program in [&fixtures.old, &fixtures.new] {
        identities.insert(
            program.label.to_string(),
            ProgramIdentity {
                path: program.path.clone(),
                stored_bytes: program.stored_bytes,
                elf_len: program.elf.len(),
                code_hash: program.code_hash.clone(),
            },
        );
    }

    let cases = [
        ("A", 100_000_000u64),
        ("B", d),
        ("C", d.checked_add(1).expect("D + 1")),
        ("E", u64::MAX),
    ];
    let mut evidence_cases = Vec::new();
    let mut disclosed_mutations = None;
    for (id, amount) in cases {
        let old = execute(&fixtures, &keys, &fixtures.old, id, amount);
        let new = execute(&fixtures, &keys, &fixtures.new, id, amount);
        for current_mutations in [&old.disclosed_mutations, &new.disclosed_mutations] {
            if current_mutations.is_empty() {
                continue;
            }
            if let Some(previous) = &disclosed_mutations {
                assert_eq!(
                    previous, current_mutations,
                    "mutations differ between executions"
                );
            } else {
                disclosed_mutations = Some(current_mutations.clone());
            }
        }
        let verdict = verdict(&old, &new);
        println!(
            "case {id}: amount {amount}; old {}; new {}; verdict {verdict}",
            old.result.error.as_deref().unwrap_or("Ok"),
            new.result.error.as_deref().unwrap_or("Ok")
        );
        evidence_cases.push(CaseEvidence {
            id: id.to_string(),
            collateral_amount: amount,
            old,
            new,
            verdict,
        });
    }

    let evidence = Evidence {
        schema: "H4 Klend sentinel evidence; one fixed operation, no adapter semantics".to_string(),
        fixture_manifest_sha256: fixtures.manifest_hash,
        fixture_slot: fixtures.slot,
        deposited_amount_d: d,
        binaries: identities,
        disclosed_mutations: disclosed_mutations.unwrap_or_default(),
        cases: evidence_cases,
    };
    let bytes = serde_json::to_vec_pretty(&evidence).expect("serialize evidence");
    let path = workspace_root().join("evidence/h4-sentinel.json");
    std::fs::create_dir_all(path.parent().expect("evidence directory")).expect("create evidence");
    std::fs::write(&path, &bytes).expect("write evidence");
    println!("wrote {} ({} bytes)", path.display(), bytes.len());
}

impl Keys {
    fn from_fixtures(fx: &Fixtures) -> Self {
        let account = |name: &str| fx.accounts[name].pubkey;
        let lending_market = account("lending_market");
        let klend = pk(KLEND);
        let (lending_market_authority, bump) =
            Pubkey::find_program_address(&[b"lma", lending_market.as_ref()], &klend);
        assert_eq!(
            lending_market_authority.to_string(),
            derived_string(fx, "lending_market_authority"),
            "fixture lending market authority"
        );
        assert_eq!(
            u64::from(bump),
            derived_u64(fx, "lending_market_authority_bump"),
            "fixture lending market authority bump"
        );
        // A deterministic public key; it has no meaning outside this one harness.
        let destination_usdc = Pubkey::new_from_array([0x48; 32]);
        Self {
            klend,
            farms: pk(FARMS),
            token_program: pk(TOKEN_PROGRAM),
            instructions_sysvar: pk(INSTRUCTIONS_SYSVAR),
            compute_budget: pk(COMPUTE_BUDGET),
            obligation: account("obligation"),
            reserve_sol: account("reserve_sol"),
            reserve_usdc: account("reserve_usdc"),
            lending_market,
            lending_market_authority,
            scope_prices: account("scope_prices"),
            usdc_liquidity_mint: account("usdc_liquidity_mint"),
            usdc_liquidity_supply_vault: account("usdc_liquidity_supply_vault"),
            usdc_collateral_mint: account("usdc_collateral_mint"),
            usdc_collateral_supply_vault: account("usdc_collateral_supply_vault"),
            usdc_reserve_farm_state: account("usdc_reserve_farm_state"),
            usdc_obligation_farm_user_state: account("usdc_obligation_farm_user_state"),
            destination_usdc,
        }
    }
}

fn execute(
    fx: &Fixtures,
    keys: &Keys,
    program: &ProgramFixture,
    case: &str,
    amount: u64,
) -> Execution {
    println!("execution {case}/{}", program.label);
    let mut harness = match setup(fx, keys, program) {
        Ok(harness) => harness,
        Err(error) => {
            println!(
                "execution {case}/{}: cannot load real binary: {error}",
                program.label
            );
            return Execution {
                binary: program.label.to_string(),
                executed: false,
                disclosed_mutations: Vec::new(),
                result: ResultRecord {
                    status: "Err".to_string(),
                    error: Some(error),
                    custom_code: None,
                },
                token_deltas: Vec::new(),
                post_state: Vec::new(),
                auxiliary: Auxiliary {
                    preamble_result: ResultRecord {
                        status: "Err".to_string(),
                        error: Some("not run: binary did not load".to_string()),
                        custom_code: None,
                    },
                    preamble_logs: Vec::new(),
                    preamble_compute_units: 0,
                    logs: Vec::new(),
                    compute_units: 0,
                    return_data_hex: String::new(),
                },
            };
        }
    };
    for mutation in &harness.mutations {
        print_mutation(mutation);
    }
    let preamble = send(&mut harness, keys, &refresh_instructions(keys));
    print_auxiliary("preamble", &preamble);
    let withdrawal = if preamble.result.status == "Ok" {
        let owner = harness.payer.pubkey();
        let report = send(
            &mut harness,
            keys,
            &[withdraw_instruction(keys, &owner, amount)],
        );
        print_auxiliary("withdrawal", &report);
        report
    } else {
        // This exact case did not reach the fixed withdrawal input.  It remains an
        // execution failure, rather than being repaired with a different preamble.
        RunReport {
            result: ResultRecord {
                status: "Err".to_string(),
                error: Some(format!(
                    "preamble failed; withdrawal not executed: {}",
                    preamble
                        .result
                        .error
                        .as_deref()
                        .unwrap_or("unknown preamble error")
                )),
                custom_code: preamble.result.custom_code,
            },
            logs: Vec::new(),
            compute_units: 0,
            return_data: Vec::new(),
        }
    };
    let token_deltas = token_deltas(&harness.svm, fx, keys);
    let post_state = post_state(&harness.svm, fx);
    Execution {
        binary: program.label.to_string(),
        executed: true,
        disclosed_mutations: harness.mutations.clone(),
        result: withdrawal.result,
        token_deltas,
        post_state,
        auxiliary: Auxiliary {
            preamble_result: preamble.result,
            preamble_logs: preamble.logs,
            preamble_compute_units: preamble.compute_units,
            logs: withdrawal.logs,
            compute_units: withdrawal.compute_units,
            return_data_hex: hex(&withdrawal.return_data),
        },
    }
}

fn setup(fx: &Fixtures, keys: &Keys, program: &ProgramFixture) -> Result<Harness, String> {
    assert_program(program);
    assert_program(&fx.farms);
    let payer = deterministic_payer();
    let mut svm = LiteSVM::new();
    let mut clock: solana_clock::Clock = svm.get_sysvar();
    clock.slot = fx.slot;
    clock.unix_timestamp = fx.unix_timestamp;
    clock.epoch = fx.slot / 432_000;
    svm.set_sysvar(&clock);
    svm.warp_to_slot(fx.slot);
    let mut clock: solana_clock::Clock = svm.get_sysvar();
    clock.unix_timestamp = fx.unix_timestamp;
    svm.set_sysvar(&clock);
    svm.airdrop(&payer.pubkey(), 100_000_000_000)
        .expect("airdrop deterministic payer");
    svm.add_program(keys.klend, &program.elf)
        .map_err(|error| format!("{error:?}"))?;
    svm.add_program(keys.farms, &fx.farms.elf)
        .map_err(|error| format!("{error:?}"))?;
    for cloned in fx.accounts.values() {
        svm.set_account(
            cloned.pubkey,
            Account {
                lamports: cloned.lamports,
                data: cloned.data.clone(),
                owner: cloned.owner,
                executable: false,
                rent_epoch: u64::MAX,
            },
        )
        .expect("install fixture account");
    }

    let mut mutations = Vec::new();
    let mut obligation = svm
        .get_account(&keys.obligation)
        .expect("fixture obligation");
    let before = obligation.data[64..96].to_vec();
    assert_eq!(
        Pubkey::new_from_array(before.clone().try_into().expect("owner length")).to_string(),
        derived_string(fx, "obligation_owner_as_fetched"),
        "fixture obligation owner"
    );
    obligation.data[64..96].copy_from_slice(payer.pubkey().as_ref());
    mutations.push(Mutation {
        account: "obligation".to_string(),
        pubkey: keys.obligation.to_string(),
        offset: 64,
        len: 32,
        before: hex(&before),
        after: hex(payer.pubkey().as_ref()),
        why: "make the fixed top-level signer the cloned obligation owner".to_string(),
    });
    svm.set_account(keys.obligation, obligation)
        .expect("reinstall repointed obligation");

    let destination_data = token_account(&keys.usdc_liquidity_mint, &payer.pubkey(), 0);
    mutations.push(Mutation {
        account: "destination_usdc".to_string(),
        pubkey: keys.destination_usdc.to_string(),
        offset: 0,
        len: destination_data.len(),
        before: "absent".to_string(),
        after: hex(&destination_data),
        why: "create deterministic USDC destination token account for the harness signer"
            .to_string(),
    });
    svm.set_account(
        keys.destination_usdc,
        Account {
            lamports: 5_000_000,
            data: destination_data,
            owner: keys.token_program,
            executable: false,
            rent_epoch: u64::MAX,
        },
    )
    .expect("install destination token account");
    Ok(Harness {
        svm,
        payer,
        mutations,
    })
}

fn send(harness: &mut Harness, keys: &Keys, instructions: &[Instruction]) -> RunReport {
    let mut all = Vec::with_capacity(instructions.len() + 1);
    all.push(compute_budget_instruction(keys));
    all.extend_from_slice(instructions);
    let message = v0::Message::try_compile(
        &harness.payer.pubkey(),
        &all,
        &[],
        harness.svm.latest_blockhash(),
    )
    .expect("compile transaction");
    let transaction =
        VersionedTransaction::try_new(VersionedMessage::V0(message), &[&harness.payer])
            .expect("sign transaction");
    match harness.svm.send_transaction(transaction) {
        Ok(meta) => RunReport {
            result: ResultRecord {
                status: "Ok".to_string(),
                error: None,
                custom_code: None,
            },
            logs: meta.logs,
            compute_units: meta.compute_units_consumed,
            return_data: meta.return_data.data,
        },
        Err(failure) => RunReport {
            result: ResultRecord {
                status: "Err".to_string(),
                error: Some(format!("{:?}", failure.err)),
                custom_code: custom_code(&failure.err),
            },
            logs: failure.meta.logs,
            compute_units: failure.meta.compute_units_consumed,
            return_data: failure.meta.return_data.data,
        },
    }
}

fn refresh_instructions(keys: &Keys) -> [Instruction; 3] {
    let refresh_reserve = |reserve| Instruction {
        program_id: keys.klend,
        accounts: vec![
            AccountMeta::new(reserve, false),
            AccountMeta::new_readonly(keys.lending_market, false),
            AccountMeta::new_readonly(keys.klend, false),
            AccountMeta::new_readonly(keys.klend, false),
            AccountMeta::new_readonly(keys.klend, false),
            AccountMeta::new_readonly(keys.scope_prices, false),
        ],
        data: anchor_disc("refresh_reserve").to_vec(),
    };
    [
        refresh_reserve(keys.reserve_usdc),
        refresh_reserve(keys.reserve_sol),
        Instruction {
            program_id: keys.klend,
            accounts: vec![
                AccountMeta::new_readonly(keys.lending_market, false),
                AccountMeta::new(keys.obligation, false),
                AccountMeta::new_readonly(keys.reserve_usdc, false),
                AccountMeta::new_readonly(keys.reserve_sol, false),
            ],
            data: anchor_disc("refresh_obligation").to_vec(),
        },
    ]
}

fn withdraw_instruction(keys: &Keys, owner: &Pubkey, amount: u64) -> Instruction {
    let mut data =
        anchor_disc("withdraw_obligation_collateral_and_redeem_reserve_collateral_v2").to_vec();
    data.extend_from_slice(&amount.to_le_bytes());
    Instruction {
        program_id: keys.klend,
        accounts: vec![
            AccountMeta::new(*owner, true),
            AccountMeta::new(keys.obligation, false),
            AccountMeta::new_readonly(keys.lending_market, false),
            AccountMeta::new_readonly(keys.lending_market_authority, false),
            AccountMeta::new(keys.reserve_usdc, false),
            AccountMeta::new_readonly(keys.usdc_liquidity_mint, false),
            AccountMeta::new(keys.usdc_collateral_supply_vault, false),
            AccountMeta::new(keys.usdc_collateral_mint, false),
            AccountMeta::new(keys.usdc_liquidity_supply_vault, false),
            AccountMeta::new(keys.destination_usdc, false),
            AccountMeta::new_readonly(keys.klend, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.instructions_sysvar, false),
            AccountMeta::new(keys.usdc_obligation_farm_user_state, false),
            AccountMeta::new(keys.usdc_reserve_farm_state, false),
            AccountMeta::new_readonly(keys.farms, false),
        ],
        data,
    }
}

fn token_deltas(svm: &LiteSVM, fx: &Fixtures, keys: &Keys) -> Vec<TokenDelta> {
    [
        ("destination_usdc", keys.destination_usdc),
        (
            "usdc_liquidity_supply_vault",
            keys.usdc_liquidity_supply_vault,
        ),
        (
            "usdc_collateral_supply_vault",
            keys.usdc_collateral_supply_vault,
        ),
    ]
    .map(|(name, key)| {
        let after_data = svm.get_account(&key).expect("tracked token account").data;
        let before = match name {
            "destination_usdc" => 0,
            "usdc_liquidity_supply_vault" => token_amount(&fx.accounts[name].data),
            "usdc_collateral_supply_vault" => token_amount(&fx.accounts[name].data),
            _ => unreachable!("only the three fixed token accounts are tracked"),
        };
        TokenDelta {
            account: name.to_string(),
            pubkey: key.to_string(),
            before,
            after: token_amount(&after_data),
        }
    })
    .to_vec()
}

fn post_state(svm: &LiteSVM, fx: &Fixtures) -> Vec<PostAccount> {
    fx.accounts
        .iter()
        .map(|(name, cloned)| {
            let post = svm
                .get_account(&cloned.pubkey)
                .expect("post fixture account");
            PostAccount {
                account: name.clone(),
                pubkey: cloned.pubkey.to_string(),
                sha256: sha256_hex(&post.data),
                differing_ranges_from_fixture: differing_ranges(&cloned.data, &post.data),
            }
        })
        .collect()
}

fn verdict(old: &Execution, new: &Execution) -> String {
    if !old.executed || !new.executed {
        "unknown".to_string()
    } else if old.result != new.result || old.token_deltas != new.token_deltas {
        "breaking".to_string()
    } else if old.post_state != new.post_state {
        "unknown".to_string()
    } else {
        "compatible".to_string()
    }
}

fn load_fixtures() -> Fixtures {
    let dir = workspace_root().join("fixtures/h4");
    let manifest_bytes = std::fs::read(dir.join("MANIFEST.json")).expect("H4 manifest");
    let manifest: Value = serde_json::from_slice(&manifest_bytes).expect("H4 manifest JSON");
    let mut accounts = BTreeMap::new();
    for name in ACCOUNT_NAMES {
        let filename = format!("{name}.json");
        let file = dir.join("accounts").join(&filename);
        let file_bytes = std::fs::read(&file).expect("fixture account JSON");
        let value: Value =
            serde_json::from_slice(&file_bytes).expect("fixture account JSON parses");
        let item = &manifest["accounts"][&filename];
        assert_eq!(
            sha256_hex(&file_bytes),
            item["file_sha256"].as_str().expect("file hash")
        );
        let data = base64::engine::general_purpose::STANDARD
            .decode(value["data_b64"].as_str().expect("data_b64"))
            .expect("account base64");
        assert_eq!(
            data.len() as u64,
            item["account_len"].as_u64().expect("account len")
        );
        assert_eq!(
            sha256_hex(&data),
            item["account_sha256"].as_str().expect("account hash")
        );
        let pubkey =
            Pubkey::from_str(value["pubkey"].as_str().expect("pubkey")).expect("pubkey parse");
        let owner = Pubkey::from_str(value["owner"].as_str().expect("owner")).expect("owner parse");
        assert_eq!(
            pubkey.to_string(),
            item["pubkey"].as_str().expect("manifest pubkey")
        );
        assert_eq!(
            owner.to_string(),
            item["owner"].as_str().expect("manifest owner")
        );
        accounts.insert(
            name.to_string(),
            ClonedAccount {
                pubkey,
                owner,
                lamports: value["lamports"].as_u64().expect("lamports"),
                data,
            },
        );
    }
    let derived = manifest["derived_from_source_manifest"]
        .as_object()
        .expect("derived object")
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    Fixtures {
        slot: manifest["provenance"]["accounts_and_old_binary"]["source_manifest_slot"]
            .as_u64()
            .expect("fixture slot"),
        unix_timestamp: FIXTURE_UNIX_TIMESTAMP,
        manifest_hash: sha256_hex(&manifest_bytes),
        accounts,
        derived,
        farms: load_program(&dir, &manifest, "farms"),
        old: load_program(&dir, &manifest, "klend_old"),
        new: load_program(&dir, &manifest, "klend_new"),
    }
}

fn load_program(dir: &Path, manifest: &Value, name: &'static str) -> ProgramFixture {
    let record = &manifest["binaries"][name];
    let path = record["path"].as_str().expect("program path");
    let gzip = std::fs::read(dir.join(path)).expect("program gzip");
    let mut stored = Vec::new();
    flate2::read::GzDecoder::new(&gzip[..])
        .read_to_end(&mut stored)
        .expect("decompress program");
    assert_eq!(
        stored.len() as u64,
        record["stored_len"].as_u64().expect("stored len")
    );
    while stored.last() == Some(&0) {
        stored.pop();
    }
    let code_hash = sha256_hex(&stored);
    assert_eq!(code_hash, record["code_hash"].as_str().expect("code hash"));
    assert_eq!(
        stored.len() as u64,
        record["elf_len"].as_u64().expect("elf len")
    );
    ProgramFixture {
        label: name,
        path: format!("fixtures/h4/{path}"),
        stored_bytes: record["stored_len"].as_u64().expect("stored len") as usize,
        elf: stored,
        code_hash,
    }
}

fn assert_program(program: &ProgramFixture) {
    let expected = match program.label {
        "klend_old" => OLD_HASH,
        "klend_new" => NEW_HASH,
        "farms" => "9ca00de8e9e13eb1e5283754b940ca9c3a007b61f4e195483241db454455a240",
        _ => panic!("unexpected program"),
    };
    assert_eq!(sha256_hex(&program.elf), expected, "loaded ELF hash");
}

fn deterministic_payer() -> Keypair {
    let seed: [u8; 32] = core::array::from_fn(|index| index as u8 + 1);
    Keypair::new_from_array(seed)
}

fn token_account(mint: &Pubkey, owner: &Pubkey, amount: u64) -> Vec<u8> {
    let mut data = vec![0u8; 165];
    data[0..32].copy_from_slice(mint.as_ref());
    data[32..64].copy_from_slice(owner.as_ref());
    data[64..72].copy_from_slice(&amount.to_le_bytes());
    data[108] = 1;
    data
}

fn compute_budget_instruction(keys: &Keys) -> Instruction {
    let mut data = vec![0x02];
    data.extend_from_slice(&COMPUTE_UNIT_LIMIT.to_le_bytes());
    Instruction {
        program_id: keys.compute_budget,
        accounts: vec![],
        data,
    }
}

fn anchor_disc(name: &str) -> [u8; 8] {
    let digest = Sha256::digest(format!("global:{name}").as_bytes());
    digest[..8].try_into().expect("discriminator length")
}

fn deposited_amount(data: &[u8], reserve: &Pubkey) -> u64 {
    for index in 0..8 {
        let offset = 96 + 136 * index;
        if pubkey_at(data, offset) == *reserve {
            return u64_at(data, offset + 32);
        }
    }
    panic!("USDC reserve absent from obligation deposits")
}

fn token_amount(data: &[u8]) -> u64 {
    u64_at(data, 64)
}

fn u64_at(data: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(data[offset..offset + 8].try_into().expect("u64 bytes"))
}

fn pubkey_at(data: &[u8], offset: usize) -> Pubkey {
    Pubkey::new_from_array(data[offset..offset + 32].try_into().expect("pubkey bytes"))
}

fn differing_ranges(before: &[u8], after: &[u8]) -> Vec<ByteRange> {
    let mut ranges = Vec::new();
    let shared = before.len().min(after.len());
    let mut index = 0;
    while index < shared {
        if before[index] == after[index] {
            index += 1;
            continue;
        }
        let start = index;
        while index < shared && before[index] != after[index] {
            index += 1;
        }
        ranges.push(ByteRange {
            offset: start,
            len: index - start,
        });
    }
    if before.len() != after.len() {
        ranges.push(ByteRange {
            offset: shared,
            len: before.len().max(after.len()) - shared,
        });
    }
    ranges
}

fn custom_code(error: &TransactionError) -> Option<u32> {
    match error {
        TransactionError::InstructionError(_, InstructionError::Custom(code)) => Some(*code),
        _ => None,
    }
}

fn derived_string(fx: &Fixtures, key: &str) -> String {
    fx.derived[key]
        .as_str()
        .expect("derived string")
        .to_string()
}

fn derived_u64(fx: &Fixtures, key: &str) -> u64 {
    fx.derived[key].as_u64().expect("derived u64")
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
        .to_path_buf()
}

fn pk(value: &str) -> Pubkey {
    Pubkey::from_str(value).expect("literal pubkey")
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

fn hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn print_mutation(mutation: &Mutation) {
    println!(
        "MUTATION account={} offset={} len={} before={} after={} why={}",
        mutation.account,
        mutation.offset,
        mutation.len,
        mutation.before,
        mutation.after,
        mutation.why
    );
}

fn print_auxiliary(label: &str, report: &RunReport) {
    println!(
        "{label}: result {} {:?}; compute_units {}; return_data {}",
        report.result.status,
        report.result.error,
        report.compute_units,
        hex(&report.return_data)
    );
    println!("{label}: logs (printed only; never asserted):");
    for line in &report.logs {
        println!("  {line}");
    }
}
