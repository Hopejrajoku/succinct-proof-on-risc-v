# Grant Swarm: Verifiable Matrix Multiplication (ZK-MatMul)

This project implements **Verifiable Compute** for decentralized AI swarms. Using **SP1 (Succinct Prover 1)** and a **RISC-V zkVM**, we prove that matrix multiplication was performed correctly without requiring the verifier to re-run the computation.

## The Vision
In a decentralized "Swarm" of AI agents, nodes must perform heavy mathematical lifting (like Matrix Multiplication for Neural Networks). Traditionally, you have to trust the node or re-run the math. **ZK-MatMul** changes this: Nodes provide a result PLUS a cryptographic "receipt" (proof) that confirms the result is 100% accurate according to the source code.

## Proof of Execution
![Terminal Proof](./terminal_proof.png)
*Figure 1: Successful ZK Proof generation and verification on RISC-V.*

## Performance Logs (Verified Run)
- **Target**: 16x16 Matrix Multiplication.
- **RISC-V Instructions**: 540,735 cycles.
- **Proof Generation Time**: 275.80 seconds.
- **Verifiable Result**: 48.0 (Example Output).
- **Integrity**:  **Cryptographically Verified**

## Tech Stack
- **Guest Program**: Rust-based MatMul compiled to **RISC-V**.
- **Prover**: SP1 zkVM (STARK-based).
- **Host**: Rust SDK for proof generation and verification.
- **Environment**: GitHub Codespaces.

## How to Run
1. **Build Guest**: `cd matmul-prover/program && cargo prove build`
2. **Run Prover**: `cd matmul-prover/script && cargo run --release`