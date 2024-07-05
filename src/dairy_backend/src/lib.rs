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
fn remove_cow(id: u64) -> bool {
    COWS.with(|cows| {
        let mut cows = cows.borrow_mut();
        if let Some(pos) = cows.iter().position(|cow| cow.id == id) {
            cows.remove(pos);
            true
        } else {
            false
        }
    })
}

#[update]
fn record_milk_production(id: u64, amount: u64) -> bool {
    COWS.with(|cows| {
        if let Some(cow) = cows.borrow_mut().iter_mut().find(|cow| cow.id == id) {
            cow.milk_produced += amount;
            true
        } else {
            false
        }
    })
}

#[query]
fn get_cow(id: u64) -> Option<Cow> {
    COWS.with(|cows| cows.borrow().iter().find(|&cow| cow.id == id).cloned())
}

#[query]
fn list_cows() -> Vec<Cow> {
    COWS.with(|cows| cows.borrow().clone())
}

#[query]
fn get_total_milk_produced() -> u64 {
    COWS.with(|cows| cows.borrow().iter().map(|cow| cow.milk_produced).sum())
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

    #[test]
    fn test_add_and_get_cow() {
        add_cow(1, "Bessie".to_string(), 5);
        let cow = get_cow(1).expect("Cow should be present");
        assert_eq!(cow.name, "Bessie");
        assert_eq!(cow.age, 5);
        assert_eq!(cow.milk_produced, 0);
    }

    #[test]
    fn test_record_milk_production() {
        add_cow(2, "Molly".to_string(), 4);
        let success = record_milk_production(2, 10);
        assert!(success);
        let cow = get_cow(2).expect("Cow should be present");
        assert_eq!(cow.milk_produced, 10);
    }

    #[test]
    fn test_remove_cow() {
        add_cow(3, "Daisy".to_string(), 3);
        let removed = remove_cow(3);
        assert!(removed);
        let cow = get_cow(3);
        assert!(cow.is_none());
    }

    #[test]
    fn test_list_cows() {
        add_cow(4, "Lily".to_string(), 2);
        add_cow(5, "Rose".to_string(), 1);
        let cows = list_cows();
        assert_eq!(cows.len(), 2);
    }

    #[test]
    fn test_get_total_milk_produced() {
        add_cow(6, "Bella".to_string(), 3);
        add_cow(7, "Lucy".to_string(), 2);
        record_milk_production(6, 8);
        record_milk_production(7, 6);
        let total_milk = get_total_milk_produced();
        assert_eq!(total_milk, 14);
    }
}
