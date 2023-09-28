#![allow(clippy::result_large_err)]
#![deny(arithmetic_overflow)]
#![deny(unused_must_use)]
// #![deny(clippy::arithmetic_side_effects)]
// #![deny(clippy::integer_arithmetic)]

extern crate core;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

pub mod errors;
pub mod instructions;
pub mod state;
mod utils;

declare_id!("SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf");

#[program]
pub mod multisig {
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
    )]
    pub fn multisig_create(ctx: Context<MultisigCreate>, args: MultisigCreateArgs) -> Result<()> {
        MultisigCreate::multisig_create(ctx, args)
    }

    /// Add a new member to the multisig.
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

    /// Remove a member/key from the multisig.
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

    /// Set the `time_lock` config parameter for the multisig.
    #[succeeds_if(
        ctx.accounts.config_authority.key() == ctx.accounts.multisig.config_authority
    )]
    pub fn multisig_set_time_lock(
        ctx: Context<MultisigConfig>,
        args: MultisigSetTimeLockArgs,
    ) -> Result<()> {
        MultisigConfig::multisig_set_time_lock(ctx, args)
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

    /// Create a new config transaction.
    #[succeeds_if(
        !args.actions.is_empty()
        && ctx.accounts.multisig.config_authority == Pubkey::default()
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.creator.key(), Permission::Initiate)
        && ctx.accounts.multisig.transaction_index < u64::MAX
    )]
    pub fn config_transaction_create(
        ctx: Context<ConfigTransactionCreate>,
        args: ConfigTransactionCreateArgs,
    ) -> Result<()> {
        kani::assume(ctx.accounts.multisig.transaction_index < u64::MAX);
        ConfigTransactionCreate::config_transaction_create(ctx, args)
    }

    /// Used in `config_transaction_execute` function's verification
    pub fn confix_tx_execute_validation_helper<'info>(
        ctx: &Context<'_, '_, '_, 'info, ConfigTransactionExecute<'info>>,
    ) -> Result<()> {
        kani::assume(ctx.accounts.transaction.actions.len() <= 3);
        kani::assume(ctx.accounts.multisig.members.len() <= 5);
        kani::assume(ctx.remaining_accounts.len() <= 3);
        kani::assume(
            ctx.accounts.multisig.members.len()
                + ctx
                    .accounts
                    .transaction
                    .actions
                    .iter()
                    .filter(|&action| matches!(action, ConfigAction::AddMember { .. }))
                    .count()
                <= 10,
        );

        let mut threshold = ctx.accounts.multisig.threshold;
        let members_after = ctx.accounts.transaction.actions.iter().fold(
            Some(ctx.accounts.multisig.members),
            |acc, action| match acc {
                Some(mut members) => match action {
                    ConfigAction::AddMember { new_member } => {
                        members.push(*new_member);
                        Some(members)
                    }
                    ConfigAction::RemoveMember { old_member } => {
                        if let Some(index) = members.iter().position(|m| m.key == *old_member) {
                            members.remove(index);
                            Some(members)
                        } else {
                            None
                        }
                    }
                    ConfigAction::ChangeThreshold { new_threshold } => {
                        threshold = *new_threshold;
                        Some(members)
                    }
                    ConfigAction::AddSpendingLimit {
                        create_key: _,
                        vault_index: _,
                        mint: _,
                        amount: _,
                        period: _,
                        members: spending_limit_members,
                        destinations: _,
                    } => {
                        kani::assume(spending_limit_members.len() <= 2);
                        if !spending_limit_members.is_empty()
                            && !spending_limit_members
                                .windows(2)
                                .any(|win| win[0] == win[1])
                            && ctx.accounts.system_program.is_some()
                            && ctx.accounts.rent_payer.is_some()
                        {
                            Some(members)
                        } else {
                            None
                        }
                    }
                    ConfigAction::RemoveSpendingLimit {
                        spending_limit: spending_limit_key,
                    } => {
                        let spending_limit_account = ctx
                            .remaining_accounts
                            .iter()
                            .find(|acc| acc.key == spending_limit_key);
                        if let Some(account) = spending_limit_account {
                            if ctx.accounts.rent_payer.is_some()
                                && Account::<SpendingLimit>::try_from(account)
                                    .map_or(false, |acc| {
                                        acc.multisig == ctx.accounts.multisig.key()
                                    })
                            {
                                Some(members)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    _ => Some(members),
                },
                None => None,
            },
        );

        let are_members_after_ok = match members_after {
            Some(valid_members) => {
                !valid_members.windows(2).any(|win| win[0].key == win[1].key)
                    && valid_members.len() > 0
                    && valid_members.len() <= usize::from(u16::MAX)
                    && valid_members
                        .iter()
                        .any(|m| m.permissions.has(Permission::Execute))
                    && valid_members
                        .iter()
                        .any(|m| m.permissions.has(Permission::Initiate))
                    && valid_members
                        .iter()
                        .filter(|m| m.permissions.has(Permission::Vote))
                        .count()
                        >= threshold as usize
                    && valid_members.iter().all(|m| m.permissions.mask < 8)
                    && if valid_members.len() > ctx.accounts.multisig.members.len() {
                        ctx.accounts.system_program.is_some() && ctx.accounts.rent_payer.is_some()
                    } else {
                        true
                    }
                    && threshold > 0
            }
            None => false,
        };

        if are_members_after_ok {
            Ok(())
        } else {
            Err(Error::AccountDidNotSerialize)
        }
    }

    /// Execute a config transaction.
    /// The transaction must be `Approved`.
    #[succeeds_if(
        ctx.accounts.multisig.key() == ctx.accounts.proposal.multisig
        && ctx.accounts.multisig.key() == ctx.accounts.transaction.multisig
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(), Permission::Execute)
        && ctx.accounts.proposal.transaction_index > ctx.accounts.multisig.stale_transaction_index
        && matches!(ctx.accounts.proposal.status, ProposalStatus::Approved { .. })
        && multisig::confix_tx_execute_validation_helper(&ctx).is_ok()
    )]
    pub fn config_transaction_execute<'info>(
        ctx: Context<'_, '_, '_, 'info, ConfigTransactionExecute<'info>>,
    ) -> Result<()> {
        ConfigTransactionExecute::config_transaction_execute(ctx)
    }

    /// Create a new vault transaction.
    #[succeeds_if(
        ctx.accounts.multisig.member_has_permission(ctx.accounts.creator.key(), Permission::Initiate)
    )]
    pub fn vault_transaction_create(
        ctx: Context<VaultTransactionCreate>,
        args: VaultTransactionCreateArgs,
    ) -> Result<()> {
        kani::assume(ctx.accounts.multisig.transaction_index < u64::MAX - 1);
        kani::assume(args.ephemeral_signers <= 10);
        VaultTransactionCreate::vault_transaction_create(ctx, args)
    }

    /// Execute a vault transaction.
    /// The transaction must be `Approved`.
    #[succeeds_if(
        ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(), Permission::Execute)
        && matches!(ctx.accounts.proposal.status, ProposalStatus::Approved { .. })
        && ctx.accounts.multisig.key() == ctx.accounts.proposal.multisig
        && ctx.accounts.multisig.key() == ctx.accounts.transaction.multisig
        && ctx.remaining_accounts.len() == 
            ctx.accounts.transaction.message.address_table_lookups.len() + ctx.accounts.transaction.message.num_all_account_keys()        
    )]
    pub fn vault_transaction_execute(ctx: Context<VaultTransactionExecute>) -> Result<()> {
        kani::assume(ctx.accounts.transaction.ephemeral_signer_bumps.len() <= 10);
        kani::assume(ctx.remaining_accounts.len() <= 10);
        VaultTransactionExecute::vault_transaction_execute(ctx)
    }

    /// Create a new batch.
    #[succeeds_if(
        ctx.accounts.multisig.member_has_permission(ctx.accounts.creator.key(), Permission::Initiate)
    )]
    pub fn batch_create(ctx: Context<BatchCreate>, args: BatchCreateArgs) -> Result<()> {
        kani::assume(ctx.accounts.multisig.transaction_index < u64::MAX - 1);
        BatchCreate::batch_create(ctx, args)
    }

    /// Add a transaction to the batch.
    #[succeeds_if(
        ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(),  Permission::Initiate)
        && matches!(ctx.accounts.proposal.status, ProposalStatus::Draft { .. })
        && ctx.accounts.batch.size >= ctx.accounts.batch.executed_transaction_index
     )]
    pub fn batch_add_transaction(
        ctx: Context<BatchAddTransaction>,
        args: BatchAddTransactionArgs,
    ) -> Result<()> {
        kani::assume(ctx.accounts.batch.size < u32::MAX);
        kani::assume(args.ephemeral_signers <= 10);
        BatchAddTransaction::batch_add_transaction(ctx, args)
    }

    /// Execute a transaction from the batch.
    #[succeeds_if(
        ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(), Permission::Execute)
        && matches!(ctx.accounts.proposal.status, ProposalStatus::Approved { .. })
        && ctx.accounts.multisig.key() == ctx.accounts.proposal.multisig
        && ctx.accounts.multisig.key() == ctx.accounts.batch.multisig
        && ctx.remaining_accounts.len() == 
            ctx.accounts.transaction.message.address_table_lookups.len() + ctx.accounts.transaction.message.num_all_account_keys()
        && ctx.accounts.batch.executed_transaction_index < ctx.accounts.batch.size
    )]
    pub fn batch_execute_transaction(ctx: Context<BatchExecuteTransaction>) -> Result<()> {
        kani::assume(ctx.accounts.transaction.ephemeral_signer_bumps.len() <= 5);
        kani::assume(ctx.remaining_accounts.len() <= 5);
        kani::assume(ctx.accounts.batch.executed_transaction_index < ctx.accounts.batch.size);
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
