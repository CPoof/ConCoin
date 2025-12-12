//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use vdf_lib::PublicValuesStruct;

use p3_baby_bear::{BabyBear, Poseidon2BabyBear};
use p3_challenger::{CanObserve, CanSample, DuplexChallenger};
use p3_field::integers::QuotientMap;
use p3_field::PrimeField64;
use p3_poseidon2::ExternalLayerConstants;
use p3_symmetric::Permutation;

const WIDTH: usize = 16;
const RATE: usize = 8;

type F = BabyBear;

fn bytes_to_field_elements(bytes: &[u8]) -> Vec<F> {
    let mut elements = Vec::with_capacity(bytes.len().div_ceil(8));
    for chunk in bytes.chunks(8) {
        let mut val: u64 = 0;
        for (i, &byte) in chunk.iter().enumerate() {
            val |= (byte as u64) << (i * 8);
        }

        // Complier cannot verify the guarantee that any u64 can fit into goldilocks field
        unsafe {
            elements.push(F::from_canonical_unchecked(val));
        }
    }
    elements
}
pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let total_revealed_values = sp1_zkvm::io::read::<String>();

    let input_string_bytes: Vec<u8> = total_revealed_values.as_bytes().to_vec();
    let input_elements: Vec<F> = bytes_to_field_elements(&input_string_bytes);

    //Vectors are empty since we pass inputs via the challenger/sponge
    let initial_vec: Vec<[F; WIDTH]> = Vec::new();
    let terminal_vec: Vec<[F; WIDTH]> = Vec::new();
    let external_constants = ExternalLayerConstants::new(initial_vec, terminal_vec);

    let internal_constants_vec: Vec<F> = Vec::new();

    let poseidon =
        Poseidon2BabyBear::<WIDTH>::new(external_constants.clone(), internal_constants_vec.clone());

    let poseidon_config =
        Poseidon2BabyBear::<WIDTH>::new(external_constants.clone(), internal_constants_vec.clone());

    type SpecificChallenger = DuplexChallenger<F, Poseidon2BabyBear<WIDTH>, WIDTH, RATE>;

    // Challenge/Sponge
    // Pads fields to 16 elements
    let mut challenger: SpecificChallenger = DuplexChallenger::new(poseidon_config);

    // Absorb/Input elements
    challenger.observe(vec![input_elements]);

    // Squeeze out exactly 16 elements (the WIDTH) into a Vec<F>
    let initial_vdf_state_vec: Vec<F> = challenger.sample_vec(WIDTH);

    // 3. Convert the Vec<F> into a fixed-size array [F; WIDTH]
    let mut state: [F; WIDTH] = initial_vdf_state_vec.try_into().expect(
        "Squeezed data was not exactly 16 elements. This should not happen if sample_vec(16) was called correctly.",
    );
    // 'state' is now a mutable [F; 16] array.

    // Apply the VDF loop using the low-level permute function
    for _ in 0..10000 {
        // Pass the mutable reference to the fixed-size array
        poseidon.permute_mut(&mut state);
    }

    // Squeeze out only 8 elements for security
    // This results in 2^32 * 8 bits of security, 2^256
    let final_digest: &[F] = &state[0..RATE];

    let mut hex_string = String::new();
    for &element in final_digest {
        let value: u64 = element.as_canonical_u64();
        hex_string.push_str(&format!("{:016x}", value));
    }

    let commit_hash: String = hex_string;

    // Encode the public values of the program.
    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct { commit_hash });
    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    sp1_zkvm::io::commit_slice(&bytes);
}
