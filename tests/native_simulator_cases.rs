#[cfg(feature = "native-simulator")]
mod native_simulator_cases {

    use ckb_testtool::{
        ckb_types::{
            bytes::Bytes,
            core::{ScriptHashType, TransactionBuilder},
            packed::{CellInput, CellOutput},
            prelude::*,
        },
        context::Context,
    };

    const MAX_CYCLES: u64 = 500_000_000_000;

    #[test]
    fn test_spawn() {
        let mut context = Context::default();

        context.add_contract_dir("test-contracts/target/debug/");
        context.add_contract_dir("tests/test-contracts/target/debug/");
        context.add_contract_dir("test-contracts/build/release/");
        context.add_contract_dir("tests/test-contracts/build/release/");

        let out_point_parent = context.deploy_cell_by_name("simple-spawn");

        let args: Vec<u8> = vec![];

        let lock_script = context
            .build_script_with_hash_type(
                &out_point_parent,
                ScriptHashType::Data2,
                Default::default(),
            )
            .expect("script")
            .as_builder()
            .args(args.pack())
            .build();
        let input: CellInput = CellInput::new_builder()
            .previous_output(
                context.create_cell(
                    CellOutput::new_builder()
                        .capacity(1000)
                        .lock(lock_script.clone())
                        .build(),
                    Bytes::new(),
                ),
            )
            .build();

        let outputs = vec![
            CellOutput::new_builder()
                .capacity(500)
                .lock(lock_script.clone())
                .build(),
            CellOutput::new_builder()
                .capacity(500)
                .lock(lock_script)
                .build(),
        ];

        let outputs_data = vec![Bytes::new(); 2];

        // build transaction
        let tx = TransactionBuilder::default()
            // .set_inputs(vec![input, input3, input2])
            .set_inputs(vec![input])
            .outputs(outputs)
            .outputs_data(outputs_data.pack())
            .build();

        let tx = context.complete_tx(tx);

        // run
        let result = context.verify_tx(&tx, MAX_CYCLES);
        let cycles = result.unwrap();

        println!("Cycles: {}", cycles);
    }
}
