use candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
use std::cell::RefCell;

#[derive(CandidType, Deserialize, Clone)]
struct Cow {
    id: u64,
    name: String,
    age: u64,
    milk_produced: u64,
    health_status: String,
}

thread_local! {
    static COWS: RefCell<Vec<Cow>> = RefCell::new(Vec::new());
    static NEXT_ID: RefCell<u64> = RefCell::new(1);
}

#[init]
fn init() {
    ic_cdk::println!("Dairy Backend initialized");
}

#[update]
fn add_cow(name: String, age: u64) -> Cow {
    let id = NEXT_ID.with(|next_id| {
        let current = *next_id.borrow();
        *next_id.borrow_mut() += 1;
        current
    });
    let cow = Cow { id, name, age, milk_produced: 0, health_status: "Healthy".to_string() };
    COWS.with(|cows| cows.borrow_mut().push(cow.clone()));
    cow
}

#[update]
fn update_cow(id: u64, name: Option<String>, age: Option<u64>) -> Option<Cow> {
    COWS.with(|cows| {
        let mut cows = cows.borrow_mut();
        if let Some(cow) = cows.iter_mut().find(|cow| cow.id == id) {
            if let Some(new_name) = name {
                cow.name = new_name;
            }
            if let Some(new_age) = age {
                cow.age = new_age;
            }
            Some(cow.clone())
        } else {
            None
        }
    })
}

#[update]
fn delete_cow(id: u64) -> bool {
    COWS.with(|cows| {
        let mut cows = cows.borrow_mut();
        if let Some(index) = cows.iter().position(|cow| cow.id == id) {
            cows.remove(index);
            true
        } else {
            false
        }
    })
}

#[update]
fn record_milk_production(id: u64, amount: u64) -> Option<u64> {
    COWS.with(|cows| {
        if let Some(cow) = cows.borrow_mut().iter_mut().find(|cow| cow.id == id) {
            cow.milk_produced += amount;
            Some(cow.milk_produced)
        } else {
            None
        }
    })
}

#[query]
fn get_cow(id: u64) -> Option<Cow> {
    COWS.with(|cows| cows.borrow().iter().find(|&cow| cow.id == id).cloned())
}

#[query]
fn get_all_cows() -> Vec<Cow> {
    COWS.with(|cows| cows.borrow().clone())
}

#[update]
fn update_health_status(id: u64, status: String) -> Option<Cow> {
    COWS.with(|cows| {
        let mut cows = cows.borrow_mut();
        if let Some(cow) = cows.iter_mut().find(|cow| cow.id == id) {
            cow.health_status = status;
            Some(cow.clone())
        } else {
            None
        }
    })
}

#[query]
fn get_total_milk_production() -> u64 {
    COWS.with(|cows| cows.borrow().iter().map(|cow| cow.milk_produced).sum())
}

// This is needed for candid interface generation
ic_cdk::export_candid!();