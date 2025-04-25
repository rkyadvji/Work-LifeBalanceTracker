#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short, contracttype, String};

#[contracttype]
#[derive(Clone)]
pub struct Tracker {
    pub total_work: u64,
    pub total_life: u64,
}

const BALANCE_KEY: Symbol = symbol_short!("BALANCE");

#[contract]
pub struct WorkLifeContract;

#[contractimpl]
impl WorkLifeContract {
    pub fn log_work(env: Env, hours: u64) {
        let mut tracker = Self::get_tracker(&env);
        tracker.total_work += hours;
        env.storage().instance().set(&BALANCE_KEY, &tracker);
    }

    pub fn log_life(env: Env, hours: u64) {
        let mut tracker = Self::get_tracker(&env);
        tracker.total_life += hours;
        env.storage().instance().set(&BALANCE_KEY, &tracker);
    }

    pub fn get_balance(env: Env) -> Tracker {
        Self::get_tracker(&env)
    }

    fn get_tracker(env: &Env) -> Tracker {
        env.storage().instance().get(&BALANCE_KEY).unwrap_or(Tracker {
            total_work: 0,
            total_life: 0,
        })
    }
}
