#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, String, Symbol, symbol_short, log};

// Define the Badge structure
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Badge {
    pub id: u64,
    pub name: String,
    pub recipient: Symbol, // Simplified representation of a user
    pub date_issued: u64,
}

// Storage keys
const BADGE_COUNT: Symbol = symbol_short!("B_COUNT");

#[contract]
pub struct BadgeSystem;

#[contractimpl]
impl BadgeSystem {

    // 1. Function to mint a new badge (Admin Action)
    pub fn mint_badge(env: Env, name: String, recipient: Symbol) -> u64 {
        let mut count: u64 = env.storage().instance().get(&BADGE_COUNT).unwrap_or(0);
        count += 1;

        let new_badge = Badge {
            id: count,
            name,
            recipient: recipient.clone(),
            date_issued: env.ledger().timestamp(),
        };

        // Store the badge using its ID as part of the key
        env.storage().instance().set(&count, &new_badge);
        
        // Update the total badge count
        env.storage().instance().set(&BADGE_COUNT, &count);
        
        log!(&env, "Badge minted! ID: {}, To: {}", count, recipient);
        count
    }

    // 2. Function to view a specific badge by ID
    pub fn view_badge(env: Env, badge_id: u64) -> Badge {
        env.storage().instance().get(&badge_id).expect("Badge does not exist")
    }

    // 3. Function to get the total number of badges issued
    pub fn total_badges(env: Env) -> u64 {
        env.storage().instance().get(&BADGE_COUNT).unwrap_or(0)
    }
}