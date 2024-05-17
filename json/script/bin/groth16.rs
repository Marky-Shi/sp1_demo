//! A simple script to generate and verify the proof of a given program.

use lib::{Account, Transaction};
use sp1_sdk::{utils, ProverClient, SP1Stdin};

const JSON_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // setup tracer for logging.
    utils::setup_logger();

    // Generate proof.
    let mut stdin = SP1Stdin::new();

    // Generic sample JSON (as a string input).
    let data_str = r#"
            {
                "name": "Jane Doe",
                "age": "25",
                "net_worth" : "$1000000"
            }"#
    .to_string();
    let key = "net_worth".to_string();

    // Custom struct example.
    let initial_account_state = Account {
        account_name: "John".to_string(),
        balance: 200,
    };
    let transactions = vec![
        Transaction {
            from: "John".to_string(),
            to: "Uma".to_string(),
            value: 50,
        },
        Transaction {
            from: "Uma".to_string(),
            to: "John".to_string(),
            value: 100,
        },
    ];

    stdin.write(&data_str);
    stdin.write(&key);
    stdin.write(&initial_account_state);
    stdin.write(&transactions);

    let client = ProverClient::new();
    let (pk,vk) = client.setup(JSON_ELF);
    let mut groth16_proof = client.prove_groth16(&pk, stdin).expect("proving failed");

    // Read output.
    let val = groth16_proof.public_values.read::<String>();
    println!("Value of {} is {}", key, val);

    let account_state = groth16_proof.public_values.read::<Account>();
    println!(
        "Final account state: {}",
        serde_json::to_string(&account_state).unwrap()
    );

    // Verify proof.
    client
        .verify_groth16(&groth16_proof,&vk)
        .expect("verification failed");

    // Save proof.
    groth16_proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}
