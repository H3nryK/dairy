use candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
use std::cell::RefCell;

#[derive(CandidType, Deserialize, Clone)]
struct Cow {
    id: u64,
    name: String,
    age: u64,
    milk_produced: u64,
}

thread_local! {
    static COWS: RefCell<Vec<Cow>> = RefCell::new(Vec::new());
}

#[update]
fn add_cow(id: u64, name: String, age: u64) {
    let cow = Cow { id, name, age, milk_produced: 0 };
    COWS.with(|cows| cows.borrow_mut().push(cow));
}

#[update]
fn record_milk_production(id: u64, amount: u64) {
    COWS.with(|cows| {
        if let Some(cow) = cows.borrow_mut().iter_mut().find(|cow| cow.id == id) {
            cow.milk_produced += amount;
        }
    });
}

#[query]
fn get_cow(id: u64) -> Option<Cow> {
    COWS.with(|cows| cows.borrow().iter().find(|&cow| cow.id == id).cloned())
}

// This block generates the Candid interface
candid::export_service!();

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn generate_did_file() {
        let did = __export_service();
        let mut file = File::create("dairy_backend.did").expect("Could not create file");
        file.write_all(did.as_bytes()).expect("Could not write to file");
    }
}
