use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod calculator_app {
    use super::*;
    pub fn create(ctx:Context<Create>, init_message: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }
    // Imperative approach
    pub fn exec(ctx: Context<Operation>, operation: String, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        match &*operation.to_lowercase() {
            "add" => {
                calculator.result = num1 + num2;
            },
            "minus" => {
                calculator.result = num1 - num2;
            },
            "multiply" => {
                calculator.result = num1 * num2;
            },
            "divide" => {
                calculator.result = num1 / num2;
                calculator.remainder = num1 % num2;
            },
            _ => println!("Well, supported commands are 'add', 'minus', 'multiply' or 'divide'"),
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 64 + 64 + 64 + 64)]
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Operation<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,

}

#[account]
pub struct Calculator {
    pub greeting: String,
    pub result: i64,
    pub remainder: i64,
}
