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
        && args.members.iter().filter(|m| m.permissions.has(Permission::Initiate)).count() > 0
        && args.members.iter().filter(|m| m.permissions.has(Permission::Execute)).count() > 0
        && args.members.iter().filter(|m| m.permissions.has(Permission::Vote)).count() > 0
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
        && if ctx.accounts.multisig.members.iter().any(|m| m.key == args.old_member && m.permissions.has(Permission::Execute)) {
            ctx.accounts.multisig.members.iter().filter(|m| m.permissions.has(Permission::Execute)).count() > 1
        } else {
            true
        }
        && if ctx.accounts.multisig.members.iter().any(|m| m.key == args.old_member && m.permissions.has(Permission::Initiate)) {
            ctx.accounts.multisig.members.iter().filter(|m| m.permissions.has(Permission::Initiate)).count() > 1
        } else {
            true
        }
        && if ctx.accounts.multisig.members.iter().any(|m| m.key == args.old_member && m.permissions.has(Permission::Vote)) {
            ctx.accounts.multisig.members.iter().filter(|m| m.permissions.has(Permission::Vote)).count() > ctx.accounts.multisig.threshold as usize
            && ctx.accounts.multisig.threshold > 1
        } else {
            true
        }
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
    ) -> bool {
        kani::assume(ctx.accounts.multisig.members.len() <= 10);
        kani::assume(ctx.accounts.transaction.actions.len() <= 3);
        // let old_threshold = ctx.accounts.multisig.threshold;
        // let old_members = ctx.accounts.multisig.members.clone();
        let mut multisig = ctx.accounts.multisig.clone();
        ctx.accounts
            .transaction
            .actions
            .iter()
            .for_each(|action| match action {
                ConfigAction::AddMember { new_member } => {
                    multisig.add_member(*new_member);
                }
                ConfigAction::RemoveMember { old_member } => {
                    let _ = multisig.remove_member(*old_member);
                }
                ConfigAction::ChangeThreshold { new_threshold } => {
                    multisig.threshold = *new_threshold;
                }
                _ => {}
            });

        !multisig
            .members
            .windows(2)
            .any(|win| win[0].key == win[1].key)
            && !multisig.members.is_empty()
            && multisig.members.len() <= usize::from(u16::MAX)
            && multisig
                .members
                .iter()
                .any(|m| m.permissions.has(Permission::Execute))
            && multisig
                .members
                .iter()
                .any(|m| m.permissions.has(Permission::Initiate))
            && multisig
                .members
                .iter()
                .filter(|m| m.permissions.has(Permission::Vote))
                .count()
                >= multisig.threshold as usize
            && multisig.members.iter().all(|m| m.permissions.mask < 8)
            && if multisig.members.len() > ctx.accounts.multisig.members.len() {
                ctx.accounts.system_program.is_some() && ctx.accounts.rent_payer.is_some()
            } else {
                true
            }
            && multisig.threshold > 0
    }

    /// Execute a config transaction.
    /// The transaction must be `Approved`.
    #[succeeds_if(
        ctx.accounts.multisig.key() == ctx.accounts.proposal.multisig
        && ctx.accounts.multisig.key() == ctx.accounts.transaction.multisig
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(), Permission::Execute)
        && ctx.accounts.proposal.transaction_index > ctx.accounts.multisig.stale_transaction_index
        && matches!(ctx.accounts.proposal.status, ProposalStatus::Approved { .. })
        && multisig::confix_tx_execute_validation_helper(&ctx)
        && ctx.accounts.transaction.actions.iter().fold(true, |acc, action| match action {
            ConfigAction::AddSpendingLimit {
                create_key: _,
                vault_index: _,
                mint: _,
                amount: _,
                period: _,
                members,
                destinations: _,
            } => !members.is_empty() && !members.windows(2).any(|win| win[0] == win[1]) && ctx.accounts.system_program.is_some() && ctx.accounts.rent_payer.is_some(),
            ConfigAction::RemoveSpendingLimit {
                spending_limit: spending_limit_key,
            } => {
                acc
                && ctx.remaining_accounts.iter().any(|acc| acc.key == spending_limit_key)
                && ctx.accounts.rent_payer.is_some()
                && Account::<SpendingLimit>::try_from(ctx.remaining_accounts.iter().find(|acc| acc.key == spending_limit_key).as_ref().unwrap()).is_ok()
            },
            _ => acc,
        })
    )]
    pub fn config_transaction_execute<'info>(
        ctx: Context<'_, '_, '_, 'info, ConfigTransactionExecute<'info>>,
    ) -> Result<()> {
        kani::assume(ctx.accounts.transaction.actions.len() <= 3);
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
    pub fn spending_limit_use(
        ctx: Context<SpendingLimitUse>,
        args: SpendingLimitUseArgs,
    ) -> Result<()> {
        SpendingLimitUse::spending_limit_use(ctx, args)
    }
}
