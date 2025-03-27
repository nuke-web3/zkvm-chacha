use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::ChaCha20;
use lz4_flex::compress_prepend_size;
use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};

use common::INPUT_BYTES_LENGTH;

fn main() {
    let start = env::cycle_count();

    let mut key: [u8; 32] = [0; 32];
    env::read_slice(&mut key);
    let mut nonce: [u8; 12] = [0; 12];
    env::read_slice(&mut nonce);
    // Expects to be filled with plaintext to be encrypted
    let mut buffer: [u8; INPUT_BYTES_LENGTH] = [0; INPUT_BYTES_LENGTH];
    env::read_slice(&mut buffer);

    // Hash plaintext & commit
    let digest = Impl::hash_bytes(&buffer);
    env::commit(&digest);

    // TODO:
    // - Hash key and/or nonce & commit?

    let mut compressed_input = compress_prepend_size(&buffer);
    // Key and IV must be references to the `GenericArray` type.
    // Here we use the `Into` trait to convert arrays into it.
    let mut cipher = ChaCha20::new(&key.into(), &nonce.into());

    // Write ciphertext into buffer by applying keystream to plaintext
    cipher.apply_keystream(&mut compressed_input);

    // write public output to the journal
    env::commit_slice(&compressed_input);

    let end = env::cycle_count();
    eprintln!("*-*-*-* CYCLE COUNT: {}", end - start);
}
