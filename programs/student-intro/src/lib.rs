use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use mpl_token_metadata::instruction::create_metadata_accounts_v2;
use mpl_token_metadata::ID as MetadataID;

declare_id!("G5yKMSQaQVHo6LSJQ9fqXHDLZ2ckXCZSXmgy3DgtEx2p");

#[program]
pub mod student_intro {
    use super::*;

    pub fn add_student_intro(
        ctx: Context<AddStudentIntro>,
        name: String,
        message: String,
    ) -> Result<()> {
        msg!("Student Intro Account Created");
        msg!("Name: {}", name);
        msg!("Message: {}", message);

        let student_intro = &mut ctx.accounts.student_intro;
        student_intro.student = ctx.accounts.student.key();
        student_intro.name = name;
        student_intro.message = message;

        msg!("Movie Comment Counter Account Created");
        let reply_counter = &mut ctx.accounts.reply_counter;
        reply_counter.counter = 0;
        msg!("Counter: {}", reply_counter.counter);

        let seeds = &["mint".as_bytes(), &[*ctx.bumps.get("reward_mint").unwrap()]];

        let signer = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.reward_mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.reward_mint.to_account_info(),
            },
            &signer,
        );

        token::mint_to(cpi_ctx, 10000000)?;
        msg!("Minted Tokens");

        Ok(())
    }

    pub fn update_student_intro(
        ctx: Context<UpdateStudentIntro>,
        name: String,
        message: String,
    ) -> Result<()> {
        msg!("Updating Student Intro Account");
        msg!("Name: {}", name);
        msg!("Message: {}", message);

        let student_intro = &mut ctx.accounts.student_intro;
        student_intro.student = ctx.accounts.student.key();
        student_intro.name = name;
        student_intro.message = message;

        Ok(())
    }

    pub fn add_reply(ctx: Context<AddReply>, reply: String) -> Result<()> {
        msg!("Reply Account Created");
        msg!("Reply: {}", reply);

        let reply_account = &mut ctx.accounts.reply_account;
        let reply_counter = &mut ctx.accounts.reply_counter;

        reply_account.studentinfo = ctx.accounts.student.key();
        reply_account.reply = reply;

        reply_counter.counter += 1;

        let seeds = &["mint".as_bytes(), &[*ctx.bumps.get("reward_mint").unwrap()]];

        let signer = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.reward_mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.reward_mint.to_account_info(),
            },
            &signer,
        );

        token::mint_to(cpi_ctx, 5000000)?;
        msg!("Minted Tokens");

        Ok(())
    }

    pub fn close(_ctx: Context<Close>) -> Result<()> {
        Ok(())
    }

    pub fn create_reward_mint(
        ctx: Context<CreateTokenReward>,
        uri: String,
        name: String,
        symbol: String,
    ) -> Result<()> {
        msg!("Create Reward Token");

        let seeds = &["mint".as_bytes(), &[*ctx.bumps.get("reward_mint").unwrap()]];

        let signer = [&seeds[..]];

        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.reward_mint.to_account_info(),
            ctx.accounts.reward_mint.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        invoke_signed(
            &create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.reward_mint.key(),
                ctx.accounts.reward_mint.key(),
                ctx.accounts.user.key(),
                ctx.accounts.user.key(),
                name,
                symbol,
                uri,
                None,
                0,
                true,
                true,
                None,
                None,
            ),
            account_info.as_slice(),
            &signer,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name:String, message:String)]
pub struct AddStudentIntro<'info> {
    #[account(
        init,
        seeds = [student.key().as_ref()],
        bump,
        payer = student,
        space = 8 + 32 + 4 + name.len() + 4 + message.len()
    )]
    pub student_intro: Account<'info, StudentInfo>,
    #[account(
        init,
        seeds = ["counter".as_bytes(), student_intro.key().as_ref()],
        bump,
        payer = student,
        space = 8 + 8
    )]
    pub reply_counter: Account<'info, ReplyCounter>,
    #[account(mut,
        seeds = ["mint".as_bytes().as_ref()],
        bump
    )]
    pub reward_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = student,
        associated_token::mint = reward_mint,
        associated_token::authority = student
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub student: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name:String, message:String)]
pub struct UpdateStudentIntro<'info> {
    #[account(
        mut,
        seeds = [student.key().as_ref()],
        bump,
        realloc = 8 + 32 + 4 + name.len() + 4 + message.len(),
        realloc::payer = student,
        realloc::zero = false,
    )]
    pub student_intro: Account<'info, StudentInfo>,
    #[account(mut)]
    pub student: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(reply:String)]
pub struct AddReply<'info> {
    #[account(
        init,
        seeds = [student_intro.key().as_ref(), &reply_counter.counter.to_le_bytes()],
        bump,
        payer = student,
        space = 8 + 32 + 4 + reply.len()
    )]
    pub reply_account: Account<'info, Reply>,
    #[account(
        seeds = [student.key().as_ref()],
        bump,
    )]
    pub student_intro: Account<'info, StudentInfo>,
    #[account(
        seeds = ["counter".as_bytes(), student_intro.key().as_ref()],
        bump,
    )]
    pub reply_counter: Account<'info, ReplyCounter>,
    #[account(mut,
        seeds = ["mint".as_bytes().as_ref()],
        bump
    )]
    pub reward_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = student,
        associated_token::mint = reward_mint,
        associated_token::authority = student
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub student: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, close = student, has_one = student, seeds = [student.key().as_ref()],
        bump,)]
    student_intro: Account<'info, StudentInfo>,
    #[account(mut)]
    student: Signer<'info>,
}

#[derive(Accounts)]
pub struct CreateTokenReward<'info> {
    #[account(
        init,
        seeds = ["mint".as_bytes().as_ref()],
        bump,
        payer = user,
        mint::decimals = 6,
        mint::authority = reward_mint,

    )]
    pub reward_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,

    /// CHECK:
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    pub token_metadata_program: Program<'info, Metadata>,
}

#[account]
pub struct StudentInfo {
    pub student: Pubkey, // 32
    pub name: String,    // 4 + len()
    pub message: String, // 4 + len()
}

#[account]
pub struct ReplyCounter {
    pub counter: u8,
}

#[account]
pub struct Reply {
    pub studentinfo: Pubkey,
    pub reply: String,
}

#[derive(Clone)]
pub struct Metadata;
impl anchor_lang::Id for Metadata {
    fn id() -> Pubkey {
        MetadataID
    }
}
