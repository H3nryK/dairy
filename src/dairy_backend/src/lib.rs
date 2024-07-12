#[macro_use]
extern crate serde;
use candid::{CandidType, Decode, Deserialize, Encode};
use ic_cdk::api::time;
use ic_cdk_macros::*;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Define the memory type
type Memory = VirtualMemory<DefaultMemoryImpl>;

// Define a structure for Cow
#[derive(CandidType, Serialize, Deserialize, Clone, Default)]
struct Cow {
    id: u64,
    name: String,
    age: u64,
    milk_produced: u64,
    created_at: u64,
}

impl Cow {
    // Constructor for Cow
    fn new(id: u64, name: String, age: u64) -> Self {
        Self {
            id,
            name,
            age,
            milk_produced: 0,
            created_at: time(),
        }
    }

    // Update function for Cow
    fn update(&mut self, name: String, age: u64) {
        self.name = name;
        self.age = age;
    }

    // Function to record milk production
    fn record_milk_production(&mut self, amount: u64) {
        self.milk_produced += amount;
    }
}

// Implement Storable for Cow
impl Storable for Cow {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// Implement BoundedStorable for Cow
impl BoundedStorable for Cow {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define thread-local storage and memory management
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static COWS_STORAGE: RefCell<StableBTreeMap<u64, Cow, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))))
    );

    static ID_COUNTER: RefCell<Cell<u64, Memory>> = RefCell::new(
        Cell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), 0)
            .expect("Cannot create a counter")
    );
}

// Payloads for Cow and MilkProduction
#[derive(CandidType, Deserialize, Serialize)]
struct CowPayload {
    name: String,
    age: u64,
}

#[derive(CandidType, Deserialize, Serialize)]
struct MilkProductionPayload {
    id: u64,
    amount: u64,
}

// Function to add a new cow
#[update]
fn add_cow(payload: CowPayload) -> Result<Cow, String> {
    if payload.name.is_empty() {
        return Err("Name is required".to_string());
    }

    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter
            .borrow_mut()
            .set(current_value + 1)
            .expect("Failed to increment ID counter");
        current_value
    });

    let cow = Cow::new(id, payload.name, payload.age);
    COWS_STORAGE.with(|storage| storage.borrow_mut().insert(cow.id, cow.clone()));

    Ok(cow)
}

// Function to update an existing cow
#[update]
fn update_cow(id: u64, payload: CowPayload) -> Result<Cow, String> {
    COWS_STORAGE.with(|storage| {
        if let Some(mut cow) = storage.borrow_mut().get_mut(&id) {
            cow.update(payload.name, payload.age);
            Ok(cow.clone())
        } else {
            Err("Cow not found".to_string())
        }
    })
}

// Function to delete an existing cow
#[update]
fn delete_cow(id: u64) -> Result<(), String> {
    COWS_STORAGE.with(|storage| {
        if storage.borrow().contains_key(&id) {
            storage.borrow_mut().remove(&id);
            Ok(())
        } else {
            Err("Cow not found".to_string())
        }
    })
}

// Function to record milk production for a cow
#[update]
fn record_milk_production(payload: MilkProductionPayload) -> Result<Cow, String> {
    COWS_STORAGE.with(|storage| {
        if let Some(mut cow) = storage.borrow_mut().get_mut(&payload.id) {
            cow.record_milk_production(payload.amount);
            Ok(cow.clone())
        } else {
            Err("Cow not found".to_string())
        }
    })
}

// Query function to get a cow by ID
#[query]
fn get_cow(id: u64) -> Option<Cow> {
    COWS_STORAGE.with(|storage| storage.borrow().get(&id).cloned())
}

// Query function to get all cows
#[query]
fn get_all_cows() -> Vec<Cow> {
    COWS_STORAGE.with(|storage| storage.borrow().iter().map(|(_, cow)| cow.clone()).collect())
}

// This block generates the Candid interface
candid::export_service!();

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    // Test to generate the Candid interface file
    #[test]
    fn generate_did_file() {
        let did = __export_service();
        let mut file = File::create("dairy_backend.did").expect("Could not create file");
        file.write_all(did.as_bytes()).expect("Could not write to file");
    }
}
