use risc0_zkvm::guest::env;

fn main() {
    let plaintext: [u8; 16] = env::read();
    let keystream: [u8; 16] = env::read();

    // TODO:
    // - Hash plaintext & commit?
    // - Hash keystream & commit?
   
    let mut buffer = plaintext.clone();
    xor_arrays(&plaintext,&keystream, &mut buffer);
    
    // write public output to the journal
    env::commit(&buffer);
}

/// UNSAFE: We demand input and output arrays here are the SAME length. 
fn xor_arrays(arr1: &[u8], arr2: &[u8], output: &mut [u8]) {
    // assert_eq!(arr1.len(), arr2.len(), "Arrays must have the same length");
    // assert_eq!(arr1.len(), output.len(), "Output array must have the same length");

    for i in 0..arr1.len() {
        // We assume that input arrays are the same length, skipping checks to save cycles
        unsafe {
            *output.get_unchecked_mut(i) = arr1.get_unchecked(i) ^ arr2.get_unchecked(i);
        }
    }
}
