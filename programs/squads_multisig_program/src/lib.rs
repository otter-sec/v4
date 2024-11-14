#![allow(clippy::result_large_err)]
#![deny(arithmetic_overflow)]
#![deny(unused_must_use)]
// #![deny(clippy::arithmetic_side_effects)]
// #![deny(clippy::integer_arithmetic)]

// Re-export anchor_lang for convenience.
pub use anchor_lang;
use anchor_lang::prelude::*;
// #[cfg(not(feature = "no-entrypoint"))]
// use solana_security_txt::security_txt;

pub use instructions::ProgramConfig;
pub use instructions::*;
pub use state::*;
pub use utils::SmallVec;

pub mod allocator;
pub mod errors;
pub mod instructions;
pub mod state;
mod utils;

// #[cfg(not(feature = "no-entrypoint"))]
// security_txt! {
//     name: "Squads Multisig Program",
//     project_url: "https://squads.so",
//     contacts: "email:security@sqds.io,email:contact@osec.io",
//     policy: "https://github.com/Squads-Protocol/v4/blob/main/SECURITY.md",
//     preferred_languages: "en",
//     source_code: "https://github.com/squads-protocol/v4",
//     auditors: "OtterSec, Neodyme"
// }

// #[cfg(not(feature = "testing"))]
declare_id!("SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf");

// #[cfg(feature = "testing")]
// declare_id!("GyhGAqjokLwF9UXdQ2dR5Zwiup242j4mX4J1tSMKyAmD");

#[program]
pub mod squads_multisig_program {
    use errors::MultisigError;

    use super::*;

    #[succeeds_if(
        args.authority != Pubkey::default()
        && args.treasury != Pubkey::default()
    )]
    /// Initialize the program config.
    pub fn program_config_init(
        ctx: Context<ProgramConfigInit>,
        args: ProgramConfigInitArgs,
    ) -> Result<()> {
        ProgramConfigInit::program_config_init(ctx, args)
    }

    #[succeeds_if(
        args.new_authority != Pubkey::default()
        && ctx.accounts.program_config.treasury != Pubkey::default()
        && ctx.accounts.program_config.authority == ctx.accounts.authority.key()
    )]
    /// Set the `authority` parameter of the program config.
    pub fn program_config_set_authority(
        ctx: Context<ProgramConfig>,
        args: ProgramConfigSetAuthorityArgs,
    ) -> Result<()> {
        ProgramConfig::program_config_set_authority(ctx, args)
    }

    #[succeeds_if(
        ctx.accounts.program_config.authority != Pubkey::default()
        && ctx.accounts.program_config.treasury != Pubkey::default()
        && ctx.accounts.program_config.authority == ctx.accounts.authority.key()
    )]
    /// Set the `multisig_creation_fee` parameter of the program config.
    pub fn program_config_set_multisig_creation_fee(
        ctx: Context<ProgramConfig>,
        args: ProgramConfigSetMultisigCreationFeeArgs,
    ) -> Result<()> {
        ProgramConfig::program_config_set_multisig_creation_fee(ctx, args)
    }

    #[succeeds_if(
        args.new_treasury != Pubkey::default()
        && ctx.accounts.program_config.authority != Pubkey::default()
        && ctx.accounts.program_config.authority == ctx.accounts.authority.key()
    )]
    /// Set the `treasury` parameter of the program config.
    pub fn program_config_set_treasury(
        ctx: Context<ProgramConfig>,
        args: ProgramConfigSetTreasuryArgs,
    ) -> Result<()> {
        ProgramConfig::program_config_set_treasury(ctx, args)
    }

    /// Create a multisig.
    pub fn multisig_create(_ctx: Context<Deprecated>) -> Result<()> {
        msg!("multisig_create has been deprecated. Use multisig_create_v2 instead.");
        Err(MultisigError::MultisigCreateDeprecated.into())
    }

    /// Create a multisig.
    #[succeeds_if(
        args.members.len() <= u16::MAX as usize
        && args.members.windows(2).all(|win| win[0].key != win[1].key)
        && args.members.iter().all(|m| m.permissions.mask < 8)
        && args.members.iter().any(|m| m.permissions.has(Permission::Initiate))
        && args.members.iter().any(|m| m.permissions.has(Permission::Execute))
        && args.threshold > 0
        && args.threshold as usize <= args.members.iter().filter(|m| m.permissions.has(Permission::Vote)).count()
        && args.time_lock <= MAX_TIME_LOCK
        && ctx.accounts.treasury.key() == ctx.accounts.program_config.treasury
    )]
    pub fn multisig_create_v2(
        ctx: Context<MultisigCreateV2>,
        args: MultisigCreateArgsV2,
    ) -> Result<()> {
        MultisigCreateV2::multisig_create(ctx, args)
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

    #[succeeds_if(
        ctx.accounts.config_authority.key() == ctx.accounts.multisig.config_authority
        && args.time_lock > 0
        && args.time_lock <= MAX_TIME_LOCK
    )]
    /// Set the `time_lock` config parameter for the controlled multisig.
    pub fn multisig_set_time_lock(
        ctx: Context<MultisigConfig>,
        args: MultisigSetTimeLockArgs,
    ) -> Result<()> {
        MultisigConfig::multisig_set_time_lock(ctx, args)
    }

    #[succeeds_if(
        ctx.accounts.config_authority.key() == ctx.accounts.multisig.config_authority
        && args.new_threshold > 0
        && args.new_threshold as usize <= ctx.accounts.multisig.members.iter().filter(|m| m.permissions.has(Permission::Vote)).count()
    )]
    /// Set the `threshold` config parameter for the controlled multisig.
    pub fn multisig_change_threshold(
        ctx: Context<MultisigConfig>,
        args: MultisigChangeThresholdArgs,
    ) -> Result<()> {
        MultisigConfig::multisig_change_threshold(ctx, args)
    }

    #[succeeds_if(
        ctx.accounts.config_authority.key() == ctx.accounts.multisig.config_authority
    )]
    /// Set the multisig `config_authority`.
    pub fn multisig_set_config_authority(
        ctx: Context<MultisigConfig>,
        args: MultisigSetConfigAuthorityArgs,
    ) -> Result<()> {
        MultisigConfig::multisig_set_config_authority(ctx, args)
    }

    /// Set the multisig `rent_collector`.
    #[succeeds_if(
        ctx.accounts.config_authority.key() == ctx.accounts.multisig.config_authority
    )]
    pub fn multisig_set_rent_collector(
        ctx: Context<MultisigConfig>,
        args: MultisigSetRentCollectorArgs,
    ) -> Result<()> {
        MultisigConfig::multisig_set_rent_collector(ctx, args)
    }

    /// Create a new spending limit for the controlled multisig.
    #[succeeds_if(
        args.amount != 0
        && args.members.len() > 0
        && args.members.windows(2).all(|win| win[0] != win[1])
        && ctx.accounts.config_authority.key() == ctx.accounts.multisig.config_authority
    )]
    pub fn multisig_add_spending_limit(
        ctx: Context<MultisigAddSpendingLimit>,
        args: MultisigAddSpendingLimitArgs,
    ) -> Result<()> {
        MultisigAddSpendingLimit::multisig_add_spending_limit(ctx, args)
    }

    /// Remove the spending limit from the controlled multisig.
    #[succeeds_if(
        ctx.accounts.config_authority.key() == ctx.accounts.multisig.config_authority
        && ctx.accounts.spending_limit.multisig == ctx.accounts.multisig.key()
    )]
    pub fn multisig_remove_spending_limit(
        ctx: Context<MultisigRemoveSpendingLimit>,
        args: MultisigRemoveSpendingLimitArgs,
    ) -> Result<()> {
        MultisigRemoveSpendingLimit::multisig_remove_spending_limit(ctx, args)
    }

    /// Create a new config transaction.
    #[succeeds_if(
        ctx.accounts.multisig.config_authority == Pubkey::default()
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.creator.key(), Permission::Initiate)
        && !args.actions.is_empty()
        && args.actions.iter().all(|action| {
            if let ConfigAction::SetTimeLock { new_time_lock, .. } = action {
                *new_time_lock <= MAX_TIME_LOCK
            } else {
                true
            }
        })
    )]
    pub fn config_transaction_create(
        ctx: Context<ConfigTransactionCreate>,
        args: ConfigTransactionCreateArgs,
    ) -> Result<()> {
        ConfigTransactionCreate::config_transaction_create(ctx, args)
    }

    /// Execute a config transaction.
    /// The transaction must be `Approved`.
    #[succeeds_if(
        ctx.accounts.multisig.is_member(ctx.accounts.member.key()).is_some()
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(), Permission::Execute)
        && matches!(ctx.accounts.proposal.status, ProposalStatus::Approved { .. })
        && ctx.accounts.proposal.transaction_index > ctx.accounts.multisig.stale_transaction_index
        && squads_multisig_program::tx_execute_validation_helper(&ctx).is_ok()
    )]
    pub fn config_transaction_execute<'info>(
        ctx: Context<'_, '_, 'info, 'info, ConfigTransactionExecute<'info>>,
    ) -> Result<()> {
        ConfigTransactionExecute::config_transaction_execute(ctx)
    }

    /// Create a new vault transaction.
    #[succeeds_if(
        ctx.accounts.multisig.is_member(ctx.accounts.creator.key()).is_some()
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.creator.key(), Permission::Initiate)
    )]
    pub fn vault_transaction_create(
        ctx: Context<VaultTransactionCreate>,
        args: VaultTransactionCreateArgs,
    ) -> Result<()> {
        kani::assume(args.ephemeral_signers < 10);
        kani::assume(ctx.accounts.multisig.transaction_index < u64::MAX);
        VaultTransactionCreate::vault_transaction_create(ctx, args)
    }

    /// Create a transaction buffer account.
    #[succeeds_if(
        ctx.accounts.multisig.is_member(ctx.accounts.creator.key()).is_some()
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.creator.key(), Permission::Initiate)
        && args.final_buffer_size as usize <= MAX_BUFFER_SIZE
        && args.buffer.len() <= MAX_BUFFER_SIZE
        && args.buffer.len() <= args.final_buffer_size as usize
    )]
    pub fn transaction_buffer_create(
        ctx: Context<TransactionBufferCreate>,
        args: TransactionBufferCreateArgs,
    ) -> Result<()> {
        TransactionBufferCreate::transaction_buffer_create(ctx, args)
    }

    /// Close a transaction buffer account.
    #[succeeds_if(
        ctx.accounts.multisig.is_member(ctx.accounts.creator.key()).is_some()
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.creator.key(), Permission::Initiate)
        && ctx.accounts.transaction_buffer.creator == ctx.accounts.creator.key()
    )]
    pub fn transaction_buffer_close(ctx: Context<TransactionBufferClose>) -> Result<()> {
        TransactionBufferClose::transaction_buffer_close(ctx)
    }

    /// Extend a transaction buffer account.
    #[succeeds_if(
        ctx.accounts.multisig.is_member(ctx.accounts.creator.key()).is_some()
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.creator.key(), Permission::Initiate)
        && ctx.accounts.transaction_buffer.creator == ctx.accounts.creator.key()
        && args.buffer.len() + ctx.accounts.transaction_buffer.buffer.len() <= ctx.accounts.transaction_buffer.final_buffer_size as usize
    )]
    pub fn transaction_buffer_extend(
        ctx: Context<TransactionBufferExtend>,
        args: TransactionBufferExtendArgs,
    ) -> Result<()> {
        TransactionBufferExtend::transaction_buffer_extend(ctx, args)
    }

    /// Create a new vault transaction from a completed transaction buffer.
    /// Finalized buffer hash must match `final_buffer_hash`
    #[succeeds_if(
        args.transaction_message == vec![0, 0, 0, 0, 0, 0].into() 
        && ctx.accounts.transaction_buffer.buffer.len() == ctx.accounts.transaction_buffer.final_buffer_size as usize
        && ctx.accounts.vault_transaction_create.multisig.is_member(ctx.accounts.vault_transaction_create.creator.key()).is_some()
        && ctx.accounts.vault_transaction_create.multisig.member_has_permission(ctx.accounts.vault_transaction_create.creator.key(), Permission::Initiate)
        && ctx.accounts.vault_transaction_create.multisig.invariant().is_ok()
    )]
    pub fn vault_transaction_create_from_buffer<'info>(
        ctx: Context<'_, '_, 'info, 'info, VaultTransactionCreateFromBuffer<'info>>,
        args: VaultTransactionCreateArgs,
    ) -> Result<()> {
        kani::assume(ctx.accounts.transaction_buffer.buffer.len() <= MAX_BUFFER_SIZE);
        kani::assume(ctx.accounts.transaction_buffer.final_buffer_size as usize <= MAX_BUFFER_SIZE);
        VaultTransactionCreateFromBuffer::vault_transaction_create_from_buffer(ctx, args)
    }

    /// Execute a vault transaction.
    /// The transaction must be `Approved`.
    pub fn vault_transaction_execute(ctx: Context<VaultTransactionExecute>) -> Result<()> {
        VaultTransactionExecute::vault_transaction_execute(ctx)
    }

    /// Create a new batch.
    #[succeeds_if(
        ctx.accounts.multisig.is_member(ctx.accounts.creator.key()).is_some()
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.creator.key(), Permission::Initiate)
    )]
    pub fn batch_create(ctx: Context<BatchCreate>, args: BatchCreateArgs) -> Result<()> {
        BatchCreate::batch_create(ctx, args)
    }

    /// Add a transaction to the batch.
    #[succeeds_if(
        ctx.accounts.multisig.is_member(ctx.accounts.member.key()).is_some()
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(), Permission::Initiate)
        && ctx.accounts.batch.creator == ctx.accounts.member.key()
        && matches!(ctx.accounts.proposal.status, ProposalStatus::Draft { .. })
    )]
    pub fn batch_add_transaction(
        ctx: Context<BatchAddTransaction>,
        args: BatchAddTransactionArgs,
    ) -> Result<()> {
        kani::assume(args.ephemeral_signers < 10);
        kani::assume(ctx.accounts.batch.size < u32::MAX);
        BatchAddTransaction::batch_add_transaction(ctx, args)
    }

    /// Execute a transaction from the batch.
    #[succeeds_if(
        ctx.accounts.multisig.is_member(ctx.accounts.member.key()).is_some()
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(), Permission::Execute)
        && matches!(ctx.accounts.proposal.status, ProposalStatus::Approved { .. })
        && ctx.accounts.multisig.key() == ctx.accounts.proposal.multisig
        && ctx.accounts.multisig.key() == ctx.accounts.batch.multisig
        && ctx.accounts.proposal.transaction_index == ctx.accounts.batch.index
        && ctx.remaining_accounts.len() == 
            ctx.accounts.transaction.message.address_table_lookups.len() + ctx.accounts.transaction.message.num_all_account_keys()
        && ctx.accounts.batch.executed_transaction_index < ctx.accounts.batch.size
    )]
    pub fn batch_execute_transaction(ctx: Context<BatchExecuteTransaction>) -> Result<()> {
        kani::assume(ctx.accounts.transaction.ephemeral_signer_bumps.len() <= 3);
        kani::assume(ctx.remaining_accounts.len() <= 3);
        kani::assume(ctx.accounts.batch.executed_transaction_index < ctx.accounts.batch.size);
        BatchExecuteTransaction::batch_execute_transaction(ctx)
    }

    /// Create a new multisig proposal.
    #[succeeds_if(
        ctx.accounts.multisig.is_member(ctx.accounts.creator.key()).is_some()
        && (
            ctx.accounts.multisig.member_has_permission(ctx.accounts.creator.key(), Permission::Initiate)
            || ctx.accounts.multisig.member_has_permission(ctx.accounts.creator.key(), Permission::Vote))
        && args.transaction_index <= ctx.accounts.multisig.transaction_index
        && args.transaction_index > ctx.accounts.multisig.stale_transaction_index
    )]
    pub fn proposal_create(ctx: Context<ProposalCreate>, args: ProposalCreateArgs) -> Result<()> {
        ProposalCreate::proposal_create(ctx, args)
    }

    /// Update status of a multisig proposal from `Draft` to `Active`.
    #[succeeds_if(
        ctx.accounts.multisig.is_member(ctx.accounts.member.key()).is_some()
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(), Permission::Initiate)
        && matches!(ctx.accounts.proposal.status, ProposalStatus::Draft { .. })
        && ctx.accounts.proposal.transaction_index > ctx.accounts.multisig.stale_transaction_index
    )]
    pub fn proposal_activate(ctx: Context<ProposalActivate>) -> Result<()> {
        ProposalActivate::proposal_activate(ctx)
    }

    /// Approve a multisig proposal on behalf of the `member`.
    /// The proposal must be `Active`.
    #[succeeds_if(
        ctx.accounts.multisig.is_member(ctx.accounts.member.key()).is_some()
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(), Permission::Vote)
        && matches!(ctx.accounts.proposal.status, ProposalStatus::Active { .. })
        && ctx.accounts.proposal.transaction_index > ctx.accounts.multisig.stale_transaction_index
        && !ctx.accounts.proposal.approved.contains(&ctx.accounts.member.key())
    )]
    pub fn proposal_approve(ctx: Context<ProposalVote>, args: ProposalVoteArgs) -> Result<()> {
        ProposalVote::proposal_approve(ctx, args)
    }

    /// Reject a multisig proposal on behalf of the `member`.
    /// The proposal must be `Active`.
    #[succeeds_if(
        ctx.accounts.multisig.is_member(ctx.accounts.member.key()).is_some()
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(), Permission::Vote)
        && matches!(ctx.accounts.proposal.status, ProposalStatus::Active { .. })
        && ctx.accounts.proposal.transaction_index > ctx.accounts.multisig.stale_transaction_index
        && !ctx.accounts.proposal.rejected.contains(&ctx.accounts.member.key())
    )]
    pub fn proposal_reject(ctx: Context<ProposalVote>, args: ProposalVoteArgs) -> Result<()> {
        ProposalVote::proposal_reject(ctx, args)
    }

    /// Cancel a multisig proposal on behalf of the `member`.
    /// The proposal must be `Approved`.
    #[succeeds_if(
        ctx.accounts.multisig.is_member(ctx.accounts.member.key()).is_some()
        && ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(), Permission::Vote)
        && matches!(ctx.accounts.proposal.status, ProposalStatus::Approved { .. })
        && !ctx.accounts.proposal.cancelled.contains(&ctx.accounts.member.key())

    )]
    pub fn proposal_cancel(ctx: Context<ProposalVote>, args: ProposalVoteArgs) -> Result<()> {
        ProposalVote::proposal_cancel(ctx, args)
    }

    /// Cancel a multisig proposal on behalf of the `member`.
    /// The proposal must be `Approved`.
    /// This was introduced to incorporate proper state update, as old multisig members
    /// may have lingering votes, and the proposal size may need to be reallocated to
    /// accommodate the new amount of cancel votes.
    /// The previous implemenation still works if the proposal size is in line with the
    /// threshold size.
    #[succeeds_if(
        ctx.accounts.proposal_vote.multisig.is_member(ctx.accounts.proposal_vote.member.key()).is_some()
        && ctx.accounts.proposal_vote.multisig.member_has_permission(ctx.accounts.proposal_vote.member.key(), Permission::Vote)
        && matches!(ctx.accounts.proposal_vote.proposal.status, ProposalStatus::Approved { .. })
        && !ctx.accounts.proposal_vote.proposal.cancelled.contains(&ctx.accounts.proposal_vote.member.key())
    )]
    pub fn proposal_cancel_v2<'info>(
        ctx: Context<'_, '_, 'info, 'info, ProposalCancelV2<'info>>,
        args: ProposalVoteArgs,
    ) -> Result<()> {
        ProposalCancelV2::proposal_cancel_v2(ctx, args)
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

    /// Closes a `ConfigTransaction` and the corresponding `Proposal`.
    /// `transaction` can be closed if either:
    /// - the `proposal` is in a terminal state: `Executed`, `Rejected`, or `Cancelled`.
    /// - the `proposal` is stale.
    #[succeeds_if(
        (
            !ctx.accounts.proposal.data.borrow.is_empty()
            && (Proposal::try_deserialize(
                    &mut &*ctx.accounts.proposal.data.borrow()
                ).is_ok().is_some()
            &&
        (
            let proposal = Proposal::try_deserialize(
                &mut &*ctx.accounts.proposal.data.borrow()
            ).unwrap();
            (   // Stale and execution not started
                ctx.accounts.transaction.index <= ctx.accounts.multisig.stale_transaction_index &&
                (matches!(proposal.status, ProposalStatus::Draft { .. })
                || matches!(proposal.status, ProposalStatus::Active { .. })
                || matches!(proposal.status, ProposalStatus::Approved { .. })
                )
            ) // Terminal state 
            || matches!(proposal.status, ProposalStatus::Executed { .. })            
            || matches!(proposal.status, ProposalStatus::Rejected { .. })
            || matches!(proposal.status, ProposalStatus::Cancelled { .. })
            && !matches!(proposal.status, ProposalStatus::Executing { .. }) // Not executing
        )) ||
        (
            ctx.accounts.proposal.data.borrow().is_empty() && 
            ctx.accounts.transaction.index <= ctx.accounts.multisig.stale_transaction_index 
        ))
    )]
    pub fn config_transaction_accounts_close(
        ctx: Context<ConfigTransactionAccountsClose>,
    ) -> Result<()> {
        ConfigTransactionAccountsClose::config_transaction_accounts_close(ctx)
    }

    /// Closes a `VaultTransaction` and the corresponding `Proposal`.
    /// `transaction` can be closed if either:
    /// - the `proposal` is in a terminal state: `Executed`, `Rejected`, or `Cancelled`.
    /// - the `proposal` is stale and not `Approved`.
    #[succeeds_if(
        (
            !ctx.accounts.proposal.data.borrow.is_empty()
            && (Proposal::try_deserialize(
                    &mut &*ctx.accounts.proposal.data.borrow()
                ).is_ok().is_some()
            &&
        (
            let proposal = Proposal::try_deserialize(
                &mut &*ctx.accounts.proposal.data.borrow()
            ).unwrap();
            (
                ctx.accounts.transaction.index <= ctx.accounts.multisig.stale_transaction_index &&
                (matches!(proposal.status, ProposalStatus::Draft { .. })
                || matches!(proposal.status, ProposalStatus::Active { .. })
                )
            )
            || matches!(proposal.status, ProposalStatus::Executed { .. })            
            || matches!(proposal.status, ProposalStatus::Rejected { .. })
            || matches!(proposal.status, ProposalStatus::Cancelled { .. })
            && !matches!(proposal.status, ProposalStatus::Executing { .. }) 
            && !matches!(proposal.status, ProposalStatus::Approved { .. }) 
        )) ||
        (
            ctx.accounts.proposal.data.borrow().is_empty() && 
            ctx.accounts.transaction.index <= ctx.accounts.multisig.stale_transaction_index 
        ))
    )]
    pub fn vault_transaction_accounts_close(
        ctx: Context<VaultTransactionAccountsClose>,
    ) -> Result<()> {
        VaultTransactionAccountsClose::vault_transaction_accounts_close(ctx)
    }

    /// Closes a `VaultBatchTransaction` belonging to the `batch` and `proposal`.
    /// `transaction` can be closed if either:
    /// - it's marked as executed within the `batch`;
    /// - the `proposal` is in a terminal state: `Executed`, `Rejected`, or `Cancelled`.
    /// - the `proposal` is stale and not `Approved`.
    #[succeeds_if(
        (
            ctx.accounts.proposal.transaction_index <= ctx.accounts.multisig.stale_transaction_index
            && (matches!(ctx.accounts.proposal.status, ProposalStatus::Draft { .. })
            || matches!(ctx.accounts.proposal.status, ProposalStatus::Active { .. }))
        )
        || matches!(ctx.accounts.proposal.status, ProposalStatus::Executed { .. })            
        || matches!(ctx.accounts.proposal.status, ProposalStatus::Rejected { .. })
        || matches!(ctx.accounts.proposal.status, ProposalStatus::Cancelled { .. })
        && !matches!(ctx.accounts.proposal.status, ProposalStatus::Executing { .. })
        && !matches!(ctx.accounts.proposal.status, ProposalStatus::Approved { .. })
    )]
    pub fn vault_batch_transaction_account_close(
        ctx: Context<VaultBatchTransactionAccountClose>,
    ) -> Result<()> {
        VaultBatchTransactionAccountClose::vault_batch_transaction_account_close(ctx)
    }

    /// Closes Batch and the corresponding Proposal accounts for proposals in terminal states:
    /// `Executed`, `Rejected`, or `Cancelled` or stale proposals that aren't `Approved`.
    ///
    /// This instruction is only allowed to be executed when all `VaultBatchTransaction` accounts
    /// in the `batch` are already closed: `batch.size == 0`.
    #[succeeds_if(
        (
            !ctx.accounts.proposal.data.borrow.is_empty()
            && (Proposal::try_deserialize(
                    &mut &*ctx.accounts.proposal.data.borrow()
                ).is_ok().is_some()
            &&
        (
            let proposal = Proposal::try_deserialize(
                &mut &*ctx.accounts.proposal.data.borrow()
            ).unwrap();
            (
                ctx.accounts.batch.index <= ctx.accounts.multisig.stale_transaction_index &&
                (matches!(proposal.status, ProposalStatus::Draft { .. })
                || matches!(proposal.status, ProposalStatus::Active { .. })
                )
            )
            || matches!(proposal.status, ProposalStatus::Executed { .. })            
            || matches!(proposal.status, ProposalStatus::Rejected { .. })
            || matches!(proposal.status, ProposalStatus::Cancelled { .. })
            && !matches!(proposal.status, ProposalStatus::Executing { .. }) 
            && !matches!(proposal.status, ProposalStatus::Approved { .. }) 
        )) ||
        (
            ctx.accounts.proposal.data.borrow().is_empty() && 
            ctx.accounts.transaction.index <= ctx.accounts.multisig.stale_transaction_index 
        ))
    )]
    pub fn batch_accounts_close(ctx: Context<BatchAccountsClose>) -> Result<()> {
        BatchAccountsClose::batch_accounts_close(ctx)
    }



    pub fn tx_execute_validation_helper<'info>(
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
                    ConfigAction::SetTimeLock { new_time_lock }=> {
                        if *new_time_lock <= MAX_TIME_LOCK {
                            Some(members)
                        } else {
                            None
                        }
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
}
