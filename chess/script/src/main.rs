use sp1_sdk::{SP1Prover, SP1Stdin, SP1Verifier};

const CHESS_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
    stdin.write(&fen);

    let san = "d4".to_string();
    stdin.write(&san);


    let mut proof = SP1Prover::prove(CHESS_ELF, stdin).expect("proving failed");

    // Read output.
    let is_valid_move = proof.stdout.read::<bool>();
    
    println!("is vaild move : {}", is_valid_move);
    
    // Verify proof.
    SP1Verifier::verify(CHESS_ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
}
