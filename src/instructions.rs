use crate::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use mercurial_stable_swap_n_pool_instructions::instruction::{
    add_liquidity as add_liquidity_ix, remove_liquidity as remove_liquidity_ix,
    remove_liquidity_one_token as remove_liquidity_one_token_ix,
};

pub fn add_liquidity<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Liquidity<'info>>,
    in_amounts: Vec<u64>,
    min_mint_amount: u64,
) -> Result<()> {
    let len_half = ctx.remaining_accounts.len() >> 1; // div by 2
    let remaing_accounts: Vec<AccountInfo<'info>> = ctx.remaining_accounts.to_vec();
    let swap_token_accounts = &remaing_accounts[0..len_half];
    let user_token_accounts = &remaing_accounts[len_half..ctx.remaining_accounts.len()];

    let ix = add_liquidity_ix(
        &ID,
        ctx.accounts.stable_swap.key,
        ctx.accounts.token_program.key,
        ctx.accounts.pool_authority.key,
        ctx.accounts.user.key,
        swap_token_accounts.iter().map(|acc| acc.key).collect(),
        ctx.accounts.pool_mint.key,
        user_token_accounts.iter().map(|acc| acc.key).collect(),
        ctx.accounts.user_pool_token.key,
        in_amounts,
        min_mint_amount,
    )?;
    invoke_signed(
        &ix,
        &[
            ctx.to_account_infos(),
            ctx.remaining_accounts.to_account_infos(),
        ]
        .concat(),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn remove_liquidity<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'_, '_, '_, 'info, Liquidity<'info>>,
    burn_amount: u64,
    min_out_amounts: Vec<u64>,
) -> Result<()> {
    let len_half = ctx.remaining_accounts.len() >> 1; // div by 2
    let remaing_accounts: Vec<AccountInfo<'info>> = ctx.remaining_accounts.to_vec();
    let swap_token_accounts = &remaing_accounts[0..len_half];
    let user_token_accounts = &remaing_accounts[len_half..ctx.remaining_accounts.len()];

    let ix = remove_liquidity_ix(
        &ID,
        ctx.accounts.stable_swap.key,
        ctx.accounts.token_program.key,
        ctx.accounts.pool_authority.key,
        ctx.accounts.user.key,
        swap_token_accounts.iter().map(|acc| acc.key).collect(),
        ctx.accounts.pool_mint.key,
        user_token_accounts.iter().map(|acc| acc.key).collect(),
        ctx.accounts.user_pool_token.key,
        burn_amount,
        min_out_amounts,
    )?;
    invoke_signed(
        &ix,
        &[
            ctx.to_account_infos(),
            ctx.remaining_accounts.to_account_infos(),
        ]
        .concat(),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}

pub fn remove_liquidity_one_token<'a, 'b, 'c, 'info>(
    ctx: CpiContext<'_, '_, '_, 'info, RemoveLiquidityOneToken<'info>>,
    burn_amount: u64,
    min_out_amount: u64,
) -> Result<()> {
    let remaing_accounts: Vec<AccountInfo<'info>> = ctx.remaining_accounts.to_vec();
    let swap_token_accounts = &remaing_accounts[..];

    let ix = remove_liquidity_one_token_ix(
        &ID,
        ctx.accounts.liquidity.stable_swap.key,
        ctx.accounts.liquidity.token_program.key,
        ctx.accounts.liquidity.pool_authority.key,
        ctx.accounts.liquidity.user.key,
        swap_token_accounts.iter().map(|acc| acc.key).collect(),
        ctx.accounts.liquidity.pool_mint.key,
        ctx.accounts.destination_token.key,
        ctx.accounts.liquidity.user_pool_token.key,
        burn_amount,
        min_out_amount,
    )?;
    invoke_signed(
        &ix,
        &[
            ctx.to_account_infos(),
            ctx.remaining_accounts.to_account_infos(),
        ]
        .concat(),
        ctx.signer_seeds,
    )
    .map_err(Into::into)
}
