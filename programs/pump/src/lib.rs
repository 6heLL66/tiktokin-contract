use anchor_lang::prelude::*;

pub mod consts;
pub mod errors;
pub mod instructions;
pub mod states;
pub mod utils;

use crate::instructions::*;

declare_id!("GbQQb1eHf8cjFHqqsaDcUyuCZ8eFB17nXCVSvJ1kQ38Q");

#[program]
pub mod pump {
    use super::*;

    //  called by admin to set global config
    //  need to check the signer is authority
    pub fn configure(ctx: Context<Configure>, new_config: states::Config) -> Result<()> {
        ctx.accounts.process(new_config)
    }

    //  called by a creator to launch a token on the platform
    pub fn launch<'info>(
        ctx: Context<'_, '_, '_, 'info, Launch<'info>>,

        //  metadata
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        ctx.accounts
            .process(name, symbol, uri, ctx.bumps.global_config)
    }

    //  called by a user to swap token/sol
    pub fn swap<'info>(
        ctx: Context<'_, '_, '_, 'info, Swap<'info>>,
        amount: u64,
        direction: u8,
        min_out: u64,
    ) -> Result<()> {
        ctx.accounts
            .process(amount, direction, min_out, ctx.bumps.bonding_curve)
    }

    ////////////////////    DM if you want full implementation  ////////////////////
    // telegram - https://t.me/microgift88
    // discord - https://discord.com/users/1074514238325927956

    //  migrate the token to raydium once a curve reaches the limit
    pub fn migrate<'info>(
        ctx: Context<'_, '_, '_, 'info, Migrate<'info>>,
        nonce: u8,
        open_time: u64,
        init_pc_amount: u64,
        init_coin_amount: u64,
    ) -> Result<()> {
        ctx.accounts.process(ctx, nonce, open_time, init_pc_amount, init_coin_amount)
    }
}
