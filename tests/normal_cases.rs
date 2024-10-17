use ckb_crypto::secp::{Generator, Privkey};
use ckb_hash::{blake2b_256, new_blake2b};
use ckb_system_scripts::BUNDLED_CELL;
use ckb_testtool::{ckb_types::core::ScriptHashType, context::Context, *};
use ckb_types::{
    bytes::Bytes,
    core::{TransactionBuilder, TransactionView},
    packed::{self, *},
    prelude::*,
    H256,
};

const MAX_CYCLES: u64 = 500_0000;

fn blake160(data: &[u8]) -> [u8; 20] {
    let mut buf = [0u8; 20];
    let hash = blake2b_256(data);
    buf.clone_from_slice(&hash[..20]);
    buf
}

fn sign_tx(tx: TransactionView, key: &Privkey) -> TransactionView {
    const SIGNATURE_SIZE: usize = 65;

    let witnesses_len = tx.witnesses().len();
    let tx_hash = tx.hash();
    let mut signed_witnesses: Vec<packed::Bytes> = Vec::new();
    let mut blake2b = new_blake2b();
    let mut message = [0u8; 32];
    blake2b.update(&tx_hash.raw_data());
    // digest the first witness
    let witness = WitnessArgs::default();
    let zero_lock: Bytes = {
        let mut buf = Vec::new();
        buf.resize(SIGNATURE_SIZE, 0);
        buf.into()
    };
    let witness_for_digest = witness
        .clone()
        .as_builder()
        .lock(Some(zero_lock).pack())
        .build();
    let witness_len = witness_for_digest.as_bytes().len() as u64;
    blake2b.update(&witness_len.to_le_bytes());
    blake2b.update(&witness_for_digest.as_bytes());
    (1..witnesses_len).for_each(|n| {
        let witness = tx.witnesses().get(n).unwrap();
        let witness_len = witness.raw_data().len() as u64;
        blake2b.update(&witness_len.to_le_bytes());
        blake2b.update(&witness.raw_data());
    });
    blake2b.finalize(&mut message);
    let message = H256::from(message);
    let sig = key.sign_recoverable(&message).expect("sign");
    signed_witnesses.push(
        witness
            .as_builder()
            .lock(Some(Bytes::from(sig.serialize())).pack())
            .build()
            .as_bytes()
            .pack(),
    );
    for i in 1..witnesses_len {
        signed_witnesses.push(tx.witnesses().get(i).unwrap());
    }
    tx.as_advanced_builder()
        .set_witnesses(signed_witnesses)
        .build()
}

fn test_sighash_all_unlock(hash_type: ScriptHashType) {
    println!(
        "Running sighash_all_unlock test case, hash_type: {:?}",
        hash_type
    );
    // generate key pair
    let privkey = Generator::random_privkey();
    let pubkey = privkey.pubkey().expect("pubkey");
    let pubkey_hash = blake160(&pubkey.serialize());

    // deploy contract
    let mut context = Context::default();
    let secp256k1_data_bin = BUNDLED_CELL.get("specs/cells/secp256k1_data").unwrap();
    let secp256k1_sighash_all_bin = BUNDLED_CELL
        .get("specs/cells/secp256k1_blake160_sighash_all")
        .unwrap();
    let secp256k1_data_out_point = context.deploy_cell(secp256k1_data_bin.to_vec().into());
    let lock_out_point = context.deploy_cell(secp256k1_sighash_all_bin.to_vec().into());
    let lock_script = context
        .build_script_with_hash_type(&lock_out_point, hash_type, Default::default())
        .expect("script")
        .as_builder()
        .args(pubkey_hash.to_vec().pack())
        .build();
    let secp256k1_data_dep = CellDep::new_builder()
        .out_point(secp256k1_data_out_point)
        .build();

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();

    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script)
            .build(),
    ];

    let outputs_data = vec![Bytes::new(); 2];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(secp256k1_data_dep)
        .build();
    let tx = context.complete_tx(tx);

    // sign
    let tx = sign_tx(tx, &privkey);

    // run
    context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
}

#[test]
fn test_sighash_all_unlock_data() {
    test_sighash_all_unlock(ScriptHashType::Data);
}

#[test]
fn test_sighash_all_unlock_type() {
    test_sighash_all_unlock(ScriptHashType::Type);
}

#[test]
fn test_deterministic_rng_context() {
    let data = BUNDLED_CELL.get("specs/cells/secp256k1_data").unwrap();

    let out_point_1 = {
        let mut context = Context::default();
        context.deploy_cell(data.to_vec().into())
    };
    let out_point_2 = {
        let mut context = Context::default();
        context.deploy_cell(data.to_vec().into())
    };
    assert_ne!(out_point_1, out_point_2);

    let out_point_1 = {
        let mut context = Context::new_with_deterministic_rng();
        context.deploy_cell(data.to_vec().into())
    };
    let out_point_2 = {
        let mut context = Context::new_with_deterministic_rng();
        context.deploy_cell(data.to_vec().into())
    };
    assert_eq!(out_point_1, out_point_2);
}
