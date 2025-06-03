use anchor_lang::prelude::*;
use crate::amm_cpi;

#[derive(Accounts)]
pub struct InitializeAmm<'info> {
    /// AMM программа которую мы будем вызывать
    pub amm_program: Program<'info, amm_cpi::program::Amm>,
    
    /// Аккаунт AMM который будет создан
    #[account(mut)]
    pub amm: AccountInfo<'info>,
    
    /// Authority для AMM
    pub amm_authority: AccountInfo<'info>,
    
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    
    #[account(mut)]
    pub amm_lp_mint: AccountInfo<'info>,
    
    pub amm_coin_mint: AccountInfo<'info>,
    pub amm_pc_mint: AccountInfo<'info>,
    
    #[account(mut)]
    pub amm_coin_vault: AccountInfo<'info>,
    
    #[account(mut)]
    pub amm_pc_vault: AccountInfo<'info>,
    
    #[account(mut)]
    pub amm_target_orders: AccountInfo<'info>,
    
    pub amm_config: AccountInfo<'info>,
    
    #[account(mut)]
    pub create_fee_destination: AccountInfo<'info>,
    
    pub market_program: AccountInfo<'info>,
    pub market: AccountInfo<'info>,
    
    #[account(mut, signer)]
    pub user_wallet: AccountInfo<'info>,
    
    #[account(mut)]
    pub user_token_coin: AccountInfo<'info>,
    
    #[account(mut)]
    pub user_token_pc: AccountInfo<'info>,
    
    #[account(mut)]
    pub user_token_lp: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<InitializeAmm>,
    nonce: u8,
    open_time: u64,
    init_pc_amount: u64,
    init_coin_amount: u64,
) -> Result<()> {
    // Создаем CpiContext для вызова initialize
    let cpi_ctx = CpiContext::new(
        ctx.accounts.amm_program.to_account_info(),
        amm_cpi::Initialize2 {
            amm: ctx.accounts.amm.to_account_info(),
            amm_authority: ctx.accounts.amm_authority.to_account_info(),
            amm_open_orders: ctx.accounts.amm_open_orders.to_account_info(),
            amm_lp_mint: ctx.accounts.amm_lp_mint.to_account_info(),
            amm_coin_mint: ctx.accounts.amm_coin_mint.to_account_info(),
            amm_pc_mint: ctx.accounts.amm_pc_mint.to_account_info(),
            amm_coin_vault: ctx.accounts.amm_coin_vault.to_account_info(),
            amm_pc_vault: ctx.accounts.amm_pc_vault.to_account_info(),
            amm_target_orders: ctx.accounts.amm_target_orders.to_account_info(),
            amm_config: ctx.accounts.amm_config.to_account_info(),
            create_fee_destination: ctx.accounts.create_fee_destination.to_account_info(),
            market_program: ctx.accounts.market_program.to_account_info(),
            market: ctx.accounts.market.to_account_info(),
            user_wallet: ctx.accounts.user_wallet.to_account_info(),
            user_token_coin: ctx.accounts.user_token_coin.to_account_info(),
            user_token_pc: ctx.accounts.user_token_pc.to_account_info(),
            user_token_lp: ctx.accounts.user_token_lp.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
    );

    // Вызываем initialize через CPI
    amm_cpi::initialize(
        cpi_ctx,
        nonce,
        open_time,
        init_pc_amount,
        init_coin_amount,
    )?;

    Ok(())
} 