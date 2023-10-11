use crate::state::*;
use anchor_lang::prelude::*;

/// Return a tuple of ephemeral_signer_keys and ephemeral_signer_seeds derived
/// from the given `ephemeral_signer_bumps` and `transaction_key`.
pub fn derive_ephemeral_signers(
    transaction_key: Pubkey,
    ephemeral_signer_bumps: &[u8],
) -> (Vec<Pubkey>, Vec<Vec<Vec<u8>>>) {
    ephemeral_signer_bumps
        .iter()
        .enumerate()
        .map(|(index, bump)| {
            let seeds: Vec<Vec<u8>> = vec![
                SEED_PREFIX.to_vec().into(),
                transaction_key.to_bytes().to_vec().into(),
                SEED_EPHEMERAL_SIGNER.to_vec().into(),
                u8::try_from(index).unwrap().to_le_bytes().to_vec().into(),
                vec![*bump].into(),
            ]
            .into();

            (
                Pubkey::create_program_address(
                    seeds
                        .iter()
                        .map(Vec::as_slice)
                        .collect::<Vec<&[u8]>>()
                        .as_slice(),
                    &crate::id(),
                )
                .unwrap(),
                seeds,
            )
        })
        .collect::<Vec<(Pubkey, Vec<Vec<u8>>)>>();

    let mut keys = Vec::new();
    let mut bytes = Vec::new();
    for (k, b) in signers.iter().cloned() {
        keys.push(k);
        bytes.push(b);
    }
    (keys, bytes)
}
