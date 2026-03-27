#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Vec, Address};

// Charity structure
#[contracttype]
#[derive(Clone)]
pub struct Charity {
    pub id: u32,
    pub name: Symbol,
    pub votes: u32,
}

// Storage keys
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    CharityList,
    Voted(Address),
}

// ✅ Add #[contract] here — required in Soroban SDK v0.9+
#[contract]
pub struct CharityVotingContract;

// Contract implementation
#[contractimpl]
impl CharityVotingContract {

    pub fn init(env: Env, names: Vec<Symbol>) {
        let mut charities: Vec<Charity> = Vec::new(&env);
        for (i, name) in names.iter().enumerate() {
            charities.push_back(Charity {
                id: i as u32,
                name,
                votes: 0,
            });
        }
        env.storage().instance().set(&DataKey::CharityList, &charities);
    }

    pub fn vote(env: Env, voter: Address, charity_id: u32) {
        voter.require_auth();

        if env.storage().instance().has(&DataKey::Voted(voter.clone())) {
            panic!("Already voted!");
        }

        let mut charities: Vec<Charity> = env
            .storage()
            .instance()
            .get(&DataKey::CharityList)
            .unwrap();

        let mut charity = charities.get(charity_id).unwrap();
        charity.votes = charity.votes + 1;
        charities.set(charity_id, charity);

        env.storage().instance().set(&DataKey::CharityList, &charities);
        env.storage().instance().set(&DataKey::Voted(voter), &true);
    }

    pub fn get_charities(env: Env) -> Vec<Charity> {
        env.storage()
            .instance()
            .get(&DataKey::CharityList)
            .unwrap()
    }
}