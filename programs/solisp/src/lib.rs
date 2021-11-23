use std::{rc::Rc, cell::RefCell};
use std::convert::TryFrom;
use rust_lisp::{parse,eval,default_env,utils::{require_string_parameter, require_int_parameter},model::{Value,RuntimeError}};
use phf::phf_map;
use anchor_spl::token::{ self, accessor::amount, Transfer };
use anchor_lang::prelude::*;

declare_id!("BExCNxMXMW3nydDzuLbQjyFmdYirxeVK12buBeAVATs5");

#[derive(Clone)]
pub enum Cmd {
    TokenBalance,
    TokenTransfer,
}

static CMDS: phf::Map<&'static str, Cmd> = phf_map! {
    "token_balance" => Cmd::TokenBalance,
    "token_transfer" => Cmd::TokenTransfer,
};

fn token_balance<'info>(
    token: &AccountInfo<'info>,
) -> Result<u64, ProgramError> {
    amount(token)
}
 
fn token_transfer<'info>(
    prog: &AccountInfo<'info>,
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    auth: &AccountInfo<'info>,
    amt: u64,
    dcm: u32,
) -> ProgramResult {
    let cpi_accounts = Transfer {
        from: from.clone(),
        to: to.clone(),
        authority: auth.clone(),
    };
    let cpi_ctx = CpiContext::new(prog.clone(), cpi_accounts);
    let final_amount = amt * 10u64.pow(dcm);
    token::transfer(cpi_ctx, final_amount)?;
    Ok(())
}

#[program]
pub mod solisp {
    use super::*;
    pub fn run<'info>(ctx: Context<'_, '_, '_, 'info, Runtime>, script: String) -> ProgramResult {
        let env = Rc::new(RefCell::new(default_env()));
        let mut accts = Vec::<AccountInfo>::new();
        for i in ctx.remaining_accounts {
            accts.push(i.clone());
        }
        env.borrow_mut().entries.insert(
            String::from("cmd"),
            Value::NativeFunc(|_env, args, accts| {
                let cmdname = require_string_parameter("cmd", args, 0)?;
                let cmd = CMDS.get(cmdname).cloned();
                if cmd.is_none() {
                    return Err(RuntimeError {
                        msg: format!("Command not found: {}", cmdname),
                    });
                }
                match cmd.unwrap() {
                    Cmd::TokenBalance => {
                        let tkn_index = require_int_parameter("cmd", args, 1)?;
                        let tkn: &AccountInfo = accts.get(tkn_index as usize).unwrap();
                        let res = token_balance(tkn);
                        if res.is_err() {
                            return Err(RuntimeError { msg: format!("Command failed: {}", cmdname) })
                        }
                        return Ok(Value::Int(res.unwrap() as i128));
                    },
                    Cmd::TokenTransfer => {
                        let prg_index = require_int_parameter("cmd", args, 1)?;
                        let from_index = require_int_parameter("cmd", args, 2)?;
                        let to_index = require_int_parameter("cmd", args, 3)?;
                        let auth_index = require_int_parameter("cmd", args, 4)?;
                        let amt = require_int_parameter("cmd", args, 5)?;
                        let dcm = require_int_parameter("cmd", args, 6)?;
                        let prg: &AccountInfo = accts.get(prg_index as usize).unwrap();
                        let from: &AccountInfo = accts.get(from_index as usize).unwrap();
                        let to: &AccountInfo = accts.get(to_index as usize).unwrap();
                        let auth: &AccountInfo = accts.get(auth_index as usize).unwrap();
                        let res = token_transfer(prg, from, to, auth, u64::try_from(amt).unwrap(), u32::try_from(dcm).unwrap());
                        if res.is_err() {
                            return Err(RuntimeError { msg: format!("Command failed: {}", cmdname) })
                        }
                    },
                };

                return Ok(Value::True);
            })
        );
        let mut ast_iter = parse(script.as_str());
        let expr = ast_iter.next().unwrap().unwrap();
        let result = eval(env.clone(), &expr, &accts).unwrap();
        msg!("Atellix: Result: {}", &result);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Runtime {}
