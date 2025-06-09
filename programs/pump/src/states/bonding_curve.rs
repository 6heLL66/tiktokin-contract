use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::errors::PumpError;
use crate::utils::{sol_transfer_from_user, sol_transfer_with_signer, token_transfer_user, token_transfer_with_signer, sol_transfer_with_pda_signer};

#[account]
pub struct BondingCurve {
    //  vitual balances on the curve
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,

    //  real balances on the curve
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,

    //  token supply
    pub token_total_supply: u64,

    //  true - if the curve reached the limit
    pub is_completed: bool,

    //  fees collected
    pub fees_collected: u64,
}

impl<'info> BondingCurve {
    pub const SEED_PREFIX: &'static str = "bonding-curve";
    pub const LEN: usize = 8 * 5 + 1;

    //  get signer for bonding curve PDA
    pub fn get_signer<'a>(mint: &'a Pubkey, bump: &'a u8) -> [&'a [u8]; 3] {
        [
            Self::SEED_PREFIX.as_bytes(),
            mint.as_ref(),
            std::slice::from_ref(bump),
        ]
    }

    //  update reserve balance on the curve PDA
    pub fn update_reserves(&mut self, reserve_lamport: u64, reserve_token: u64) -> Result<bool> {
        self.virtual_sol_reserves = reserve_lamport;
        self.virtual_token_reserves = reserve_token;

        Ok(false)
    }

    //  update real reserve balance on the curve PDA
    pub fn update_real_reserves(&mut self, reserve_lamport: u64, reserve_token: u64) -> Result<bool> {
        self.real_sol_reserves = reserve_lamport;
        self.real_token_reserves = reserve_token;

        Ok(false)
    }

    //  swap sol for token
    pub fn buy(
        &mut self,
        token_mint: &Account<'info, Mint>, //  token mint address
        curve_limit: u64,                  //  bonding curve limit
        user: &Signer<'info>,              //  user address

        curve_pda: &mut AccountInfo<'info>, 
        liquidity_pda: &mut AccountInfo<'info>, //  liquidity PDA
        fee_recipient: &mut AccountInfo<'info>, //  team wallet address to get fee

        user_ata: &mut AccountInfo<'info>, //  associated toke accounts for user
        liquidity_ata: &mut AccountInfo<'info>, //  associated toke accounts for liquidity

        amount_in: u64,      //  sol amount to pay
        min_amount_out: u64, //  minimum amount out
        fee_percent: f64,    //  buy fee

        liquidity_bump: u8, // bump for signer

        system_program: &AccountInfo<'info>, //  system program
        token_program: &AccountInfo<'info>,  //  token program
    ) -> Result<bool> {
        require!(
            self.is_completed == false,
            PumpError::CurveCompleted
        );

        let token = token_mint.key();
        let signer_seeds: &[&[&[u8]]] = &[&crate::pda_accounts::LiquidityPda::get_signer(&token, &liquidity_bump)];
        
        let (amount_out, fee_lamports) =
            self.calc_amount_out_for_buy(amount_in, fee_percent)?;

        self.fees_collected = self.fees_collected.checked_add(fee_lamports).ok_or(PumpError::OverflowOrUnderflowOccurred)?;

        //  check min amount out
        require!(
            amount_out >= min_amount_out,
            PumpError::ReturnAmountTooSmall
        );

        //  transfer fee to team wallet
        sol_transfer_from_user(&user, fee_recipient, system_program, fee_lamports)?;
        //  transfer adjusted amount to curve
        sol_transfer_from_user(&user, liquidity_pda, system_program, amount_in - fee_lamports)?;
        //  transfer token from PDA to user
        token_transfer_with_signer(
            liquidity_ata,
            liquidity_pda,
            user_ata,
            token_program,
            signer_seeds,
            amount_out,
        )?;

        //  calculate new reserves
        let new_token_reserves = self
            .virtual_token_reserves
            .checked_sub(amount_out)
            .ok_or(PumpError::OverflowOrUnderflowOccurred)?;

        let new_sol_reserves = self
            .virtual_sol_reserves
            .checked_add(amount_in - fee_lamports)
            .ok_or(PumpError::OverflowOrUnderflowOccurred)?;

        let new_sol_real_reserves = self
            .real_sol_reserves
            .checked_add(amount_in - fee_lamports)
            .ok_or(PumpError::OverflowOrUnderflowOccurred)?;

        let new_token_real_reserves = self
            .real_token_reserves
            .checked_sub(amount_out)
            .ok_or(PumpError::OverflowOrUnderflowOccurred)?;

        msg! {"Reserves:: Token: {:?} SOL: {:?}", new_token_reserves, new_sol_reserves};

        //  update reserves on the curve
        self.update_reserves(new_sol_reserves, new_token_reserves)?;
        self.update_real_reserves(new_sol_real_reserves, new_token_real_reserves)?;
        //  return true if the curve reached the limit
        if new_sol_reserves >= curve_limit {
            self.is_completed = true;
            return Ok(true);
        }

        //  return false, curve is not reached the limit
        Ok(false)
    }

    //  swap token for sol
    pub fn sell(
        &mut self,
        token_mint: &Account<'info, Mint>, //  token mint address
        user: &Signer<'info>,              //  user address

        curve_pda: &mut AccountInfo<'info>, //  bonding curve PDA
        liquidity_pda: &mut AccountInfo<'info>, //  liquidity PDA
        fee_recipient: &mut AccountInfo<'info>, //  team wallet address to get fee

        user_ata: &mut AccountInfo<'info>, //  associated toke accounts for user
        liquidity_ata: &mut AccountInfo<'info>, //  associated toke accounts for liquidity

        amount_in: u64,      //  tokens amount to pay
        min_amount_out: u64, //  minimum amount out
        fee_percent: f64,    //  sell fee

        liquidity_bump: u8, // bump for signer
        
        system_program: &AccountInfo<'info>, //  system program
        token_program: &AccountInfo<'info>,  //  token program
    ) -> Result<()> {
        require!(
            self.is_completed == false,
            PumpError::CurveCompleted
        );

        let (amount_out, fee_lamports) =
            self.calc_amount_out_for_sell(amount_in, fee_percent)?;

        self.fees_collected = self.fees_collected.checked_add(fee_lamports).ok_or(PumpError::OverflowOrUnderflowOccurred)?;
        
        msg!("Calculated amount_out: {}, fee_lamports: {}", amount_out, fee_lamports);

        require!(
            amount_out >= min_amount_out,
            PumpError::ReturnAmountTooSmall
        );

        let token = token_mint.key();
        let signer_seeds: &[&[&[u8]]] = &[&crate::pda_accounts::LiquidityPda::get_signer(&token, &liquidity_bump)];

        msg!("Transferring fee {} to recipient {}", fee_lamports, fee_recipient.key);
        sol_transfer_from_user(&user, fee_recipient, system_program, fee_lamports)?;

        msg!("Transferring tokens {} from user to curve", amount_out);
        token_transfer_user(
            user_ata,
            user,
            liquidity_ata,
            token_program,
            amount_in,
        )?;

        msg!("Transferring SOL {} to user {}", amount_in - fee_lamports, user.key);
        sol_transfer_with_pda_signer(
            liquidity_pda,
            &user,
            amount_out - fee_lamports,
        )?;

        msg!("Calculating new reserves with:");
        msg!("amount_in: {}, amount_out: {}, fee_lamports: {}", amount_in, amount_out, fee_lamports);
        msg!("Current virtual reserves - token: {}, sol: {}", 
            self.virtual_token_reserves, self.virtual_sol_reserves);
        msg!("Current real reserves - token: {}, sol: {}", 
            self.real_token_reserves, self.real_sol_reserves);

        let new_token_reserves = self
            .virtual_token_reserves
            .checked_add(amount_in)
            .ok_or(PumpError::OverflowOrUnderflowOccurred)?;
        msg!("new_token_reserves: {}", new_token_reserves);

        let new_sol_reserves = self
            .virtual_sol_reserves
            .checked_sub(amount_out - fee_lamports)
            .ok_or(PumpError::OverflowOrUnderflowOccurred)?;
        msg!("new_sol_reserves: {}", new_sol_reserves);

        let new_sol_real_reserves = self
            .real_sol_reserves
            .checked_sub(amount_out - fee_lamports)
            .ok_or(PumpError::OverflowOrUnderflowOccurred)?;
        msg!("new_sol_real_reserves: {}", new_sol_real_reserves);

        let new_token_real_reserves = self
            .real_token_reserves
            .checked_add(amount_in)
            .ok_or(PumpError::OverflowOrUnderflowOccurred)?;
        msg!("new_token_real_reserves: {}", new_token_real_reserves);

        msg!("New reserves calculated:");
        msg!("Virtual - SOL: {}, Token: {}", new_sol_reserves, new_token_reserves);
        msg!("Real - SOL: {}, Token: {}", new_sol_real_reserves, new_token_real_reserves);

        self.update_reserves(new_sol_reserves, new_token_reserves)?;
        self.update_real_reserves(new_sol_real_reserves, new_token_real_reserves)?;

        msg!("Sell operation completed successfully");
        Ok(())
    }

    //  calculate amount out and fee lamports
    fn calc_amount_out_for_buy(
        &mut self,
        _amount_in: u64,
        _fee_percent: f64,
    ) -> Result<(u64, u64)> {
        let fee_lamports = ((_amount_in as f64) * _fee_percent / 100.0).round() as u64;
        let net_amount_in = _amount_in - fee_lamports;

        if self.virtual_token_reserves == 0 || self.virtual_sol_reserves == 0 {
            return Ok((0, fee_lamports));
        }

        let amount_out = (net_amount_in as u128)
        .saturating_mul(self.virtual_token_reserves as u128)
        .checked_div((self.virtual_sol_reserves as u128).checked_add(net_amount_in as u128)
            .ok_or(PumpError::OverflowOrUnderflowOccurred)?)
        .unwrap_or(0) as u64;

        Ok((amount_out, fee_lamports))
    }

    fn calc_amount_out_for_sell(
        &mut self,
        _amount_in: u64,
        _fee_percent: f64,
    ) -> Result<(u64, u64)> {
        if self.virtual_token_reserves == 0 || self.virtual_sol_reserves == 0 {
            return Ok((0, 0));
        }

        let amount_out = (_amount_in as u128)
                .saturating_mul(self.virtual_sol_reserves as u128)
                .checked_div((self.virtual_token_reserves as u128).checked_add(_amount_in as u128)
                    .ok_or(PumpError::OverflowOrUnderflowOccurred)?)
                .unwrap_or(0) as u64;

        let fee_lamports = ((amount_out as f64) * _fee_percent / 100.0).round() as u64;

        Ok((amount_out, fee_lamports))
    }
}
