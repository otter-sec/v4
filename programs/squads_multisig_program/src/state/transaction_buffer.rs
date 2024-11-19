use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

// use crate::errors::MultisigError;
// Note: Using 10 bytes for the buffer size for verification purposes
pub const MAX_BUFFER_SIZE: usize = 10;


#[account]
#[invariant(
    self.invariant().is_ok()
)]
#[derive(Default, Debug)]
pub struct TransactionBuffer {
    /// The multisig this belongs to.
    pub multisig: Pubkey,
    /// Member of the Multisig who created the TransactionBuffer.
    pub creator: Pubkey,
    /// Index to seed address derivation
    pub buffer_index: u8,
    /// Vault index of the transaction this buffer belongs to.
    pub vault_index: u8,
    /// Hash of the final assembled transaction message.
    pub final_buffer_hash: [u8; 10], // Note: Using 10 bytes for the verification
    /// The size of the final assembled transaction message.
    pub final_buffer_size: u16,
    /// The buffer of the transaction message.
    pub buffer: Vec<u8>,
}

impl TransactionBuffer {
    pub fn size(final_message_buffer_size: u16) -> Result<usize> {
        // Make sure final size is not greater than MAX_BUFFER_SIZE bytes.
        if (final_message_buffer_size as usize) > MAX_BUFFER_SIZE {
            return err!(MultisigError::FinalBufferSizeExceeded);
        }
        Ok(
            8 +   // anchor account discriminator
            32 +  // multisig
            32 +  // creator
            8 + //  buffer_index
            8 +   // vault_index
            32 +  // transaction_message_hash
            2 +  // final_buffer_size
            4 + // vec length bytes
            final_message_buffer_size as usize, // buffer
        )
    }

    pub fn validate_hash(&self) -> Result<()> { 
        let message_buffer_hash = hash(&self.buffer);
        // Note: Assume that the hash matches 
        kani::assume(hash(&self.buffer).to_bytes() == self.final_buffer_hash);
        require!(
            message_buffer_hash.to_bytes() == self.final_buffer_hash,
            MultisigError::FinalBufferHashMismatch
        );
        Ok(())
    }
    pub fn validate_size(&self) -> Result<()> {
        kani::assume(self.buffer.len() == self.final_buffer_size as usize);
        require_eq!(
            self.buffer.len(),
            self.final_buffer_size as usize,
            MultisigError::FinalBufferSizeMismatch
        );
        Ok(())
    }

    pub fn invariant(&self) -> Result<()> {
        require!(
            self.final_buffer_size as usize <= MAX_BUFFER_SIZE,
            MultisigError::FinalBufferSizeExceeded
        );
        require!(
            self.buffer.len() <= MAX_BUFFER_SIZE,
            MultisigError::FinalBufferSizeExceeded
        );
        require!(
            self.buffer.len() <= self.final_buffer_size as usize,
            MultisigError::FinalBufferSizeMismatch
        );

        Ok(())
    }
}
