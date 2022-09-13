use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Liquidity<'info> {
    pub stable_swap: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub pool_authority: AccountInfo<'info>,
    pub user: AccountInfo<'info>,
    pub pool_mint: AccountInfo<'info>,
    pub user_pool_token: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RemoveLiquidityOneToken<'info> {
    pub liquidity: Liquidity<'info>,
    pub destination_token: AccountInfo<'info>,
}
