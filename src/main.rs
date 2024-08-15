use itertools::Itertools;
use sha2::{Digest, Sha256};

use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::types::PrimeField64;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2x::backend::wrapper::plonky2_config::PoseidonBN128GoldilocksConfig;

fn main() {
    let file_content =
        std::fs::read_to_string("./data/block_proof.json").unwrap();
    let block_proof: ProofWithPublicInputs<GoldilocksField, PoseidonGoldilocksConfig, 2> = serde_json::from_str(&file_content).unwrap();

    let mut byte_size = vec![4usize; 16];
    byte_size.extend(vec![1; 32]);
    byte_size.extend(vec![8; 68]);

    let reduce = |x: u64, size: usize| {
        let mut bytes = x.to_le_bytes().to_vec();
        bytes.resize(size, 0);
        bytes.reverse();

        bytes
    };
    let public_inputs = block_proof
        .public_inputs
        .iter()
        .map(|e| e.to_canonical_u64())
        .collect_vec();
    let input_bytes = public_inputs
        .into_iter()
        .zip_eq(byte_size.into_iter())
        .flat_map(|(x, s)| reduce(x, s))
        .collect_vec();

    let sha256 = |input: Vec<u8>| {
        let mut h = Sha256::new();
        h.update(input);
        let mut r = h.finalize();
        r[0] = r[0] & 0x1f;

        r
    };
    let pi = sha256(vec![]).into_iter().chain(sha256(input_bytes).into_iter()).collect_vec();

    let file_content =
        std::fs::read_to_string("./data/proof_with_public_inputs.json").unwrap();
    let wrapped_proof: ProofWithPublicInputs<GoldilocksField, PoseidonBN128GoldilocksConfig, 2> = serde_json::from_str(&file_content).unwrap();
    let wrapped_pi = wrapped_proof.public_inputs.into_iter().take(64).map(|x| x.to_canonical_u64() as u8).collect_vec();

    assert_eq!(pi, wrapped_pi);
}
