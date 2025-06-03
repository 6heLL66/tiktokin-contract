use anchor_lang::prelude::*;
use amm_cpi::{cpi::accounts::Initialize2, program::AmmApi};

impl<'info> Migrate<'info> {
    pub fn process(&mut self, ctx: CpiContext<'a, 'b, 'c, 'info, Initialize2<'info>>, nonce: u8, open_time: u64, init_pc_amount: u64, init_coin_amount: u64) -> Result<()> {
        amm_cpi::cpi::initialize2(ctx, nonce, open_time, init_pc_amount, init_coin_amount)
    }
}
