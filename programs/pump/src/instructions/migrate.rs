use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Token, SyncNative, Burn};
use anchor_spl::token_interface::{Mint, TokenInterface, TokenAccount};
use crate::errors::PumpError;
use raydium_cp_swap::{
    cpi,
    program::RaydiumCpSwap,
    states::{AmmConfig, OBSERVATION_SEED, POOL_LP_MINT_SEED, POOL_SEED, POOL_VAULT_SEED},
};
use crate::states::BondingCurve;
use crate::pda_accounts::LiquidityPda;
use crate::utils::{sol_transfer_with_signer, token_transfer_with_signer, sol_transfer_with_pda_signer};

#[derive(Accounts)]
pub struct Migrate<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    pub cp_swap_program: Program<'info, RaydiumCpSwap>,

    /// Which config the pool belongs to.
    pub amm_config: Box<Account<'info, AmmConfig>>,

    /// CHECK: liquidity pda
    #[account(
        mut,
        seeds = [LiquidityPda::SEED_PREFIX.as_bytes(), &token_1_mint.key().to_bytes()],
        bump
    )]
    liquidity_pda: Box<Account<'info, LiquidityPda>>,

    /// CHECK: curve token ata
    #[account(
        mut,
        token::mint = token_1_mint,
        token::authority = liquidity_pda,  // PDA владеет токенами
    )]
    pub liquidity_token_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK: pool vault and lp mint authority
    #[account(
        seeds = [
            raydium_cp_swap::AUTH_SEED.as_bytes(),
        ],
        seeds::program = cp_swap_program,
        bump,
    )]
    pub authority: UncheckedAccount<'info>,

    /// CHECK: Initialize an account to store the pool state, init by cp-swap
    #[account(
        mut,
        seeds = [
            POOL_SEED.as_bytes(),
            amm_config.key().as_ref(),
            token_0_mint.key().as_ref(),
            token_1_mint.key().as_ref(),
        ],
        seeds::program = cp_swap_program,
        bump,
    )]
    pub pool_state: UncheckedAccount<'info>,

    /// Token_0 mint, the key must smaller then token_1 mint.
    #[account(
        constraint = token_0_mint.key() < token_1_mint.key(),
        mint::token_program = token_0_program,
    )]
    pub token_0_mint: Box<InterfaceAccount<'info, Mint>>,

    /// Token_1 mint, the key must grater then token_0 mint.
    #[account(
        mint::token_program = token_1_program,
    )]
    pub token_1_mint: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK: pool lp mint, init by cp-swap
    #[account(
        mut,
        seeds = [
            POOL_LP_MINT_SEED.as_bytes(),
            pool_state.key().as_ref(),
        ],
        seeds::program = cp_swap_program,
        bump,
    )]
    pub lp_mint: UncheckedAccount<'info>,

    /// payer token0 account - принадлежит curve PDA
    #[account(
        mut,
        token::mint = token_0_mint,
        token::authority = creator,  // PDA владеет токенами
    )]
    pub creator_token_0: Box<InterfaceAccount<'info, TokenAccount>>,

    /// creator token1 account - принадлежит curve PDA
    #[account(
        mut,
        token::mint = token_1_mint,
        token::authority = creator,  // PDA владеет токенами
    )]
    pub creator_token_1: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK: creator lp ATA token account, init by cp-swap
    #[account(mut)]
    pub creator_lp_token: UncheckedAccount<'info>,

    /// CHECK: Token_0 vault for the pool, init by cp-swap
    #[account(
        mut,
        seeds = [
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            token_0_mint.key().as_ref()
        ],
        seeds::program = cp_swap_program,
        bump,
    )]
    pub token_0_vault: UncheckedAccount<'info>,

    /// CHECK: Token_1 vault for the pool, init by cp-swap
    #[account(
        mut,
        seeds = [
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            token_1_mint.key().as_ref()
        ],
        seeds::program = cp_swap_program,
        bump,
    )]
    pub token_1_vault: UncheckedAccount<'info>,

    /// create pool fee account
    #[account(
        mut,
        address= raydium_cp_swap::create_pool_fee_reveiver::ID,
    )]
    pub create_pool_fee: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK: an account to store oracle observations, init by cp-swap
    #[account(
        mut,
        seeds = [
            OBSERVATION_SEED.as_bytes(),
            pool_state.key().as_ref(),
        ],
        seeds::program = cp_swap_program,
        bump,
    )]
    pub observation_state: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [BondingCurve::SEED_PREFIX.as_bytes(), token_1_mint.key().as_ref()],
        bump,
    )]
    bonding_curve: Box<Account<'info, BondingCurve>>,

    /// Program to create mint account and mint tokens
    pub token_program: Program<'info, Token>,
    /// Spl token program or token program 2022
    pub token_0_program: Interface<'info, TokenInterface>,
    /// Spl token program or token program 2022
    pub token_1_program: Interface<'info, TokenInterface>,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// To create a new program account
    pub system_program: Program<'info, System>,
    /// Sysvar for program account
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Migrate<'info> {
    pub fn process(&mut self, open_time: u64, bump_liquidity: u8) -> Result<()> {
        let curve_pda = self.bonding_curve.to_account_info();
        let token_mint_key = self.token_1_mint.key();
        let signer_seeds: &[&[&[u8]]] = &[&LiquidityPda::get_signer(&token_mint_key, &bump_liquidity)];

        require!(
            self.bonding_curve.is_completed == true,
            PumpError::CurveNotCompleted
        );

        token_transfer_with_signer(
            &self.liquidity_token_ata.to_account_info(),
            &self.liquidity_pda.to_account_info(),
            &self.creator_token_1.to_account_info(),
            &self.system_program.to_account_info(),
            signer_seeds,
            self.bonding_curve.real_token_reserves,
        )?;

        sol_transfer_with_pda_signer(
            &self.liquidity_pda.to_account_info(),
            &self.creator.to_account_info(),
            self.bonding_curve.real_sol_reserves,
        )?;

        anchor_spl::token::sync_native(CpiContext::new(
            self.token_program.to_account_info(),
            SyncNative {
                account: self.creator_token_0.to_account_info(),
            }
        ))?;

        let cpi_accounts = cpi::accounts::Initialize {
            creator: self.creator.to_account_info(),
            amm_config: self.amm_config.to_account_info(),
            authority: self.authority.to_account_info(),
            pool_state: self.pool_state.to_account_info(),
            token_0_mint: self.token_0_mint.to_account_info(),
            token_1_mint: self.token_1_mint.to_account_info(),
            lp_mint: self.lp_mint.to_account_info(),
            creator_token_0: self.creator_token_0.to_account_info(),
            creator_token_1: self.creator_token_1.to_account_info(),
            creator_lp_token: self.creator_lp_token.to_account_info(),
            token_0_vault: self.token_0_vault.to_account_info(),
            token_1_vault: self.token_1_vault.to_account_info(),
            create_pool_fee: self.create_pool_fee.to_account_info(),
            observation_state: self.observation_state.to_account_info(),
            token_program: self.token_program.to_account_info(),
            token_0_program: self.token_0_program.to_account_info(),
            token_1_program: self.token_1_program.to_account_info(),
            associated_token_program: self.associated_token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        let cpi_context = CpiContext::new(self.cp_swap_program.to_account_info(), cpi_accounts).with_remaining_accounts(vec![self.liquidity_pda.to_account_info()]);
        cpi::initialize(cpi_context, self.bonding_curve.real_sol_reserves, self.bonding_curve.real_token_reserves, open_time)?;

        // Burn LP tokens after migration
        let burn_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            Burn {
                mint: self.lp_mint.to_account_info(),
                from: self.creator_lp_token.to_account_info(),
                authority: self.creator.to_account_info(),
            },
        );
        anchor_spl::token::burn(burn_ctx, u64::MAX)?;

        Ok(())
    }
}
