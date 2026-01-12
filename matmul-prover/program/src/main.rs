#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // 1. Read input data sent by the Host
    let n = sp1_zkvm::io::read::<usize>();
    let a = sp1_zkvm::io::read::<Vec<f32>>();
    let b = sp1_zkvm::io::read::<Vec<f32>>();

    // 2. Perform the computation (The part we are proving)
    let mut c = vec![0.0; n * n];
    for i in 0..n {
        for k in 0..n {
            let va = a[i * n + k];
            for j in 0..n {
                c[i * n + j] += va * b[k * n + j];
            }
        }
    }

    // 3. Commit the result (Make it public for the proof)
    sp1_zkvm::io::commit(&c[0]); 
}