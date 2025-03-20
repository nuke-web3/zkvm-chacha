# Verifiable Encryption Demo

Goal: Encrypt private data (plaintext) while committing to the hash of the data and the resulting ciphertext prodced by some encryption. Correct encryption should be _succintly verifiable_ without decrypting the plaintext.

## Running encryption inside a zkVM

This repo contains a minimal [RISC Zero](https://risczero.com/) example of running [ChaCha20](https://en.wikipedia.org/wiki/Salsa20#ChaCha_variant) on some input data & committing to the hash of the input and resulting ciphertext.

## Development

To build all methods and execute the method within the zkVM, run the following
command:

```bash
# Run full proof
cargo r -r

# Execution ONLY testing
RISC0_DEV_MODE=1 cargo r
```

### Executing the Project Locally in Development Mode

During development, faster iteration upon code changes can be achieved by leveraging [dev-mode], we strongly suggest activating it during your early development phase. Furthermore, you might want to get insights into the execution statistics of your project, and this can be achieved by specifying the environment variable `RUST_LOG="[executor]=info"` before running your project.

Put together, the command to run your project in development mode while getting execution statistics is:

```bash
RUST_LOG="info" RISC0_DEV_MODE=1 cargo r
```

