use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::ChaCha20;

// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::GUEST_ENCRYPT_ELF;
use risc0_zkvm::sha::rust_crypto::{Digest, Sha256};
use risc0_zkvm::{default_prover, ExecutorEnv};

use common::INPUT_BYTES;

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // FIXME: need a method to gen and pass sym. key around those that need it (pehaps MPC methods)
    let key = [0x42; 32];
    // FIXME: need a good random nonce here!
    let nonce = [0x24; 12];

    // 16 byte test
    let plaintext = INPUT_BYTES;

    // zkVM
    let env = ExecutorEnv::builder()
        .write_slice(&key)
        .write_slice(&nonce)
        .write_slice(&plaintext)
        .build()
        .unwrap();

    // testing Execution ONLY
    let prover = default_prover();

    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
    let prove_info = prover.prove(env, GUEST_ENCRYPT_ELF).unwrap();

    // extract the receipt.
    let receipt = prove_info.receipt;

    let output = receipt.journal.bytes.clone();
    // sha256 = 16 bytes committed first
    let output_digest: [u8; 32] = output[..32].try_into().expect("sha256 hash reading erorr");
    // Ciphertext is the rest of the journal bytes
    let mut output_buffer: Vec<u8> = output[32..]
        .try_into()
        .expect("Ciphertext unable to populate buffer");

    println!(
        "zkVM  -> plaintext hash: 0x{}",
        bytes_to_hex(&output_digest)
    );

    // Check against the input
    let input_plaintext_digest = Sha256::digest(&plaintext);
    println!(
        "Input -> plaintext hash: 0x{}",
        bytes_to_hex(&input_plaintext_digest)
    );

    let ciphertext_digest = Sha256::digest(&output_buffer);
    println!(
        "zkVM -> ciphertext hash: 0x{}",
        bytes_to_hex(&ciphertext_digest)
    );

    // Key and IV must be references to the `GenericArray` type.
    // Here we use the `Into` trait to convert arrays into it.
    let mut cipher = ChaCha20::new(&key.into(), &nonce.into());

    // decrypt ciphertext by applying keystream again
    cipher.apply_keystream(&mut output_buffer);

    assert_eq!(output_buffer, plaintext);
    println!("Decryption of zkVM ciphertext matches input!");
    println!(
        "Output size to publish to DA = {} bytes (seal), {} bytes (ciphertext)",
        receipt.seal_size(),
        output_buffer.len()
    );
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let digest_hex: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    digest_hex
}
