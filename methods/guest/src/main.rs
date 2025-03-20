use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
}
;use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher};

fn main() {
    let key: [u8; 32] = env::read();
    let nonce: [u8; 12] = env::read();
    // Expects to be filled with plaintext to be encrypted
    let mut buffer: [u8; 16] = env::read();

    // Hash plaintext & commit
    let digest = Impl::hash_bytes(&buffer);
    env::commit(&digest);

    // TODO:
    // - Hash key and/or nonce & commit?

    // Key and IV must be references to the `GenericArray` type.
    // Here we use the `Into` trait to convert arrays into it.
    let mut cipher = ChaCha20::new(&key.into(), &nonce.into());

    // Write ciphertext into buffer by applying keystream to plaintext
    cipher.apply_keystream(&mut buffer);

    // write public output to the journal
    env::commit(&buffer);
}
