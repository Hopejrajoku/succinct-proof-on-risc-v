use sp1_sdk::{ProverClient, SP1Stdin};
use std::time::Instant;

pub const MATMUL_ELF: &[u8] = include_bytes!("../../../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/matmul-program");

fn main() {
    sp1_sdk::utils::setup_logger();

    // 1. Data Setup (16x16)
    let n = 16usize;
    let a = vec![1.2f32; n * n];
    let b = vec![2.5f32; n * n];

    let mut stdin = SP1Stdin::new();
    stdin.write(&n);
    stdin.write(&a);
    stdin.write(&b);

    // 2. Prover Setup
    let client = ProverClient::from_env();
    let (pk, vk) = client.setup(MATMUL_ELF);

    println!(" --- SWARM NODE INITIALIZED --- ");
    
    // 3. Execution (To get the cycle count/report)
    let (public_values, report) = client.execute(MATMUL_ELF, &stdin).run().unwrap();
    println!("RISC-V Execution Complete");
    println!("Total RISC-V Instructions: {}", report.total_instruction_count());

    // 4. Proving (The ZK part)
    println!("Generating ZK Proof...");
    let start_time = Instant::now();
    let proof = client.prove(&pk, &stdin).run().expect("Proving failed");
    let duration = start_time.elapsed();

    // 5. Output Stats & Verification
    println!("PROOF GENERATED");
    println!("---------------------------------------");
    println!("PERFORMANCE METRICS:");
    println!("Proof Generation Time: {:?}", duration);
    
    client.verify(&proof, &vk).expect("Verification failed");

    // 6. Read the Result
    let mut public_values = proof.public_values;
    let result_top_left = public_values.read::<f32>();
    
    println!("---------------------------------------");
    println!("VERIFIABLE RESULT: {}", result_top_left);
    println!("STATUS: Cryptographic Integrity Confirmed");
    println!("---------------------------------------");
}