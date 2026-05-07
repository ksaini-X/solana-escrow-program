use anchor_lang::prelude::*;
use crate::error::EscrowError;
use crate::state::Escrow;
use anchor_spl::{token_2022::TransferChecked, token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked}};
#[derive(Accounts)]
pub struct Take<'info> {
    
    #[account(mut)]
    pub taker: Signer<'info>,
    
    pub maker:SystemAccount<'info>, 

    pub token_mint_a: InterfaceAccount<'info, Mint>,
    pub token_mint_b: InterfaceAccount<'info, Mint>,

   
    #[account(
        mut, 
        seeds = [
            b"escrow", 
            maker.key().as_ref(), taker.key().as_ref(), 
            token_mint_a.key().as_ref(), token_mint_b.key().as_ref()
        ], 
        bump, 
        has_one = maker, 
        has_one = token_mint_a, 
        has_one = token_mint_b, 
        constraint = escrow.maker == maker.key() @ EscrowError::InvalidMaker,
        close = maker
    )]
    pub escrow : Account<'info, Escrow>, 

    #[account(
        mut, 
        token::mint = token_mint_a, 
        token::authority = escrow, 
        token::token_program = token_program
    )]
    pub vault : InterfaceAccount<'info, TokenAccount>, 

    #[account(
        init_if_needed, 
        payer = taker, 
        token::mint = token_mint_a, 
        token::authority = taker, 
        token::token_program = token_program
    )]
    pub taker_token_mint_a_account : InterfaceAccount<'info, TokenAccount>, 

    #[account(
        init_if_needed, 
        payer = taker, 
        token::mint = token_mint_b, 
        token::authority = maker, 
        token::token_program = token_program
    )]
    pub maker_token_mint_b_account : InterfaceAccount<'info, TokenAccount>, 

    #[account(
        mut, 
        token::mint = token_mint_a, 
        token::authority = taker, 
        token::token_program = token_program
    )]
    pub taker_token_mint_b_account : InterfaceAccount<'info, TokenAccount>, 

    pub token_program : Interface<'info, TokenInterface>, 
    pub system_program : Program<'info, System>, 


}

pub fn take(ctx:Context<Take>)->Result<()>{
    require!(ctx.accounts.escrow.expiry > Clock::get()?.unix_timestamp, EscrowError::EscrowExpired);

    transfer_checked(CpiContext::new_with_signer(
        *ctx.accounts.token_program.key, 
        TransferChecked{
            authority:ctx.accounts.escrow.to_account_info(), 
            from:ctx.accounts.vault.to_account_info(), 
            mint:ctx.accounts.token_mint_a.to_account_info(), 
            to:ctx.accounts.taker_token_mint_a_account.to_account_info()
        }, &[
            &[
                b"escrow", 
                ctx.accounts.maker.key().as_ref(),
                ctx.accounts.taker.key().as_ref(),
                ctx.accounts.token_mint_a.key().as_ref(),
                ctx.accounts.token_mint_b.key().as_ref(),
                ctx.accounts.escrow.bump.to_le_bytes().as_ref()
            ]
        ]), ctx.accounts.vault.amount, ctx.accounts.token_mint_a.decimals)?;

        transfer_checked(CpiContext::new(
            *ctx.accounts.token_program.key, 
        TransferChecked{
            authority:ctx.accounts.taker.to_account_info(), 
            from:ctx.accounts.taker_token_mint_b_account.to_account_info(), 
            mint:ctx.accounts.token_mint_b.to_account_info(), 
            to:ctx.accounts.maker_token_mint_b_account.to_account_info()
        }), 
        ctx.accounts.escrow.recieve, 
        ctx.accounts.token_mint_b.decimals
    )?;

    Ok(())
}