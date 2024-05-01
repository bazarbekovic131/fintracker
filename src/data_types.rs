use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Date {
    day: u8,
    month: u8,
    year: u16,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Expense {
    id: u32,
    amount: f32,
    category: String,
    date: Date,
    description: Option<String>, // Optional description
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Income {
    id: u32,
    amount: f32,
    category: String,
    date: Date,
    description: Option<String>, // Optional description
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditCard {
    card_number: String,
    expiration_date: Date,
    cardholder_name: String,
    cvv: u16, // 2^8 < 999 < 2^16
    balance: f64,
    limit: f64,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Investment {
    id: u32,
    type_: String, // Types can include stocks, bonds, real estate, etc.
    amount: f64,
    date: Date,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deposit {
    id: u32,
    type_: String,
    amount: f64,
    interest: f32,
    date: Date,
    bank_name: String,
    iik: String, // IBAN of my account
    bik: String, // BIK - Bankovsky Identifikacjonny Kod
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Account {
    id: u32,
    type_: String,
    balance: f64,
    deposits: Vec<Deposit>,
    expenses: Vec<Expense>,
    incomes: Vec<Income>,
    investments: Vec<Investment>,
    credit_cards: Vec<CreditCard>,
}

#[wasm_bindgen]
pub enum ExpenseCategory {
    additional_services,
    food,
    shopping,
    technology,
    games,
    transport,
    personal_care,
    streaming,
    education,
    communication,
    clothes,
    hobbies,
    apple_services,
    healthcare_fitness,
    entertainment_culture,
    gifts,
}

enum IncomeSources {
    Work,
    Stipendium,
    Transfers,
    FromInvestition,
    SideHustle,
}

// #[wasm_bindgen]
// impl ExpenseCategory {
//     pub fn budget_limit(&self) -> f64 {
//         match self {
//             Self::additional_services => 250.0, // make this not hard-coded
//             Self::food => 250.0,
//             Self::shopping => 100.0,
//             Self::technology => 30.0,
//             Self::games => 30.0,
//             Self::transport => 0.0,
//             Self::personal_care => 0.0,
//             Self::streaming => 0.0,
//             Self::education => 0.0,
//             Self::communication => 0.0,
//             Self::clothes => 0.0,
//             Self::hobbies => 0.0,
//             Self::apple_services => 0.0,
//             Self::healthcare_fitness => 0.0,
//             Self::entertainment_culture => 0.0,
//             Self::gifts => 0.0,
//         }
//     }
//     pub fn as_str(&self) -> String {
//         match self {
//             Self::additional_services => "Zusatzdienste",
//         }
//     }
// }
//
//
impl Date {
    pub fn new(day: u8, month: u8, year: u16) -> Date {
        Date { day, month, year }
    }
}

#[wasm_bindgen]
impl Expense {
    #[wasm_bindgen(constructor)]
    pub fn new(
        id: u32,
        amount: f32,
        category: String,
        date: Date,
        description: Option<String>,
    ) -> Expense {
        Expense {
            id,
            amount,
            category,
            date,
            description,
        }
    }

    #[wasm_bindgen]
    pub fn update_amount(&mut self, new_amount: f32) {
        self.amount = new_amount;
    }
}

#[wasm_bindgen]
pub fn get_expense_as_json(expense: &Expense) -> String {
    serde_json::to_string(expense).unwrap()
}

#[wasm_bindgen]
pub fn create_expense_from_json(json_data: &str) -> Expense {
    serde_json::from_str(json_data).unwrap()
}
