#[kani :: proof] #[kani :: unwind(100usize)] pub fn verify_multisig_create()
{
    let args : MultisigCreateArgs = kani :: any() ; let conc : anchor_lang ::
    context :: ConcreteContext < MultisigCreate > = kani :: any() ; let ctx =
    conc.to_ctx() ; kani :: assume(conc.to_ctx().accounts.__pre_invariants())
    ; let result = multisig :: multisig_create(ctx, args) ; kani ::
    assert(result.is_err() || conc.to_ctx().accounts.__post_invariants(),
    "Function failed",) ;
} #[kani :: proof] #[kani :: unwind(100usize)] pub fn
succeeds_if_multisig_create()
{
    let args : MultisigCreateArgs = kani :: any() ; let conc : anchor_lang ::
    context :: ConcreteContext < MultisigCreate > = kani :: any() ; let ctx =
    conc.to_ctx() ; kani :: assume(conc.to_ctx().accounts.__pre_invariants())
    ; let precondition = args.members.len() <= usize :: from(u16 :: MAX) &&
    args.members.windows(2).all(| win | win [0].key != win [1].key) &&
    args.members.iter().all(| m | m.permissions.mask < 8) &&
    args.members.iter().any(| m | m.permissions.has(Permission :: Initiate))
    && args.members.iter().any(| m | m.permissions.has(Permission :: Execute))
    && args.threshold > 0 && args.threshold as usize <=
    args.members.iter().filter(| m |
    m.permissions.has(Permission :: Vote)).count() ; kani ::
    assume(precondition) ; let constraints = true ; let result = if
    constraints { multisig :: multisig_create(ctx, args) } else
    { err! ("constraint check failed") } ; kani ::
    assert(result.is_ok(), "function failed to succeed given a precondition")
    ;
}
#[kani :: proof] #[kani :: unwind(100usize)] pub fn
verify_multisig_add_member()
{
    let args : MultisigAddMemberArgs = kani :: any() ; let conc : anchor_lang
    :: context :: ConcreteContext < MultisigConfig > = kani :: any() ; let ctx
    = conc.to_ctx() ; kani ::
    assume(conc.to_ctx().accounts.__pre_invariants()) ; let result = multisig
    :: multisig_add_member(ctx, args) ; kani ::
    assert(result.is_err() || conc.to_ctx().accounts.__post_invariants(),
    "Function failed",) ;
} #[kani :: proof] #[kani :: unwind(100usize)] pub fn
succeeds_if_multisig_add_member()
{
    let args : MultisigAddMemberArgs = kani :: any() ; let conc : anchor_lang
    :: context :: ConcreteContext < MultisigConfig > = kani :: any() ; let ctx
    = conc.to_ctx() ; kani ::
    assume(conc.to_ctx().accounts.__pre_invariants()) ; let precondition =
    ctx.accounts.multisig.members.len() <= usize :: from(u16 :: MAX - 1) &&
    ctx.accounts.multisig.members.iter().all(| m | m.key !=
    args.new_member.key) && ctx.accounts.system_program.is_some() &&
    ctx.accounts.rent_payer.is_some() && ctx.accounts.config_authority.key()
    == ctx.accounts.multisig.config_authority &&
    args.new_member.permissions.mask < 8 ; kani :: assume(precondition) ; let
    constraints = true ; let result = if constraints
    { multisig :: multisig_add_member(ctx, args) } else
    { err! ("constraint check failed") } ; kani ::
    assert(result.is_ok(), "function failed to succeed given a precondition")
    ;
}
#[kani :: proof] #[kani :: unwind(100usize)] pub fn
verify_multisig_remove_member()
{
    let args : MultisigRemoveMemberArgs = kani :: any() ; let conc :
    anchor_lang :: context :: ConcreteContext < MultisigConfig > = kani ::
    any() ; let ctx = conc.to_ctx() ; kani ::
    assume(conc.to_ctx().accounts.__pre_invariants()) ; let result = multisig
    :: multisig_remove_member(ctx, args) ; kani ::
    assert(result.is_err() || conc.to_ctx().accounts.__post_invariants(),
    "Function failed",) ;
} #[kani :: proof] #[kani :: unwind(100usize)] pub fn
succeeds_if_multisig_remove_member()
{
    let args : MultisigRemoveMemberArgs = kani :: any() ; let conc :
    anchor_lang :: context :: ConcreteContext < MultisigConfig > = kani ::
    any() ; let ctx = conc.to_ctx() ; kani ::
    assume(conc.to_ctx().accounts.__pre_invariants()) ; let precondition =
    ctx.accounts.multisig.members.len() > 1 &&
    ctx.accounts.multisig.members.iter().any(| m | m.key == args.old_member)
    &&
    ctx.accounts.multisig.members.iter().any(| m | m.key != args.old_member &&
    m.permissions.has(Permission :: Execute)) &&
    ctx.accounts.multisig.members.iter().any(| m | m.key != args.old_member &&
    m.permissions.has(Permission :: Initiate)) &&
    ctx.accounts.multisig.members.iter().filter(| m | m.key != args.old_member
    && m.permissions.has(Permission :: Vote)).count() >=
    ctx.accounts.multisig.threshold as usize &&
    ctx.accounts.config_authority.key() ==
    ctx.accounts.multisig.config_authority &&
    ctx.accounts.multisig.is_member(args.old_member).is_some() &&
    ctx.accounts.multisig.members.windows(3).all(| win | win [0].key != win
    [1].key && win [0].key != win [2].key) ; kani :: assume(precondition) ;
    let constraints = true ; let result = if constraints
    { multisig :: multisig_remove_member(ctx, args) } else
    { err! ("constraint check failed") } ; kani ::
    assert(result.is_ok(), "function failed to succeed given a precondition")
    ;
}
#[kani :: proof] #[kani :: unwind(100usize)] pub fn
verify_multisig_set_time_lock()
{
    let args : MultisigSetTimeLockArgs = kani :: any() ; let conc :
    anchor_lang :: context :: ConcreteContext < MultisigConfig > = kani ::
    any() ; let ctx = conc.to_ctx() ; kani ::
    assume(conc.to_ctx().accounts.__pre_invariants()) ; let result = multisig
    :: multisig_set_time_lock(ctx, args) ; kani ::
    assert(result.is_err() || conc.to_ctx().accounts.__post_invariants(),
    "Function failed",) ;
} #[kani :: proof] #[kani :: unwind(100usize)] pub fn
succeeds_if_multisig_set_time_lock()
{
    let args : MultisigSetTimeLockArgs = kani :: any() ; let conc :
    anchor_lang :: context :: ConcreteContext < MultisigConfig > = kani ::
    any() ; let ctx = conc.to_ctx() ; kani ::
    assume(conc.to_ctx().accounts.__pre_invariants()) ; let precondition =
    ctx.accounts.config_authority.key() ==
    ctx.accounts.multisig.config_authority ; kani :: assume(precondition) ;
    let constraints = true ; let result = if constraints
    { multisig :: multisig_set_time_lock(ctx, args) } else
    { err! ("constraint check failed") } ; kani ::
    assert(result.is_ok(), "function failed to succeed given a precondition")
    ;
}
#[kani :: proof] #[kani :: unwind(100usize)] pub fn
verify_multisig_set_config_authority()
{
    let args : MultisigSetConfigAuthorityArgs = kani :: any() ; let conc :
    anchor_lang :: context :: ConcreteContext < MultisigConfig > = kani ::
    any() ; let ctx = conc.to_ctx() ; kani ::
    assume(conc.to_ctx().accounts.__pre_invariants()) ; let result = multisig
    :: multisig_set_config_authority(ctx, args) ; kani ::
    assert(result.is_err() || conc.to_ctx().accounts.__post_invariants(),
    "Function failed",) ;
} #[kani :: proof] #[kani :: unwind(100usize)] pub fn
succeeds_if_multisig_set_config_authority()
{
    let args : MultisigSetConfigAuthorityArgs = kani :: any() ; let conc :
    anchor_lang :: context :: ConcreteContext < MultisigConfig > = kani ::
    any() ; let ctx = conc.to_ctx() ; kani ::
    assume(conc.to_ctx().accounts.__pre_invariants()) ; let precondition =
    ctx.accounts.config_authority.key() ==
    ctx.accounts.multisig.config_authority ; kani :: assume(precondition) ;
    let constraints = true ; let result = if constraints
    { multisig :: multisig_set_config_authority(ctx, args) } else
    { err! ("constraint check failed") } ; kani ::
    assert(result.is_ok(), "function failed to succeed given a precondition")
    ;
}
#[kani :: proof] #[kani :: unwind(100usize)] pub fn
verify_config_transaction_create()
{
    let args : ConfigTransactionCreateArgs = kani :: any() ; let conc :
    anchor_lang :: context :: ConcreteContext < ConfigTransactionCreate > =
    kani :: any() ; let ctx = conc.to_ctx() ; kani ::
    assume(conc.to_ctx().accounts.__pre_invariants()) ; let result = multisig
    :: config_transaction_create(ctx, args) ; kani ::
    assert(result.is_err() || conc.to_ctx().accounts.__post_invariants(),
    "Function failed",) ;
} #[kani :: proof] #[kani :: unwind(100usize)] pub fn
succeeds_if_config_transaction_create()
{
    let args : ConfigTransactionCreateArgs = kani :: any() ; let conc :
    anchor_lang :: context :: ConcreteContext < ConfigTransactionCreate > =
    kani :: any() ; let ctx = conc.to_ctx() ; kani ::
    assume(conc.to_ctx().accounts.__pre_invariants()) ; let precondition =!
    args.actions.is_empty() && ctx.accounts.multisig.config_authority ==
    Pubkey :: default() &&
    ctx.accounts.multisig.member_has_permission(ctx.accounts.creator.key(),
    Permission :: Initiate) && ctx.accounts.multisig.transaction_index < u64
    :: MAX ; kani :: assume(precondition) ; let constraints = true ; let
    result = if constraints
    { multisig :: config_transaction_create(ctx, args) } else
    { err! ("constraint check failed") } ; kani ::
    assert(result.is_ok(), "function failed to succeed given a precondition")
    ;
}
ignored harness for: Ident { ident: "confix_tx_execute_validation_helper", span: #0 bytes(4379..4414) }
#[kani :: proof] #[kani :: unwind(100usize)] pub fn
verify_config_transaction_execute < 'info > ()
{
    let conc : anchor_lang :: context :: ConcreteContext < '_, '_, '_, 'info,
    ConfigTransactionExecute < 'info > > = kani :: any() ; let ctx =
    conc.to_ctx() ; kani :: assume(conc.to_ctx().accounts.__pre_invariants())
    ; let result = multisig :: config_transaction_execute(ctx) ; kani ::
    assert(result.is_err() || conc.to_ctx().accounts.__post_invariants(),
    "Function failed",) ;
} #[kani :: proof] #[kani :: unwind(100usize)] pub fn
succeeds_if_config_transaction_execute < 'info > ()
{
    let conc : anchor_lang :: context :: ConcreteContext < '_, '_, '_, 'info,
    ConfigTransactionExecute < 'info > > = kani :: any() ; let ctx =
    conc.to_ctx() ; kani :: assume(conc.to_ctx().accounts.__pre_invariants())
    ; let precondition = ctx.accounts.multisig.key() ==
    ctx.accounts.proposal.multisig && ctx.accounts.multisig.key() ==
    ctx.accounts.transaction.multisig &&
    ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(),
    Permission :: Execute) && ctx.accounts.proposal.transaction_index >
    ctx.accounts.multisig.stale_transaction_index && matches!
    (ctx.accounts.proposal.status, ProposalStatus :: Approved { .. }) &&
    multisig :: confix_tx_execute_validation_helper(& ctx).is_ok() ; kani ::
    assume(precondition) ; let constraints = true ; let result = if
    constraints { multisig :: config_transaction_execute(ctx) } else
    { err! ("constraint check failed") } ; kani ::
    assert(result.is_ok(), "function failed to succeed given a precondition")
    ;
}
#[kani :: proof] #[kani :: unwind(100usize)] pub fn
verify_vault_transaction_create()
{
    let args : VaultTransactionCreateArgs = kani :: any() ; let conc :
    anchor_lang :: context :: ConcreteContext < VaultTransactionCreate > =
    kani :: any() ; let ctx = conc.to_ctx() ; kani ::
    assume(conc.to_ctx().accounts.__pre_invariants()) ; let result = multisig
    :: vault_transaction_create(ctx, args) ; kani ::
    assert(result.is_err() || conc.to_ctx().accounts.__post_invariants(),
    "Function failed",) ;
} #[kani :: proof] #[kani :: unwind(100usize)] pub fn
succeeds_if_vault_transaction_create()
{
    let args : VaultTransactionCreateArgs = kani :: any() ; let conc :
    anchor_lang :: context :: ConcreteContext < VaultTransactionCreate > =
    kani :: any() ; let ctx = conc.to_ctx() ; kani ::
    assume(conc.to_ctx().accounts.__pre_invariants()) ; let precondition =
    ctx.accounts.multisig.member_has_permission(ctx.accounts.creator.key(),
    Permission :: Initiate) ; kani :: assume(precondition) ; let constraints =
    true ; let result = if constraints
    { multisig :: vault_transaction_create(ctx, args) } else
    { err! ("constraint check failed") } ; kani ::
    assert(result.is_ok(), "function failed to succeed given a precondition")
    ;
}
#[kani :: proof] #[kani :: unwind(100usize)] pub fn
verify_vault_transaction_execute()
{
    let conc : anchor_lang :: context :: ConcreteContext <
    VaultTransactionExecute > = kani :: any() ; let ctx = conc.to_ctx() ; kani
    :: assume(conc.to_ctx().accounts.__pre_invariants()) ; let result =
    multisig :: vault_transaction_execute(ctx) ; kani ::
    assert(result.is_err() || conc.to_ctx().accounts.__post_invariants(),
    "Function failed",) ;
} #[kani :: proof] #[kani :: unwind(100usize)] pub fn
succeeds_if_vault_transaction_execute()
{
    let conc : anchor_lang :: context :: ConcreteContext <
    VaultTransactionExecute > = kani :: any() ; let ctx = conc.to_ctx() ; kani
    :: assume(conc.to_ctx().accounts.__pre_invariants()) ; let precondition =
    ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(),
    Permission :: Execute) && matches!
    (ctx.accounts.proposal.status, ProposalStatus :: Approved { .. }) &&
    ctx.accounts.multisig.key() == ctx.accounts.proposal.multisig &&
    ctx.accounts.multisig.key() == ctx.accounts.transaction.multisig &&
    ctx.remaining_accounts.len() ==
    ctx.accounts.transaction.message.address_table_lookups.len() +
    ctx.accounts.transaction.message.num_all_account_keys() ; kani ::
    assume(precondition) ; let constraints = true ; let result = if
    constraints { multisig :: vault_transaction_execute(ctx) } else
    { err! ("constraint check failed") } ; kani ::
    assert(result.is_ok(), "function failed to succeed given a precondition")
    ;
}
#[kani :: proof] #[kani :: unwind(100usize)] pub fn verify_batch_create()
{
    let args : BatchCreateArgs = kani :: any() ; let conc : anchor_lang ::
    context :: ConcreteContext < BatchCreate > = kani :: any() ; let ctx =
    conc.to_ctx() ; kani :: assume(conc.to_ctx().accounts.__pre_invariants())
    ; let result = multisig :: batch_create(ctx, args) ; kani ::
    assert(result.is_err() || conc.to_ctx().accounts.__post_invariants(),
    "Function failed",) ;
} #[kani :: proof] #[kani :: unwind(100usize)] pub fn
succeeds_if_batch_create()
{
    let args : BatchCreateArgs = kani :: any() ; let conc : anchor_lang ::
    context :: ConcreteContext < BatchCreate > = kani :: any() ; let ctx =
    conc.to_ctx() ; kani :: assume(conc.to_ctx().accounts.__pre_invariants())
    ; let precondition =
    ctx.accounts.multisig.member_has_permission(ctx.accounts.creator.key(),
    Permission :: Initiate) ; kani :: assume(precondition) ; let constraints =
    true ; let result = if constraints { multisig :: batch_create(ctx, args) }
    else { err! ("constraint check failed") } ; kani ::
    assert(result.is_ok(), "function failed to succeed given a precondition")
    ;
}
#[kani :: proof] #[kani :: unwind(100usize)] pub fn
verify_batch_add_transaction()
{
    let args : BatchAddTransactionArgs = kani :: any() ; let conc :
    anchor_lang :: context :: ConcreteContext < BatchAddTransaction > = kani
    :: any() ; let ctx = conc.to_ctx() ; kani ::
    assume(conc.to_ctx().accounts.__pre_invariants()) ; let result = multisig
    :: batch_add_transaction(ctx, args) ; kani ::
    assert(result.is_err() || conc.to_ctx().accounts.__post_invariants(),
    "Function failed",) ;
} #[kani :: proof] #[kani :: unwind(100usize)] pub fn
succeeds_if_batch_add_transaction()
{
    let args : BatchAddTransactionArgs = kani :: any() ; let conc :
    anchor_lang :: context :: ConcreteContext < BatchAddTransaction > = kani
    :: any() ; let ctx = conc.to_ctx() ; kani ::
    assume(conc.to_ctx().accounts.__pre_invariants()) ; let precondition =
    ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(),
    Permission :: Initiate) && matches!
    (ctx.accounts.proposal.status, ProposalStatus :: Draft { .. }) &&
    ctx.accounts.batch.size >= ctx.accounts.batch.executed_transaction_index ;
    kani :: assume(precondition) ; let constraints = true ; let result = if
    constraints { multisig :: batch_add_transaction(ctx, args) } else
    { err! ("constraint check failed") } ; kani ::
    assert(result.is_ok(), "function failed to succeed given a precondition")
    ;
}
#[kani :: proof] #[kani :: unwind(100usize)] pub fn
verify_batch_execute_transaction()
{
    let conc : anchor_lang :: context :: ConcreteContext <
    BatchExecuteTransaction > = kani :: any() ; let ctx = conc.to_ctx() ; kani
    :: assume(conc.to_ctx().accounts.__pre_invariants()) ; let result =
    multisig :: batch_execute_transaction(ctx) ; kani ::
    assert(result.is_err() || conc.to_ctx().accounts.__post_invariants(),
    "Function failed",) ;
} #[kani :: proof] #[kani :: unwind(100usize)] pub fn
succeeds_if_batch_execute_transaction()
{
    let conc : anchor_lang :: context :: ConcreteContext <
    BatchExecuteTransaction > = kani :: any() ; let ctx = conc.to_ctx() ; kani
    :: assume(conc.to_ctx().accounts.__pre_invariants()) ; let precondition =
    ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(),
    Permission :: Execute) && matches!
    (ctx.accounts.proposal.status, ProposalStatus :: Approved { .. }) &&
    ctx.accounts.multisig.key() == ctx.accounts.proposal.multisig &&
    ctx.accounts.multisig.key() == ctx.accounts.batch.multisig &&
    ctx.remaining_accounts.len() ==
    ctx.accounts.transaction.message.address_table_lookups.len() +
    ctx.accounts.transaction.message.num_all_account_keys() &&
    ctx.accounts.batch.executed_transaction_index < ctx.accounts.batch.size ;
    kani :: assume(precondition) ; let constraints = true ; let result = if
    constraints { multisig :: batch_execute_transaction(ctx) } else
    { err! ("constraint check failed") } ; kani ::
    assert(result.is_ok(), "function failed to succeed given a precondition")
    ;
}
#[kani :: proof] #[kani :: unwind(100usize)] pub fn verify_proposal_create()
{
    let args : ProposalCreateArgs = kani :: any() ; let conc : anchor_lang ::
    context :: ConcreteContext < ProposalCreate > = kani :: any() ; let ctx =
    conc.to_ctx() ; kani :: assume(conc.to_ctx().accounts.__pre_invariants())
    ; let result = multisig :: proposal_create(ctx, args) ; kani ::
    assert(result.is_err() || conc.to_ctx().accounts.__post_invariants(),
    "Function failed",) ;
} #[kani :: proof] #[kani :: unwind(100usize)] pub fn
succeeds_if_proposal_create()
{
    let args : ProposalCreateArgs = kani :: any() ; let conc : anchor_lang ::
    context :: ConcreteContext < ProposalCreate > = kani :: any() ; let ctx =
    conc.to_ctx() ; kani :: assume(conc.to_ctx().accounts.__pre_invariants())
    ; let precondition = args.transaction_index <=
    ctx.accounts.multisig.transaction_index && args.transaction_index >
    ctx.accounts.multisig.stale_transaction_index &&
    (ctx.accounts.multisig.member_has_permission(ctx.accounts.creator.key(),
    Permission :: Initiate) ||
    ctx.accounts.multisig.member_has_permission(ctx.accounts.creator.key(),
    Permission :: Vote)) ; kani :: assume(precondition) ; let constraints =
    true ; let result = if constraints
    { multisig :: proposal_create(ctx, args) } else
    { err! ("constraint check failed") } ; kani ::
    assert(result.is_ok(), "function failed to succeed given a precondition")
    ;
}
#[kani :: proof] #[kani :: unwind(100usize)] pub fn verify_proposal_activate()
{
    let conc : anchor_lang :: context :: ConcreteContext < ProposalActivate >
    = kani :: any() ; let ctx = conc.to_ctx() ; kani ::
    assume(conc.to_ctx().accounts.__pre_invariants()) ; let result = multisig
    :: proposal_activate(ctx) ; kani ::
    assert(result.is_err() || conc.to_ctx().accounts.__post_invariants(),
    "Function failed",) ;
} #[kani :: proof] #[kani :: unwind(100usize)] pub fn
succeeds_if_proposal_activate()
{
    let conc : anchor_lang :: context :: ConcreteContext < ProposalActivate >
    = kani :: any() ; let ctx = conc.to_ctx() ; kani ::
    assume(conc.to_ctx().accounts.__pre_invariants()) ; let precondition =
    ctx.accounts.proposal.transaction_index >
    ctx.accounts.multisig.stale_transaction_index &&
    ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(),
    Permission :: Initiate) && matches!
    (ctx.accounts.proposal.status, ProposalStatus :: Draft { .. }) &&
    ctx.accounts.proposal.multisig == ctx.accounts.multisig.key() ; kani ::
    assume(precondition) ; let constraints = true ; let result = if
    constraints { multisig :: proposal_activate(ctx) } else
    { err! ("constraint check failed") } ; kani ::
    assert(result.is_ok(), "function failed to succeed given a precondition")
    ;
}
#[kani :: proof] #[kani :: unwind(100usize)] pub fn verify_proposal_approve()
{
    let args : ProposalVoteArgs = kani :: any() ; let conc : anchor_lang ::
    context :: ConcreteContext < ProposalVote > = kani :: any() ; let ctx =
    conc.to_ctx() ; kani :: assume(conc.to_ctx().accounts.__pre_invariants())
    ; let result = multisig :: proposal_approve(ctx, args) ; kani ::
    assert(result.is_err() || conc.to_ctx().accounts.__post_invariants(),
    "Function failed",) ;
} #[kani :: proof] #[kani :: unwind(100usize)] pub fn
succeeds_if_proposal_approve()
{
    let args : ProposalVoteArgs = kani :: any() ; let conc : anchor_lang ::
    context :: ConcreteContext < ProposalVote > = kani :: any() ; let ctx =
    conc.to_ctx() ; kani :: assume(conc.to_ctx().accounts.__pre_invariants())
    ; let precondition = ctx.accounts.proposal.transaction_index >
    ctx.accounts.multisig.stale_transaction_index &&
    ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(),
    Permission :: Vote) && matches!
    (ctx.accounts.proposal.status, ProposalStatus :: Active { .. }) &&
    ctx.accounts.proposal.multisig == ctx.accounts.multisig.key() &&!
    ctx.accounts.proposal.approved.contains(& ctx.accounts.member.key()) ;
    kani :: assume(precondition) ; let constraints = true ; let result = if
    constraints { multisig :: proposal_approve(ctx, args) } else
    { err! ("constraint check failed") } ; kani ::
    assert(result.is_ok(), "function failed to succeed given a precondition")
    ;
}
#[kani :: proof] #[kani :: unwind(100usize)] pub fn verify_proposal_reject()
{
    let args : ProposalVoteArgs = kani :: any() ; let conc : anchor_lang ::
    context :: ConcreteContext < ProposalVote > = kani :: any() ; let ctx =
    conc.to_ctx() ; kani :: assume(conc.to_ctx().accounts.__pre_invariants())
    ; let result = multisig :: proposal_reject(ctx, args) ; kani ::
    assert(result.is_err() || conc.to_ctx().accounts.__post_invariants(),
    "Function failed",) ;
} #[kani :: proof] #[kani :: unwind(100usize)] pub fn
succeeds_if_proposal_reject()
{
    let args : ProposalVoteArgs = kani :: any() ; let conc : anchor_lang ::
    context :: ConcreteContext < ProposalVote > = kani :: any() ; let ctx =
    conc.to_ctx() ; kani :: assume(conc.to_ctx().accounts.__pre_invariants())
    ; let precondition = ctx.accounts.proposal.transaction_index >
    ctx.accounts.multisig.stale_transaction_index &&
    ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(),
    Permission :: Vote) && matches!
    (ctx.accounts.proposal.status, ProposalStatus :: Active { .. }) &&
    ctx.accounts.proposal.multisig == ctx.accounts.multisig.key() &&!
    ctx.accounts.proposal.rejected.contains(& ctx.accounts.member.key()) ;
    kani :: assume(precondition) ; let constraints = true ; let result = if
    constraints { multisig :: proposal_reject(ctx, args) } else
    { err! ("constraint check failed") } ; kani ::
    assert(result.is_ok(), "function failed to succeed given a precondition")
    ;
}
#[kani :: proof] #[kani :: unwind(100usize)] pub fn verify_proposal_cancel()
{
    let args : ProposalVoteArgs = kani :: any() ; let conc : anchor_lang ::
    context :: ConcreteContext < ProposalVote > = kani :: any() ; let ctx =
    conc.to_ctx() ; kani :: assume(conc.to_ctx().accounts.__pre_invariants())
    ; let result = multisig :: proposal_cancel(ctx, args) ; kani ::
    assert(result.is_err() || conc.to_ctx().accounts.__post_invariants(),
    "Function failed",) ;
} #[kani :: proof] #[kani :: unwind(100usize)] pub fn
succeeds_if_proposal_cancel()
{
    let args : ProposalVoteArgs = kani :: any() ; let conc : anchor_lang ::
    context :: ConcreteContext < ProposalVote > = kani :: any() ; let ctx =
    conc.to_ctx() ; kani :: assume(conc.to_ctx().accounts.__pre_invariants())
    ; let precondition =
    ctx.accounts.multisig.member_has_permission(ctx.accounts.member.key(),
    Permission :: Vote) && matches!
    (ctx.accounts.proposal.status, ProposalStatus :: Approved { .. }) &&
    ctx.accounts.proposal.multisig == ctx.accounts.multisig.key() &&!
    ctx.accounts.proposal.cancelled.contains(& ctx.accounts.member.key()) ;
    kani :: assume(precondition) ; let constraints = true ; let result = if
    constraints { multisig :: proposal_cancel(ctx, args) } else
    { err! ("constraint check failed") } ; kani ::
    assert(result.is_ok(), "function failed to succeed given a precondition")
    ;
}
#[kani :: proof] #[kani :: unwind(100usize)] pub fn
verify_spending_limit_use()
{
    let args : SpendingLimitUseArgs = kani :: any() ; let conc : anchor_lang
    :: context :: ConcreteContext < SpendingLimitUse > = kani :: any() ; let
    ctx = conc.to_ctx() ; kani ::
    assume(conc.to_ctx().accounts.__pre_invariants()) ; let result = multisig
    :: spending_limit_use(ctx, args) ; kani ::
    assert(result.is_err() || conc.to_ctx().accounts.__post_invariants(),
    "Function failed",) ;
} #[kani :: proof] #[kani :: unwind(100usize)] pub fn
succeeds_if_spending_limit_use()
{
    let args : SpendingLimitUseArgs = kani :: any() ; let conc : anchor_lang
    :: context :: ConcreteContext < SpendingLimitUse > = kani :: any() ; let
    ctx = conc.to_ctx() ; kani ::
    assume(conc.to_ctx().accounts.__pre_invariants()) ; let precondition =
    ctx.accounts.multisig.is_member(ctx.accounts.member.key()).is_some() &&
    ctx.accounts.spending_limit.members.contains(& ctx.accounts.member.key())
    && ctx.accounts.spending_limit.multisig == ctx.accounts.multisig.key() &&
    args.amount <= ctx.accounts.spending_limit.amount &&
    (ctx.accounts.spending_limit.destinations.is_empty() ||
    ctx.accounts.spending_limit.destinations.contains(&
    ctx.accounts.destination.key())) &&
    (if ctx.accounts.spending_limit.mint == Pubkey :: default()
    {
        ctx.accounts.mint.is_none() && ctx.accounts.system_program.is_some()
        && args.decimals == 9 && ctx.accounts.vault.lamports() >= args.amount
    } else
    {
        ctx.accounts.mint.is_some() && ctx.accounts.spending_limit.mint ==
        ctx.accounts.mint.as_ref().unwrap().key() &&
        ctx.accounts.vault_token_account.is_some() &&
        ctx.accounts.destination_token_account.is_some() &&
        ctx.accounts.token_program.is_some()
    }) ; kani :: assume(precondition) ; let constraints = true ; let result =
    if constraints { multisig :: spending_limit_use(ctx, args) } else
    { err! ("constraint check failed") } ; kani ::
    assert(result.is_ok(), "function failed to succeed given a precondition")
    ;
}
#![feature(prelude_import)]
#![allow(clippy::result_large_err)]
#![deny(arithmetic_overflow)]
#![deny(unused_must_use)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
extern crate core;
use anchor_lang::prelude::*;
pub use instructions::*;
pub use state::*;
pub mod errors {
    use anchor_lang::prelude::*;
    pub enum MultisigError {
        DuplicateMember,
        EmptyMembers,
        TooManyMembers,
        InvalidThreshold,
        Unauthorized,
        NotAMember,
        InvalidTransactionMessage,
        StaleProposal,
        InvalidProposalStatus,
        InvalidTransactionIndex,
        AlreadyApproved,
        AlreadyRejected,
        AlreadyCancelled,
        InvalidNumberOfAccounts,
        InvalidAccount,
        ExecuteReentrancy,
        RemoveLastMember,
        NoVoters,
        NoProposers,
        NoExecutors,
        InvalidStaleTransactionIndex,
        NotSupportedForControlled,
        TimeLockNotReleased,
        NoActions,
        MissingAccount,
        InvalidMint,
        InvalidDestination,
        SpendingLimitExceeded,
        DecimalsMismatch,
        UnknownPermission,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MultisigError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    MultisigError::DuplicateMember => "DuplicateMember",
                    MultisigError::EmptyMembers => "EmptyMembers",
                    MultisigError::TooManyMembers => "TooManyMembers",
                    MultisigError::InvalidThreshold => "InvalidThreshold",
                    MultisigError::Unauthorized => "Unauthorized",
                    MultisigError::NotAMember => "NotAMember",
                    MultisigError::InvalidTransactionMessage => {
                        "InvalidTransactionMessage"
                    }
                    MultisigError::StaleProposal => "StaleProposal",
                    MultisigError::InvalidProposalStatus => "InvalidProposalStatus",
                    MultisigError::InvalidTransactionIndex => "InvalidTransactionIndex",
                    MultisigError::AlreadyApproved => "AlreadyApproved",
                    MultisigError::AlreadyRejected => "AlreadyRejected",
                    MultisigError::AlreadyCancelled => "AlreadyCancelled",
                    MultisigError::InvalidNumberOfAccounts => "InvalidNumberOfAccounts",
                    MultisigError::InvalidAccount => "InvalidAccount",
                    MultisigError::ExecuteReentrancy => "ExecuteReentrancy",
                    MultisigError::RemoveLastMember => "RemoveLastMember",
                    MultisigError::NoVoters => "NoVoters",
                    MultisigError::NoProposers => "NoProposers",
                    MultisigError::NoExecutors => "NoExecutors",
                    MultisigError::InvalidStaleTransactionIndex => {
                        "InvalidStaleTransactionIndex"
                    }
                    MultisigError::NotSupportedForControlled => {
                        "NotSupportedForControlled"
                    }
                    MultisigError::TimeLockNotReleased => "TimeLockNotReleased",
                    MultisigError::NoActions => "NoActions",
                    MultisigError::MissingAccount => "MissingAccount",
                    MultisigError::InvalidMint => "InvalidMint",
                    MultisigError::InvalidDestination => "InvalidDestination",
                    MultisigError::SpendingLimitExceeded => "SpendingLimitExceeded",
                    MultisigError::DecimalsMismatch => "DecimalsMismatch",
                    MultisigError::UnknownPermission => "UnknownPermission",
                },
            )
        }
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for MultisigError {}
    impl std::fmt::Display for MultisigError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            <Self as std::fmt::Debug>::fmt(self, f)
        }
    }
    impl From<MultisigError> for anchor_lang::prelude::Error {
        fn from(value: MultisigError) -> Self {
            anchor_lang::prelude::Error::CustomError(value.to_string())
        }
    }
}
pub mod instructions {
    pub use batch_add_transaction::*;
    pub use batch_create::*;
    pub use batch_execute_transaction::*;
    pub use config_transaction_create::*;
    pub use config_transaction_execute::*;
    pub use multisig_config::*;
    pub use multisig_create::*;
    pub use proposal_activate::*;
    pub use proposal_create::*;
    pub use proposal_vote::*;
    pub use spending_limit_use::*;
    pub use vault_transaction_create::*;
    pub use vault_transaction_execute::*;
    mod batch_add_transaction {
        use anchor_lang::prelude::*;
        use crate::state::*;
        use crate::TransactionMessage;
        pub struct BatchAddTransactionArgs {
            /// Number of ephemeral signing PDAs required by the transaction.
            pub ephemeral_signers: u8,
            pub transaction_message: Vec<u8>,
        }
        impl borsh::ser::BorshSerialize for BatchAddTransactionArgs
        where
            u8: borsh::ser::BorshSerialize,
            Vec<u8>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.ephemeral_signers, writer)?;
                borsh::BorshSerialize::serialize(&self.transaction_message, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for BatchAddTransactionArgs
        where
            u8: borsh::BorshDeserialize,
            Vec<u8>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    ephemeral_signers: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    transaction_message: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                })
            }
        }
        impl kani::Arbitrary for BatchAddTransactionArgs {
            fn any() -> Self {
                BatchAddTransactionArgs {
                    ephemeral_signers: kani::any(),
                    transaction_message: kani::any(),
                }
            }
        }
        #[instruction(args:BatchAddTransactionArgs)]
        pub struct BatchAddTransaction<'info> {
            /// Multisig account this batch belongs to.
            #[account(
                seeds = [SEED_PREFIX,
                SEED_MULTISIG,
                multisig.create_key.as_ref()],
                bump = multisig.bump,
            )]
            pub multisig: Account<'info, Multisig>,
            /// Member of the multisig.
            #[account(mut)]
            pub member: Signer<'info>,
            /// The proposal account associated with the batch.
            #[account(
                seeds = [SEED_PREFIX,
                multisig.key().as_ref(),
                SEED_TRANSACTION,
                &batch.index.to_le_bytes(),
                SEED_PROPOSAL,
                ],
                bump = proposal.bump,
            )]
            pub proposal: Account<'info, Proposal>,
            #[account(
                mut,
                seeds = [SEED_PREFIX,
                multisig.key().as_ref(),
                SEED_TRANSACTION,
                &batch.index.to_le_bytes(),
                ],
                bump = batch.bump,
            )]
            pub batch: Account<'info, Batch>,
            /// `VaultBatchTransaction` account to initialize and add to the `batch`.
            #[account(
                init,
                payer = member,
                space = VaultBatchTransaction::size(
                    args.ephemeral_signers,
                    &args.transaction_message
                )?,
                seeds = [SEED_PREFIX,
                multisig.key().as_ref(),
                SEED_TRANSACTION,
                &batch.index.to_le_bytes(),
                SEED_BATCH_TRANSACTION,
                &batch.size.checked_add(1).unwrap().to_le_bytes(),
                ],
                bump
            )]
            pub transaction: Account<'info, VaultBatchTransaction>,
            pub system_program: Program<'info, System>,
        }
        impl<'info> kani::Arbitrary for BatchAddTransaction<'info> {
            fn any() -> Self {
                Self {
                    multisig: kani::any(),
                    member: kani::any(),
                    proposal: kani::any(),
                    batch: kani::any(),
                    transaction: kani::any(),
                    system_program: kani::any(),
                }
            }
        }
        impl<'info> BatchAddTransaction<'info> {
            pub fn __pre_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
                    && self.proposal.account._check_invariant()
                    && self.batch.account._check_invariant()
            }
        }
        impl<'info> BatchAddTransaction<'info> {
            pub fn __post_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
                    && self.proposal.account._check_invariant()
                    && self.batch.account._check_invariant()
                    && self.transaction.account._check_invariant()
            }
        }
        impl<'info> BatchAddTransaction<'info> {
            pub fn __check_constraints(&self, args: BatchAddTransactionArgs) -> bool {
                true
            }
        }
        impl BatchAddTransaction<'_> {
            fn validate(&self) -> Result<()> {
                let Self { multisig, member, proposal, .. } = self;
                if !(multisig.is_member(member.key()).is_some()) {
                    return Err(Error::Generic);
                }
                if !(multisig.member_has_permission(member.key(), Permission::Initiate))
                {
                    return Err(Error::Generic);
                }
                if !(match proposal.status {
                    ProposalStatus::Draft { .. } => true,
                    _ => false,
                }) {
                    return Err(Error::Generic);
                }
                Ok(())
            }
            /// Add a transaction to the batch.
            pub fn batch_add_transaction(
                ctx: Context<Self>,
                args: BatchAddTransactionArgs,
            ) -> Result<()> {
                ctx.accounts.validate()?;
                let batch = &mut ctx.accounts.batch;
                let transaction = &mut ctx.accounts.transaction;
                let batch_key = batch.key();
                let ephemeral_signer_bumps: Vec<u8> = (0..args.ephemeral_signers)
                    .map(|ephemeral_signer_index| {
                        let ephemeral_signer_seeds = &[
                            SEED_PREFIX,
                            batch_key.as_ref(),
                            SEED_EPHEMERAL_SIGNER,
                            &ephemeral_signer_index.to_le_bytes(),
                        ];
                        let (_, bump) = Pubkey::find_program_address(
                            ephemeral_signer_seeds,
                            ctx.program_id,
                        );
                        bump
                    })
                    .collect();
                transaction.bump = *ctx.bumps.get("transaction").unwrap();
                transaction.ephemeral_signer_bumps = ephemeral_signer_bumps;
                batch.size = batch.size.checked_add(1).expect("overflow");
                ();
                ();
                Ok(())
            }
        }
    }
    mod batch_create {
        use anchor_lang::prelude::*;
        use crate::state::*;
        pub struct BatchCreateArgs {
            /// Index of the vault this transaction belongs to.
            pub vault_index: u8,
        }
        impl borsh::ser::BorshSerialize for BatchCreateArgs
        where
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.vault_index, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for BatchCreateArgs
        where
            u8: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    vault_index: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        impl kani::Arbitrary for BatchCreateArgs {
            fn any() -> Self {
                BatchCreateArgs {
                    vault_index: kani::any(),
                }
            }
        }
        pub struct BatchCreate<'info> {
            #[account(
                mut,
                seeds = [SEED_PREFIX,
                SEED_MULTISIG,
                multisig.create_key.as_ref()],
                bump = multisig.bump,
            )]
            pub multisig: Account<'info, Multisig>,
            #[account(mut)]
            pub creator: Signer<'info>,
            #[account(
                init,
                payer = creator,
                space = 8+Batch::INIT_SPACE,
                seeds = [SEED_PREFIX,
                multisig.key().as_ref(),
                SEED_TRANSACTION,
                &multisig.transaction_index.checked_add(1).unwrap().to_le_bytes(),
                ],
                bump
            )]
            pub batch: Account<'info, Batch>,
            pub system_program: Program<'info, System>,
        }
        impl<'info> kani::Arbitrary for BatchCreate<'info> {
            fn any() -> Self {
                Self {
                    multisig: kani::any(),
                    creator: kani::any(),
                    batch: kani::any(),
                    system_program: kani::any(),
                }
            }
        }
        impl<'info> BatchCreate<'info> {
            pub fn __pre_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
            }
        }
        impl<'info> BatchCreate<'info> {
            pub fn __post_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
                    && self.batch.account._check_invariant()
            }
        }
        impl<'info> BatchCreate<'info> {
            pub fn __check_constraints(&self) -> bool {
                true
            }
        }
        impl BatchCreate<'_> {
            fn validate(&self) -> Result<()> {
                let Self { multisig, creator, .. } = self;
                if !(multisig.is_member(creator.key()).is_some()) {
                    return Err(Error::Generic);
                }
                if !(multisig.member_has_permission(creator.key(), Permission::Initiate))
                {
                    return Err(Error::Generic);
                }
                Ok(())
            }
            /// Create a new batch.
            pub fn batch_create(
                ctx: Context<Self>,
                args: BatchCreateArgs,
            ) -> Result<()> {
                ctx.accounts.validate()?;
                let multisig = &mut ctx.accounts.multisig;
                let creator = &mut ctx.accounts.creator;
                let batch = &mut ctx.accounts.batch;
                let multisig_key = multisig.key();
                let index = multisig.transaction_index.checked_add(1).expect("overflow");
                let vault_seeds = &[
                    SEED_PREFIX,
                    multisig_key.as_ref(),
                    SEED_VAULT,
                    &args.vault_index.to_le_bytes(),
                ];
                let (_, vault_bump) = Pubkey::find_program_address(
                    vault_seeds,
                    ctx.program_id,
                );
                batch.multisig = multisig_key;
                batch.creator = creator.key();
                batch.index = index;
                batch.bump = *ctx.bumps.get("batch").unwrap();
                batch.vault_index = args.vault_index;
                batch.vault_bump = vault_bump;
                batch.size = 0;
                batch.executed_transaction_index = 0;
                multisig.transaction_index = index;
                multisig.invariant()?;
                ();
                Ok(())
            }
        }
    }
    mod batch_execute_transaction {
        use anchor_lang::prelude::*;
        use crate::errors::*;
        use crate::state::*;
        use crate::utils::*;
        pub struct BatchExecuteTransaction<'info> {
            /// Multisig account this batch belongs to.
            #[account(
                seeds = [SEED_PREFIX,
                SEED_MULTISIG,
                multisig.create_key.as_ref()],
                bump = multisig.bump,
            )]
            pub multisig: Account<'info, Multisig>,
            /// Member of the multisig.
            pub member: Signer<'info>,
            /// The proposal account associated with the batch.
            /// If `transaction` is the last in the batch, the `proposal` status will be set to `Executed`.
            #[account(
                mut,
                seeds = [SEED_PREFIX,
                multisig.key().as_ref(),
                SEED_TRANSACTION,
                &batch.index.to_le_bytes(),
                SEED_PROPOSAL,
                ],
                bump = proposal.bump,
            )]
            pub proposal: Account<'info, Proposal>,
            #[account(
                mut,
                seeds = [SEED_PREFIX,
                multisig.key().as_ref(),
                SEED_TRANSACTION,
                &batch.index.to_le_bytes(),
                ],
                bump = batch.bump,
            )]
            pub batch: Account<'info, Batch>,
            /// Batch transaction to execute.
            #[account(
                seeds = [SEED_PREFIX,
                multisig.key().as_ref(),
                SEED_TRANSACTION,
                &batch.index.to_le_bytes(),
                SEED_BATCH_TRANSACTION,
                &batch.executed_transaction_index.checked_add(1).unwrap().to_le_bytes(),
                ],
                bump = transaction.bump,
            )]
            pub transaction: Account<'info, VaultBatchTransaction>,
        }
        impl<'info> kani::Arbitrary for BatchExecuteTransaction<'info> {
            fn any() -> Self {
                Self {
                    multisig: kani::any(),
                    member: kani::any(),
                    proposal: kani::any(),
                    batch: kani::any(),
                    transaction: kani::any(),
                }
            }
        }
        impl<'info> BatchExecuteTransaction<'info> {
            pub fn __pre_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
                    && self.proposal.account._check_invariant()
                    && self.batch.account._check_invariant()
                    && self.transaction.account._check_invariant()
            }
        }
        impl<'info> BatchExecuteTransaction<'info> {
            pub fn __post_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
                    && self.proposal.account._check_invariant()
                    && self.batch.account._check_invariant()
                    && self.transaction.account._check_invariant()
            }
        }
        impl<'info> BatchExecuteTransaction<'info> {
            pub fn __check_constraints(&self) -> bool {
                true
            }
        }
        impl BatchExecuteTransaction<'_> {
            fn validate(&self) -> Result<()> {
                let Self { multisig, member, proposal, .. } = self;
                if !(multisig.is_member(member.key()).is_some()) {
                    return Err(Error::Generic);
                }
                if !(multisig.member_has_permission(member.key(), Permission::Execute)) {
                    return Err(Error::Generic);
                }
                match proposal.status {
                    ProposalStatus::Approved { timestamp } => {
                        let now = Clock::get()?.unix_timestamp;
                        kani::assume(now > timestamp);
                        kani::assume(timestamp > 0);
                        if !(now.checked_sub(timestamp).unwrap()
                            >= i64::from(multisig.time_lock))
                        {
                            return Err(Error::Generic);
                        }
                    }
                    _ => return Err(Error::Generic),
                }
                Ok(())
            }
            /// Execute a transaction from the batch.
            pub fn batch_execute_transaction(
                ctx: Context<BatchExecuteTransaction>,
            ) -> Result<()> {
                ctx.accounts.validate()?;
                let multisig = &mut ctx.accounts.multisig;
                let proposal = &mut ctx.accounts.proposal;
                let batch = &mut ctx.accounts.batch;
                let transaction = &mut ctx.accounts.transaction;
                let multisig_key = multisig.key();
                let batch_key = batch.key();
                let vault_seeds = &[
                    SEED_PREFIX,
                    multisig_key.as_ref(),
                    SEED_VAULT,
                    &batch.vault_index.to_le_bytes(),
                    &[batch.vault_bump],
                ];
                let transaction_message = &transaction.message;
                let num_lookups = transaction_message.address_table_lookups.len();
                let message_account_infos = ctx
                    .remaining_accounts
                    .get(num_lookups..)
                    .ok_or(MultisigError::InvalidNumberOfAccounts)?;
                let address_lookup_table_account_infos = ctx
                    .remaining_accounts
                    .get(..num_lookups)
                    .ok_or(MultisigError::InvalidNumberOfAccounts)?;
                let vault_pubkey = Pubkey::create_program_address(
                    vault_seeds,
                    ctx.program_id,
                );
                let (ephemeral_signer_keys, ephemeral_signer_seeds) = derive_ephemeral_signers(
                    batch_key,
                    &transaction.ephemeral_signer_bumps,
                );
                let current_status = proposal.status.clone();
                proposal.status = ProposalStatus::Executing;
                let proposal_account_info = proposal.to_account_info();
                proposal
                    .try_serialize(
                        &mut &mut proposal_account_info.data.borrow_mut()[..],
                    )?;
                proposal.status = current_status;
                batch
                    .executed_transaction_index = batch
                    .executed_transaction_index
                    .checked_add(1)
                    .expect("overflow");
                if batch.executed_transaction_index == batch.size {
                    proposal
                        .status = ProposalStatus::Executed {
                        timestamp: Clock::get()?.unix_timestamp,
                    };
                }
                batch.invariant()?;
                Ok(())
            }
        }
    }
    mod config_transaction_create {
        use anchor_lang::prelude::*;
        use crate::state::*;
        pub struct ConfigTransactionCreateArgs {
            pub actions: Vec<ConfigAction>,
        }
        impl borsh::ser::BorshSerialize for ConfigTransactionCreateArgs
        where
            Vec<ConfigAction>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.actions, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for ConfigTransactionCreateArgs
        where
            Vec<ConfigAction>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    actions: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        impl kani::Arbitrary for ConfigTransactionCreateArgs {
            fn any() -> Self {
                ConfigTransactionCreateArgs {
                    actions: kani::any(),
                }
            }
        }
        #[instruction(args:ConfigTransactionCreateArgs)]
        pub struct ConfigTransactionCreate<'info> {
            #[account(
                mut,
                seeds = [SEED_PREFIX,
                SEED_MULTISIG,
                multisig.create_key.as_ref()],
                bump = multisig.bump,
            )]
            pub multisig: Account<'info, Multisig>,
            #[account(
                init,
                payer = creator,
                space = ConfigTransaction::size(&args.actions),
                seeds = [SEED_PREFIX,
                multisig.key().as_ref(),
                SEED_TRANSACTION,
                &multisig.transaction_index.checked_add(1).unwrap().to_le_bytes(),
                ],
                bump
            )]
            pub transaction: Account<'info, ConfigTransaction>,
            #[account(mut)]
            pub creator: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        impl<'info> kani::Arbitrary for ConfigTransactionCreate<'info> {
            fn any() -> Self {
                Self {
                    multisig: kani::any(),
                    transaction: kani::any(),
                    creator: kani::any(),
                    system_program: kani::any(),
                }
            }
        }
        impl<'info> ConfigTransactionCreate<'info> {
            pub fn __pre_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
            }
        }
        impl<'info> ConfigTransactionCreate<'info> {
            pub fn __post_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
                    && self.transaction.account._check_invariant()
            }
        }
        impl<'info> ConfigTransactionCreate<'info> {
            pub fn __check_constraints(
                &self,
                args: ConfigTransactionCreateArgs,
            ) -> bool {
                true
            }
        }
        impl ConfigTransactionCreate<'_> {
            fn validate(&self) -> Result<()> {
                if self.multisig.config_authority != Pubkey::default() {
                    return Err(Error::Generic);
                }
                if !(self.multisig.is_member(self.creator.key()).is_some()) {
                    return Err(Error::Generic);
                }
                if !(self
                    .multisig
                    .member_has_permission(self.creator.key(), Permission::Initiate))
                {
                    return Err(Error::Generic);
                }
                Ok(())
            }
            /// Create a new config transaction.
            pub fn config_transaction_create(
                ctx: Context<Self>,
                args: ConfigTransactionCreateArgs,
            ) -> Result<()> {
                ctx.accounts.validate()?;
                if !(!args.actions.is_empty()) {
                    return Err(Error::Generic);
                }
                let multisig = &mut ctx.accounts.multisig;
                let transaction = &mut ctx.accounts.transaction;
                let creator = &mut ctx.accounts.creator;
                let multisig_key = multisig.key();
                let transaction_index = multisig
                    .transaction_index
                    .checked_add(1)
                    .unwrap();
                transaction.multisig = multisig_key;
                transaction.creator = creator.key();
                transaction.index = transaction_index;
                transaction.bump = *ctx.bumps.get("transaction").unwrap();
                transaction.actions = args.actions;
                multisig.transaction_index = transaction_index;
                multisig.invariant()?;
                ();
                Ok(())
            }
        }
    }
    mod config_transaction_execute {
        use anchor_lang::prelude::*;
        use crate::errors::*;
        use crate::id;
        use crate::state::*;
        use crate::utils::*;
        pub struct ConfigTransactionExecute<'info> {
            /// The multisig account that owns the transaction.
            #[account(
                mut,
                seeds = [SEED_PREFIX,
                SEED_MULTISIG,
                multisig.create_key.as_ref()],
                bump = multisig.bump,
            )]
            pub multisig: Box<Account<'info, Multisig>>,
            /// One of the multisig members with `Execute` permission.
            pub member: Signer<'info>,
            /// The proposal account associated with the transaction.
            #[account(
                mut,
                seeds = [SEED_PREFIX,
                multisig.key().as_ref(),
                SEED_TRANSACTION,
                &transaction.index.to_le_bytes(),
                SEED_PROPOSAL,
                ],
                bump = proposal.bump,
            )]
            pub proposal: Account<'info, Proposal>,
            /// The transaction to execute.
            #[account(
                mut,
                seeds = [SEED_PREFIX,
                multisig.key().as_ref(),
                SEED_TRANSACTION,
                &transaction.index.to_le_bytes(),
                ],
                bump = transaction.bump,
            )]
            pub transaction: Account<'info, ConfigTransaction>,
            /// The account that will be charged/credited in case the config transaction causes space reallocation,
            /// for example when adding a new member, adding or removing a spending limit.
            /// This is usually the same as `member`, but can be a different account if needed.
            #[account(mut)]
            pub rent_payer: Option<Signer<'info>>,
            /// We might need it in case reallocation is needed.
            pub system_program: Option<Program<'info, System>>,
        }
        impl<'info> kani::Arbitrary for ConfigTransactionExecute<'info> {
            fn any() -> Self {
                Self {
                    multisig: Box::new(kani::any()),
                    member: kani::any(),
                    proposal: kani::any(),
                    transaction: kani::any(),
                    rent_payer: kani::any(),
                    system_program: kani::any(),
                }
            }
        }
        impl<'info> ConfigTransactionExecute<'info> {
            pub fn __pre_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
                    && self.proposal.account._check_invariant()
                    && self.transaction.account._check_invariant()
            }
        }
        impl<'info> ConfigTransactionExecute<'info> {
            pub fn __post_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
                    && self.proposal.account._check_invariant()
                    && self.transaction.account._check_invariant()
            }
        }
        impl<'info> ConfigTransactionExecute<'info> {
            pub fn __check_constraints(&self) -> bool {
                true
            }
        }
        impl<'info> ConfigTransactionExecute<'info> {
            fn validate(&self) -> Result<()> {
                let Self { multisig, proposal, member, .. } = self;
                if !(multisig.is_member(member.key()).is_some()) {
                    return Err(Error::Generic);
                }
                if !(multisig.member_has_permission(member.key(), Permission::Execute)) {
                    return Err(Error::Generic);
                }
                match proposal.status {
                    ProposalStatus::Approved { timestamp } => {
                        let now = Clock::get()?.unix_timestamp;
                        kani::assume(now > timestamp);
                        kani::assume(timestamp > 0);
                        if !(now.checked_sub(timestamp).unwrap()
                            >= i64::from(multisig.time_lock))
                        {
                            return Err(Error::Generic);
                        }
                    }
                    _ => return Err(Error::Generic),
                }
                if !(proposal.transaction_index > multisig.stale_transaction_index) {
                    return Err(Error::Generic);
                }
                Ok(())
            }
            /// Execute the multisig transaction.
            /// The transaction must be `Approved`.
            pub fn config_transaction_execute(
                ctx: Context<'_, '_, '_, 'info, Self>,
            ) -> Result<()> {
                ctx.accounts.validate()?;
                let multisig = &mut ctx.accounts.multisig;
                let transaction = &mut ctx.accounts.transaction;
                let proposal = &mut ctx.accounts.proposal;
                let rent = Rent::get()?;
                let new_members_length = members_length_after_actions(
                    multisig.members.len(),
                    &transaction.actions,
                );
                if new_members_length > multisig.members.len() {
                    let rent_payer = &ctx
                        .accounts
                        .rent_payer
                        .as_ref()
                        .ok_or(MultisigError::MissingAccount)?;
                    let system_program = &ctx
                        .accounts
                        .system_program
                        .as_ref()
                        .ok_or(MultisigError::MissingAccount)?;
                }
                for action in transaction.actions.iter() {
                    match action {
                        ConfigAction::AddMember { new_member } => {
                            multisig.add_member(new_member.to_owned());
                            multisig.invalidate_prior_transactions();
                        }
                        ConfigAction::RemoveMember { old_member } => {
                            kani::assume(multisig.is_member(*old_member).is_some());
                            multisig.remove_member(old_member.to_owned())?;
                            multisig.invalidate_prior_transactions();
                        }
                        ConfigAction::ChangeThreshold { new_threshold } => {
                            multisig.threshold = *new_threshold;
                            multisig.invalidate_prior_transactions();
                        }
                        ConfigAction::SetTimeLock { new_time_lock } => {
                            multisig.time_lock = *new_time_lock;
                            multisig.invalidate_prior_transactions();
                        }
                        ConfigAction::AddSpendingLimit {
                            create_key,
                            vault_index,
                            mint,
                            amount,
                            period,
                            members,
                            destinations,
                        } => {
                            let (spending_limit_key, spending_limit_bump) = Pubkey::find_program_address(
                                &[
                                    &SEED_PREFIX,
                                    multisig.key().as_ref(),
                                    &SEED_SPENDING_LIMIT,
                                    create_key.as_ref(),
                                ],
                                ctx.program_id,
                            );
                            kani::assume(
                                ctx
                                    .remaining_accounts
                                    .iter()
                                    .find(|acc| acc.key == &spending_limit_key)
                                    .is_some(),
                            );
                            let spending_limit_info = ctx
                                .remaining_accounts
                                .iter()
                                .find(|acc| acc.key == &spending_limit_key)
                                .ok_or(MultisigError::MissingAccount)?;
                            let rent_payer = &ctx
                                .accounts
                                .rent_payer
                                .as_ref()
                                .ok_or(MultisigError::MissingAccount)?;
                            let system_program = &ctx
                                .accounts
                                .system_program
                                .as_ref()
                                .ok_or(MultisigError::MissingAccount)?;
                            create_account(
                                rent_payer,
                                spending_limit_info,
                                system_program,
                                &id(),
                                &rent,
                                SpendingLimit::size(members.len(), destinations.len()),
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            SEED_PREFIX.to_vec().into(),
                                            multisig.key().as_ref().to_vec().into(),
                                            SEED_SPENDING_LIMIT.to_vec().into(),
                                            create_key.as_ref().to_vec().into(),
                                            <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([spending_limit_bump]),
                                                )
                                                .into(),
                                        ]),
                                    )
                                    .into(),
                            )?;
                            let mut members = members.to_vec();
                            kani::assume(members.windows(2).all(|w| w[0] < w[1]));
                            let spending_limit = SpendingLimit {
                                multisig: multisig.key().to_owned(),
                                create_key: create_key.to_owned(),
                                vault_index: *vault_index,
                                amount: *amount,
                                mint: *mint,
                                period: *period,
                                remaining_amount: *amount,
                                last_reset: Clock::get()?.unix_timestamp,
                                bump: spending_limit_bump,
                                members: members.into(),
                                destinations: destinations.to_vec().into(),
                            };
                            spending_limit.invariant()?;
                            spending_limit
                                .try_serialize(
                                    &mut &mut spending_limit_info.data.borrow_mut()[..],
                                )?;
                        }
                        ConfigAction::RemoveSpendingLimit {
                            spending_limit: spending_limit_key,
                        } => {
                            let spending_limit_info = ctx
                                .remaining_accounts
                                .iter()
                                .find(|acc| acc.key == spending_limit_key)
                                .ok_or(MultisigError::MissingAccount)?;
                            let rent_payer = &ctx
                                .accounts
                                .rent_payer
                                .as_ref()
                                .ok_or(MultisigError::MissingAccount)?;
                            let spending_limit = Account::<
                                SpendingLimit,
                            >::try_from(spending_limit_info)?;
                            if spending_limit.multisig != multisig.key() {
                                return Err(Error::Generic);
                            }
                            spending_limit.close(rent_payer.to_account_info())?;
                        }
                        _ => {}
                    }
                }
                if usize::from(multisig.threshold) > multisig.members.len() {
                    multisig
                        .threshold = multisig
                        .members
                        .len()
                        .try_into()
                        .expect("didn't expect more than `u16::MAX` members");
                }
                kani::assume(
                    multisig.members.windows(2).all(|win| win[0].key < win[1].key),
                );
                multisig.invariant()?;
                proposal
                    .status = ProposalStatus::Executed {
                    timestamp: Clock::get()?.unix_timestamp,
                };
                Ok(())
            }
        }
        fn members_length_after_actions(
            members_length: usize,
            actions: &[ConfigAction],
        ) -> usize {
            let members_delta: isize = actions
                .iter()
                .fold(
                    0,
                    |acc, action| match action {
                        ConfigAction::AddMember { .. } => {
                            acc.checked_add(1).expect("overflow")
                        }
                        ConfigAction::RemoveMember { .. } => {
                            acc.checked_sub(1).expect("overflow")
                        }
                        ConfigAction::ChangeThreshold { .. } => acc,
                        ConfigAction::SetTimeLock { .. } => acc,
                        ConfigAction::AddSpendingLimit { .. } => acc,
                        ConfigAction::RemoveSpendingLimit { .. } => acc,
                        ConfigAction::NoAction => acc,
                    },
                );
            kani::assume(members_delta.checked_abs().is_some());
            let abs_members_delta = usize::try_from(
                    members_delta.checked_abs().expect("overflow"),
                )
                .expect("overflow");
            if members_delta.is_negative() {
                kani::assume(members_length.checked_sub(abs_members_delta).is_some());
                members_length.checked_sub(abs_members_delta).expect("overflow")
            } else {
                kani::assume(members_length.checked_add(abs_members_delta).is_some());
                members_length.checked_add(abs_members_delta).expect("overflow")
            }
        }
    }
    mod multisig_config {
        use anchor_lang::prelude::*;
        use crate::errors::*;
        use crate::state::*;
        pub struct MultisigAddMemberArgs {
            pub new_member: Member,
        }
        impl borsh::ser::BorshSerialize for MultisigAddMemberArgs
        where
            Member: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.new_member, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for MultisigAddMemberArgs
        where
            Member: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    new_member: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        impl kani::Arbitrary for MultisigAddMemberArgs {
            fn any() -> Self {
                MultisigAddMemberArgs {
                    new_member: kani::any(),
                }
            }
        }
        pub struct MultisigRemoveMemberArgs {
            pub old_member: Pubkey,
        }
        impl borsh::ser::BorshSerialize for MultisigRemoveMemberArgs
        where
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.old_member, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for MultisigRemoveMemberArgs
        where
            Pubkey: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    old_member: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        impl kani::Arbitrary for MultisigRemoveMemberArgs {
            fn any() -> Self {
                MultisigRemoveMemberArgs {
                    old_member: kani::any(),
                }
            }
        }
        pub struct MultisigChangeThresholdArgs {
            new_threshold: u16,
        }
        impl borsh::ser::BorshSerialize for MultisigChangeThresholdArgs
        where
            u16: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.new_threshold, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for MultisigChangeThresholdArgs
        where
            u16: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    new_threshold: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        impl kani::Arbitrary for MultisigChangeThresholdArgs {
            fn any() -> Self {
                MultisigChangeThresholdArgs {
                    new_threshold: kani::any(),
                }
            }
        }
        pub struct MultisigSetTimeLockArgs {
            time_lock: u32,
        }
        impl borsh::ser::BorshSerialize for MultisigSetTimeLockArgs
        where
            u32: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.time_lock, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for MultisigSetTimeLockArgs
        where
            u32: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    time_lock: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        impl kani::Arbitrary for MultisigSetTimeLockArgs {
            fn any() -> Self {
                MultisigSetTimeLockArgs {
                    time_lock: kani::any(),
                }
            }
        }
        pub struct MultisigSetConfigAuthorityArgs {
            config_authority: Pubkey,
        }
        impl borsh::ser::BorshSerialize for MultisigSetConfigAuthorityArgs
        where
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.config_authority, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for MultisigSetConfigAuthorityArgs
        where
            Pubkey: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    config_authority: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                })
            }
        }
        impl kani::Arbitrary for MultisigSetConfigAuthorityArgs {
            fn any() -> Self {
                MultisigSetConfigAuthorityArgs {
                    config_authority: kani::any(),
                }
            }
        }
        pub struct MultisigConfig<'info> {
            #[account(
                mut,
                seeds = [SEED_PREFIX,
                SEED_MULTISIG,
                multisig.create_key.as_ref()],
                bump = multisig.bump,
            )]
            pub multisig: Account<'info, Multisig>,
            /// Multisig `config_authority` that must authorize the configuration change.
            pub config_authority: Signer<'info>,
            /// The account that will be charged or credited in case the multisig account needs to reallocate space,
            /// for example when adding a new member or a spending limit.
            /// This is usually the same as `config_authority`, but can be a different account if needed.
            #[account(mut)]
            pub rent_payer: Option<Signer<'info>>,
            /// We might need it in case reallocation is needed.
            pub system_program: Option<Program<'info, System>>,
        }
        impl<'info> kani::Arbitrary for MultisigConfig<'info> {
            fn any() -> Self {
                Self {
                    multisig: kani::any(),
                    config_authority: kani::any(),
                    rent_payer: kani::any(),
                    system_program: kani::any(),
                }
            }
        }
        impl<'info> MultisigConfig<'info> {
            pub fn __pre_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
            }
        }
        impl<'info> MultisigConfig<'info> {
            pub fn __post_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
            }
        }
        impl<'info> MultisigConfig<'info> {
            pub fn __check_constraints(&self) -> bool {
                true
            }
        }
        impl MultisigConfig<'_> {
            fn validate(&self) -> Result<()> {
                if self.config_authority.key() != self.multisig.config_authority {
                    return Err(Error::Generic);
                }
                Ok(())
            }
            /// Add a member/key to the multisig and reallocate space if necessary.
            ///
            /// NOTE: This instruction must be called only by the `config_authority` if one is set (Controlled Multisig).
            ///       Uncontrolled Mustisigs should use `config_transaction_create` instead.
            pub fn multisig_add_member(
                ctx: Context<Self>,
                args: MultisigAddMemberArgs,
            ) -> Result<()> {
                ctx.accounts.validate()?;
                let MultisigAddMemberArgs { new_member, .. } = args;
                let system_program = &ctx
                    .accounts
                    .system_program
                    .as_ref()
                    .ok_or(MultisigError::MissingAccount)?;
                let rent_payer = &ctx
                    .accounts
                    .rent_payer
                    .as_ref()
                    .ok_or(MultisigError::MissingAccount)?;
                let multisig = &mut ctx.accounts.multisig;
                let reallocated = Multisig::realloc_if_needed(
                    multisig.to_account_info(),
                    multisig.members.len() + 1,
                    rent_payer.to_account_info(),
                    system_program.to_account_info(),
                )?;
                if reallocated {
                    multisig.reload()?;
                }
                multisig.add_member(new_member);
                multisig.invariant()?;
                multisig.invalidate_prior_transactions();
                Ok(())
            }
            /// Remove a member/key from the multisig.
            ///
            /// NOTE: This instruction must be called only by the `config_authority` if one is set (Controlled Multisig).
            ///       Uncontrolled Mustisigs should use `config_transaction_create` instead.
            pub fn multisig_remove_member(
                ctx: Context<Self>,
                args: MultisigRemoveMemberArgs,
            ) -> Result<()> {
                ctx.accounts.validate()?;
                let multisig = &mut ctx.accounts.multisig;
                if !(multisig.members.len() > 1) {
                    return Err(Error::Generic);
                }
                multisig.remove_member(args.old_member)?;
                if usize::from(multisig.threshold) > multisig.members.len() {
                    multisig
                        .threshold = multisig
                        .members
                        .len()
                        .try_into()
                        .expect("didn't expect more that `u16::MAX` members");
                }
                multisig.invariant()?;
                multisig.invalidate_prior_transactions();
                Ok(())
            }
            /// NOTE: This instruction must be called only by the `config_authority` if one is set (Controlled Multisig).
            ///       Uncontrolled Mustisigs should use `config_transaction_create` instead.
            pub fn multisig_change_threshold(
                ctx: Context<Self>,
                args: MultisigChangeThresholdArgs,
            ) -> Result<()> {
                ctx.accounts.validate()?;
                let MultisigChangeThresholdArgs { new_threshold, .. } = args;
                let multisig = &mut ctx.accounts.multisig;
                multisig.threshold = new_threshold;
                multisig.invariant()?;
                multisig.invalidate_prior_transactions();
                Ok(())
            }
            /// Set the `time_lock` config parameter for the multisig.
            ///
            /// NOTE: This instruction must be called only by the `config_authority` if one is set (Controlled Multisig).
            ///       Uncontrolled Mustisigs should use `config_transaction_create` instead.
            pub fn multisig_set_time_lock(
                ctx: Context<Self>,
                args: MultisigSetTimeLockArgs,
            ) -> Result<()> {
                ctx.accounts.validate()?;
                let multisig = &mut ctx.accounts.multisig;
                multisig.time_lock = args.time_lock;
                multisig.invariant()?;
                multisig.invalidate_prior_transactions();
                Ok(())
            }
            /// Set the multisig `config_authority`.
            ///
            /// NOTE: This instruction must be called only by the `config_authority` if one is set (Controlled Multisig).
            ///       Uncontrolled Mustisigs should use `config_transaction_create` instead.
            pub fn multisig_set_config_authority(
                ctx: Context<Self>,
                args: MultisigSetConfigAuthorityArgs,
            ) -> Result<()> {
                ctx.accounts.validate()?;
                let multisig = &mut ctx.accounts.multisig;
                multisig.config_authority = args.config_authority;
                multisig.invariant()?;
                multisig.invalidate_prior_transactions();
                Ok(())
            }
        }
    }
    mod multisig_create {
        use anchor_lang::prelude::*;
        use crate::state::*;
        pub struct MultisigCreateArgs {
            /// The authority that can configure the multisig: add/remove members, change the threshold, etc.
            /// Should be set to `None` for autonomous multisigs.
            pub config_authority: Option<Pubkey>,
            /// The number of signatures required to execute a transaction.
            pub threshold: u16,
            /// The members of the multisig.
            pub members: Vec<Member>,
            /// How many seconds must pass between transaction voting settlement and execution.
            pub time_lock: u32,
        }
        impl borsh::ser::BorshSerialize for MultisigCreateArgs
        where
            Option<Pubkey>: borsh::ser::BorshSerialize,
            u16: borsh::ser::BorshSerialize,
            Vec<Member>: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.config_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.threshold, writer)?;
                borsh::BorshSerialize::serialize(&self.members, writer)?;
                borsh::BorshSerialize::serialize(&self.time_lock, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for MultisigCreateArgs
        where
            Option<Pubkey>: borsh::BorshDeserialize,
            u16: borsh::BorshDeserialize,
            Vec<Member>: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    config_authority: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    threshold: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    members: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    time_lock: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        impl kani::Arbitrary for MultisigCreateArgs {
            fn any() -> Self {
                MultisigCreateArgs {
                    config_authority: kani::any(),
                    threshold: kani::any(),
                    members: kani::any(),
                    time_lock: kani::any(),
                }
            }
        }
        #[instruction(args:MultisigCreateArgs)]
        pub struct MultisigCreate<'info> {
            #[account(
                init,
                payer = creator,
                space = Multisig::size(args.members.len()),
                seeds = [SEED_PREFIX,
                SEED_MULTISIG,
                create_key.key().as_ref()],
                bump
            )]
            pub multisig: Account<'info, Multisig>,
            /// A random public key that is used as a seed for the Multisig PDA.
            /// CHECK: This can be any random public key.
            pub create_key: AccountInfo<'info>,
            /// The creator of the multisig.
            #[account(mut)]
            pub creator: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        impl<'info> kani::Arbitrary for MultisigCreate<'info> {
            fn any() -> Self {
                Self {
                    multisig: kani::any(),
                    create_key: kani::any(),
                    creator: kani::any(),
                    system_program: kani::any(),
                }
            }
        }
        impl<'info> MultisigCreate<'info> {
            pub fn __pre_invariants(&self) -> bool {
                true
            }
        }
        impl<'info> MultisigCreate<'info> {
            pub fn __post_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
            }
        }
        impl<'info> MultisigCreate<'info> {
            pub fn __check_constraints(&self, args: MultisigCreateArgs) -> bool {
                true
            }
        }
        impl MultisigCreate<'_> {
            fn validate(&self) -> Result<()> {
                Ok(())
            }
            /// Creates a multisig.
            pub fn multisig_create(
                ctx: Context<Self>,
                args: MultisigCreateArgs,
            ) -> Result<()> {
                ctx.accounts.validate()?;
                let mut members = args.members;
                let multisig = &mut ctx.accounts.multisig;
                multisig.config_authority = args.config_authority.unwrap_or_default();
                multisig.threshold = args.threshold;
                multisig.time_lock = args.time_lock;
                multisig.transaction_index = 0;
                multisig.stale_transaction_index = 0;
                multisig.create_key = ctx.accounts.create_key.to_account_info().key();
                multisig.bump = *ctx.bumps.get("multisig").unwrap();
                multisig.members = members;
                multisig.invariant()?;
                Ok(())
            }
        }
    }
    mod proposal_activate {
        use anchor_lang::prelude::*;
        use crate::state::*;
        pub struct ProposalActivate<'info> {
            #[account(
                seeds = [SEED_PREFIX,
                SEED_MULTISIG,
                multisig.create_key.as_ref()],
                bump = multisig.bump,
            )]
            pub multisig: Account<'info, Multisig>,
            #[account(mut)]
            pub member: Signer<'info>,
            #[account(
                mut,
                seeds = [SEED_PREFIX,
                multisig.key().as_ref(),
                SEED_TRANSACTION,
                &proposal.transaction_index.to_le_bytes(),
                SEED_PROPOSAL,
                ],
                bump = proposal.bump,
            )]
            pub proposal: Account<'info, Proposal>,
        }
        impl<'info> kani::Arbitrary for ProposalActivate<'info> {
            fn any() -> Self {
                Self {
                    multisig: kani::any(),
                    member: kani::any(),
                    proposal: kani::any(),
                }
            }
        }
        impl<'info> ProposalActivate<'info> {
            pub fn __pre_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
                    && self.proposal.account._check_invariant()
            }
        }
        impl<'info> ProposalActivate<'info> {
            pub fn __post_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
                    && self.proposal.account._check_invariant()
            }
        }
        impl<'info> ProposalActivate<'info> {
            pub fn __check_constraints(&self) -> bool {
                true
            }
        }
        impl ProposalActivate<'_> {
            fn validate(&self) -> Result<()> {
                let Self { multisig, proposal, member, .. } = self;
                if !(multisig.is_member(member.key()).is_some()) {
                    return Err(Error::Generic);
                }
                if !(multisig.member_has_permission(member.key(), Permission::Initiate))
                {
                    return Err(Error::Generic);
                }
                if !(match proposal.status {
                    ProposalStatus::Draft { .. } => true,
                    _ => false,
                }) {
                    return Err(Error::Generic);
                }
                if !(proposal.transaction_index > multisig.stale_transaction_index) {
                    return Err(Error::Generic);
                }
                Ok(())
            }
            /// Update status of a multisig proposal from `Draft` to `Active`.
            pub fn proposal_activate(ctx: Context<Self>) -> Result<()> {
                ctx.accounts.validate()?;
                ctx
                    .accounts
                    .proposal
                    .status = ProposalStatus::Active {
                    timestamp: Clock::get()?.unix_timestamp,
                };
                Ok(())
            }
        }
    }
    mod proposal_create {
        use anchor_lang::prelude::*;
        use crate::state::*;
        pub struct ProposalCreateArgs {
            /// Index of the multisig transaction this proposal is associated with.
            pub transaction_index: u64,
            /// Whether the proposal should be initialized with status `Draft`.
            pub draft: bool,
        }
        impl borsh::ser::BorshSerialize for ProposalCreateArgs
        where
            u64: borsh::ser::BorshSerialize,
            bool: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.transaction_index, writer)?;
                borsh::BorshSerialize::serialize(&self.draft, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for ProposalCreateArgs
        where
            u64: borsh::BorshDeserialize,
            bool: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    transaction_index: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    draft: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        impl kani::Arbitrary for ProposalCreateArgs {
            fn any() -> Self {
                ProposalCreateArgs {
                    transaction_index: kani::any(),
                    draft: kani::any(),
                }
            }
        }
        #[instruction(args:ProposalCreateArgs)]
        pub struct ProposalCreate<'info> {
            #[account(
                seeds = [SEED_PREFIX,
                SEED_MULTISIG,
                multisig.create_key.as_ref()],
                bump = multisig.bump,
            )]
            pub multisig: Account<'info, Multisig>,
            #[account(
                init,
                payer = creator,
                space = Proposal::size(multisig.members.len()),
                seeds = [SEED_PREFIX,
                multisig.key().as_ref(),
                SEED_TRANSACTION,
                &args.transaction_index.to_le_bytes(),
                SEED_PROPOSAL,
                ],
                bump
            )]
            pub proposal: Account<'info, Proposal>,
            #[account(mut)]
            pub creator: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        impl<'info> kani::Arbitrary for ProposalCreate<'info> {
            fn any() -> Self {
                Self {
                    multisig: kani::any(),
                    proposal: kani::any(),
                    creator: kani::any(),
                    system_program: kani::any(),
                }
            }
        }
        impl<'info> ProposalCreate<'info> {
            pub fn __pre_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
            }
        }
        impl<'info> ProposalCreate<'info> {
            pub fn __post_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
                    && self.proposal.account._check_invariant()
            }
        }
        impl<'info> ProposalCreate<'info> {
            pub fn __check_constraints(&self, args: ProposalCreateArgs) -> bool {
                true
            }
        }
        impl ProposalCreate<'_> {
            fn validate(&self, args: &ProposalCreateArgs) -> Result<()> {
                let Self { multisig, creator, .. } = self;
                let creator_key = creator.key();
                if !(args.transaction_index <= multisig.transaction_index) {
                    return Err(Error::Generic);
                }
                if !(args.transaction_index > multisig.stale_transaction_index) {
                    return Err(Error::Generic);
                }
                if !(self.multisig.is_member(self.creator.key()).is_some()) {
                    return Err(Error::Generic);
                }
                if !(self
                    .multisig
                    .member_has_permission(creator_key, Permission::Initiate)
                    || self
                        .multisig
                        .member_has_permission(creator_key, Permission::Vote))
                {
                    return Err(Error::Generic);
                }
                Ok(())
            }
            /// Create a new multisig proposal.
            pub fn proposal_create(
                ctx: Context<Self>,
                args: ProposalCreateArgs,
            ) -> Result<()> {
                ctx.accounts.validate(&args)?;
                let proposal = &mut ctx.accounts.proposal;
                proposal.multisig = ctx.accounts.multisig.key();
                proposal.transaction_index = args.transaction_index;
                proposal
                    .status = if args.draft {
                    ProposalStatus::Draft {
                        timestamp: Clock::get()?.unix_timestamp,
                    }
                } else {
                    ProposalStatus::Active {
                        timestamp: Clock::get()?.unix_timestamp,
                    }
                };
                proposal.bump = *ctx.bumps.get("proposal").unwrap();
                proposal.approved = Vec::new();
                proposal.rejected = Vec::new();
                proposal.cancelled = Vec::new();
                Ok(())
            }
        }
    }
    mod proposal_vote {
        use anchor_lang::prelude::*;
        use crate::state::*;
        pub struct ProposalVoteArgs {}
        impl borsh::ser::BorshSerialize for ProposalVoteArgs {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for ProposalVoteArgs {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {})
            }
        }
        impl kani::Arbitrary for ProposalVoteArgs {
            fn any() -> Self {
                ProposalVoteArgs {}
            }
        }
        pub struct ProposalVote<'info> {
            #[account(
                seeds = [SEED_PREFIX,
                SEED_MULTISIG,
                multisig.create_key.as_ref()],
                bump = multisig.bump,
            )]
            pub multisig: Account<'info, Multisig>,
            #[account(mut)]
            pub member: Signer<'info>,
            #[account(
                mut,
                seeds = [SEED_PREFIX,
                multisig.key().as_ref(),
                SEED_TRANSACTION,
                &proposal.transaction_index.to_le_bytes(),
                SEED_PROPOSAL,
                ],
                bump = proposal.bump,
            )]
            pub proposal: Account<'info, Proposal>,
            pub system_program: Program<'info, System>,
        }
        impl<'info> kani::Arbitrary for ProposalVote<'info> {
            fn any() -> Self {
                Self {
                    multisig: kani::any(),
                    member: kani::any(),
                    proposal: kani::any(),
                    system_program: kani::any(),
                }
            }
        }
        impl<'info> ProposalVote<'info> {
            pub fn __pre_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
                    && self.proposal.account._check_invariant()
            }
        }
        impl<'info> ProposalVote<'info> {
            pub fn __post_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
                    && self.proposal.account._check_invariant()
            }
        }
        impl<'info> ProposalVote<'info> {
            pub fn __check_constraints(&self) -> bool {
                true
            }
        }
        impl ProposalVote<'_> {
            fn validate(&self, vote: Vote) -> Result<()> {
                let Self { multisig, proposal, member, .. } = self;
                if !(multisig.is_member(member.key()).is_some()) {
                    return Err(Error::Generic);
                }
                if !(multisig.member_has_permission(member.key(), Permission::Vote)) {
                    return Err(Error::Generic);
                }
                match vote {
                    Vote::Approve | Vote::Reject => {
                        if !(match proposal.status {
                            ProposalStatus::Active { .. } => true,
                            _ => false,
                        }) {
                            return Err(Error::Generic);
                        }
                        if !(proposal.transaction_index
                            > multisig.stale_transaction_index)
                        {
                            return Err(Error::Generic);
                        }
                    }
                    Vote::Cancel => {
                        if !(match proposal.status {
                            ProposalStatus::Approved { .. } => true,
                            _ => false,
                        }) {
                            return Err(Error::Generic);
                        }
                    }
                }
                Ok(())
            }
            /// Approve a multisig proposal on behalf of the `member`.
            /// The proposal must be `Active`.
            pub fn proposal_approve(
                ctx: Context<Self>,
                _args: ProposalVoteArgs,
            ) -> Result<()> {
                ctx.accounts.validate(Vote::Approve)?;
                let multisig = &mut ctx.accounts.multisig;
                let proposal = &mut ctx.accounts.proposal;
                let member = &mut ctx.accounts.member;
                proposal.approve(member.key(), usize::from(multisig.threshold))?;
                Ok(())
            }
            /// Reject a multisig proposal on behalf of the `member`.
            /// The proposal must be `Active`.
            pub fn proposal_reject(
                ctx: Context<Self>,
                _args: ProposalVoteArgs,
            ) -> Result<()> {
                ctx.accounts.validate(Vote::Reject)?;
                let multisig = &mut ctx.accounts.multisig;
                let proposal = &mut ctx.accounts.proposal;
                let member = &mut ctx.accounts.member;
                let cutoff = Multisig::cutoff(multisig);
                proposal.reject(member.key(), cutoff)?;
                Ok(())
            }
            /// Cancel a multisig proposal on behalf of the `member`.
            /// The proposal must be `Approved`.
            pub fn proposal_cancel(
                ctx: Context<Self>,
                _args: ProposalVoteArgs,
            ) -> Result<()> {
                ctx.accounts.validate(Vote::Cancel)?;
                let multisig = &mut ctx.accounts.multisig;
                let proposal = &mut ctx.accounts.proposal;
                let member = &mut ctx.accounts.member;
                proposal.cancel(member.key(), usize::from(multisig.threshold))?;
                Ok(())
            }
        }
        pub enum Vote {
            Approve,
            Reject,
            Cancel,
        }
    }
    mod spending_limit_use {
        use anchor_lang::prelude::*;
        use anchor_spl::token_2022::TransferChecked;
        use anchor_spl::token_interface;
        use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
        use crate::errors::*;
        use crate::state::*;
        pub struct SpendingLimitUseArgs {
            /// Amount of tokens to transfer.
            pub amount: u64,
            /// Decimals of the token mint. Used for double-checking against incorrect order of magnitude of `amount`.
            pub decimals: u8,
        }
        impl borsh::ser::BorshSerialize for SpendingLimitUseArgs
        where
            u64: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.amount, writer)?;
                borsh::BorshSerialize::serialize(&self.decimals, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for SpendingLimitUseArgs
        where
            u64: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    decimals: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SpendingLimitUseArgs {
            #[inline]
            fn clone(&self) -> SpendingLimitUseArgs {
                SpendingLimitUseArgs {
                    amount: ::core::clone::Clone::clone(&self.amount),
                    decimals: ::core::clone::Clone::clone(&self.decimals),
                }
            }
        }
        impl kani::Arbitrary for SpendingLimitUseArgs {
            fn any() -> Self {
                SpendingLimitUseArgs {
                    amount: kani::any(),
                    decimals: kani::any(),
                }
            }
        }
        pub struct SpendingLimitUse<'info> {
            /// The multisig account the `spending_limit` is for.
            #[account(
                mut,
                seeds = [SEED_PREFIX,
                SEED_MULTISIG,
                multisig.create_key.as_ref()],
                bump = multisig.bump,
            )]
            pub multisig: Box<Account<'info, Multisig>>,
            pub member: Signer<'info>,
            /// The SpendingLimit account to use.
            #[account(
                mut,
                seeds = [SEED_PREFIX,
                multisig.key().as_ref(),
                SEED_SPENDING_LIMIT,
                spending_limit.create_key.key().as_ref(),
                ],
                bump = spending_limit.bump,
            )]
            pub spending_limit: Account<'info, SpendingLimit>,
            /// Multisig vault account to transfer tokens from.
            /// CHECK: All the required checks are done by checking the seeds.
            #[account(
                mut,
                seeds = [SEED_PREFIX,
                multisig.key().as_ref(),
                SEED_VAULT,
                &spending_limit.vault_index.to_le_bytes(),
                ],
                bump
            )]
            pub vault: AccountInfo<'info>,
            /// Destination account to transfer tokens to.
            /// CHECK: We do the checks in `SpendingLimitUse::validate`.
            #[account(mut)]
            pub destination: AccountInfo<'info>,
            /// In case `spending_limit.mint` is SOL.
            pub system_program: Option<Program<'info, System>>,
            /// The mint of the tokens to transfer in case `spending_limit.mint` is an SPL token.
            /// CHECK: We do the checks in `SpendingLimitUse::validate`.
            pub mint: Option<InterfaceAccount<'info, Mint>>,
            /// Multisig vault token account to transfer tokens from in case `spending_limit.mint` is an SPL token.
            #[account(mut, token::mint = mint, token::authority = vault)]
            pub vault_token_account: Option<InterfaceAccount<'info, TokenAccount>>,
            /// Destination token account in case `spending_limit.mint` is an SPL token.
            #[account(mut, token::mint = mint, token::authority = destination)]
            pub destination_token_account: Option<InterfaceAccount<'info, TokenAccount>>,
            /// In case `spending_limit.mint` is an SPL token.
            pub token_program: Option<Interface<'info, TokenInterface>>,
        }
        impl<'info> kani::Arbitrary for SpendingLimitUse<'info> {
            fn any() -> Self {
                Self {
                    multisig: Box::new(kani::any()),
                    member: kani::any(),
                    spending_limit: kani::any(),
                    vault: kani::any(),
                    destination: kani::any(),
                    system_program: kani::any(),
                    mint: kani::any(),
                    vault_token_account: kani::any(),
                    destination_token_account: kani::any(),
                    token_program: kani::any(),
                }
            }
        }
        impl<'info> SpendingLimitUse<'info> {
            pub fn __pre_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
                    && self.spending_limit.account._check_invariant()
            }
        }
        impl<'info> SpendingLimitUse<'info> {
            pub fn __post_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
                    && self.spending_limit.account._check_invariant()
            }
        }
        impl<'info> SpendingLimitUse<'info> {
            pub fn __check_constraints(&self) -> bool {
                true
            }
        }
        impl SpendingLimitUse<'_> {
            fn validate(&self) -> Result<()> {
                let Self { multisig, member, spending_limit, mint, .. } = self;
                if !(multisig.is_member(member.key()).is_some()) {
                    return Err(Error::Generic);
                }
                if !(spending_limit.members.contains(&member.key())) {
                    return Err(Error::Generic);
                }
                if spending_limit.mint == Pubkey::default() {
                    if !(mint.is_none()) {
                        return Err(Error::Generic);
                    }
                } else {
                    kani::assume(mint.is_some());
                    if !(spending_limit.mint == mint.as_ref().unwrap().key()) {
                        return Err(Error::Generic);
                    }
                }
                if !spending_limit.destinations.is_empty() {
                    if !(spending_limit.destinations.contains(&self.destination.key())) {
                        return Err(Error::Generic);
                    }
                }
                Ok(())
            }
            /// Use a spending limit to transfer tokens from a multisig vault to a destination account.
            pub fn spending_limit_use(
                ctx: Context<Self>,
                args: SpendingLimitUseArgs,
            ) -> Result<()> {
                ctx.accounts.validate()?;
                let spending_limit = &mut ctx.accounts.spending_limit;
                let vault = &mut ctx.accounts.vault;
                let destination = &mut ctx.accounts.destination;
                let multisig_key = ctx.accounts.multisig.key();
                let vault_bump = *ctx.bumps.get("vault").unwrap();
                let now = Clock::get()?.unix_timestamp;
                if let Some(reset_period) = spending_limit.period.to_seconds() {
                    kani::assume(now > spending_limit.last_reset);
                    let passed_since_last_reset = now
                        .checked_sub(spending_limit.last_reset)
                        .unwrap();
                    if passed_since_last_reset > reset_period {
                        spending_limit.remaining_amount = spending_limit.amount;
                        let periods_passed = passed_since_last_reset
                            .checked_div(reset_period)
                            .unwrap();
                        kani::assume(periods_passed.checked_mul(reset_period).is_some());
                        kani::assume(
                            spending_limit
                                .last_reset
                                .checked_add(
                                    periods_passed.checked_mul(reset_period).unwrap(),
                                )
                                .is_some(),
                        );
                        spending_limit
                            .last_reset = spending_limit
                            .last_reset
                            .checked_add(
                                periods_passed.checked_mul(reset_period).unwrap(),
                            )
                            .unwrap();
                    }
                }
                spending_limit
                    .remaining_amount = spending_limit
                    .remaining_amount
                    .checked_sub(args.amount)
                    .ok_or(MultisigError::SpendingLimitExceeded)?;
                if spending_limit.mint == Pubkey::default() {
                    let system_program = &ctx
                        .accounts
                        .system_program
                        .as_ref()
                        .ok_or(MultisigError::MissingAccount)?;
                    if !(args.decimals == 9) {
                        return Err(Error::Generic);
                    }
                    anchor_lang::system_program::transfer(
                        CpiContext::new_with_signer(
                            system_program.to_account_info(),
                            anchor_lang::system_program::Transfer {
                                from: vault.clone(),
                                to: destination.clone(),
                            },
                            &[
                                &[
                                    &SEED_PREFIX,
                                    multisig_key.as_ref(),
                                    &SEED_VAULT,
                                    &spending_limit.vault_index.to_le_bytes(),
                                    &[vault_bump],
                                ],
                            ],
                        ),
                        args.amount,
                    )?
                } else {
                    let mint = &ctx
                        .accounts
                        .mint
                        .as_ref()
                        .ok_or(MultisigError::MissingAccount)?;
                    let vault_token_account = &ctx
                        .accounts
                        .vault_token_account
                        .as_ref()
                        .ok_or(MultisigError::MissingAccount)?;
                    let destination_token_account = &ctx
                        .accounts
                        .destination_token_account
                        .as_ref()
                        .ok_or(MultisigError::MissingAccount)?;
                    let token_program = &ctx
                        .accounts
                        .token_program
                        .as_ref()
                        .ok_or(MultisigError::MissingAccount)?;
                    ();
                    token_interface::transfer_checked(
                        CpiContext::new_with_signer(
                            token_program.to_account_info(),
                            TransferChecked {
                                from: vault_token_account.to_account_info(),
                                mint: mint.to_account_info(),
                                to: destination_token_account.to_account_info(),
                                authority: vault.clone(),
                            },
                            &[
                                &[
                                    &SEED_PREFIX,
                                    multisig_key.as_ref(),
                                    &SEED_VAULT,
                                    &spending_limit.vault_index.to_le_bytes(),
                                    &[vault_bump],
                                ],
                            ],
                        ),
                        args.amount,
                        args.decimals,
                    )?;
                }
                Ok(())
            }
        }
    }
    mod vault_transaction_create {
        use anchor_lang::prelude::*;
        use crate::state::*;
        use crate::utils::*;
        pub struct VaultTransactionCreateArgs {
            /// Index of the vault this transaction belongs to.
            pub vault_index: u8,
            /// Number of ephemeral signing PDAs required by the transaction.
            pub ephemeral_signers: u8,
            pub transaction_message: Vec<u8>,
        }
        impl borsh::ser::BorshSerialize for VaultTransactionCreateArgs
        where
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            Vec<u8>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.vault_index, writer)?;
                borsh::BorshSerialize::serialize(&self.ephemeral_signers, writer)?;
                borsh::BorshSerialize::serialize(&self.transaction_message, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for VaultTransactionCreateArgs
        where
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            Vec<u8>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    vault_index: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    ephemeral_signers: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    transaction_message: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                })
            }
        }
        impl kani::Arbitrary for VaultTransactionCreateArgs {
            fn any() -> Self {
                VaultTransactionCreateArgs {
                    vault_index: kani::any(),
                    ephemeral_signers: kani::any(),
                    transaction_message: kani::any(),
                }
            }
        }
        #[instruction(args:VaultTransactionCreateArgs)]
        pub struct VaultTransactionCreate<'info> {
            #[account(
                mut,
                seeds = [SEED_PREFIX,
                SEED_MULTISIG,
                multisig.create_key.as_ref()],
                bump = multisig.bump,
            )]
            pub multisig: Account<'info, Multisig>,
            #[account(
                init,
                payer = creator,
                space = VaultTransaction::size(
                    args.ephemeral_signers,
                    &args.transaction_message
                )?,
                seeds = [SEED_PREFIX,
                multisig.key().as_ref(),
                SEED_TRANSACTION,
                &multisig.transaction_index.checked_add(1).unwrap().to_le_bytes(),
                ],
                bump
            )]
            pub transaction: Account<'info, VaultTransaction>,
            #[account(mut)]
            pub creator: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        impl<'info> kani::Arbitrary for VaultTransactionCreate<'info> {
            fn any() -> Self {
                Self {
                    multisig: kani::any(),
                    transaction: kani::any(),
                    creator: kani::any(),
                    system_program: kani::any(),
                }
            }
        }
        impl<'info> VaultTransactionCreate<'info> {
            pub fn __pre_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
            }
        }
        impl<'info> VaultTransactionCreate<'info> {
            pub fn __post_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
                    && self.transaction.account._check_invariant()
            }
        }
        impl<'info> VaultTransactionCreate<'info> {
            pub fn __check_constraints(&self, args: VaultTransactionCreateArgs) -> bool {
                true
            }
        }
        impl VaultTransactionCreate<'_> {
            fn validate(&self) -> Result<()> {
                let Self { multisig, creator, .. } = self;
                if !(multisig.is_member(creator.key()).is_some()) {
                    return Err(Error::Generic);
                }
                if !(multisig.member_has_permission(creator.key(), Permission::Initiate))
                {
                    return Err(Error::Generic);
                }
                Ok(())
            }
            /// Create a new vault transaction.
            pub fn vault_transaction_create(
                ctx: Context<Self>,
                args: VaultTransactionCreateArgs,
            ) -> Result<()> {
                ctx.accounts.validate()?;
                let multisig = &mut ctx.accounts.multisig;
                let transaction = &mut ctx.accounts.transaction;
                let creator = &mut ctx.accounts.creator;
                let multisig_key = multisig.key();
                let transaction_key = transaction.key();
                let vault_seeds = &[
                    SEED_PREFIX,
                    multisig_key.as_ref(),
                    SEED_VAULT,
                    &args.vault_index.to_le_bytes(),
                ];
                let (_, vault_bump) = Pubkey::find_program_address(
                    vault_seeds,
                    ctx.program_id,
                );
                let ephemeral_signer_bumps: Vec<u8> = (0..args.ephemeral_signers)
                    .map(|ephemeral_signer_index| {
                        let ephemeral_signer_seeds = &[
                            SEED_PREFIX,
                            transaction_key.as_ref(),
                            SEED_EPHEMERAL_SIGNER,
                            &ephemeral_signer_index.to_le_bytes(),
                        ];
                        let (_, bump) = Pubkey::find_program_address(
                            ephemeral_signer_seeds,
                            ctx.program_id,
                        );
                        bump
                    })
                    .collect();
                let transaction_index = multisig
                    .transaction_index
                    .checked_add(1)
                    .unwrap();
                transaction.multisig = multisig_key;
                transaction.creator = creator.key();
                transaction.index = transaction_index;
                transaction.bump = *ctx.bumps.get("transaction").unwrap();
                transaction.vault_index = args.vault_index;
                transaction.vault_bump = vault_bump;
                transaction.ephemeral_signer_bumps = ephemeral_signer_bumps;
                multisig.transaction_index = transaction_index;
                multisig.invariant()?;
                ();
                Ok(())
            }
        }
        /// Unvalidated instruction data, must be treated as untrusted.
        pub struct TransactionMessage {
            /// The number of signer pubkeys in the account_keys vec.
            pub num_signers: u8,
            /// The number of writable signer pubkeys in the account_keys vec.
            pub num_writable_signers: u8,
            /// The number of writable non-signer pubkeys in the account_keys vec.
            pub num_writable_non_signers: u8,
            /// The list of unique account public keys (including program IDs) that will be used in the provided instructions.
            pub account_keys: SmallVec<u8, Pubkey>,
            /// The list of instructions to execute.
            pub instructions: SmallVec<u8, CompiledInstruction>,
            /// List of address table lookups used to load additional accounts
            /// for this transaction.
            pub address_table_lookups: SmallVec<u8, MessageAddressTableLookup>,
        }
        impl borsh::de::BorshDeserialize for TransactionMessage
        where
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            SmallVec<u8, Pubkey>: borsh::BorshDeserialize,
            SmallVec<u8, CompiledInstruction>: borsh::BorshDeserialize,
            SmallVec<u8, MessageAddressTableLookup>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    num_signers: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    num_writable_signers: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    num_writable_non_signers: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    account_keys: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    instructions: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    address_table_lookups: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TransactionMessage {
            #[inline]
            fn clone(&self) -> TransactionMessage {
                TransactionMessage {
                    num_signers: ::core::clone::Clone::clone(&self.num_signers),
                    num_writable_signers: ::core::clone::Clone::clone(
                        &self.num_writable_signers,
                    ),
                    num_writable_non_signers: ::core::clone::Clone::clone(
                        &self.num_writable_non_signers,
                    ),
                    account_keys: ::core::clone::Clone::clone(&self.account_keys),
                    instructions: ::core::clone::Clone::clone(&self.instructions),
                    address_table_lookups: ::core::clone::Clone::clone(
                        &self.address_table_lookups,
                    ),
                }
            }
        }
        pub struct CompiledInstruction {
            pub program_id_index: u8,
            /// Indices into the tx's `account_keys` list indicating which accounts to pass to the instruction.
            pub account_indexes: SmallVec<u8, u8>,
            /// Instruction data.
            pub data: SmallVec<u16, u8>,
        }
        impl borsh::de::BorshDeserialize for CompiledInstruction
        where
            u8: borsh::BorshDeserialize,
            SmallVec<u8, u8>: borsh::BorshDeserialize,
            SmallVec<u16, u8>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    program_id_index: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    account_indexes: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    data: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CompiledInstruction {
            #[inline]
            fn clone(&self) -> CompiledInstruction {
                CompiledInstruction {
                    program_id_index: ::core::clone::Clone::clone(
                        &self.program_id_index,
                    ),
                    account_indexes: ::core::clone::Clone::clone(&self.account_indexes),
                    data: ::core::clone::Clone::clone(&self.data),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for CompiledInstruction {
            #[inline]
            fn default() -> CompiledInstruction {
                CompiledInstruction {
                    program_id_index: ::core::default::Default::default(),
                    account_indexes: ::core::default::Default::default(),
                    data: ::core::default::Default::default(),
                }
            }
        }
        /// Address table lookups describe an on-chain address lookup table to use
        /// for loading more readonly and writable accounts in a single tx.
        pub struct MessageAddressTableLookup {
            /// Address lookup table account key
            pub account_key: Pubkey,
            /// List of indexes used to load writable account addresses
            pub writable_indexes: SmallVec<u8, u8>,
            /// List of indexes used to load readonly account addresses
            pub readonly_indexes: SmallVec<u8, u8>,
        }
        impl borsh::de::BorshDeserialize for MessageAddressTableLookup
        where
            Pubkey: borsh::BorshDeserialize,
            SmallVec<u8, u8>: borsh::BorshDeserialize,
            SmallVec<u8, u8>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    account_key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    writable_indexes: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    readonly_indexes: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for MessageAddressTableLookup {
            #[inline]
            fn clone(&self) -> MessageAddressTableLookup {
                MessageAddressTableLookup {
                    account_key: ::core::clone::Clone::clone(&self.account_key),
                    writable_indexes: ::core::clone::Clone::clone(
                        &self.writable_indexes,
                    ),
                    readonly_indexes: ::core::clone::Clone::clone(&self.readonly_indexes),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for MessageAddressTableLookup {
            #[inline]
            fn default() -> MessageAddressTableLookup {
                MessageAddressTableLookup {
                    account_key: ::core::default::Default::default(),
                    writable_indexes: ::core::default::Default::default(),
                    readonly_indexes: ::core::default::Default::default(),
                }
            }
        }
    }
    mod vault_transaction_execute {
        use anchor_lang::prelude::*;
        use crate::errors::*;
        use crate::state::*;
        use crate::utils::*;
        pub struct VaultTransactionExecute<'info> {
            #[account(
                seeds = [SEED_PREFIX,
                SEED_MULTISIG,
                multisig.create_key.as_ref()],
                bump = multisig.bump,
            )]
            pub multisig: Box<Account<'info, Multisig>>,
            /// The proposal account associated with the transaction.
            #[account(
                mut,
                seeds = [SEED_PREFIX,
                multisig.key().as_ref(),
                SEED_TRANSACTION,
                &transaction.index.to_le_bytes(),
                SEED_PROPOSAL,
                ],
                bump = proposal.bump,
            )]
            pub proposal: Account<'info, Proposal>,
            /// The transaction to execute.
            #[account(
                mut,
                seeds = [SEED_PREFIX,
                multisig.key().as_ref(),
                SEED_TRANSACTION,
                &transaction.index.to_le_bytes(),
                ],
                bump = transaction.bump,
            )]
            pub transaction: Account<'info, VaultTransaction>,
            #[account(mut)]
            pub member: Signer<'info>,
        }
        impl<'info> kani::Arbitrary for VaultTransactionExecute<'info> {
            fn any() -> Self {
                Self {
                    multisig: Box::new(kani::any()),
                    proposal: kani::any(),
                    transaction: kani::any(),
                    member: kani::any(),
                }
            }
        }
        impl<'info> VaultTransactionExecute<'info> {
            pub fn __pre_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
                    && self.proposal.account._check_invariant()
                    && self.transaction.account._check_invariant()
            }
        }
        impl<'info> VaultTransactionExecute<'info> {
            pub fn __post_invariants(&self) -> bool {
                self.multisig.account._check_invariant()
                    && self.proposal.account._check_invariant()
                    && self.transaction.account._check_invariant()
            }
        }
        impl<'info> VaultTransactionExecute<'info> {
            pub fn __check_constraints(&self) -> bool {
                true
            }
        }
        impl VaultTransactionExecute<'_> {
            fn validate(&self) -> Result<()> {
                let Self { multisig, proposal, member, .. } = self;
                if !(multisig.is_member(member.key()).is_some()) {
                    return Err(Error::Generic);
                }
                if !(multisig.member_has_permission(member.key(), Permission::Execute)) {
                    return Err(Error::Generic);
                }
                Ok(())
            }
            /// Execute the multisig transaction.
            /// The transaction must be `Approved`.
            pub fn vault_transaction_execute(ctx: Context<Self>) -> Result<()> {
                ctx.accounts.validate()?;
                let multisig = &mut ctx.accounts.multisig;
                let proposal = &mut ctx.accounts.proposal;
                let transaction = &mut ctx.accounts.transaction;
                let multisig_key = multisig.key();
                let transaction_key = transaction.key();
                let vault_seeds = &[
                    SEED_PREFIX,
                    multisig_key.as_ref(),
                    SEED_VAULT,
                    &transaction.vault_index.to_le_bytes(),
                    &[transaction.vault_bump],
                ];
                let transaction_message = &transaction.message;
                let num_lookups = transaction_message.address_table_lookups.len();
                let message_account_infos = ctx
                    .remaining_accounts
                    .get(num_lookups..)
                    .ok_or(MultisigError::InvalidNumberOfAccounts)?;
                let address_lookup_table_account_infos = ctx
                    .remaining_accounts
                    .get(..num_lookups)
                    .ok_or(MultisigError::InvalidNumberOfAccounts)?;
                let vault_pubkey = Pubkey::create_program_address(
                    vault_seeds,
                    ctx.program_id,
                );
                let (ephemeral_signer_keys, ephemeral_signer_seeds) = derive_ephemeral_signers(
                    transaction_key,
                    &transaction.ephemeral_signer_bumps,
                );
                let executable_message = ExecutableTransactionMessage::new_validated(
                    transaction_message,
                    message_account_infos,
                    address_lookup_table_account_infos,
                    &vault_pubkey,
                    &ephemeral_signer_keys,
                )?;
                proposal.status = ProposalStatus::Executing;
                let proposal_account_info = proposal.to_account_info();
                proposal
                    .try_serialize(
                        &mut &mut proposal_account_info.data.borrow_mut()[..],
                    )?;
                proposal
                    .status = ProposalStatus::Executed {
                    timestamp: Clock::get()?.unix_timestamp,
                };
                Ok(())
            }
        }
    }
}
pub mod state {
    pub use batch::*;
    pub use config_transaction::*;
    pub use multisig::*;
    pub use proposal::*;
    pub use seeds::*;
    pub use spending_limit::*;
    pub use vault_transaction::*;
    mod batch {
        use anchor_lang::prelude::*;
        use anchor_lang::solana_program::borsh::get_instance_packed_len;
        use crate::{TransactionMessage, VaultTransactionMessage};
        /// Stores data required for serial execution of a batch of multisig vault transactions.
        /// Vault transaction is a transaction that's executed on behalf of the multisig vault PDA
        /// and wraps arbitrary Solana instructions, typically calling into other Solana programs.
        /// The transactions themselves are stored in separate PDAs associated with the this account.
        pub struct Batch {
            /// The multisig this belongs to.
            pub multisig: Pubkey,
            /// Member of the Multisig who submitted the batch.
            pub creator: Pubkey,
            /// Index of this batch within the multisig transactions.
            pub index: u64,
            /// PDA bump.
            pub bump: u8,
            /// Index of the vault this batch belongs to.
            pub vault_index: u8,
            /// Derivation bump of the vault PDA this batch belongs to.
            pub vault_bump: u8,
            /// Number of transactions in the batch.
            pub size: u32,
            /// Index of the last executed transaction within the batch.
            /// 0 means that no transactions have been executed yet.
            pub executed_transaction_index: u32,
        }
        impl Space for Batch {
            const INIT_SPACE: usize = 0;
        }
        impl Batch {
            pub fn _check_invariant(&self) -> bool {
                self.size >= self.executed_transaction_index
            }
        }
        impl kani::Arbitrary for Batch {
            fn any() -> Self {
                Batch {
                    multisig: kani::any(),
                    creator: kani::any(),
                    index: kani::any(),
                    bump: kani::any(),
                    vault_index: kani::any(),
                    vault_bump: kani::any(),
                    size: kani::any(),
                    executed_transaction_index: kani::any(),
                }
            }
        }
        impl borsh::de::BorshDeserialize for Batch
        where
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    multisig: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    creator: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    index: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    vault_index: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    vault_bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    size: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    executed_transaction_index: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                })
            }
        }
        impl borsh::ser::BorshSerialize for Batch
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.multisig, writer)?;
                borsh::BorshSerialize::serialize(&self.creator, writer)?;
                borsh::BorshSerialize::serialize(&self.index, writer)?;
                borsh::BorshSerialize::serialize(&self.bump, writer)?;
                borsh::BorshSerialize::serialize(&self.vault_index, writer)?;
                borsh::BorshSerialize::serialize(&self.vault_bump, writer)?;
                borsh::BorshSerialize::serialize(&self.size, writer)?;
                borsh::BorshSerialize::serialize(
                    &self.executed_transaction_index,
                    writer,
                )?;
                Ok(())
            }
        }
        impl AccountSerialize for Batch {}
        impl AccountDeserialize for Batch {}
        impl Batch {
            pub fn invariant(&self) -> Result<()> {
                if self.size < self.executed_transaction_index {
                    return Err(Error::Generic);
                }
                Ok(())
            }
        }
        /// Stores data required for execution of one transaction from a batch.
        pub struct VaultBatchTransaction {
            /// PDA bump.
            pub bump: u8,
            /// Derivation bumps for additional signers.
            /// Some transactions require multiple signers. Often these additional signers are "ephemeral" keypairs
            /// that are generated on the client with a sole purpose of signing the transaction and be discarded immediately after.
            /// When wrapping such transactions into multisig ones, we replace these "ephemeral" signing keypairs
            /// with PDAs derived from the transaction's `transaction_index` and controlled by the Multisig Program;
            /// during execution the program includes the seeds of these PDAs into the `invoke_signed` calls,
            /// thus "signing" on behalf of these PDAs.
            pub ephemeral_signer_bumps: Vec<u8>,
            /// data required for executing the transaction.
            pub message: VaultTransactionMessage,
        }
        impl VaultBatchTransaction {
            pub fn _check_invariant(&self) -> bool {
                true
            }
        }
        impl kani::Arbitrary for VaultBatchTransaction {
            fn any() -> Self {
                VaultBatchTransaction {
                    bump: kani::any(),
                    ephemeral_signer_bumps: kani::any(),
                    message: kani::any(),
                }
            }
        }
        impl borsh::de::BorshDeserialize for VaultBatchTransaction
        where
            u8: borsh::BorshDeserialize,
            Vec<u8>: borsh::BorshDeserialize,
            VaultTransactionMessage: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    ephemeral_signer_bumps: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    message: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        impl borsh::ser::BorshSerialize for VaultBatchTransaction
        where
            u8: borsh::ser::BorshSerialize,
            Vec<u8>: borsh::ser::BorshSerialize,
            VaultTransactionMessage: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.bump, writer)?;
                borsh::BorshSerialize::serialize(&self.ephemeral_signer_bumps, writer)?;
                borsh::BorshSerialize::serialize(&self.message, writer)?;
                Ok(())
            }
        }
        impl AccountSerialize for VaultBatchTransaction {}
        impl AccountDeserialize for VaultBatchTransaction {}
        impl VaultBatchTransaction {
            pub fn size(
                ephemeral_signers_length: u8,
                transaction_message: &[u8],
            ) -> Result<usize> {
                let transaction_message: VaultTransactionMessage = TransactionMessage::deserialize(
                        &mut &transaction_message[..],
                    )?
                    .try_into()?;
                let message_size = get_instance_packed_len(&transaction_message)
                    .unwrap_or_default();
                Ok(8 + 1 + (4 + usize::from(ephemeral_signers_length)) + message_size)
            }
        }
    }
    mod config_transaction {
        use anchor_lang::prelude::*;
        use anchor_lang::solana_program::borsh::get_instance_packed_len;
        use super::*;
        /// Stores data required for execution of a multisig configuration transaction.
        /// Config transaction can perform a predefined set of actions on the Multisig PDA, such as adding/removing members,
        /// changing the threshold, etc.
        pub struct ConfigTransaction {
            /// The multisig this belongs to.
            pub multisig: Pubkey,
            /// Member of the Multisig who submitted the transaction.
            pub creator: Pubkey,
            /// Index of this transaction within the multisig.
            pub index: u64,
            /// bump for the transaction seeds.
            pub bump: u8,
            /// Action to be performed on the multisig.
            pub actions: Vec<ConfigAction>,
        }
        impl ConfigTransaction {
            pub fn _check_invariant(&self) -> bool {
                true
            }
        }
        impl kani::Arbitrary for ConfigTransaction {
            fn any() -> Self {
                ConfigTransaction {
                    multisig: kani::any(),
                    creator: kani::any(),
                    index: kani::any(),
                    bump: kani::any(),
                    actions: kani::any(),
                }
            }
        }
        impl borsh::de::BorshDeserialize for ConfigTransaction
        where
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            Vec<ConfigAction>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    multisig: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    creator: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    index: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    actions: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        impl borsh::ser::BorshSerialize for ConfigTransaction
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            Vec<ConfigAction>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.multisig, writer)?;
                borsh::BorshSerialize::serialize(&self.creator, writer)?;
                borsh::BorshSerialize::serialize(&self.index, writer)?;
                borsh::BorshSerialize::serialize(&self.bump, writer)?;
                borsh::BorshSerialize::serialize(&self.actions, writer)?;
                Ok(())
            }
        }
        impl AccountSerialize for ConfigTransaction {}
        impl AccountDeserialize for ConfigTransaction {}
        impl ConfigTransaction {
            pub fn size(actions: &[ConfigAction]) -> usize {
                let actions_size: usize = actions
                    .iter()
                    .map(|action| get_instance_packed_len(action).unwrap())
                    .sum();
                8 + 32 + 32 + 8 + 1 + 4 + actions_size
            }
        }
        #[non_exhaustive]
        pub enum ConfigAction {
            /// Add a new member to the multisig.
            AddMember { new_member: Member },
            /// Remove a member from the multisig.
            RemoveMember { old_member: Pubkey },
            /// Change the `threshold` of the multisig.
            ChangeThreshold { new_threshold: u16 },
            /// Change the `time_lock` of the multisig.
            SetTimeLock { new_time_lock: u32 },
            /// Change the `time_lock` of the multisig.
            AddSpendingLimit {
                /// Key that is used to seed the SpendingLimit PDA.
                create_key: Pubkey,
                /// The index of the vault that the spending limit is for.
                vault_index: u8,
                /// The token mint the spending limit is for.
                mint: Pubkey,
                /// The amount of tokens that can be spent in a period.
                /// This amount is in decimals of the mint,
                /// so 1 SOL would be `1_000_000_000` and 1 USDC would be `1_000_000`.
                amount: u64,
                /// The reset period of the spending limit.
                /// When it passes, the remaining amount is reset, unless it's `Period::OneTime`.
                period: Period,
                /// Members of the multisig that can use the spending limit.
                /// In case a member is removed from the multisig, the spending limit will remain existent
                /// (until explicitly deleted), but the removed member will not be able to use it anymore.
                members: Vec<Pubkey>,
                /// The destination addresses the spending limit is allowed to sent funds to.
                /// If empty, funds can be sent to any address.
                destinations: Vec<Pubkey>,
            },
            /// Remove a spending limit from the multisig.
            RemoveSpendingLimit { spending_limit: Pubkey },
            #[default]
            NoAction,
        }
        impl borsh::ser::BorshSerialize for ConfigAction
        where
            Member: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            u16: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            Period: borsh::ser::BorshSerialize,
            Vec<Pubkey>: borsh::ser::BorshSerialize,
            Vec<Pubkey>: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                let variant_idx: u8 = match self {
                    ConfigAction::AddMember { .. } => 0u8,
                    ConfigAction::RemoveMember { .. } => 1u8,
                    ConfigAction::ChangeThreshold { .. } => 2u8,
                    ConfigAction::SetTimeLock { .. } => 3u8,
                    ConfigAction::AddSpendingLimit { .. } => 4u8,
                    ConfigAction::RemoveSpendingLimit { .. } => 5u8,
                    ConfigAction::NoAction => 6u8,
                };
                writer.write_all(&variant_idx.to_le_bytes())?;
                match self {
                    ConfigAction::AddMember { new_member } => {
                        borsh::BorshSerialize::serialize(new_member, writer)?;
                    }
                    ConfigAction::RemoveMember { old_member } => {
                        borsh::BorshSerialize::serialize(old_member, writer)?;
                    }
                    ConfigAction::ChangeThreshold { new_threshold } => {
                        borsh::BorshSerialize::serialize(new_threshold, writer)?;
                    }
                    ConfigAction::SetTimeLock { new_time_lock } => {
                        borsh::BorshSerialize::serialize(new_time_lock, writer)?;
                    }
                    ConfigAction::AddSpendingLimit {
                        create_key,
                        vault_index,
                        mint,
                        amount,
                        period,
                        members,
                        destinations,
                    } => {
                        borsh::BorshSerialize::serialize(create_key, writer)?;
                        borsh::BorshSerialize::serialize(vault_index, writer)?;
                        borsh::BorshSerialize::serialize(mint, writer)?;
                        borsh::BorshSerialize::serialize(amount, writer)?;
                        borsh::BorshSerialize::serialize(period, writer)?;
                        borsh::BorshSerialize::serialize(members, writer)?;
                        borsh::BorshSerialize::serialize(destinations, writer)?;
                    }
                    ConfigAction::RemoveSpendingLimit { spending_limit } => {
                        borsh::BorshSerialize::serialize(spending_limit, writer)?;
                    }
                    ConfigAction::NoAction => {}
                }
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for ConfigAction
        where
            Member: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            u16: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            Period: borsh::BorshDeserialize,
            Vec<Pubkey>: borsh::BorshDeserialize,
            Vec<Pubkey>: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                let tag = <u8 as borsh::de::BorshDeserialize>::deserialize_reader(
                    reader,
                )?;
                <Self as borsh::de::EnumExt>::deserialize_variant(reader, tag)
            }
        }
        impl borsh::de::EnumExt for ConfigAction
        where
            Member: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            u16: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            Period: borsh::BorshDeserialize,
            Vec<Pubkey>: borsh::BorshDeserialize,
            Vec<Pubkey>: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
        {
            fn deserialize_variant<R: borsh::maybestd::io::Read>(
                reader: &mut R,
                variant_idx: u8,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                let mut return_value = match variant_idx {
                    0u8 => {
                        ConfigAction::AddMember {
                            new_member: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        }
                    }
                    1u8 => {
                        ConfigAction::RemoveMember {
                            old_member: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        }
                    }
                    2u8 => {
                        ConfigAction::ChangeThreshold {
                            new_threshold: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        }
                    }
                    3u8 => {
                        ConfigAction::SetTimeLock {
                            new_time_lock: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        }
                    }
                    4u8 => {
                        ConfigAction::AddSpendingLimit {
                            create_key: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                            vault_index: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                            mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                            amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                            period: borsh::BorshDeserialize::deserialize_reader(reader)?,
                            members: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                            destinations: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        }
                    }
                    5u8 => {
                        ConfigAction::RemoveSpendingLimit {
                            spending_limit: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        }
                    }
                    6u8 => ConfigAction::NoAction,
                    _ => {
                        return Err(
                            borsh::maybestd::io::Error::new(
                                borsh::maybestd::io::ErrorKind::InvalidInput,
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("Unexpected variant index: {0:?}", variant_idx),
                                    );
                                    res
                                },
                            ),
                        );
                    }
                };
                Ok(return_value)
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ConfigAction {
            #[inline]
            fn clone(&self) -> ConfigAction {
                match self {
                    ConfigAction::AddMember { new_member: __self_0 } => {
                        ConfigAction::AddMember {
                            new_member: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    ConfigAction::RemoveMember { old_member: __self_0 } => {
                        ConfigAction::RemoveMember {
                            old_member: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    ConfigAction::ChangeThreshold { new_threshold: __self_0 } => {
                        ConfigAction::ChangeThreshold {
                            new_threshold: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    ConfigAction::SetTimeLock { new_time_lock: __self_0 } => {
                        ConfigAction::SetTimeLock {
                            new_time_lock: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    ConfigAction::AddSpendingLimit {
                        create_key: __self_0,
                        vault_index: __self_1,
                        mint: __self_2,
                        amount: __self_3,
                        period: __self_4,
                        members: __self_5,
                        destinations: __self_6,
                    } => {
                        ConfigAction::AddSpendingLimit {
                            create_key: ::core::clone::Clone::clone(__self_0),
                            vault_index: ::core::clone::Clone::clone(__self_1),
                            mint: ::core::clone::Clone::clone(__self_2),
                            amount: ::core::clone::Clone::clone(__self_3),
                            period: ::core::clone::Clone::clone(__self_4),
                            members: ::core::clone::Clone::clone(__self_5),
                            destinations: ::core::clone::Clone::clone(__self_6),
                        }
                    }
                    ConfigAction::RemoveSpendingLimit { spending_limit: __self_0 } => {
                        ConfigAction::RemoveSpendingLimit {
                            spending_limit: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    ConfigAction::NoAction => ConfigAction::NoAction,
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ConfigAction {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ConfigAction {
            #[inline]
            fn eq(&self, other: &ConfigAction) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (
                            ConfigAction::AddMember { new_member: __self_0 },
                            ConfigAction::AddMember { new_member: __arg1_0 },
                        ) => *__self_0 == *__arg1_0,
                        (
                            ConfigAction::RemoveMember { old_member: __self_0 },
                            ConfigAction::RemoveMember { old_member: __arg1_0 },
                        ) => *__self_0 == *__arg1_0,
                        (
                            ConfigAction::ChangeThreshold { new_threshold: __self_0 },
                            ConfigAction::ChangeThreshold { new_threshold: __arg1_0 },
                        ) => *__self_0 == *__arg1_0,
                        (
                            ConfigAction::SetTimeLock { new_time_lock: __self_0 },
                            ConfigAction::SetTimeLock { new_time_lock: __arg1_0 },
                        ) => *__self_0 == *__arg1_0,
                        (
                            ConfigAction::AddSpendingLimit {
                                create_key: __self_0,
                                vault_index: __self_1,
                                mint: __self_2,
                                amount: __self_3,
                                period: __self_4,
                                members: __self_5,
                                destinations: __self_6,
                            },
                            ConfigAction::AddSpendingLimit {
                                create_key: __arg1_0,
                                vault_index: __arg1_1,
                                mint: __arg1_2,
                                amount: __arg1_3,
                                period: __arg1_4,
                                members: __arg1_5,
                                destinations: __arg1_6,
                            },
                        ) => {
                            *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1
                                && *__self_2 == *__arg1_2 && *__self_3 == *__arg1_3
                                && *__self_4 == *__arg1_4 && *__self_5 == *__arg1_5
                                && *__self_6 == *__arg1_6
                        }
                        (
                            ConfigAction::RemoveSpendingLimit {
                                spending_limit: __self_0,
                            },
                            ConfigAction::RemoveSpendingLimit {
                                spending_limit: __arg1_0,
                            },
                        ) => *__self_0 == *__arg1_0,
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for ConfigAction {}
        #[automatically_derived]
        impl ::core::cmp::Eq for ConfigAction {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Member>;
                let _: ::core::cmp::AssertParamIsEq<Pubkey>;
                let _: ::core::cmp::AssertParamIsEq<u16>;
                let _: ::core::cmp::AssertParamIsEq<u32>;
                let _: ::core::cmp::AssertParamIsEq<u8>;
                let _: ::core::cmp::AssertParamIsEq<u64>;
                let _: ::core::cmp::AssertParamIsEq<Period>;
                let _: ::core::cmp::AssertParamIsEq<Vec<Pubkey>>;
                let _: ::core::cmp::AssertParamIsEq<Vec<Pubkey>>;
            }
        }
        impl kani::Arbitrary for ConfigAction {
            fn any() -> Self {
                match kani::any() {
                    0 => {
                        ConfigAction::AddMember {
                            new_member: kani::any(),
                        }
                    }
                    1 => {
                        ConfigAction::RemoveMember {
                            old_member: kani::any(),
                        }
                    }
                    2 => {
                        ConfigAction::ChangeThreshold {
                            new_threshold: kani::any(),
                        }
                    }
                    3 => {
                        ConfigAction::SetTimeLock {
                            new_time_lock: kani::any(),
                        }
                    }
                    4 => {
                        ConfigAction::AddSpendingLimit {
                            create_key: kani::any(),
                            vault_index: kani::any(),
                            mint: kani::any(),
                            amount: kani::any(),
                            period: kani::any(),
                            members: kani::any(),
                            destinations: kani::any(),
                        }
                    }
                    5 => {
                        ConfigAction::RemoveSpendingLimit {
                            spending_limit: kani::any(),
                        }
                    }
                    _ => ConfigAction::NoAction,
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for ConfigAction {
            #[inline]
            fn default() -> ConfigAction {
                Self::NoAction
            }
        }
    }
    mod multisig {
        use std::cmp::max;
        use anchor_lang::prelude::*;
        use anchor_lang::system_program;
        pub struct Multisig {
            /// Key that is used to seed the multisig PDA.
            pub create_key: Pubkey,
            /// The authority that can change the multisig config.
            /// This is a very important parameter as this authority can change the members and threshold.
            ///
            /// The convention is to set this to `Pubkey::default()`.
            /// In this case, the multisig becomes autonomous, so every config change goes through
            /// the normal process of voting by the members.
            ///
            /// However, if this parameter is set to any other key, all the config changes for this multisig
            /// will need to be signed by the `config_authority`. We call such a multisig a "controlled multisig".
            pub config_authority: Pubkey,
            /// Threshold for signatures.
            pub threshold: u16,
            /// How many seconds must pass between transaction voting settlement and execution.
            pub time_lock: u32,
            /// Last transaction index. 0 means no transactions have been created.
            pub transaction_index: u64,
            /// Last stale transaction index. All transactions up until this index are stale.
            /// This index is updated when multisig config (members/threshold/time_lock) changes.
            pub stale_transaction_index: u64,
            /// Reserved for future use.
            /// Bump for the multisig PDA seed.
            pub bump: u8,
            /// Members of the multisig.
            pub members: Vec<Member>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Multisig {
            #[inline]
            fn clone(&self) -> Multisig {
                Multisig {
                    create_key: ::core::clone::Clone::clone(&self.create_key),
                    config_authority: ::core::clone::Clone::clone(
                        &self.config_authority,
                    ),
                    threshold: ::core::clone::Clone::clone(&self.threshold),
                    time_lock: ::core::clone::Clone::clone(&self.time_lock),
                    transaction_index: ::core::clone::Clone::clone(
                        &self.transaction_index,
                    ),
                    stale_transaction_index: ::core::clone::Clone::clone(
                        &self.stale_transaction_index,
                    ),
                    bump: ::core::clone::Clone::clone(&self.bump),
                    members: ::core::clone::Clone::clone(&self.members),
                }
            }
        }
        impl Multisig {
            pub fn _check_invariant(&self) -> bool {
                !self.members.is_empty() && self.members.len() <= usize::from(u16::MAX)
                    && self.threshold <= self.members.len() as u16 && self.threshold > 0
                    && !self.members.windows(2).any(|win| win[0].key == win[1].key)
                    && self.members.iter().all(|m| m.permissions.mask < 8)
                    && Self::num_proposers(&self.members) > 0
                    && Self::num_executors(&self.members) > 0
                    && Self::num_voters(&self.members) > 0
                    && usize::from(self.threshold) <= Self::num_voters(&self.members)
                    && self.stale_transaction_index <= self.transaction_index
            }
        }
        impl kani::Arbitrary for Multisig {
            fn any() -> Self {
                Multisig {
                    create_key: kani::any(),
                    config_authority: kani::any(),
                    threshold: kani::any(),
                    time_lock: kani::any(),
                    transaction_index: kani::any(),
                    stale_transaction_index: kani::any(),
                    bump: kani::any(),
                    members: kani::any(),
                }
            }
        }
        impl borsh::de::BorshDeserialize for Multisig
        where
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            u16: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            Vec<Member>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    create_key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    config_authority: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    threshold: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    time_lock: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    transaction_index: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    stale_transaction_index: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    members: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        impl borsh::ser::BorshSerialize for Multisig
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            u16: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            Vec<Member>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.create_key, writer)?;
                borsh::BorshSerialize::serialize(&self.config_authority, writer)?;
                borsh::BorshSerialize::serialize(&self.threshold, writer)?;
                borsh::BorshSerialize::serialize(&self.time_lock, writer)?;
                borsh::BorshSerialize::serialize(&self.transaction_index, writer)?;
                borsh::BorshSerialize::serialize(&self.stale_transaction_index, writer)?;
                borsh::BorshSerialize::serialize(&self.bump, writer)?;
                borsh::BorshSerialize::serialize(&self.members, writer)?;
                Ok(())
            }
        }
        impl AccountSerialize for Multisig {}
        impl AccountDeserialize for Multisig {}
        impl Multisig {
            pub fn size(members_length: usize) -> usize {
                8 + 32 + 32 + 2 + 4 + 8 + 8 + 1 + 1 + 4
                    + members_length * Member::INIT_SPACE
            }
            pub fn num_voters(members: &[Member]) -> usize {
                members.iter().filter(|m| m.permissions.has(Permission::Vote)).count()
            }
            pub fn num_proposers(members: &[Member]) -> usize {
                members
                    .iter()
                    .filter(|m| m.permissions.has(Permission::Initiate))
                    .count()
            }
            pub fn num_executors(members: &[Member]) -> usize {
                members.iter().filter(|m| m.permissions.has(Permission::Execute)).count()
            }
            /// Check if the multisig account space needs to be reallocated to accommodate `members_length`.
            /// Returns `true` if the account was reallocated.
            pub fn realloc_if_needed<'a>(
                multisig: AccountInfo<'a>,
                members_length: usize,
                rent_payer: AccountInfo<'a>,
                system_program: AccountInfo<'a>,
            ) -> Result<bool> {
                let current_account_size = multisig.data.borrow().len();
                let account_size_to_fit_members = Multisig::size(members_length);
                if current_account_size >= account_size_to_fit_members {
                    return Ok(false);
                }
                let new_size = max(
                    current_account_size + (10 * Member::INIT_SPACE),
                    account_size_to_fit_members,
                );
                AccountInfo::realloc(&multisig, new_size, false)?;
                let rent_exempt_lamports = kani::any::<u64>().max(1);
                let top_up_lamports = rent_exempt_lamports
                    .saturating_sub(multisig.to_account_info().lamports());
                if top_up_lamports > 0 {
                    system_program::transfer(
                        CpiContext::new(
                            system_program,
                            system_program::Transfer {
                                from: rent_payer,
                                to: multisig,
                            },
                        ),
                        top_up_lamports,
                    )?;
                }
                Ok(true)
            }
            pub fn invariant(&self) -> Result<()> {
                let Self {
                    threshold,
                    members,
                    transaction_index,
                    stale_transaction_index,
                    ..
                } = self;
                if !(members.len() <= usize::from(u16::MAX)) {
                    return Err(Error::Generic);
                }
                let has_duplicates = members
                    .windows(2)
                    .any(|win| win[0].key == win[1].key);
                if !(!has_duplicates) {
                    return Err(Error::Generic);
                }
                if !(members.iter().all(|m| m.permissions.mask < 8)) {
                    return Err(Error::Generic);
                }
                let num_proposers = Self::num_proposers(members);
                if !(num_proposers > 0) {
                    return Err(Error::Generic);
                }
                let num_executors = Self::num_executors(members);
                if !(num_executors > 0) {
                    return Err(Error::Generic);
                }
                let num_voters = Self::num_voters(members);
                if !(num_voters > 0) {
                    return Err(Error::Generic);
                }
                if !(*threshold > 0) {
                    return Err(Error::Generic);
                }
                if !(usize::from(*threshold) <= num_voters) {
                    return Err(Error::Generic);
                }
                if !(stale_transaction_index <= transaction_index) {
                    return Err(Error::Generic);
                }
                Ok(())
            }
            /// Makes the transactions created up until this moment stale.
            /// Should be called whenever any multisig parameter related to the voting consensus is changed.
            pub fn invalidate_prior_transactions(&mut self) {
                self.stale_transaction_index = self.transaction_index;
            }
            /// Returns `Some(index)` if `member_pubkey` is a member, with `index` into the `members` vec.
            /// `None` otherwise.
            pub fn is_member(&self, member_pubkey: Pubkey) -> Option<usize> {
                self.members.iter().position(|m| m.key == member_pubkey)
            }
            pub fn member_has_permission(
                &self,
                member_pubkey: Pubkey,
                permission: Permission,
            ) -> bool {
                match self.is_member(member_pubkey) {
                    Some(index) => self.members[index].permissions.has(permission),
                    _ => false,
                }
            }
            /// How many "reject" votes are enough to make the transaction "Rejected".
            /// The cutoff must be such that it is impossible for the remaining voters to reach the approval threshold.
            /// For example: total voters = 7, threshold = 3, cutoff = 5.
            pub fn cutoff(&self) -> usize {
                Self::num_voters(&self.members)
                    .checked_sub(usize::from(self.threshold))
                    .unwrap()
                    .checked_add(1)
                    .unwrap()
            }
            /// Add `new_member` to the multisig `members` vec and sort the vec.
            pub fn add_member(&mut self, new_member: Member) {
                self.members.push(new_member);
            }
            /// Remove `member_pubkey` from the multisig `members` vec.
            ///
            /// # Errors
            /// - `MultisigError::NotAMember` if `member_pubkey` is not a member.
            pub fn remove_member(&mut self, member_pubkey: Pubkey) -> Result<()> {
                let old_member_index = match self.is_member(member_pubkey) {
                    Some(old_member_index) => old_member_index,
                    None => return Err(Error::Generic),
                };
                self.members.remove(old_member_index);
                Ok(())
            }
        }
        pub struct Member {
            pub key: Pubkey,
            pub permissions: Permissions,
        }
        impl borsh::de::BorshDeserialize for Member
        where
            Pubkey: borsh::BorshDeserialize,
            Permissions: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    permissions: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        impl borsh::ser::BorshSerialize for Member
        where
            Pubkey: borsh::ser::BorshSerialize,
            Permissions: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.key, writer)?;
                borsh::BorshSerialize::serialize(&self.permissions, writer)?;
                Ok(())
            }
        }
        impl Space for Member {
            const INIT_SPACE: usize = 0;
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Member {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Member {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Pubkey>;
                let _: ::core::cmp::AssertParamIsEq<Permissions>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Member {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Member {
            #[inline]
            fn eq(&self, other: &Member) -> bool {
                self.key == other.key && self.permissions == other.permissions
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Member {
            #[inline]
            fn clone(&self) -> Member {
                let _: ::core::clone::AssertParamIsClone<Pubkey>;
                let _: ::core::clone::AssertParamIsClone<Permissions>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Member {
            #[inline]
            fn default() -> Member {
                Member {
                    key: ::core::default::Default::default(),
                    permissions: ::core::default::Default::default(),
                }
            }
        }
        impl kani::Arbitrary for Member {
            fn any() -> Self {
                Member {
                    key: kani::any(),
                    permissions: kani::any(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Member {}
        pub enum Permission {
            Initiate = 1 << 0,
            Vote = 1 << 1,
            Execute = 1 << 2,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Permission {
            #[inline]
            fn clone(&self) -> Permission {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Permission {}
        /// Bitmask for permissions.
        pub struct Permissions {
            pub mask: u8,
        }
        impl borsh::ser::BorshSerialize for Permissions
        where
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.mask, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for Permissions
        where
            u8: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    mask: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        impl Space for Permissions {
            const INIT_SPACE: usize = 0;
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Permissions {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Permissions {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u8>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Permissions {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Permissions {
            #[inline]
            fn eq(&self, other: &Permissions) -> bool {
                self.mask == other.mask
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Permissions {
            #[inline]
            fn clone(&self) -> Permissions {
                let _: ::core::clone::AssertParamIsClone<u8>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Permissions {}
        #[automatically_derived]
        impl ::core::default::Default for Permissions {
            #[inline]
            fn default() -> Permissions {
                Permissions {
                    mask: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Permissions {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Permissions",
                    "mask",
                    &&self.mask,
                )
            }
        }
        impl kani::Arbitrary for Permissions {
            fn any() -> Self {
                Permissions { mask: kani::any() }
            }
        }
        impl Permissions {
            pub fn from_vec(permissions: &[Permission]) -> Self {
                let mut mask = 0;
                for permission in permissions {
                    mask |= *permission as u8;
                }
                Self { mask }
            }
            pub fn has(&self, permission: Permission) -> bool {
                self.mask & (permission as u8) != 0
            }
        }
    }
    mod proposal {
        use anchor_lang::prelude::*;
        /// Stores the data required for tracking the status of a multisig proposal.
        /// Each `Proposal` has a 1:1 association with a transaction account, e.g. a `VaultTransaction` or a `ConfigTransaction`;
        /// the latter can be executed only after the `Proposal` has been approved and its time lock is released.
        pub struct Proposal {
            /// The multisig this belongs to.
            pub multisig: Pubkey,
            /// Index of the multisig transaction this proposal is associated with.
            pub transaction_index: u64,
            /// The status of the transaction.
            pub status: ProposalStatus,
            /// PDA bump.
            pub bump: u8,
            /// Keys that have approved/signed.
            pub approved: Vec<Pubkey>,
            /// Keys that have rejected.
            pub rejected: Vec<Pubkey>,
            /// Keys that have cancelled (Approved only).
            pub cancelled: Vec<Pubkey>,
        }
        impl Proposal {
            pub fn _check_invariant(&self) -> bool {
                true
            }
        }
        impl kani::Arbitrary for Proposal {
            fn any() -> Self {
                Proposal {
                    multisig: kani::any(),
                    transaction_index: kani::any(),
                    status: kani::any(),
                    bump: kani::any(),
                    approved: kani::any(),
                    rejected: kani::any(),
                    cancelled: kani::any(),
                }
            }
        }
        impl borsh::de::BorshDeserialize for Proposal
        where
            Pubkey: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            ProposalStatus: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            Vec<Pubkey>: borsh::BorshDeserialize,
            Vec<Pubkey>: borsh::BorshDeserialize,
            Vec<Pubkey>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    multisig: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    transaction_index: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    status: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    approved: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    rejected: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    cancelled: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        impl borsh::ser::BorshSerialize for Proposal
        where
            Pubkey: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            ProposalStatus: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            Vec<Pubkey>: borsh::ser::BorshSerialize,
            Vec<Pubkey>: borsh::ser::BorshSerialize,
            Vec<Pubkey>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.multisig, writer)?;
                borsh::BorshSerialize::serialize(&self.transaction_index, writer)?;
                borsh::BorshSerialize::serialize(&self.status, writer)?;
                borsh::BorshSerialize::serialize(&self.bump, writer)?;
                borsh::BorshSerialize::serialize(&self.approved, writer)?;
                borsh::BorshSerialize::serialize(&self.rejected, writer)?;
                borsh::BorshSerialize::serialize(&self.cancelled, writer)?;
                Ok(())
            }
        }
        impl AccountSerialize for Proposal {}
        impl AccountDeserialize for Proposal {}
        impl Proposal {
            pub fn size(members_len: usize) -> usize {
                8 + 32 + 8 + 1 + 8 + 1 + (4 + (members_len * 32))
                    + (4 + (members_len * 32)) + (4 + (members_len * 32))
            }
            /// Register an approval vote.
            pub fn approve(&mut self, member: Pubkey, threshold: usize) -> Result<()> {
                if let Some(vote_index) = self.has_voted_reject(member.key()) {
                    self.remove_rejection_vote(vote_index);
                }
                match self.approved.binary_search(&member) {
                    Ok(_) => return Err(Error::Generic),
                    Err(pos) => self.approved.insert(pos, member),
                };
                if self.approved.len() >= threshold {
                    self
                        .status = ProposalStatus::Approved {
                        timestamp: Clock::get()?.unix_timestamp,
                    };
                }
                Ok(())
            }
            /// Register a rejection vote.
            pub fn reject(&mut self, member: Pubkey, cutoff: usize) -> Result<()> {
                if let Some(vote_index) = self.has_voted_approve(member.key()) {
                    self.remove_approval_vote(vote_index);
                }
                match self.rejected.binary_search(&member) {
                    Ok(_) => return Err(Error::Generic),
                    Err(pos) => self.rejected.insert(pos, member),
                };
                if self.rejected.len() >= cutoff {
                    self
                        .status = ProposalStatus::Rejected {
                        timestamp: Clock::get()?.unix_timestamp,
                    };
                }
                Ok(())
            }
            /// Registers a cancellation vote.
            pub fn cancel(&mut self, member: Pubkey, threshold: usize) -> Result<()> {
                match self.cancelled.binary_search(&member) {
                    Ok(_) => return Err(Error::Generic),
                    Err(pos) => self.cancelled.insert(pos, member),
                };
                if self.cancelled.len() >= threshold {
                    self
                        .status = ProposalStatus::Cancelled {
                        timestamp: Clock::get()?.unix_timestamp,
                    };
                }
                Ok(())
            }
            /// Check if the member approved the transaction.
            /// Returns `Some(index)` if `member` has approved the transaction, with `index` into the `approved` vec.
            fn has_voted_approve(&self, member: Pubkey) -> Option<usize> {
                self.approved.binary_search(&member).ok()
            }
            /// Check if the member rejected the transaction.
            /// Returns `Some(index)` if `member` has rejected the transaction, with `index` into the `rejected` vec.
            fn has_voted_reject(&self, member: Pubkey) -> Option<usize> {
                self.rejected.binary_search(&member).ok()
            }
            /// Delete the vote of rejection at the `index`.
            fn remove_rejection_vote(&mut self, index: usize) {
                self.rejected.remove(index);
            }
            /// Delete the vote of approval at the `index`.
            fn remove_approval_vote(&mut self, index: usize) {
                self.approved.remove(index);
            }
        }
        /// The status of a proposal.
        /// Each variant wraps a timestamp of when the status was set.
        #[non_exhaustive]
        pub enum ProposalStatus {
            /// Proposal is in the draft mode and can be voted on.
            Draft { timestamp: i64 },
            /// Proposal is live and ready for voting.
            Active { timestamp: i64 },
            /// Proposal has been rejected.
            Rejected { timestamp: i64 },
            /// Proposal has been approved and is pending execution.
            Approved { timestamp: i64 },
            /// Proposal is being executed. This is a transient state that always transitions to `Executed` in the span of a single transaction.
            Executing,
            /// Proposal has been executed.
            Executed { timestamp: i64 },
            /// Proposal has been cancelled.
            Cancelled { timestamp: i64 },
        }
        impl borsh::ser::BorshSerialize for ProposalStatus
        where
            i64: borsh::ser::BorshSerialize,
            i64: borsh::ser::BorshSerialize,
            i64: borsh::ser::BorshSerialize,
            i64: borsh::ser::BorshSerialize,
            i64: borsh::ser::BorshSerialize,
            i64: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                let variant_idx: u8 = match self {
                    ProposalStatus::Draft { .. } => 0u8,
                    ProposalStatus::Active { .. } => 1u8,
                    ProposalStatus::Rejected { .. } => 2u8,
                    ProposalStatus::Approved { .. } => 3u8,
                    ProposalStatus::Executing => 4u8,
                    ProposalStatus::Executed { .. } => 5u8,
                    ProposalStatus::Cancelled { .. } => 6u8,
                };
                writer.write_all(&variant_idx.to_le_bytes())?;
                match self {
                    ProposalStatus::Draft { timestamp } => {
                        borsh::BorshSerialize::serialize(timestamp, writer)?;
                    }
                    ProposalStatus::Active { timestamp } => {
                        borsh::BorshSerialize::serialize(timestamp, writer)?;
                    }
                    ProposalStatus::Rejected { timestamp } => {
                        borsh::BorshSerialize::serialize(timestamp, writer)?;
                    }
                    ProposalStatus::Approved { timestamp } => {
                        borsh::BorshSerialize::serialize(timestamp, writer)?;
                    }
                    ProposalStatus::Executing => {}
                    ProposalStatus::Executed { timestamp } => {
                        borsh::BorshSerialize::serialize(timestamp, writer)?;
                    }
                    ProposalStatus::Cancelled { timestamp } => {
                        borsh::BorshSerialize::serialize(timestamp, writer)?;
                    }
                }
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for ProposalStatus
        where
            i64: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                let tag = <u8 as borsh::de::BorshDeserialize>::deserialize_reader(
                    reader,
                )?;
                <Self as borsh::de::EnumExt>::deserialize_variant(reader, tag)
            }
        }
        impl borsh::de::EnumExt for ProposalStatus
        where
            i64: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
        {
            fn deserialize_variant<R: borsh::maybestd::io::Read>(
                reader: &mut R,
                variant_idx: u8,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                let mut return_value = match variant_idx {
                    0u8 => {
                        ProposalStatus::Draft {
                            timestamp: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        }
                    }
                    1u8 => {
                        ProposalStatus::Active {
                            timestamp: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        }
                    }
                    2u8 => {
                        ProposalStatus::Rejected {
                            timestamp: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        }
                    }
                    3u8 => {
                        ProposalStatus::Approved {
                            timestamp: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        }
                    }
                    4u8 => ProposalStatus::Executing,
                    5u8 => {
                        ProposalStatus::Executed {
                            timestamp: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        }
                    }
                    6u8 => {
                        ProposalStatus::Cancelled {
                            timestamp: borsh::BorshDeserialize::deserialize_reader(
                                reader,
                            )?,
                        }
                    }
                    _ => {
                        return Err(
                            borsh::maybestd::io::Error::new(
                                borsh::maybestd::io::ErrorKind::InvalidInput,
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("Unexpected variant index: {0:?}", variant_idx),
                                    );
                                    res
                                },
                            ),
                        );
                    }
                };
                Ok(return_value)
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ProposalStatus {
            #[inline]
            fn clone(&self) -> ProposalStatus {
                match self {
                    ProposalStatus::Draft { timestamp: __self_0 } => {
                        ProposalStatus::Draft {
                            timestamp: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    ProposalStatus::Active { timestamp: __self_0 } => {
                        ProposalStatus::Active {
                            timestamp: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    ProposalStatus::Rejected { timestamp: __self_0 } => {
                        ProposalStatus::Rejected {
                            timestamp: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    ProposalStatus::Approved { timestamp: __self_0 } => {
                        ProposalStatus::Approved {
                            timestamp: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    ProposalStatus::Executing => ProposalStatus::Executing,
                    ProposalStatus::Executed { timestamp: __self_0 } => {
                        ProposalStatus::Executed {
                            timestamp: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                    ProposalStatus::Cancelled { timestamp: __self_0 } => {
                        ProposalStatus::Cancelled {
                            timestamp: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ProposalStatus {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ProposalStatus {
            #[inline]
            fn eq(&self, other: &ProposalStatus) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (
                            ProposalStatus::Draft { timestamp: __self_0 },
                            ProposalStatus::Draft { timestamp: __arg1_0 },
                        ) => *__self_0 == *__arg1_0,
                        (
                            ProposalStatus::Active { timestamp: __self_0 },
                            ProposalStatus::Active { timestamp: __arg1_0 },
                        ) => *__self_0 == *__arg1_0,
                        (
                            ProposalStatus::Rejected { timestamp: __self_0 },
                            ProposalStatus::Rejected { timestamp: __arg1_0 },
                        ) => *__self_0 == *__arg1_0,
                        (
                            ProposalStatus::Approved { timestamp: __self_0 },
                            ProposalStatus::Approved { timestamp: __arg1_0 },
                        ) => *__self_0 == *__arg1_0,
                        (
                            ProposalStatus::Executed { timestamp: __self_0 },
                            ProposalStatus::Executed { timestamp: __arg1_0 },
                        ) => *__self_0 == *__arg1_0,
                        (
                            ProposalStatus::Cancelled { timestamp: __self_0 },
                            ProposalStatus::Cancelled { timestamp: __arg1_0 },
                        ) => *__self_0 == *__arg1_0,
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for ProposalStatus {}
        #[automatically_derived]
        impl ::core::cmp::Eq for ProposalStatus {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<i64>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ProposalStatus {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ProposalStatus::Draft { timestamp: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Draft",
                            "timestamp",
                            &__self_0,
                        )
                    }
                    ProposalStatus::Active { timestamp: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Active",
                            "timestamp",
                            &__self_0,
                        )
                    }
                    ProposalStatus::Rejected { timestamp: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Rejected",
                            "timestamp",
                            &__self_0,
                        )
                    }
                    ProposalStatus::Approved { timestamp: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Approved",
                            "timestamp",
                            &__self_0,
                        )
                    }
                    ProposalStatus::Executing => {
                        ::core::fmt::Formatter::write_str(f, "Executing")
                    }
                    ProposalStatus::Executed { timestamp: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Executed",
                            "timestamp",
                            &__self_0,
                        )
                    }
                    ProposalStatus::Cancelled { timestamp: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Cancelled",
                            "timestamp",
                            &__self_0,
                        )
                    }
                }
            }
        }
        impl kani::Arbitrary for ProposalStatus {
            fn any() -> Self {
                match kani::any() {
                    0 => {
                        ProposalStatus::Draft {
                            timestamp: kani::any(),
                        }
                    }
                    1 => {
                        ProposalStatus::Active {
                            timestamp: kani::any(),
                        }
                    }
                    2 => {
                        ProposalStatus::Rejected {
                            timestamp: kani::any(),
                        }
                    }
                    3 => {
                        ProposalStatus::Approved {
                            timestamp: kani::any(),
                        }
                    }
                    4 => ProposalStatus::Executing,
                    5 => {
                        ProposalStatus::Executed {
                            timestamp: kani::any(),
                        }
                    }
                    _ => {
                        ProposalStatus::Cancelled {
                            timestamp: kani::any(),
                        }
                    }
                }
            }
        }
    }
    mod seeds {
        pub const SEED_PREFIX: &[u8] = b"multisig";
        pub const SEED_MULTISIG: &[u8] = b"multisig";
        pub const SEED_PROPOSAL: &[u8] = b"proposal";
        pub const SEED_TRANSACTION: &[u8] = b"transaction";
        pub const SEED_BATCH_TRANSACTION: &[u8] = b"batch_transaction";
        pub const SEED_VAULT: &[u8] = b"vault";
        pub const SEED_EPHEMERAL_SIGNER: &[u8] = b"ephemeral_signer";
        pub const SEED_SPENDING_LIMIT: &[u8] = b"spending_limit";
    }
    mod spending_limit {
        use anchor_lang::prelude::*;
        pub struct SpendingLimit {
            /// The multisig this belongs to.
            pub multisig: Pubkey,
            /// Key that is used to seed the SpendingLimit PDA.
            pub create_key: Pubkey,
            /// The index of the vault that the spending limit is for.
            pub vault_index: u8,
            /// The token mint the spending limit is for.
            /// Pubkey::default() means SOL.
            /// use NATIVE_MINT for Wrapped SOL.
            pub mint: Pubkey,
            /// The amount of tokens that can be spent in a period.
            /// This amount is in decimals of the mint,
            /// so 1 SOL would be `1_000_000_000` and 1 USDC would be `1_000_000`.
            pub amount: u64,
            /// The reset period of the spending limit.
            /// When it passes, the remaining amount is reset, unless it's `Period::OneTime`.
            pub period: Period,
            /// The remaining amount of tokens that can be spent in the current period.
            /// When reaches 0, the spending limit cannot be used anymore until the period reset.
            pub remaining_amount: u64,
            /// Unix timestamp marking the last time the spending limit was reset (or created).
            pub last_reset: i64,
            /// PDA bump.
            pub bump: u8,
            /// Members of the multisig that can use the spending limit.
            /// In case a member is removed from the multisig, the spending limit will remain existent
            /// (until explicitly deleted), but the removed member will not be able to use it anymore.
            pub members: Vec<Pubkey>,
            /// The destination addresses the spending limit is allowed to sent funds to.
            /// If empty, funds can be sent to any address.
            pub destinations: Vec<Pubkey>,
        }
        impl SpendingLimit {
            pub fn _check_invariant(&self) -> bool {
                !self.members.is_empty()
                    && !self.members.windows(2).any(|win| win[0] == win[1])
                    && self.last_reset >= 0 && self.remaining_amount <= self.amount
                    && self.amount > 0
            }
        }
        impl kani::Arbitrary for SpendingLimit {
            fn any() -> Self {
                SpendingLimit {
                    multisig: kani::any(),
                    create_key: kani::any(),
                    vault_index: kani::any(),
                    mint: kani::any(),
                    amount: kani::any(),
                    period: kani::any(),
                    remaining_amount: kani::any(),
                    last_reset: kani::any(),
                    bump: kani::any(),
                    members: kani::any(),
                    destinations: kani::any(),
                }
            }
        }
        impl borsh::de::BorshDeserialize for SpendingLimit
        where
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            Period: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            i64: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            Vec<Pubkey>: borsh::BorshDeserialize,
            Vec<Pubkey>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    multisig: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    create_key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    vault_index: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    period: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    remaining_amount: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    last_reset: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    members: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    destinations: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        impl borsh::ser::BorshSerialize for SpendingLimit
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            Period: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            i64: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            Vec<Pubkey>: borsh::ser::BorshSerialize,
            Vec<Pubkey>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.multisig, writer)?;
                borsh::BorshSerialize::serialize(&self.create_key, writer)?;
                borsh::BorshSerialize::serialize(&self.vault_index, writer)?;
                borsh::BorshSerialize::serialize(&self.mint, writer)?;
                borsh::BorshSerialize::serialize(&self.amount, writer)?;
                borsh::BorshSerialize::serialize(&self.period, writer)?;
                borsh::BorshSerialize::serialize(&self.remaining_amount, writer)?;
                borsh::BorshSerialize::serialize(&self.last_reset, writer)?;
                borsh::BorshSerialize::serialize(&self.bump, writer)?;
                borsh::BorshSerialize::serialize(&self.members, writer)?;
                borsh::BorshSerialize::serialize(&self.destinations, writer)?;
                Ok(())
            }
        }
        impl AccountSerialize for SpendingLimit {}
        impl AccountDeserialize for SpendingLimit {}
        impl Owner for SpendingLimit {
            fn owner() -> Pubkey {
                kani::any()
            }
        }
        impl SpendingLimit {
            pub fn size(members_length: usize, destinations_length: usize) -> usize {
                8 + 32 + 32 + 1 + 32 + 8 + 1 + 8 + 8 + 1 + 4 + members_length * 32 + 4
                    + destinations_length * 32
            }
            pub fn invariant(&self) -> Result<()> {
                if !(!self.members.is_empty()) {
                    return Err(Error::Generic);
                }
                let has_duplicates = self.members.windows(2).any(|win| win[0] == win[1]);
                if !(!has_duplicates) {
                    return Err(Error::Generic);
                }
                Ok(())
            }
        }
        /// The reset period of the spending limit.
        pub enum Period {
            OneTime,
            Day,
            Week,
            Month,
        }
        impl borsh::ser::BorshSerialize for Period {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                let variant_idx: u8 = match self {
                    Period::OneTime => 0u8,
                    Period::Day => 1u8,
                    Period::Week => 2u8,
                    Period::Month => 3u8,
                };
                writer.write_all(&variant_idx.to_le_bytes())?;
                match self {
                    Period::OneTime => {}
                    Period::Day => {}
                    Period::Week => {}
                    Period::Month => {}
                }
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for Period {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                let tag = <u8 as borsh::de::BorshDeserialize>::deserialize_reader(
                    reader,
                )?;
                <Self as borsh::de::EnumExt>::deserialize_variant(reader, tag)
            }
        }
        impl borsh::de::EnumExt for Period {
            fn deserialize_variant<R: borsh::maybestd::io::Read>(
                reader: &mut R,
                variant_idx: u8,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                let mut return_value = match variant_idx {
                    0u8 => Period::OneTime,
                    1u8 => Period::Day,
                    2u8 => Period::Week,
                    3u8 => Period::Month,
                    _ => {
                        return Err(
                            borsh::maybestd::io::Error::new(
                                borsh::maybestd::io::ErrorKind::InvalidInput,
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("Unexpected variant index: {0:?}", variant_idx),
                                    );
                                    res
                                },
                            ),
                        );
                    }
                };
                Ok(return_value)
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Period {
            #[inline]
            fn clone(&self) -> Period {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Period {}
        #[automatically_derived]
        impl ::core::fmt::Debug for Period {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        Period::OneTime => "OneTime",
                        Period::Day => "Day",
                        Period::Week => "Week",
                        Period::Month => "Month",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Period {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Period {
            #[inline]
            fn eq(&self, other: &Period) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Period {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Period {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        impl kani::Arbitrary for Period {
            fn any() -> Self {
                match kani::any() {
                    0 => Period::OneTime,
                    1 => Period::Day,
                    2 => Period::Week,
                    _ => Period::Month,
                }
            }
        }
        impl Period {
            pub fn to_seconds(&self) -> Option<i64> {
                match self {
                    Period::OneTime => None,
                    Period::Day => Some(24 * 60 * 60),
                    Period::Week => Some(7 * 24 * 60 * 60),
                    Period::Month => Some(30 * 24 * 60 * 60),
                }
            }
        }
    }
    mod vault_transaction {
        use anchor_lang::prelude::*;
        use anchor_lang::solana_program::borsh::get_instance_packed_len;
        use crate::instructions::{
            CompiledInstruction, MessageAddressTableLookup, TransactionMessage,
        };
        /// Stores data required for tracking the voting and execution status of a vault transaction.
        /// Vault transaction is a transaction that's executed on behalf of the multisig vault PDA
        /// and wraps arbitrary Solana instructions, typically calling into other Solana programs.
        pub struct VaultTransaction {
            /// The multisig this belongs to.
            pub multisig: Pubkey,
            /// Member of the Multisig who submitted the transaction.
            pub creator: Pubkey,
            /// Index of this transaction within the multisig.
            pub index: u64,
            /// bump for the transaction seeds.
            pub bump: u8,
            /// Index of the vault this transaction belongs to.
            pub vault_index: u8,
            /// Derivation bump of the vault PDA this transaction belongs to.
            pub vault_bump: u8,
            /// Derivation bumps for additional signers.
            /// Some transactions require multiple signers. Often these additional signers are "ephemeral" keypairs
            /// that are generated on the client with a sole purpose of signing the transaction and be discarded immediately after.
            /// When wrapping such transactions into multisig ones, we replace these "ephemeral" signing keypairs
            /// with PDAs derived from the MultisigTransaction's `transaction_index` and controlled by the Multisig Program;
            /// during execution the program includes the seeds of these PDAs into the `invoke_signed` calls,
            /// thus "signing" on behalf of these PDAs.
            pub ephemeral_signer_bumps: Vec<u8>,
            /// data required for executing the transaction.
            pub message: VaultTransactionMessage,
        }
        impl VaultTransaction {
            pub fn _check_invariant(&self) -> bool {
                true
            }
        }
        impl kani::Arbitrary for VaultTransaction {
            fn any() -> Self {
                VaultTransaction {
                    multisig: kani::any(),
                    creator: kani::any(),
                    index: kani::any(),
                    bump: kani::any(),
                    vault_index: kani::any(),
                    vault_bump: kani::any(),
                    ephemeral_signer_bumps: kani::any(),
                    message: kani::any(),
                }
            }
        }
        impl borsh::de::BorshDeserialize for VaultTransaction
        where
            Pubkey: borsh::BorshDeserialize,
            Pubkey: borsh::BorshDeserialize,
            u64: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            Vec<u8>: borsh::BorshDeserialize,
            VaultTransactionMessage: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    multisig: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    creator: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    index: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    vault_index: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    vault_bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    ephemeral_signer_bumps: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    message: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        impl borsh::ser::BorshSerialize for VaultTransaction
        where
            Pubkey: borsh::ser::BorshSerialize,
            Pubkey: borsh::ser::BorshSerialize,
            u64: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            Vec<u8>: borsh::ser::BorshSerialize,
            VaultTransactionMessage: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.multisig, writer)?;
                borsh::BorshSerialize::serialize(&self.creator, writer)?;
                borsh::BorshSerialize::serialize(&self.index, writer)?;
                borsh::BorshSerialize::serialize(&self.bump, writer)?;
                borsh::BorshSerialize::serialize(&self.vault_index, writer)?;
                borsh::BorshSerialize::serialize(&self.vault_bump, writer)?;
                borsh::BorshSerialize::serialize(&self.ephemeral_signer_bumps, writer)?;
                borsh::BorshSerialize::serialize(&self.message, writer)?;
                Ok(())
            }
        }
        impl AccountSerialize for VaultTransaction {}
        impl AccountDeserialize for VaultTransaction {}
        impl VaultTransaction {
            pub fn size(
                ephemeral_signers_length: u8,
                transaction_message: &[u8],
            ) -> Result<usize> {
                let transaction_message: VaultTransactionMessage = TransactionMessage::deserialize(
                        &mut &transaction_message[..],
                    )?
                    .try_into()?;
                let message_size = get_instance_packed_len(&transaction_message)
                    .unwrap_or_default();
                Ok(
                    8 + 32 + 32 + 8 + 1 + 1 + 1
                        + (4 + usize::from(ephemeral_signers_length)) + message_size,
                )
            }
        }
        pub struct VaultTransactionMessage {
            /// The number of signer pubkeys in the account_keys vec.
            pub num_signers: u8,
            /// The number of writable signer pubkeys in the account_keys vec.
            pub num_writable_signers: u8,
            /// The number of writable non-signer pubkeys in the account_keys vec.
            pub num_writable_non_signers: u8,
            /// Unique account pubkeys (including program IDs) required for execution of the tx.
            /// The signer pubkeys appear at the beginning of the vec, with writable pubkeys first, and read-only pubkeys following.
            /// The non-signer pubkeys follow with writable pubkeys first and read-only ones following.
            /// Program IDs are also stored at the end of the vec along with other non-signer non-writable pubkeys:
            ///
            /// ```plaintext
            /// [pubkey1, pubkey2, pubkey3, pubkey4, pubkey5, pubkey6, pubkey7, pubkey8]
            ///  |---writable---|  |---readonly---|  |---writable---|  |---readonly---|
            ///  |------------signers-------------|  |----------non-singers-----------|
            /// ```
            pub account_keys: Vec<Pubkey>,
            /// List of instructions making up the tx.
            pub instructions: Vec<MultisigCompiledInstruction>,
            /// List of address table lookups used to load additional accounts
            /// for this transaction.
            pub address_table_lookups: Vec<MultisigMessageAddressTableLookup>,
        }
        impl borsh::ser::BorshSerialize for VaultTransactionMessage
        where
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            Vec<Pubkey>: borsh::ser::BorshSerialize,
            Vec<MultisigCompiledInstruction>: borsh::ser::BorshSerialize,
            Vec<MultisigMessageAddressTableLookup>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.num_signers, writer)?;
                borsh::BorshSerialize::serialize(&self.num_writable_signers, writer)?;
                borsh::BorshSerialize::serialize(
                    &self.num_writable_non_signers,
                    writer,
                )?;
                borsh::BorshSerialize::serialize(&self.account_keys, writer)?;
                borsh::BorshSerialize::serialize(&self.instructions, writer)?;
                borsh::BorshSerialize::serialize(&self.address_table_lookups, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for VaultTransactionMessage
        where
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            Vec<Pubkey>: borsh::BorshDeserialize,
            Vec<MultisigCompiledInstruction>: borsh::BorshDeserialize,
            Vec<MultisigMessageAddressTableLookup>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    num_signers: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    num_writable_signers: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    num_writable_non_signers: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    account_keys: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    instructions: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    address_table_lookups: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for VaultTransactionMessage {
            #[inline]
            fn clone(&self) -> VaultTransactionMessage {
                VaultTransactionMessage {
                    num_signers: ::core::clone::Clone::clone(&self.num_signers),
                    num_writable_signers: ::core::clone::Clone::clone(
                        &self.num_writable_signers,
                    ),
                    num_writable_non_signers: ::core::clone::Clone::clone(
                        &self.num_writable_non_signers,
                    ),
                    account_keys: ::core::clone::Clone::clone(&self.account_keys),
                    instructions: ::core::clone::Clone::clone(&self.instructions),
                    address_table_lookups: ::core::clone::Clone::clone(
                        &self.address_table_lookups,
                    ),
                }
            }
        }
        impl kani::Arbitrary for VaultTransactionMessage {
            fn any() -> Self {
                VaultTransactionMessage {
                    num_signers: kani::any(),
                    num_writable_signers: kani::any(),
                    num_writable_non_signers: kani::any(),
                    account_keys: kani::any(),
                    instructions: kani::any(),
                    address_table_lookups: kani::any(),
                }
            }
        }
        impl VaultTransactionMessage {
            /// Returns the number of all the account keys (static + dynamic) in the message.
            pub fn num_all_account_keys(&self) -> usize {
                let num_account_keys_from_lookups = self
                    .address_table_lookups
                    .iter()
                    .map(|lookup| {
                        lookup.writable_indexes.len() + lookup.readonly_indexes.len()
                    })
                    .sum::<usize>();
                self.account_keys.len() + num_account_keys_from_lookups
            }
            /// Returns true if the account at the specified index is a part of static `account_keys` and was requested to be writable.
            pub fn is_static_writable_index(&self, key_index: usize) -> bool {
                let num_account_keys = self.account_keys.len();
                let num_signers = usize::from(self.num_signers);
                let num_writable_signers = usize::from(self.num_writable_signers);
                let num_writable_non_signers = usize::from(
                    self.num_writable_non_signers,
                );
                if key_index >= num_account_keys {
                    return false;
                }
                if key_index < num_writable_signers {
                    return true;
                }
                if key_index >= num_signers {
                    let index_into_non_signers = key_index.saturating_sub(num_signers);
                    return index_into_non_signers < num_writable_non_signers;
                }
                false
            }
            /// Returns true if the account at the specified index was requested to be a signer.
            pub fn is_signer_index(&self, key_index: usize) -> bool {
                key_index < usize::from(self.num_signers)
            }
        }
        impl TryFrom<TransactionMessage> for VaultTransactionMessage {
            type Error = Error;
            fn try_from(message: TransactionMessage) -> Result<Self> {
                let account_keys: Vec<Pubkey> = message.account_keys.into();
                let instructions: Vec<CompiledInstruction> = message.instructions.into();
                let instructions: Vec<MultisigCompiledInstruction> = instructions
                    .into_iter()
                    .map(MultisigCompiledInstruction::from)
                    .collect();
                let address_table_lookups: Vec<MessageAddressTableLookup> = message
                    .address_table_lookups
                    .into();
                let num_all_account_keys = account_keys.len()
                    + address_table_lookups
                        .iter()
                        .map(|lookup| {
                            lookup.writable_indexes.len() + lookup.readonly_indexes.len()
                        })
                        .sum::<usize>();
                if !(usize::from(message.num_signers) <= account_keys.len()) {
                    return Err(Error::Generic);
                }
                if !(message.num_writable_signers <= message.num_signers) {
                    return Err(Error::Generic);
                }
                if !(usize::from(message.num_writable_non_signers)
                    <= account_keys
                        .len()
                        .saturating_sub(usize::from(message.num_signers)))
                {
                    return Err(Error::Generic);
                }
                for instruction in &instructions {
                    if !(usize::from(instruction.program_id_index)
                        < num_all_account_keys)
                    {
                        return Err(Error::Generic);
                    }
                    for account_index in &instruction.account_indexes {
                        if !(usize::from(*account_index) < num_all_account_keys) {
                            return Err(Error::Generic);
                        }
                    }
                }
                Ok(Self {
                    num_signers: message.num_signers,
                    num_writable_signers: message.num_writable_signers,
                    num_writable_non_signers: message.num_writable_non_signers,
                    account_keys,
                    instructions,
                    address_table_lookups: address_table_lookups
                        .into_iter()
                        .map(MultisigMessageAddressTableLookup::from)
                        .collect(),
                })
            }
        }
        /// Concise serialization schema for instructions that make up a transaction.
        /// Closely mimics the Solana transaction wire format.
        pub struct MultisigCompiledInstruction {
            pub program_id_index: u8,
            /// Indices into the tx's `account_keys` list indicating which accounts to pass to the instruction.
            pub account_indexes: Vec<u8>,
            /// Instruction data.
            pub data: Vec<u8>,
        }
        impl borsh::ser::BorshSerialize for MultisigCompiledInstruction
        where
            u8: borsh::ser::BorshSerialize,
            Vec<u8>: borsh::ser::BorshSerialize,
            Vec<u8>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.program_id_index, writer)?;
                borsh::BorshSerialize::serialize(&self.account_indexes, writer)?;
                borsh::BorshSerialize::serialize(&self.data, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for MultisigCompiledInstruction
        where
            u8: borsh::BorshDeserialize,
            Vec<u8>: borsh::BorshDeserialize,
            Vec<u8>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    program_id_index: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    account_indexes: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    data: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for MultisigCompiledInstruction {
            #[inline]
            fn clone(&self) -> MultisigCompiledInstruction {
                MultisigCompiledInstruction {
                    program_id_index: ::core::clone::Clone::clone(
                        &self.program_id_index,
                    ),
                    account_indexes: ::core::clone::Clone::clone(&self.account_indexes),
                    data: ::core::clone::Clone::clone(&self.data),
                }
            }
        }
        impl kani::Arbitrary for MultisigCompiledInstruction {
            fn any() -> Self {
                MultisigCompiledInstruction {
                    program_id_index: kani::any(),
                    account_indexes: kani::any(),
                    data: kani::any(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for MultisigCompiledInstruction {
            #[inline]
            fn default() -> MultisigCompiledInstruction {
                MultisigCompiledInstruction {
                    program_id_index: ::core::default::Default::default(),
                    account_indexes: ::core::default::Default::default(),
                    data: ::core::default::Default::default(),
                }
            }
        }
        impl From<CompiledInstruction> for MultisigCompiledInstruction {
            fn from(compiled_instruction: CompiledInstruction) -> Self {
                Self {
                    program_id_index: compiled_instruction.program_id_index,
                    account_indexes: compiled_instruction.account_indexes.into(),
                    data: compiled_instruction.data.into(),
                }
            }
        }
        /// Address table lookups describe an on-chain address lookup table to use
        /// for loading more readonly and writable accounts into a transaction.
        pub struct MultisigMessageAddressTableLookup {
            /// Address lookup table account key.
            pub account_key: Pubkey,
            /// List of indexes used to load writable accounts.
            pub writable_indexes: Vec<u8>,
            /// List of indexes used to load readonly accounts.
            pub readonly_indexes: Vec<u8>,
        }
        impl borsh::ser::BorshSerialize for MultisigMessageAddressTableLookup
        where
            Pubkey: borsh::ser::BorshSerialize,
            Vec<u8>: borsh::ser::BorshSerialize,
            Vec<u8>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.account_key, writer)?;
                borsh::BorshSerialize::serialize(&self.writable_indexes, writer)?;
                borsh::BorshSerialize::serialize(&self.readonly_indexes, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for MultisigMessageAddressTableLookup
        where
            Pubkey: borsh::BorshDeserialize,
            Vec<u8>: borsh::BorshDeserialize,
            Vec<u8>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    account_key: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    writable_indexes: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                    readonly_indexes: borsh::BorshDeserialize::deserialize_reader(
                        reader,
                    )?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for MultisigMessageAddressTableLookup {
            #[inline]
            fn clone(&self) -> MultisigMessageAddressTableLookup {
                MultisigMessageAddressTableLookup {
                    account_key: ::core::clone::Clone::clone(&self.account_key),
                    writable_indexes: ::core::clone::Clone::clone(
                        &self.writable_indexes,
                    ),
                    readonly_indexes: ::core::clone::Clone::clone(&self.readonly_indexes),
                }
            }
        }
        impl kani::Arbitrary for MultisigMessageAddressTableLookup {
            fn any() -> Self {
                MultisigMessageAddressTableLookup {
                    account_key: kani::any(),
                    writable_indexes: kani::any(),
                    readonly_indexes: kani::any(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for MultisigMessageAddressTableLookup {
            #[inline]
            fn default() -> MultisigMessageAddressTableLookup {
                MultisigMessageAddressTableLookup {
                    account_key: ::core::default::Default::default(),
                    writable_indexes: ::core::default::Default::default(),
                    readonly_indexes: ::core::default::Default::default(),
                }
            }
        }
        impl From<MessageAddressTableLookup> for MultisigMessageAddressTableLookup {
            fn from(m: MessageAddressTableLookup) -> Self {
                Self {
                    account_key: m.account_key,
                    writable_indexes: m.writable_indexes.into(),
                    readonly_indexes: m.readonly_indexes.into(),
                }
            }
        }
    }
}
mod utils {
    mod ephemeral_signers {
        use anchor_lang::prelude::*;
        use crate::state::*;
        /// Return a tuple of ephemeral_signer_keys and ephemeral_signer_seeds derived
        /// from the given `ephemeral_signer_bumps` and `transaction_key`.
        pub fn derive_ephemeral_signers(
            transaction_key: Pubkey,
            ephemeral_signer_bumps: &[u8],
        ) -> (Vec<Pubkey>, Vec<Vec<Vec<u8>>>) {
            let signers = ephemeral_signer_bumps
                .iter()
                .enumerate()
                .map(|(index, bump)| {
                    let seeds: Vec<Vec<u8>> = <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                SEED_PREFIX.to_vec().into(),
                                transaction_key.to_bytes().to_vec().into(),
                                SEED_EPHEMERAL_SIGNER.to_vec().into(),
                                u8::try_from(index).unwrap().to_le_bytes().to_vec().into(),
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([*bump]),
                                    )
                                    .into(),
                            ]),
                        )
                        .into();
                    (
                        Pubkey::create_program_address(
                            seeds
                                .iter()
                                .map(Vec::as_slice)
                                .collect::<Vec<&[u8]>>()
                                .as_slice(),
                            &crate::id(),
                        ),
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
    }
    mod executable_transaction_message {
        use std::convert::From;
        use anchor_lang::prelude::*;
        use anchor_lang::solana_program::instruction::Instruction;
        use solana_address_lookup_table_program::state::AddressLookupTable;
        use solana_address_lookup_table_program::state::LookupTableMeta;
        use crate::errors::*;
        use crate::state::*;
        /// Sanitized and validated combination of a `MsTransactionMessage` and `AccountInfo`s it references.
        pub struct ExecutableTransactionMessage<'a, 'info> {
            /// Message which loaded a collection of lookup table addresses.
            message: &'a VaultTransactionMessage,
            /// Resolved `account_keys` of the message.
            static_accounts: Vec<AccountInfo<'info>>,
            /// Concatenated vector of resolved `writable_indexes` from all address lookups.
            loaded_writable_accounts: Vec<AccountInfo<'info>>,
            /// Concatenated vector of resolved `readonly_indexes` from all address lookups.
            loaded_readonly_accounts: Vec<AccountInfo<'info>>,
        }
        impl<'a, 'info> ExecutableTransactionMessage<'a, 'info> {
            /// # Arguments
            /// `message` - a `MsTransactionMessage`.
            /// `message_account_infos` - AccountInfo's that are expected to be mentioned in the message.
            /// `address_lookup_table_account_infos` - AccountInfo's that are expected to correspond to the lookup tables mentioned in `message.address_table_lookups`.
            /// `vault_pubkey` - The vault PDA that is expected to sign the message.
            pub fn new_validated(
                message: &'a VaultTransactionMessage,
                message_account_infos: &'a [AccountInfo<'info>],
                address_lookup_table_account_infos: &'a [AccountInfo<'info>],
                vault_pubkey: &'a Pubkey,
                ephemeral_signer_pdas: &'a [Pubkey],
            ) -> Result<Self> {
                kani::cover(true, "cover location");
                kani::assume(ephemeral_signer_pdas.len() == 2);
                kani::assume(message.address_table_lookups.len() == 2);
                kani::assume(address_lookup_table_account_infos.len() == 2);
                if address_lookup_table_account_infos.len()
                    != message.address_table_lookups.len()
                {
                    return Err(Error::Generic);
                }
                let mut lookup_tables: HashMap<Pubkey, AccountInfo> = HashMap::new();
                for maybe_lookup_table in address_lookup_table_account_infos {
                    kani::assume(
                        maybe_lookup_table.owner
                            == &solana_address_lookup_table_program::id(),
                    );
                    if !(maybe_lookup_table.owner
                        == &solana_address_lookup_table_program::id())
                    {
                        return Err(Error::Generic);
                    }
                    kani::assume(
                        message
                            .address_table_lookups
                            .iter()
                            .any(|lookup| &lookup.account_key == maybe_lookup_table.key),
                    );
                    if !(message
                        .address_table_lookups
                        .iter()
                        .any(|lookup| &lookup.account_key == maybe_lookup_table.key))
                    {
                        return Err(Error::Generic);
                    }
                    lookup_tables.insert(*maybe_lookup_table.key, *maybe_lookup_table);
                }
                kani::assume(
                    message_account_infos.len() == message.num_all_account_keys(),
                );
                if message_account_infos.len() != message.num_all_account_keys() {
                    return Err(Error::Generic);
                }
                let mut static_accounts = Vec::new();
                kani::assume(message.account_keys.len() == 2);
                for (i, account_key) in message.account_keys.iter().enumerate() {
                    let account_info = &message_account_infos[i];
                    kani::assume(*account_info.key == *account_key);
                    if *account_info.key != *account_key {
                        return Err(Error::Generic);
                    }
                    if message.is_signer_index(i) && account_info.key != vault_pubkey
                        && !ephemeral_signer_pdas.contains(account_info.key)
                    {
                        kani::assume(account_info.is_signer);
                        if !(account_info.is_signer) {
                            return Err(Error::Generic);
                        }
                    }
                    if message.is_static_writable_index(i) {
                        kani::assume(account_info.is_writable);
                        if !(account_info.is_writable) {
                            return Err(Error::Generic);
                        }
                    }
                    static_accounts.push(account_info.clone());
                }
                let mut writable_accounts = Vec::new();
                let mut readonly_accounts = Vec::new();
                let mut message_indexes_cursor = message.account_keys.len();
                for lookup in message.address_table_lookups.iter() {
                    let lookup_table = kani::any::<AddressLookupTable>();
                    kani::assume(lookup.writable_indexes.len() == 2);
                    for (i, index_in_lookup_table) in lookup
                        .writable_indexes
                        .iter()
                        .enumerate()
                    {
                        let index = message_indexes_cursor + i;
                        kani::assume(
                            message_account_infos
                                .get(index)
                                .map_or(
                                    false,
                                    |loaded_account_info| loaded_account_info.is_writable,
                                ),
                        );
                        let loaded_account_info = &message_account_infos
                            .get(index)
                            .ok_or(MultisigError::InvalidNumberOfAccounts)?;
                        if loaded_account_info.is_writable != true {
                            return Err(Error::Generic);
                        }
                        kani::assume(
                            lookup_table
                                .addresses
                                .get(usize::from(*index_in_lookup_table))
                                .map_or(
                                    false,
                                    |pubkey_from_lookup_table| {
                                        *loaded_account_info.key == *pubkey_from_lookup_table
                                    },
                                ),
                        );
                        let pubkey_from_lookup_table = lookup_table
                            .addresses
                            .get(usize::from(*index_in_lookup_table))
                            .ok_or(MultisigError::InvalidAccount)?;
                        if *loaded_account_info.key != *pubkey_from_lookup_table {
                            return Err(Error::Generic);
                        }
                        writable_accounts.push((*loaded_account_info).clone());
                    }
                    message_indexes_cursor += lookup.writable_indexes.len();
                    kani::assume(lookup.readonly_indexes.len() == 2);
                    for (i, index_in_lookup_table) in lookup
                        .readonly_indexes
                        .iter()
                        .enumerate()
                    {
                        let index = message_indexes_cursor + i;
                        kani::assume(
                            message_account_infos
                                .get(index)
                                .map_or(
                                    false,
                                    |loaded_account_info| loaded_account_info.is_writable,
                                ),
                        );
                        let loaded_account_info = &message_account_infos
                            .get(index)
                            .ok_or(MultisigError::InvalidNumberOfAccounts)?;
                        kani::assume(
                            lookup_table
                                .addresses
                                .get(usize::from(*index_in_lookup_table))
                                .map_or(
                                    false,
                                    |pubkey_from_lookup_table| {
                                        *loaded_account_info.key == *pubkey_from_lookup_table
                                    },
                                ),
                        );
                        let pubkey_from_lookup_table = lookup_table
                            .addresses
                            .get(usize::from(*index_in_lookup_table))
                            .ok_or(MultisigError::InvalidAccount)?;
                        if *loaded_account_info.key != *pubkey_from_lookup_table {
                            return Err(Error::Generic);
                        }
                        readonly_accounts.push((*loaded_account_info).clone());
                    }
                    message_indexes_cursor += lookup.readonly_indexes.len();
                }
                Ok(Self {
                    message,
                    static_accounts,
                    loaded_writable_accounts: writable_accounts,
                    loaded_readonly_accounts: readonly_accounts,
                })
            }
            pub fn execute_message(
                &self,
                vault_seeds: &[Vec<u8>],
                ephemeral_signer_seeds: &[Vec<Vec<u8>>],
            ) -> Result<()> {
                for (ix, account_infos) in self.to_instructions_and_accounts().iter() {
                    let vault_seeds = vault_seeds
                        .iter()
                        .map(Vec::as_slice)
                        .collect::<Vec<_>>();
                    let ephemeral_signer_seeds = &ephemeral_signer_seeds
                        .iter()
                        .map(|seeds| {
                            seeds.iter().map(Vec::as_slice).collect::<Vec<&[u8]>>()
                        })
                        .collect::<Vec<Vec<&[u8]>>>();
                    let mut signer_seeds = ephemeral_signer_seeds
                        .iter()
                        .map(Vec::as_slice)
                        .collect::<Vec<&[&[u8]]>>();
                    signer_seeds.push(&vault_seeds);
                }
                Ok(())
            }
            /// Account indices are resolved in the following order:
            /// 1. Static accounts.
            /// 2. All loaded writable accounts.
            /// 3. All loaded readonly accounts.
            fn get_account_by_index(
                &'a self,
                index: usize,
            ) -> Result<&'a AccountInfo<'info>> {
                if index < self.static_accounts.len() {
                    return Ok(&self.static_accounts[index]);
                }
                let index = index - self.static_accounts.len();
                if index < self.loaded_writable_accounts.len() {
                    return Ok(&self.loaded_writable_accounts[index]);
                }
                let index = index - self.loaded_writable_accounts.len();
                if index < self.loaded_readonly_accounts.len() {
                    return Ok(&self.loaded_readonly_accounts[index]);
                }
                Err(MultisigError::InvalidTransactionMessage.into())
            }
            /// Whether the account at the `index` is requested as writable.
            fn is_writable_index(&self, index: usize) -> bool {
                if self.message.is_static_writable_index(index) {
                    return true;
                }
                if index < self.static_accounts.len() {
                    return false;
                }
                let index = index - self.static_accounts.len();
                index < self.loaded_writable_accounts.len()
            }
            pub fn to_instructions_and_accounts(
                &self,
            ) -> Vec<(Instruction, Vec<AccountInfo<'info>>)> {
                let mut executable_instructions = Vec::new();
                for ms_compiled_instruction in self.message.instructions.iter() {
                    let ix_accounts: Vec<(AccountInfo<'info>, AccountMeta)> = ms_compiled_instruction
                        .account_indexes
                        .iter()
                        .map(|account_index| {
                            let account_index = usize::from(*account_index);
                            let account_info = self
                                .get_account_by_index(account_index)
                                .unwrap();
                            let is_signer = self.message.is_signer_index(account_index);
                            let account_meta = if self.is_writable_index(account_index) {
                                AccountMeta::new(*account_info.key, is_signer)
                            } else {
                                AccountMeta::new_readonly(*account_info.key, is_signer)
                            };
                            (account_info.clone(), account_meta)
                        })
                        .collect();
                    let ix_program_account_info = self
                        .get_account_by_index(
                            usize::from(ms_compiled_instruction.program_id_index),
                        )
                        .unwrap();
                    let ix = Instruction {
                        program_id: *ix_program_account_info.key,
                        accounts: ix_accounts
                            .iter()
                            .map(|(_, account_meta)| *account_meta)
                            .collect(),
                        data: ms_compiled_instruction.data,
                    };
                    let mut account_infos: Vec<AccountInfo> = ix_accounts
                        .into_iter()
                        .map(|(account_info, _)| account_info)
                        .collect();
                    account_infos.push(ix_program_account_info.clone());
                    executable_instructions.push((ix, account_infos));
                }
                executable_instructions
            }
        }
    }
    mod small_vec {
        use std::marker::PhantomData;
        use anchor_lang::prelude::*;
        /// Concise serialization schema for vectors where the length can be represented
        /// by any type `L` (typically unsigned integer like `u8` or `u16`)
        /// that implements AnchorDeserialize and can be converted to `u32`.
        pub struct SmallVec<L, T>(Vec<T>, PhantomData<L>);
        #[automatically_derived]
        impl<L: ::core::clone::Clone, T: ::core::clone::Clone> ::core::clone::Clone
        for SmallVec<L, T> {
            #[inline]
            fn clone(&self) -> SmallVec<L, T> {
                SmallVec(
                    ::core::clone::Clone::clone(&self.0),
                    ::core::clone::Clone::clone(&self.1),
                )
            }
        }
        #[automatically_derived]
        impl<L: ::core::fmt::Debug, T: ::core::fmt::Debug> ::core::fmt::Debug
        for SmallVec<L, T> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "SmallVec",
                    &self.0,
                    &&self.1,
                )
            }
        }
        #[automatically_derived]
        impl<
            L: ::core::default::Default,
            T: ::core::default::Default,
        > ::core::default::Default for SmallVec<L, T> {
            #[inline]
            fn default() -> SmallVec<L, T> {
                SmallVec(
                    ::core::default::Default::default(),
                    ::core::default::Default::default(),
                )
            }
        }
        impl<L, T> SmallVec<L, T> {
            pub fn len(&self) -> usize {
                self.0.len()
            }
        }
        impl<L, T> From<SmallVec<L, T>> for Vec<T> {
            fn from(val: SmallVec<L, T>) -> Self {
                val.0
            }
        }
        impl<L, T> AnchorDeserialize for SmallVec<L, T>
        where
            L: AnchorDeserialize + Into<u32>,
            T: AnchorDeserialize + Default,
        {
            /// This implementation almost exactly matches standard implementation of
            /// `Vec<T>::deserialize` except that it uses `L` instead of `u32` for the length,
            /// and doesn't include `unsafe` code.
            fn deserialize(input: &mut &[u8]) -> std::io::Result<Self> {
                let len: u32 = L::deserialize(input)?.into();
                let vec = if len == 0 {
                    Vec::new()
                } else {
                    let mut result = Vec::new();
                    for _ in 0..len {
                        result.push(T::deserialize(input)?);
                    }
                    result
                };
                Ok(SmallVec(vec, PhantomData))
            }
            fn deserialize_reader<R: std::io::Read>(
                _reader: &mut R,
            ) -> std::io::Result<Self> {
                ::core::panicking::panic("not yet implemented")
            }
        }
        mod hint {
            #[inline]
            pub fn _cautious<T>(hint: u32) -> usize {
                let el_size = core::mem::size_of::<T>() as u32;
                core::cmp::max(core::cmp::min(hint, 4096 / el_size), 1) as usize
            }
        }
    }
    mod system {
        use anchor_lang::prelude::*;
        /// Creates `new_account` via a CPI into SystemProgram.
        /// Adapted from Anchor: https://github.com/coral-xyz/anchor/blob/714d5248636493a3d1db1481f16052836ee59e94/lang/syn/src/codegen/accounts/constraints.rs#L1126-L1179
        pub fn create_account<'a, 'info>(
            payer: &'a AccountInfo<'info>,
            new_account: &'a AccountInfo<'info>,
            system_program: &'a AccountInfo<'info>,
            owner_program: &Pubkey,
            rent: &Rent,
            space: usize,
            seeds: Vec<Vec<u8>>,
        ) -> Result<()> {
            let current_lamports = new_account.try_borrow_lamports()?;
            if current_lamports == 0 {
                anchor_lang::system_program::create_account(
                    CpiContext::new(
                            system_program.clone(),
                            anchor_lang::system_program::CreateAccount {
                                from: payer.clone(),
                                to: new_account.clone(),
                            },
                        )
                        .with_signer(
                            &[
                                seeds
                                    .iter()
                                    .map(|seed| seed.as_slice())
                                    .collect::<Vec<&[u8]>>()
                                    .as_slice(),
                            ],
                        ),
                    rent.minimum_balance(space),
                    space as u64,
                    owner_program,
                )
            } else {
                kani::assume(payer.key() != new_account.key());
                if payer.key() == new_account.key() {
                    return Err(Error::Generic);
                }
                let required_lamports = rent
                    .minimum_balance(space)
                    .max(1)
                    .saturating_sub(current_lamports);
                if required_lamports > 0 {
                    anchor_lang::system_program::transfer(
                        CpiContext::new(
                            system_program.clone(),
                            anchor_lang::system_program::Transfer {
                                from: payer.clone(),
                                to: new_account.clone(),
                            },
                        ),
                        required_lamports,
                    )?;
                }
                anchor_lang::system_program::allocate(
                    CpiContext::new(
                            system_program.to_account_info(),
                            anchor_lang::system_program::Allocate {
                                account_to_allocate: new_account.clone(),
                            },
                        )
                        .with_signer(
                            &[
                                seeds
                                    .iter()
                                    .map(|seed| seed.as_slice())
                                    .collect::<Vec<&[u8]>>()
                                    .as_slice(),
                            ],
                        ),
                    space as u64,
                )?;
                anchor_lang::system_program::assign(
                    CpiContext::new(
                            system_program.to_account_info(),
                            anchor_lang::system_program::Assign {
                                account_to_assign: new_account.clone(),
                            },
                        )
                        .with_signer(
                            &[
                                seeds
                                    .iter()
                                    .map(|seed| seed.as_slice())
                                    .collect::<Vec<&[u8]>>()
                                    .as_slice(),
                            ],
                        ),
                    owner_program,
                )
            }
        }
    }
    pub use ephemeral_signers::*;
    pub use executable_transaction_message::*;
    pub use small_vec::*;
    pub use system::*;
}
///this is an id
pub static ID: Pubkey = Pubkey { t: [83u8] };
///this is a function which returns an id
pub fn id() -> Pubkey {
    ID
}
pub mod multisig {
    use super::*;
    /// Create a multisig.
    pub fn multisig_create(
        ctx: Context<MultisigCreate>,
        args: MultisigCreateArgs,
    ) -> Result<()> {
        MultisigCreate::multisig_create(ctx, args)
    }
    /// Add a new member to the multisig.
    pub fn multisig_add_member(
        ctx: Context<MultisigConfig>,
        args: MultisigAddMemberArgs,
    ) -> Result<()> {
        MultisigConfig::multisig_add_member(ctx, args)
    }
    /// Remove a member/key from the multisig.
    pub fn multisig_remove_member(
        ctx: Context<MultisigConfig>,
        args: MultisigRemoveMemberArgs,
    ) -> Result<()> {
        MultisigConfig::multisig_remove_member(ctx, args)
    }
    /// Set the `time_lock` config parameter for the multisig.
    pub fn multisig_set_time_lock(
        ctx: Context<MultisigConfig>,
        args: MultisigSetTimeLockArgs,
    ) -> Result<()> {
        MultisigConfig::multisig_set_time_lock(ctx, args)
    }
    /// Set the multisig `config_authority`.
    pub fn multisig_set_config_authority(
        ctx: Context<MultisigConfig>,
        args: MultisigSetConfigAuthorityArgs,
    ) -> Result<()> {
        MultisigConfig::multisig_set_config_authority(ctx, args)
    }
    /// Create a new config transaction.
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
                    .filter(|&action| match action {
                        ConfigAction::AddMember { .. } => true,
                        _ => false,
                    })
                    .count() <= 10,
        );
        let mut threshold = ctx.accounts.multisig.threshold;
        let members_after = ctx
            .accounts
            .transaction
            .actions
            .iter()
            .fold(
                Some(ctx.accounts.multisig.members),
                |acc, action| match acc {
                    Some(mut members) => {
                        match action {
                            ConfigAction::AddMember { new_member } => {
                                members.push(*new_member);
                                Some(members)
                            }
                            ConfigAction::RemoveMember { old_member } => {
                                if let Some(index)
                                    = members.iter().position(|m| m.key == *old_member)
                                {
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
                                            .map_or(
                                                false,
                                                |acc| { acc.multisig == ctx.accounts.multisig.key() },
                                            )
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
                        }
                    }
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
                        .count() >= threshold as usize
                    && valid_members.iter().all(|m| m.permissions.mask < 8)
                    && if valid_members.len() > ctx.accounts.multisig.members.len() {
                        ctx.accounts.system_program.is_some()
                            && ctx.accounts.rent_payer.is_some()
                    } else {
                        true
                    } && threshold > 0
            }
            None => false,
        };
        if are_members_after_ok { Ok(()) } else { Err(Error::AccountDidNotSerialize) }
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
        kani::assume(ctx.accounts.multisig.transaction_index < u64::MAX - 1);
        kani::assume(args.ephemeral_signers <= 10);
        VaultTransactionCreate::vault_transaction_create(ctx, args)
    }
    /// Execute a vault transaction.
    /// The transaction must be `Approved`.
    pub fn vault_transaction_execute(
        ctx: Context<VaultTransactionExecute>,
    ) -> Result<()> {
        kani::assume(ctx.accounts.transaction.ephemeral_signer_bumps.len() <= 10);
        kani::assume(ctx.remaining_accounts.len() <= 10);
        VaultTransactionExecute::vault_transaction_execute(ctx)
    }
    /// Create a new batch.
    pub fn batch_create(ctx: Context<BatchCreate>, args: BatchCreateArgs) -> Result<()> {
        kani::assume(ctx.accounts.multisig.transaction_index < u64::MAX - 1);
        BatchCreate::batch_create(ctx, args)
    }
    /// Add a transaction to the batch.
    pub fn batch_add_transaction(
        ctx: Context<BatchAddTransaction>,
        args: BatchAddTransactionArgs,
    ) -> Result<()> {
        kani::assume(ctx.accounts.batch.size < u32::MAX);
        kani::assume(args.ephemeral_signers <= 10);
        BatchAddTransaction::batch_add_transaction(ctx, args)
    }
    /// Execute a transaction from the batch.
    pub fn batch_execute_transaction(
        ctx: Context<BatchExecuteTransaction>,
    ) -> Result<()> {
        kani::assume(ctx.accounts.transaction.ephemeral_signer_bumps.len() <= 5);
        kani::assume(ctx.remaining_accounts.len() <= 5);
        kani::assume(
            ctx.accounts.batch.executed_transaction_index < ctx.accounts.batch.size,
        );
        BatchExecuteTransaction::batch_execute_transaction(ctx)
    }
    /// Create a new multisig proposal.
    pub fn proposal_create(
        ctx: Context<ProposalCreate>,
        args: ProposalCreateArgs,
    ) -> Result<()> {
        ProposalCreate::proposal_create(ctx, args)
    }
    /// Update status of a multisig proposal from `Draft` to `Active`.
    pub fn proposal_activate(ctx: Context<ProposalActivate>) -> Result<()> {
        ProposalActivate::proposal_activate(ctx)
    }
    /// Approve a multisig proposal on behalf of the `member`.
    /// The proposal must be `Active`.
    pub fn proposal_approve(
        ctx: Context<ProposalVote>,
        args: ProposalVoteArgs,
    ) -> Result<()> {
        ProposalVote::proposal_approve(ctx, args)
    }
    /// Reject a multisig proposal on behalf of the `member`.
    /// The proposal must be `Active`.
    pub fn proposal_reject(
        ctx: Context<ProposalVote>,
        args: ProposalVoteArgs,
    ) -> Result<()> {
        ProposalVote::proposal_reject(ctx, args)
    }
    /// Cancel a multisig proposal on behalf of the `member`.
    /// The proposal must be `Approved`.
    pub fn proposal_cancel(
        ctx: Context<ProposalVote>,
        args: ProposalVoteArgs,
    ) -> Result<()> {
        ProposalVote::proposal_cancel(ctx, args)
    }
    /// Use a spending limit to transfer tokens from a multisig vault to a destination account.
    pub fn spending_limit_use(
        ctx: Context<SpendingLimitUse>,
        args: SpendingLimitUseArgs,
    ) -> Result<()> {
        kani::assume(ctx.accounts.spending_limit.remaining_amount >= args.amount);
        SpendingLimitUse::spending_limit_use(ctx, args)
    }
}
#[allow(dead_code)]
pub fn verify_multisig_create() {
    let args: MultisigCreateArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<MultisigCreate> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let result = multisig::multisig_create(ctx, args);
    kani::assert(
        result.is_err() || conc.to_ctx().accounts.__post_invariants(),
        "Function failed",
    );
}
#[allow(dead_code)]
pub fn succeeds_if_multisig_create() {
    let args: MultisigCreateArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<MultisigCreate> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let precondition = args.members.len() <= usize::from(u16::MAX)
        && args.members.windows(2).all(|win| win[0].key != win[1].key)
        && args.members.iter().all(|m| m.permissions.mask < 8)
        && args.members.iter().any(|m| m.permissions.has(Permission::Initiate))
        && args.members.iter().any(|m| m.permissions.has(Permission::Execute))
        && args.threshold > 0
        && args.threshold as usize
            <= args
                .members
                .iter()
                .filter(|m| m.permissions.has(Permission::Vote))
                .count();
    kani::assume(precondition);
    let constraints = true;
    let result = if constraints {
        multisig::multisig_create(ctx, args)
    } else {
        Err(Error::Generic)
    };
    kani::assert(result.is_ok(), "function failed to succeed given a precondition");
}
#[allow(dead_code)]
pub fn verify_multisig_add_member() {
    let args: MultisigAddMemberArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<MultisigConfig> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let result = multisig::multisig_add_member(ctx, args);
    kani::assert(
        result.is_err() || conc.to_ctx().accounts.__post_invariants(),
        "Function failed",
    );
}
#[allow(dead_code)]
pub fn succeeds_if_multisig_add_member() {
    let args: MultisigAddMemberArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<MultisigConfig> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let precondition = ctx.accounts.multisig.members.len() <= usize::from(u16::MAX - 1)
        && ctx.accounts.multisig.members.iter().all(|m| m.key != args.new_member.key)
        && ctx.accounts.system_program.is_some() && ctx.accounts.rent_payer.is_some()
        && ctx.accounts.config_authority.key() == ctx.accounts.multisig.config_authority
        && args.new_member.permissions.mask < 8;
    kani::assume(precondition);
    let constraints = true;
    let result = if constraints {
        multisig::multisig_add_member(ctx, args)
    } else {
        Err(Error::Generic)
    };
    kani::assert(result.is_ok(), "function failed to succeed given a precondition");
}
#[allow(dead_code)]
pub fn verify_multisig_remove_member() {
    let args: MultisigRemoveMemberArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<MultisigConfig> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let result = multisig::multisig_remove_member(ctx, args);
    kani::assert(
        result.is_err() || conc.to_ctx().accounts.__post_invariants(),
        "Function failed",
    );
}
#[allow(dead_code)]
pub fn succeeds_if_multisig_remove_member() {
    let args: MultisigRemoveMemberArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<MultisigConfig> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let precondition = ctx.accounts.multisig.members.len() > 1
        && ctx.accounts.multisig.members.iter().any(|m| m.key == args.old_member)
        && ctx
            .accounts
            .multisig
            .members
            .iter()
            .any(|m| m.key != args.old_member && m.permissions.has(Permission::Execute))
        && ctx
            .accounts
            .multisig
            .members
            .iter()
            .any(|m| m.key != args.old_member && m.permissions.has(Permission::Initiate))
        && ctx
            .accounts
            .multisig
            .members
            .iter()
            .filter(|m| m.key != args.old_member && m.permissions.has(Permission::Vote))
            .count() >= ctx.accounts.multisig.threshold as usize
        && ctx.accounts.config_authority.key() == ctx.accounts.multisig.config_authority
        && ctx.accounts.multisig.is_member(args.old_member).is_some()
        && ctx
            .accounts
            .multisig
            .members
            .windows(3)
            .all(|win| win[0].key != win[1].key && win[0].key != win[2].key);
    kani::assume(precondition);
    let constraints = true;
    let result = if constraints {
        multisig::multisig_remove_member(ctx, args)
    } else {
        Err(Error::Generic)
    };
    kani::assert(result.is_ok(), "function failed to succeed given a precondition");
}
#[allow(dead_code)]
pub fn verify_multisig_set_time_lock() {
    let args: MultisigSetTimeLockArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<MultisigConfig> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let result = multisig::multisig_set_time_lock(ctx, args);
    kani::assert(
        result.is_err() || conc.to_ctx().accounts.__post_invariants(),
        "Function failed",
    );
}
#[allow(dead_code)]
pub fn succeeds_if_multisig_set_time_lock() {
    let args: MultisigSetTimeLockArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<MultisigConfig> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let precondition = ctx.accounts.config_authority.key()
        == ctx.accounts.multisig.config_authority;
    kani::assume(precondition);
    let constraints = true;
    let result = if constraints {
        multisig::multisig_set_time_lock(ctx, args)
    } else {
        Err(Error::Generic)
    };
    kani::assert(result.is_ok(), "function failed to succeed given a precondition");
}
#[allow(dead_code)]
pub fn verify_multisig_set_config_authority() {
    let args: MultisigSetConfigAuthorityArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<MultisigConfig> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let result = multisig::multisig_set_config_authority(ctx, args);
    kani::assert(
        result.is_err() || conc.to_ctx().accounts.__post_invariants(),
        "Function failed",
    );
}
#[allow(dead_code)]
pub fn succeeds_if_multisig_set_config_authority() {
    let args: MultisigSetConfigAuthorityArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<MultisigConfig> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let precondition = ctx.accounts.config_authority.key()
        == ctx.accounts.multisig.config_authority;
    kani::assume(precondition);
    let constraints = true;
    let result = if constraints {
        multisig::multisig_set_config_authority(ctx, args)
    } else {
        Err(Error::Generic)
    };
    kani::assert(result.is_ok(), "function failed to succeed given a precondition");
}
#[allow(dead_code)]
pub fn verify_config_transaction_create() {
    let args: ConfigTransactionCreateArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<ConfigTransactionCreate> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let result = multisig::config_transaction_create(ctx, args);
    kani::assert(
        result.is_err() || conc.to_ctx().accounts.__post_invariants(),
        "Function failed",
    );
}
#[allow(dead_code)]
pub fn succeeds_if_config_transaction_create() {
    let args: ConfigTransactionCreateArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<ConfigTransactionCreate> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let precondition = !args.actions.is_empty()
        && ctx.accounts.multisig.config_authority == Pubkey::default()
        && ctx
            .accounts
            .multisig
            .member_has_permission(ctx.accounts.creator.key(), Permission::Initiate)
        && ctx.accounts.multisig.transaction_index < u64::MAX;
    kani::assume(precondition);
    let constraints = true;
    let result = if constraints {
        multisig::config_transaction_create(ctx, args)
    } else {
        Err(Error::Generic)
    };
    kani::assert(result.is_ok(), "function failed to succeed given a precondition");
}
#[allow(dead_code)]
pub fn verify_config_transaction_execute<'info>() {
    let conc: anchor_lang::context::ConcreteContext<
        '_,
        '_,
        '_,
        'info,
        ConfigTransactionExecute<'info>,
    > = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let result = multisig::config_transaction_execute(ctx);
    kani::assert(
        result.is_err() || conc.to_ctx().accounts.__post_invariants(),
        "Function failed",
    );
}
#[allow(dead_code)]
pub fn succeeds_if_config_transaction_execute<'info>() {
    let conc: anchor_lang::context::ConcreteContext<
        '_,
        '_,
        '_,
        'info,
        ConfigTransactionExecute<'info>,
    > = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let precondition = ctx.accounts.multisig.key() == ctx.accounts.proposal.multisig
        && ctx.accounts.multisig.key() == ctx.accounts.transaction.multisig
        && ctx
            .accounts
            .multisig
            .member_has_permission(ctx.accounts.member.key(), Permission::Execute)
        && ctx.accounts.proposal.transaction_index
            > ctx.accounts.multisig.stale_transaction_index
        && match ctx.accounts.proposal.status {
            ProposalStatus::Approved { .. } => true,
            _ => false,
        } && multisig::confix_tx_execute_validation_helper(&ctx).is_ok();
    kani::assume(precondition);
    let constraints = true;
    let result = if constraints {
        multisig::config_transaction_execute(ctx)
    } else {
        Err(Error::Generic)
    };
    kani::assert(result.is_ok(), "function failed to succeed given a precondition");
}
#[allow(dead_code)]
pub fn verify_vault_transaction_create() {
    let args: VaultTransactionCreateArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<VaultTransactionCreate> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let result = multisig::vault_transaction_create(ctx, args);
    kani::assert(
        result.is_err() || conc.to_ctx().accounts.__post_invariants(),
        "Function failed",
    );
}
#[allow(dead_code)]
pub fn succeeds_if_vault_transaction_create() {
    let args: VaultTransactionCreateArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<VaultTransactionCreate> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let precondition = ctx
        .accounts
        .multisig
        .member_has_permission(ctx.accounts.creator.key(), Permission::Initiate);
    kani::assume(precondition);
    let constraints = true;
    let result = if constraints {
        multisig::vault_transaction_create(ctx, args)
    } else {
        Err(Error::Generic)
    };
    kani::assert(result.is_ok(), "function failed to succeed given a precondition");
}
#[allow(dead_code)]
pub fn verify_vault_transaction_execute() {
    let conc: anchor_lang::context::ConcreteContext<VaultTransactionExecute> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let result = multisig::vault_transaction_execute(ctx);
    kani::assert(
        result.is_err() || conc.to_ctx().accounts.__post_invariants(),
        "Function failed",
    );
}
#[allow(dead_code)]
pub fn succeeds_if_vault_transaction_execute() {
    let conc: anchor_lang::context::ConcreteContext<VaultTransactionExecute> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let precondition = ctx
        .accounts
        .multisig
        .member_has_permission(ctx.accounts.member.key(), Permission::Execute)
        && match ctx.accounts.proposal.status {
            ProposalStatus::Approved { .. } => true,
            _ => false,
        } && ctx.accounts.multisig.key() == ctx.accounts.proposal.multisig
        && ctx.accounts.multisig.key() == ctx.accounts.transaction.multisig
        && ctx.remaining_accounts.len()
            == ctx.accounts.transaction.message.address_table_lookups.len()
                + ctx.accounts.transaction.message.num_all_account_keys();
    kani::assume(precondition);
    let constraints = true;
    let result = if constraints {
        multisig::vault_transaction_execute(ctx)
    } else {
        Err(Error::Generic)
    };
    kani::assert(result.is_ok(), "function failed to succeed given a precondition");
}
#[allow(dead_code)]
pub fn verify_batch_create() {
    let args: BatchCreateArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<BatchCreate> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let result = multisig::batch_create(ctx, args);
    kani::assert(
        result.is_err() || conc.to_ctx().accounts.__post_invariants(),
        "Function failed",
    );
}
#[allow(dead_code)]
pub fn succeeds_if_batch_create() {
    let args: BatchCreateArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<BatchCreate> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let precondition = ctx
        .accounts
        .multisig
        .member_has_permission(ctx.accounts.creator.key(), Permission::Initiate);
    kani::assume(precondition);
    let constraints = true;
    let result = if constraints {
        multisig::batch_create(ctx, args)
    } else {
        Err(Error::Generic)
    };
    kani::assert(result.is_ok(), "function failed to succeed given a precondition");
}
#[allow(dead_code)]
pub fn verify_batch_add_transaction() {
    let args: BatchAddTransactionArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<BatchAddTransaction> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let result = multisig::batch_add_transaction(ctx, args);
    kani::assert(
        result.is_err() || conc.to_ctx().accounts.__post_invariants(),
        "Function failed",
    );
}
#[allow(dead_code)]
pub fn succeeds_if_batch_add_transaction() {
    let args: BatchAddTransactionArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<BatchAddTransaction> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let precondition = ctx
        .accounts
        .multisig
        .member_has_permission(ctx.accounts.member.key(), Permission::Initiate)
        && match ctx.accounts.proposal.status {
            ProposalStatus::Draft { .. } => true,
            _ => false,
        } && ctx.accounts.batch.size >= ctx.accounts.batch.executed_transaction_index;
    kani::assume(precondition);
    let constraints = true;
    let result = if constraints {
        multisig::batch_add_transaction(ctx, args)
    } else {
        Err(Error::Generic)
    };
    kani::assert(result.is_ok(), "function failed to succeed given a precondition");
}
#[allow(dead_code)]
pub fn verify_batch_execute_transaction() {
    let conc: anchor_lang::context::ConcreteContext<BatchExecuteTransaction> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let result = multisig::batch_execute_transaction(ctx);
    kani::assert(
        result.is_err() || conc.to_ctx().accounts.__post_invariants(),
        "Function failed",
    );
}
#[allow(dead_code)]
pub fn succeeds_if_batch_execute_transaction() {
    let conc: anchor_lang::context::ConcreteContext<BatchExecuteTransaction> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let precondition = ctx
        .accounts
        .multisig
        .member_has_permission(ctx.accounts.member.key(), Permission::Execute)
        && match ctx.accounts.proposal.status {
            ProposalStatus::Approved { .. } => true,
            _ => false,
        } && ctx.accounts.multisig.key() == ctx.accounts.proposal.multisig
        && ctx.accounts.multisig.key() == ctx.accounts.batch.multisig
        && ctx.remaining_accounts.len()
            == ctx.accounts.transaction.message.address_table_lookups.len()
                + ctx.accounts.transaction.message.num_all_account_keys()
        && ctx.accounts.batch.executed_transaction_index < ctx.accounts.batch.size;
    kani::assume(precondition);
    let constraints = true;
    let result = if constraints {
        multisig::batch_execute_transaction(ctx)
    } else {
        Err(Error::Generic)
    };
    kani::assert(result.is_ok(), "function failed to succeed given a precondition");
}
#[allow(dead_code)]
pub fn verify_proposal_create() {
    let args: ProposalCreateArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<ProposalCreate> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let result = multisig::proposal_create(ctx, args);
    kani::assert(
        result.is_err() || conc.to_ctx().accounts.__post_invariants(),
        "Function failed",
    );
}
#[allow(dead_code)]
pub fn succeeds_if_proposal_create() {
    let args: ProposalCreateArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<ProposalCreate> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let precondition = args.transaction_index <= ctx.accounts.multisig.transaction_index
        && args.transaction_index > ctx.accounts.multisig.stale_transaction_index
        && (ctx
            .accounts
            .multisig
            .member_has_permission(ctx.accounts.creator.key(), Permission::Initiate)
            || ctx
                .accounts
                .multisig
                .member_has_permission(ctx.accounts.creator.key(), Permission::Vote));
    kani::assume(precondition);
    let constraints = true;
    let result = if constraints {
        multisig::proposal_create(ctx, args)
    } else {
        Err(Error::Generic)
    };
    kani::assert(result.is_ok(), "function failed to succeed given a precondition");
}
#[allow(dead_code)]
pub fn verify_proposal_activate() {
    let conc: anchor_lang::context::ConcreteContext<ProposalActivate> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let result = multisig::proposal_activate(ctx);
    kani::assert(
        result.is_err() || conc.to_ctx().accounts.__post_invariants(),
        "Function failed",
    );
}
#[allow(dead_code)]
pub fn succeeds_if_proposal_activate() {
    let conc: anchor_lang::context::ConcreteContext<ProposalActivate> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let precondition = ctx.accounts.proposal.transaction_index
        > ctx.accounts.multisig.stale_transaction_index
        && ctx
            .accounts
            .multisig
            .member_has_permission(ctx.accounts.member.key(), Permission::Initiate)
        && match ctx.accounts.proposal.status {
            ProposalStatus::Draft { .. } => true,
            _ => false,
        } && ctx.accounts.proposal.multisig == ctx.accounts.multisig.key();
    kani::assume(precondition);
    let constraints = true;
    let result = if constraints {
        multisig::proposal_activate(ctx)
    } else {
        Err(Error::Generic)
    };
    kani::assert(result.is_ok(), "function failed to succeed given a precondition");
}
#[allow(dead_code)]
pub fn verify_proposal_approve() {
    let args: ProposalVoteArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<ProposalVote> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let result = multisig::proposal_approve(ctx, args);
    kani::assert(
        result.is_err() || conc.to_ctx().accounts.__post_invariants(),
        "Function failed",
    );
}
#[allow(dead_code)]
pub fn succeeds_if_proposal_approve() {
    let args: ProposalVoteArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<ProposalVote> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let precondition = ctx.accounts.proposal.transaction_index
        > ctx.accounts.multisig.stale_transaction_index
        && ctx
            .accounts
            .multisig
            .member_has_permission(ctx.accounts.member.key(), Permission::Vote)
        && match ctx.accounts.proposal.status {
            ProposalStatus::Active { .. } => true,
            _ => false,
        } && ctx.accounts.proposal.multisig == ctx.accounts.multisig.key()
        && !ctx.accounts.proposal.approved.contains(&ctx.accounts.member.key());
    kani::assume(precondition);
    let constraints = true;
    let result = if constraints {
        multisig::proposal_approve(ctx, args)
    } else {
        Err(Error::Generic)
    };
    kani::assert(result.is_ok(), "function failed to succeed given a precondition");
}
#[allow(dead_code)]
pub fn verify_proposal_reject() {
    let args: ProposalVoteArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<ProposalVote> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let result = multisig::proposal_reject(ctx, args);
    kani::assert(
        result.is_err() || conc.to_ctx().accounts.__post_invariants(),
        "Function failed",
    );
}
#[allow(dead_code)]
pub fn succeeds_if_proposal_reject() {
    let args: ProposalVoteArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<ProposalVote> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let precondition = ctx.accounts.proposal.transaction_index
        > ctx.accounts.multisig.stale_transaction_index
        && ctx
            .accounts
            .multisig
            .member_has_permission(ctx.accounts.member.key(), Permission::Vote)
        && match ctx.accounts.proposal.status {
            ProposalStatus::Active { .. } => true,
            _ => false,
        } && ctx.accounts.proposal.multisig == ctx.accounts.multisig.key()
        && !ctx.accounts.proposal.rejected.contains(&ctx.accounts.member.key());
    kani::assume(precondition);
    let constraints = true;
    let result = if constraints {
        multisig::proposal_reject(ctx, args)
    } else {
        Err(Error::Generic)
    };
    kani::assert(result.is_ok(), "function failed to succeed given a precondition");
}
#[allow(dead_code)]
pub fn verify_proposal_cancel() {
    let args: ProposalVoteArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<ProposalVote> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let result = multisig::proposal_cancel(ctx, args);
    kani::assert(
        result.is_err() || conc.to_ctx().accounts.__post_invariants(),
        "Function failed",
    );
}
#[allow(dead_code)]
pub fn succeeds_if_proposal_cancel() {
    let args: ProposalVoteArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<ProposalVote> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let precondition = ctx
        .accounts
        .multisig
        .member_has_permission(ctx.accounts.member.key(), Permission::Vote)
        && match ctx.accounts.proposal.status {
            ProposalStatus::Approved { .. } => true,
            _ => false,
        } && ctx.accounts.proposal.multisig == ctx.accounts.multisig.key()
        && !ctx.accounts.proposal.cancelled.contains(&ctx.accounts.member.key());
    kani::assume(precondition);
    let constraints = true;
    let result = if constraints {
        multisig::proposal_cancel(ctx, args)
    } else {
        Err(Error::Generic)
    };
    kani::assert(result.is_ok(), "function failed to succeed given a precondition");
}
#[allow(dead_code)]
pub fn verify_spending_limit_use() {
    let args: SpendingLimitUseArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<SpendingLimitUse> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let result = multisig::spending_limit_use(ctx, args);
    kani::assert(
        result.is_err() || conc.to_ctx().accounts.__post_invariants(),
        "Function failed",
    );
}
#[allow(dead_code)]
pub fn succeeds_if_spending_limit_use() {
    let args: SpendingLimitUseArgs = kani::any();
    let conc: anchor_lang::context::ConcreteContext<SpendingLimitUse> = kani::any();
    let ctx = conc.to_ctx();
    kani::assume(conc.to_ctx().accounts.__pre_invariants());
    let precondition = ctx
        .accounts
        .multisig
        .is_member(ctx.accounts.member.key())
        .is_some()
        && ctx.accounts.spending_limit.members.contains(&ctx.accounts.member.key())
        && ctx.accounts.spending_limit.multisig == ctx.accounts.multisig.key()
        && args.amount <= ctx.accounts.spending_limit.amount
        && (ctx.accounts.spending_limit.destinations.is_empty()
            || ctx
                .accounts
                .spending_limit
                .destinations
                .contains(&ctx.accounts.destination.key()))
        && (if ctx.accounts.spending_limit.mint == Pubkey::default() {
            ctx.accounts.mint.is_none() && ctx.accounts.system_program.is_some()
                && args.decimals == 9 && ctx.accounts.vault.lamports() >= args.amount
        } else {
            ctx.accounts.mint.is_some()
                && ctx.accounts.spending_limit.mint
                    == ctx.accounts.mint.as_ref().unwrap().key()
                && ctx.accounts.vault_token_account.is_some()
                && ctx.accounts.destination_token_account.is_some()
                && ctx.accounts.token_program.is_some()
        });
    kani::assume(precondition);
    let constraints = true;
    let result = if constraints {
        multisig::spending_limit_use(ctx, args)
    } else {
        Err(Error::Generic)
    };
    kani::assert(result.is_ok(), "function failed to succeed given a precondition");
}
