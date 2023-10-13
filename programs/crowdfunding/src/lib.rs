use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
declare_id!("95A2UHGSZtCFUjZk4k79qvMsM8yTJDM1w7ydVtjvTvPX");

#[program]
pub mod crowdfunding {

    use super::*;

	pub fn create(ctx: Context<Create>, name: String, description: String) -> ProgramResult {
		let compaign =&mut ctx.accounts.compaign;
		compaign.name = name;
		compaign.description = description;
		compaign.amount_donated = 0;
		compaign.admin = *ctx.accounts.user.key;
		Ok(())
	}

	pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
		let compaign = &mut ctx.accounts.compaign;
		let user =  &mut ctx.accounts.user;

		if compaign.admin != *user.key {
			return Err(ProgramError::IncorrectProgramId);
		}

		let rent_balance = Rent::get()?.minimum_balance(compaign.to_account_info().data_len());
		if **compaign.to_account_info().lamports.borrow() - rent_balance < amount {
			return Err(ProgramError::InsufficientFunds);
		}
		**compaign.to_account_info().try_borrow_mut_lamports()? -= amount;
		**user.to_account_info().try_borrow_mut_lamports()? += amount;
		Ok(())
	}
}

#[derive(Accounts)]
pub struct Create<'info> {
	#[account(init, payer=user, space = 9000, seeds=[b"COMPAIGN_DEMO".as_ref(), user.key().as_ref()], bump)]
	pub compaign: Account<'info, Compaign>,
	#[account(mut)]
	pub user: Signer<'info>,
	pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
	#[account(mut)]
	pub compaign: Account<'info, Compaign>,
	#[account(mut)]
	pub user: Signer<'info>
}

#[account]
pub struct Compaign {
	pub admin: Pubkey,
	pub name: String,
	pub description: String,
	pub amount_donated: u64,
}
