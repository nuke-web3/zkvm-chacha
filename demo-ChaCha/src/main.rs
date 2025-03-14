use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
use hex_literal::hex;

fn main() {
    // FIXME: need a method to gen and pass sym. key around those that need it (pehaps MPC methods)
    let key = [0x42; 32];
    // FIXME: need a good random nonce here!
    let nonce = [0x24; 12];

    let plaintext = hex!("00010203 04050607 08090A0B 0C0D0E0F");
    let ciphertext = hex!("e405626e 4f1236b3 670ee428 332ea20e");

    // Key and IV must be references to the `GenericArray` type.
    // Here we use the `Into` trait to convert arrays into it.
    let mut cipher = ChaCha20::new(&key.into(), &nonce.into());

    // Generate raw keystream bytes
    let mut keystream = [0u8; 16]; // ChaCha20 generates keystream in 64-byte blocks, but this can be any number of bits.
    cipher.apply_keystream(&mut keystream); // keystream XOR with 0s = keystream

    println!("Keystream bytes: {:?}", keystream);

    // ChaCha ciphers support seeking, need to reset to resuse (this is NOT to be used in prod of course)
    cipher.seek(0u32);
    let mut buffer = plaintext.clone();

    // apply keystream (encrypt)
    cipher.apply_keystream(&mut buffer);
    assert_eq!(buffer, ciphertext);

    let ciphertext = buffer.clone();

    cipher.seek(0u32);

    // decrypt ciphertext by applying keystream again
    cipher.apply_keystream(&mut buffer);
    assert_eq!(buffer, plaintext);

    // stream ciphers can be used with streaming messages
    cipher.seek(0u32);
    for chunk in buffer.chunks_mut(3) {
        cipher.apply_keystream(chunk);
    }
    assert_eq!(buffer, ciphertext);

    let mut result = [0u8; 16]; // Preallocated output buffer
    xor_arrays(&plaintext, &keystream, &mut result);
    println!("result bytes: {:?}", result);
    println!("plaintext bytes: {:?}", plaintext);
    println!("ciphertext bytes: {:?}", ciphertext);
}

fn xor_arrays(arr1: &[u8], arr2: &[u8], output: &mut [u8]) {
    assert_eq!(arr1.len(), arr2.len(), "Arrays must have the same length");
    assert_eq!(
        arr1.len(),
        output.len(),
        "Output array must have the same length"
    );

    for i in 0..arr1.len() {
        unsafe {
            *output.get_unchecked_mut(i) = arr1.get_unchecked(i) ^ arr2.get_unchecked(i);
        }
    }
}
