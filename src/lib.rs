pub mod accounts;
pub mod instructions;

use anchor_lang::prelude::*;
use mercurial_stable_swap_n_pool_instructions::ID;
pub use {accounts::*, instructions::*};

#[derive(Clone)]
pub struct MercurialStableSwap;

impl anchor_lang::AccountDeserialize for MercurialStableSwap {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self> {
        MercurialStableSwap::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(_buf: &mut &[u8]) -> Result<Self> {
        Ok(MercurialStableSwap)
    }
}

impl anchor_lang::Id for MercurialStableSwap {
    fn id() -> Pubkey {
        ID
    }
}
