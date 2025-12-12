# ConCoin
A cryptocurrency with provable and unbiased randomness. This repo provides the security. The [client](https://github.com/CPoof/ConCoin-Frontend) aims to format input.\
We use [*verifiable delay functions*](https://eprint.iacr.org/2018/601.pdf) to prevent the Last Revealer Attack, which can lead to biased randomness. \
See the [brief documentation](https://github.com/CPoof/ConCoin/wiki#what-is-concoin) for more information.

# Requirements
1. Rust
2. Linux or macOS
3. SP1
4. Git

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
git clone https://github.com/CPoof/ConCoin.git
cd ConCoin
```
(Alternatively you can download the zip file from Github)\
You can now run either execute or prove:
```
cargo run --release -- --execute
cargo run --release -- --prove
```

# Security
While the underlying modules such as SP1 and Plonky3 have been audited, ConCoin is currently in the early Alpha phase.\
If you find a critical bug, please report it by making a issue.
