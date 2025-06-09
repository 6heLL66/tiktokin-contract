use anchor_lang::prelude::*;

#[account]
pub struct LiquidityPda {}

impl<'info>  LiquidityPda {
    pub const SEED_PREFIX: &'static str = "liquidity";
    pub const LEN: usize = 8 + 1;

    pub fn get_signer<'a>(mint: &'a Pubkey, bump: &'a u8) -> [&'a [u8]; 3] {
        [
            Self::SEED_PREFIX.as_bytes(),
            mint.as_ref(),
            std::slice::from_ref(bump),
        ]
    }
}
