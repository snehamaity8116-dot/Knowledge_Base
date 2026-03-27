#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, String, Map};

#[contract]
pub struct KnowledgeBase;

#[contractimpl]
impl KnowledgeBase {

    // Add or update an article
    pub fn set_article(env: Env, key: Symbol, content: String) {
        let mut kb: Map<Symbol, String> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "KB"))
            .unwrap_or(Map::new(&env));

        kb.set(key.clone(), content);
        env.storage().instance().set(&Symbol::new(&env, "KB"), &kb);
    }

    // Retrieve an article
    pub fn get_article(env: Env, key: Symbol) -> Option<String> {
        let kb: Map<Symbol, String> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "KB"))
            .unwrap_or(Map::new(&env));

        kb.get(key)
    }

    // Delete an article
    pub fn delete_article(env: Env, key: Symbol) {
        let mut kb: Map<Symbol, String> = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "KB"))
            .unwrap_or(Map::new(&env));

        kb.remove(key);
        env.storage().instance().set(&Symbol::new(&env, "KB"), &kb);
    }
}