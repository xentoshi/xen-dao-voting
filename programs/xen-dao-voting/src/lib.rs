use anchor_lang::prelude::*;

declare_id!("7XcNV2hAtWFBb6y4YfSngsDyCLGes8LbFeVhUJNrxGt7");

#[program]
pub mod xen_dao_voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String) -> Result<()> {
        require!(name.len() <= 32, ErrorCode::NameTooLong);
        let dao = &mut ctx.accounts.dao;
        dao.name = name;
        dao.proposal_count = 0;
        dao.total_points = 0;
        dao.authority = ctx.accounts.user.key();
        Ok(())
    }

    pub fn create_proposal(ctx: Context<CreateProposal>, description: String) -> Result<()> {
        require!(description.len() <= 256, ErrorCode::DescriptionTooLong);
        let dao = &mut ctx.accounts.dao;
        let proposal = &mut ctx.accounts.proposal;
    
        proposal.id = dao.proposal_count;
        proposal.description = description;
        proposal.yes_votes = 0;
        proposal.no_votes = 0;
        proposal.is_active = true;
        proposal.creator = ctx.accounts.user.key();
        proposal.voters = Vec::new();
    
        dao.proposal_count = dao.proposal_count.checked_add(1).ok_or(ErrorCode::ProposalLimitReached)?;
    
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, vote: bool) -> Result<()> {
        let dao = &mut ctx.accounts.dao;
        let proposal = &mut ctx.accounts.proposal;
        let user = &ctx.accounts.user;
    
        require!(proposal.is_active, ErrorCode::ProposalNotActive);
        require!(!proposal.voters.contains(&user.key()), ErrorCode::AlreadyVoted);
    
        if vote {
            proposal.yes_votes += 1;
        } else {
            proposal.no_votes += 1;
        }
    
        proposal.voters.push(user.key());
        dao.total_points += 1;
    
        Ok(())
    }

    pub fn close_proposal(ctx: Context<CloseProposal>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        require!(proposal.is_active, ErrorCode::ProposalAlreadyClosed);
        proposal.is_active = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 4 + 32 + 1 + 8 + 32,
        seeds = [b"dao"],
        bump
    )]
    pub dao: Account<'info, Dao>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut, has_one = authority @ ErrorCode::Unauthorized)]
    pub dao: Account<'info, Dao>,
    #[account(
        init,
        payer = user,
        space = 8 + 1 + 4 + 256 + 8 + 8 + 1 + 32 + 4 + 32 * 10,
        seeds = [b"proposal", dao.key().as_ref(), &[dao.proposal_count]],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(constraint = authority.key() == user.key() @ ErrorCode::Unauthorized)]
    pub authority: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    #[account(mut, constraint = proposal.id < dao.proposal_count @ ErrorCode::Unauthorized)]
    pub proposal: Account<'info, Proposal>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseProposal<'info> {
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    #[account(
        mut,
        seeds = [b"proposal", dao.key().as_ref(), &[proposal.id]],
        bump,
        close = user
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct Dao {
    pub name: String,
    pub proposal_count: u8,
    pub total_points: u64,
    pub authority: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Proposal {
    pub id: u8,
    pub description: String,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub is_active: bool,
    pub creator: Pubkey,
    pub voters: Vec<Pubkey>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Proposal is not active")]
    ProposalNotActive,
    #[msg("Proposal is already closed")]
    ProposalAlreadyClosed,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("DAO name is too long (max 32 characters)")]
    NameTooLong,
    #[msg("Proposal description is too long (max 256 characters)")]
    DescriptionTooLong,
    #[msg("Proposal limit reached")]
    ProposalLimitReached,
    #[msg("User has already voted on this proposal")]
    AlreadyVoted,
}
