use std::{rc::Rc, cell::RefCell};
use rust_lisp::{parse,eval,default_env,utils::require_int_parameter,model::{Value,RuntimeError}};
use anchor_spl::token::{ self, Transfer };
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

fn token_transfer<'info>(
    prog: &AccountInfo<'info>,
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    auth: &AccountInfo<'info>,
    amt: i32,
    dcm: i32,
) -> ProgramResult {
    let cpi_accounts = Transfer {
        from: from.clone(),
        to: to.clone(),
        authority: auth.clone(),
    };
    let cpi_ctx = CpiContext::new(prog.clone(), cpi_accounts);
    let mut final_amount: u64 = amt as u64;
    let decimals = dcm as u32;
    final_amount = final_amount * 10u64.pow(decimals);
    token::transfer(cpi_ctx, final_amount)?;
    Ok(())
}

#[program]
pub mod solisp {
    use super::*;
    pub fn run<'info>(ctx: Context<'_, '_, '_, 'info, Runtime<'info>>, script: String) -> ProgramResult {
        let env = Rc::new(RefCell::new(default_env()));
        let mut accts = Vec::<AccountInfo>::new();
        accts.push(ctx.accounts.token_program.clone());
        accts.push(ctx.accounts.user.clone());
        for i in ctx.remaining_accounts {
            accts.push(i.clone());
        }
        env.borrow_mut().entries.insert(
            String::from("transfer"),
            Value::NativeFunc(|_env, args, accts| {
                let from_index = require_int_parameter("nth", args, 0)?;
                let to_index = require_int_parameter("nth", args, 1)?;
                let amt = require_int_parameter("nth", args, 2)?;
                let dcm = require_int_parameter("nth", args, 3)?;
                let from: &AccountInfo = accts.get(from_index as usize + 2).unwrap();
                let to: &AccountInfo = accts.get(to_index as usize + 2).unwrap();
                let res = token_transfer(accts.get(0).unwrap(), from, to, accts.get(1).unwrap(), amt, dcm);
                if res.is_err() {
                    return Err(RuntimeError {
                        msg: "Transfer failed".to_string(),
                    });
                }
                return Ok(Value::NIL);
            })
        );
        let mut ast_iter = parse(script.as_str());
        let first_expression = ast_iter.next().unwrap().unwrap();
        let evaluation_result = eval(env.clone(), &first_expression, &accts).unwrap();
        msg!("Atellix: Result: {}", &evaluation_result);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Runtime<'info> {
    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
    #[account(signer)]
    pub user: AccountInfo<'info>,
}
