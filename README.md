# ConCoin
A cryptocurrency with provable and unbiased randomness. \
We use [*verifiable delay functions*](https://eprint.iacr.org/2018/601.pdf) to prevent the Last Revealer Attack, which can lead to biased randomness. 

# Requirements
1. Rust
2. Linux or macOS
3. SP1

SP1 is a powerful tool which speeds up the verification of the VDF. \
Install SP1 either from the [docs](https://docs.succinct.xyz/docs/sp1/getting-started/install) or from the following:
```
curl -L https://sp1up.succinct.xyz | bash
sp1up
```
Verify your installation:
``
cargo prove --version
``

# Quickstart
Once SP1 is successfully installed and verified (as confirmed in the [Requirements](#sp1-installation) section), you are ready to clone the repository:
```
git clone https://github.com/CPoof/ConCoin
cd ConCoin
```
You can now run either execute or prove:
```
cargo run --release -- --execute
cargo run --release -- --prove
```
