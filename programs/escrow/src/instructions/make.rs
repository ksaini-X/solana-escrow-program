use anchor_lang::prelude::*;
use anchor_spl::token_2022::TransferChecked;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked};
use crate::state::Escrow;
use crate::error::EscrowError;


#[derive(Accounts)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    
    pub taker:SystemAccount<'info>, 

    pub token_mint_a: InterfaceAccount<'info, Mint>,
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut, 
        token::mint = token_mint_a, 
        token::authority = maker
    )]
    pub maker_token_mint_a_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init, 
        seeds = [
            b"escrow", 
            maker.key().as_ref(), taker.key().as_ref(), 
            token_mint_a.key().as_ref(), token_mint_b.key().as_ref()
        ], 
        space = Escrow::size(), 
        bump, 
        payer = maker
    )]
    pub escrow : Account<'info, Escrow>, 

    #[account(
        init, 
        payer = maker, 
        token::mint = token_mint_a, 
        token::authority = escrow, 
        token::token_program = token_program
    )]
    pub vault : InterfaceAccount<'info, TokenAccount>, 

    pub token_program : Interface<'info, TokenInterface>, 
    pub system_program : Program<'info, System>, 
    pub rent : Sysvar<'info, Rent>

}

pub fn handler(ctx: Context<Make>, amount:u64, recieve:u64, expiry:i64) -> Result<()> {

    require!(ctx.accounts.maker.key() != ctx.accounts.taker.key(), EscrowError::SamePubkey);
    require!(recieve > 0, EscrowError::InvalidReceive);
    require!(expiry > Clock::get()?.unix_timestamp, EscrowError::InvalidExpiryTime);

    transfer_checked(CpiContext::new(
        ctx.accounts.token_program.key(), 
        TransferChecked{
                    authority:ctx.accounts.maker.to_account_info(), 
                    from:ctx.accounts.maker_token_mint_a_account.to_account_info(), 
                    mint:ctx.accounts.token_mint_a.to_account_info(), 
                    to:ctx.accounts.vault.to_account_info()
        }),
        amount, 
        ctx.accounts.token_mint_a.decimals
    )?;

    let escrow = &mut ctx.accounts.escrow;

    escrow.maker = ctx.accounts.maker.key();
    escrow.taker = *ctx.accounts.taker.key;
    
    escrow.token_mint_a = ctx.accounts.token_mint_a.key();
    escrow.token_mint_b = ctx.accounts.token_mint_b.key();

    escrow.vault = ctx.accounts.vault.key();
    
    escrow.recieve = recieve;
    escrow.expiry = expiry;
    escrow.bump = ctx.bumps.escrow;
    
    Ok(())
   
}
