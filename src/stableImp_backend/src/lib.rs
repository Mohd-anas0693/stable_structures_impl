mod memory;
mod state;
mod types;
use ic_cdk::{api::caller, export_candid, query, update};
use memory::Memory;
use state::State;
use std::cell::RefCell;
use types::PostData;

thread_local! {
    /// Initialize the state randomness with the current time.
    static STATE: RefCell<State> = RefCell::new(State::new());
}

// let mem_mgr = MemoryManager::init(DefaultMemoryImpl::default());
#[update(name = "create_post")]
fn insert(post_data: PostData) -> Result<(), String> {
    let user_principal = caller();
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.set_post_data(user_principal, post_data)
    })
}
#[query(name = "get_post")]
fn fetch() -> Result<PostData, String> {
    let user_principal = caller();
    STATE.with(|state| {
        let state = state.borrow();
        state.get_post_data(user_principal)
    })
}
#[update(name = "remove_post")]
fn remove() -> Result<PostData, String> {
    let user_principal = caller();
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.remove_user_post(user_principal)
    })
}

export_candid!();
