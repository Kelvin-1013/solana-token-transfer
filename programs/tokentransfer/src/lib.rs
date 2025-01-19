use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::associated_token::AssociatedToken;

declare_id!("ALQEfQjpxyXa7xnvakAFf7FmhvJH5xMz3hznMGK7iKXP");

#[program]
pub mod my_token_program {
    use super::*;
    pub fn transfer_token(ctx: Context<TransferToken>, amount: u64) -> Result<()> {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct TransferToken<'info> {
    pub token_mint: Account<'info, token::Mint>,

    #[account(mut)]   
    pub buyer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = token_mint,
        associated_token::authority = buyer,
    )]
    pub from: Account<'info, token::TokenAccount>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = token_mint,
        associated_token::authority = buyer,
    )]
    pub to: Account<'info, token::TokenAccount>,

    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

