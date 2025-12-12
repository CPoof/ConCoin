//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```
//! Flag: RUSTFLAGS="-C target-cpu=native -C target-feature=+avx512f" cargo run --release
//! Optionally add RUST_LOG=info in front for logs

use alloy_sol_types::SolType;
use anyhow::{Context, Result};
use clap::Parser;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use vdf_lib::PublicValuesStruct;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const VDF_ELF: &[u8] = include_elf!("vdf-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,

    #[arg(long, default_value = "20")]
    commit_hash: String,
}

fn main() -> Result<()> {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();

    stdin.write(&args.commit_hash);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(VDF_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        let decoded = PublicValuesStruct::abi_decode(output.as_slice()).unwrap();
        let PublicValuesStruct { commit_hash } = decoded;

        println!("Commited hash: {}", commit_hash);

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(VDF_ELF);

        let proof = client
            .prove(&pk, &stdin)
            .run()
            .context("Failed to generate proof")?;

        println!("Successfully generated proof!");

        client
            .verify(&proof, &vk)
            .context("Failed to verify proof")?;
        println!("Successfully verified proof!");
    }
    Ok(())
}
