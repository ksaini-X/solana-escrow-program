pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CiFLJqSVHH9ifnkXKwPzLURa3fFFbG8BeM3DzDM42qUN");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Initialize>) -> Result<()> {
        make::handler(ctx)
    }
}
