use anchor_lang::prelude::*;

declare_id!("CbJ1TUYAq2S71VXxFmSXKct4qRFXK2uuwVempV2RFxYB");

#[program]
pub mod student_records {
    use super::*;
    pub fn add_record(ctx: Context<AddRecord>, hash_data: String) -> Result<()> {
        let record = &mut ctx.accounts.record;
        record.hash_data = hash_data;
        Ok(())
    }

    pub fn verify_record(ctx: Context<VerifyRecord>, hash_data: String) -> Result<bool> {
        let record = &ctx.accounts.record;
        if record.hash_data == hash_data {
            msg!("Hash verified successfully.");
            Ok(true)
        } else {
            msg!("Hash verification failed.");
            Ok(false)
        }
    }
}

#[derive(Accounts)]
pub struct AddRecord<'info> {
    #[account(init, payer = user, space = 64 + 8)]
    pub record: Account<'info, Record>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyRecord<'info> {
    pub record: Account<'info, Record>,
}

#[account]
pub struct Record {
    pub hash_data: String,
}