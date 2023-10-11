#![allow(clippy::result_large_err)]
#![deny(arithmetic_overflow)]
#![deny(unused_must_use)]
// #![deny(clippy::arithmetic_side_effects)]
// #![deny(clippy::integer_arithmetic)]

extern crate core;

use anchor_lang::prelude::*;

// Re-export anchor_lang for convenience.
pub use anchor_lang;

pub use instructions::*;
pub use state::*;
// pub use utils::SmallVec;

pub mod errors;
pub mod instructions;
pub mod state;
mod utils;

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Squads Multisig Program",
    project_url: "https://squads.so",
    contacts: "email:security@sqds.io,email:contact@osec.io",
    policy: "https://github.com/Squads-Protocol/v4/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/squads-protocol/v4",
    auditors: "OtterSec, Neodyme"
}

declare_id!("SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf");

#[program]
pub mod squads_multisig_program {
    use super::*;

    /// Create a multisig.
    #[succeeds_if(
        args.members.len() <= usize::from(u16::MAX)
        && args.members.windows(2).all(|win| win[0].key != win[1].key)
        && args.members.iter().all(|m| m.permissions.mask < 8)
        && args.members.iter().any(|m| m.permissions.has(Permission::Initiate))
        && args.members.iter().any(|m| m.permissions.has(Permission::Execute))
        && args.threshold > 0
        && args.threshold as usize <= args.members.iter().filter(|m| m.permissions.has(Permission::Vote)).count()
        && args.time_lock <= MAX_TIME_LOCK
    )]
    pub fn multisig_create(ctx: Context<MultisigCreate>, args: MultisigCreateArgs) -> Result<()> {
        MultisigCreate::multisig_create(ctx, args)
    }

    /// Add a new member to the controlled multisig.
    #[succeeds_if(
        ctx.accounts.multisig.members.len() <= usize::from(u16::MAX-1)
        && ctx.accounts.multisig.members.iter().all(|m| m.key != args.new_member.key)
        && ctx.accounts.system_program.is_some()
        && ctx.accounts.rent_payer.is_some()
        && ctx.accounts.config_authority.key() == ctx.accounts.multisig.config_authority
        && args.new_member.permissions.mask < 8
    )]
    pub fn multisig_add_member(
        ctx: Context<MultisigConfig>,
        args: MultisigAddMemberArgs,
    ) -> Result<()> {
        MultisigConfig::multisig_add_member(ctx, args)
    }

    /// Remove a member/key from the controlled multisig.
    #[succeeds_if(
        ctx.accounts.multisig.members.len() > 1
        && ctx.accounts.multisig.members.iter().any(|m| m.key == args.old_member)
        && ctx.accounts.multisig.members.iter().any(|m| m.key != args.old_member && m.permissions.has(Permission::Execute))
        && ctx.accounts.multisig.members.iter().any(|m| m.key != args.old_member && m.permissions.has(Permission::Initiate))
        && ctx.accounts.multisig.members.iter().filter(|m| m.key != args.old_member && m.permissions.has(Permission::Vote)).count() 
            >= ctx.accounts.multisig.threshold as usize
        && ctx.accounts.config_authority.key() == ctx.accounts.multisig.config_authority
        && ctx.accounts.multisig.is_member(args.old_member).is_some()
        && ctx.accounts.multisig.members.windows(3).all(|win| win[0].key != win[1].key && win[0].key != win[2].key)
    )]
    pub fn multisig_remove_member(
        ctx: Context<MultisigConfig>,
        args: MultisigRemoveMemberArgs,
    ) -> Result<()> {
        MultisigConfig::multisig_remove_member(ctx, args)
    }

    /// Set the `time_lock` config parameter for the controlled multisig.
    #[succeeds_if(
        ctx.accounts.config_authority.key() == ctx.accounts.multisig.config_authority
        && args.time_lock <= MAX_TIME_LOCK
    )]
    pub fn multisig_set_time_lock(
        ctx: Context<MultisigConfig>,
        args: MultisigSetTimeLockArgs,
    ) -> Result<()> {
        MultisigConfig::multisig_set_time_lock(ctx, args)
    }

    /// Set the `threshold` config parameter for the controlled multisig.
    #[succeeds_if(
        ctx.accounts.config_authority.key() == ctx.accounts.multisig.config_authority
        && args.new_threshold > 0
        && args.new_threshold as usize <= ctx.accounts.multisig.members.iter().filter(|m| m.permissions.has(Permission::Vote)).count()
    )]
    pub fn multisig_change_threshold(
        ctx: Context<MultisigConfig>,
        args: MultisigChangeThresholdArgs,
    ) -> Result<()> {
        MultisigConfig::multisig_change_threshold(ctx, args)
    }

    /// Set the multisig `config_authority`.
    #[succeeds_if(
        ctx.accounts.config_authority.key() == ctx.accounts.multisig.config_authority
    )]
    pub fn multisig_set_config_authority(
        ctx: Context<MultisigConfig>,
        args: MultisigSetConfigAuthorityArgs,
    ) -> Result<()> {
        MultisigConfig::multisig_set_config_authority(ctx, args)
    }

    /// Create a new spending limit for the controlled multisig.
    pub fn multisig_add_spending_limit(
        ctx: Context<MultisigAddSpendingLimit>,
        args: MultisigAddSpendingLimitArgs,
    ) -> Result<()> {
        MultisigAddSpendingLimit::multisig_add_spending_limit(ctx, args)
    }

    /// Remove the spending limit from the controlled multisig.
    pub fn multisig_remove_spending_limit(
        ctx: Context<MultisigRemoveSpendingLimit>,
        args: MultisigRemoveSpendingLimitArgs,
    ) -> Result<()> {
        MultisigRemoveSpendingLimit::multisig_remove_spending_limit(ctx, args)
    }

    /// Create a new config transaction.
    pub fn config_transaction_create(
        ctx: Context<ConfigTransactionCreate>,
        args: ConfigTransactionCreateArgs,
    ) -> Result<()> {
        ConfigTransactionCreate::config_transaction_create(ctx, args)
    }

    /// Execute a config transaction.
    /// The transaction must be `Approved`.
    pub fn config_transaction_execute<'info>(
        ctx: Context<'_, '_, '_, 'info, ConfigTransactionExecute<'info>>,
    ) -> Result<()> {
        ConfigTransactionExecute::config_transaction_execute(ctx)
    }

    /// Create a new vault transaction.
    pub fn vault_transaction_create(
        ctx: Context<VaultTransactionCreate>,
        args: VaultTransactionCreateArgs,
    ) -> Result<()> {
        VaultTransactionCreate::vault_transaction_create(ctx, args)
    }

    /// Execute a vault transaction.
    /// The transaction must be `Approved`.
    pub fn vault_transaction_execute(ctx: Context<VaultTransactionExecute>) -> Result<()> {
        VaultTransactionExecute::vault_transaction_execute(ctx)
    }

    /// Create a new batch.
    pub fn batch_create(ctx: Context<BatchCreate>, args: BatchCreateArgs) -> Result<()> {
        BatchCreate::batch_create(ctx, args)
    }

    /// Add a transaction to the batch.
    pub fn batch_add_transaction(
        ctx: Context<BatchAddTransaction>,
        args: BatchAddTransactionArgs,
    ) -> Result<()> {
        BatchAddTransaction::batch_add_transaction(ctx, args)
    }

    /// Execute a transaction from the batch.
    pub fn batch_execute_transaction(ctx: Context<BatchExecuteTransaction>) -> Result<()> {
        BatchExecuteTransaction::batch_execute_transaction(ctx)
    }

    /// Create a new multisig proposal.
    #[succeeds_if(
        args.transaction_index <= ctx.accounts.multisig.transaction_index
        && args.transaction_index > ctx.accounts.multisig.stale_transaction_index
        && (
            ctx.accounts.multisig.member_has_permission(ctx.accounts.creator.key(), Permission::Initiate)
                || ctx.accounts.multisig.member_has_permission(ctx.accounts.creator.key(), Permission::Vote)
        )
    )]
    pub fn proposal_create(ctx: Context<ProposalCreate>, args: ProposalCreateArgs) -> Result<()> {
        ProposalCreate::proposal_create(ctx, args)
    }

    /// Update status of a multisig proposal from `Draft` to `Active`.
    #[succeeds_if(
        ctx.accounts.proposal.transaction_index > ctx.accounts.multisig.stale_transaction_index
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(), Permission::Initiate)
        && matches!(ctx.accounts.proposal.status, ProposalStatus::Draft { .. })
        && ctx.accounts.proposal.multisig == ctx.accounts.multisig.key()
    )]
    pub fn proposal_activate(ctx: Context<ProposalActivate>) -> Result<()> {
        ProposalActivate::proposal_activate(ctx)
    }

    /// Approve a multisig proposal on behalf of the `member`.
    /// The proposal must be `Active`.
    #[succeeds_if(
        ctx.accounts.proposal.transaction_index > ctx.accounts.multisig.stale_transaction_index
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(), Permission::Vote)
        && matches!(ctx.accounts.proposal.status, ProposalStatus::Active { .. })
        && ctx.accounts.proposal.multisig == ctx.accounts.multisig.key()
        && !ctx.accounts.proposal.approved.contains(&ctx.accounts.member.key())
    )]
    pub fn proposal_approve(ctx: Context<ProposalVote>, args: ProposalVoteArgs) -> Result<()> {
        ProposalVote::proposal_approve(ctx, args)
    }

    /// Reject a multisig proposal on behalf of the `member`.
    /// The proposal must be `Active`.
    #[succeeds_if(
        ctx.accounts.proposal.transaction_index > ctx.accounts.multisig.stale_transaction_index
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(), Permission::Vote)
        && matches!(ctx.accounts.proposal.status, ProposalStatus::Active { .. })
        && ctx.accounts.proposal.multisig == ctx.accounts.multisig.key()
        && !ctx.accounts.proposal.rejected.contains(&ctx.accounts.member.key())
    )]
    pub fn proposal_reject(ctx: Context<ProposalVote>, args: ProposalVoteArgs) -> Result<()> {
        ProposalVote::proposal_reject(ctx, args)
    }

    /// Cancel a multisig proposal on behalf of the `member`.
    /// The proposal must be `Approved`.
    #[succeeds_if(
        ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(), Permission::Vote)
        && matches!(ctx.accounts.proposal.status, ProposalStatus::Approved { .. })
        && ctx.accounts.proposal.multisig == ctx.accounts.multisig.key()
        && !ctx.accounts.proposal.cancelled.contains(&ctx.accounts.member.key())
    )]
    pub fn proposal_cancel(ctx: Context<ProposalVote>, args: ProposalVoteArgs) -> Result<()> {
        ProposalVote::proposal_cancel(ctx, args)
    }
    /// Use a spending limit to transfer tokens from a multisig vault to a destination account.
    #[succeeds_if(
        ctx.accounts.multisig.is_member(ctx.accounts.member.key()).is_some()
        && ctx.accounts.spending_limit.members.contains(&ctx.accounts.member.key())
        && ctx.accounts.spending_limit.multisig == ctx.accounts.multisig.key()
        && args.amount <= ctx.accounts.spending_limit.amount
        && (
            ctx.accounts.spending_limit.destinations.is_empty()
            || ctx.accounts.spending_limit.destinations.contains(&ctx.accounts.destination.key())
        )
        && (
            if ctx.accounts.spending_limit.mint == Pubkey::default() {
                ctx.accounts.mint.is_none()
                && ctx.accounts.system_program.is_some()
                && args.decimals == 9
                && ctx.accounts.vault.lamports() >= args.amount
            } else {
                ctx.accounts.mint.is_some()
                && ctx.accounts.spending_limit.mint == ctx.accounts.mint.as_ref().unwrap().key()
                && ctx.accounts.vault_token_account.is_some()
                && ctx.accounts.destination_token_account.is_some()
                && ctx.accounts.token_program.is_some()
            }
        )
    )]
    pub fn spending_limit_use(
        ctx: Context<SpendingLimitUse>,
        args: SpendingLimitUseArgs,
    ) -> Result<()> {
        kani::assume(ctx.accounts.spending_limit.remaining_amount >= args.amount);
        SpendingLimitUse::spending_limit_use(ctx, args)
    }
}
