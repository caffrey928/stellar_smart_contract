#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementorContract;

#[contractimpl]
impl IncrementorContract {
    /// Increment an internal counter; return the new value.
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().persistent().get(&COUNTER).unwrap_or(0);

        count += 1;

        log!(&env, "count: {}", count);

        env.storage().persistent().set(&COUNTER, &count);

        count
    }

    pub fn remove(env: Env) -> u32{
        let count: u32 = env.storage().persistent().get(&COUNTER).unwrap_or(0);
        env.storage().persistent().remove(&COUNTER);
        
        count
    }
}

#[cfg(test)]
mod test;