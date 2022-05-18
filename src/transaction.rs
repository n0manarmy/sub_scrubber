use serde::{Serialize, Deserialize};

use crate::prelude::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub uuid: String,
    pub date: String,
    pub desc: String,
    pub amount: f64,
    pub hash: u64,
}

impl Transaction {
    pub fn new(date: NaiveDate, description: String, amount: f64) -> Self {
        let mut trans = Self {
            uuid: Uuid::new_v4().to_string(),
            date: date.to_string(),
            desc: description.clone(),
            amount,
            hash: transaction::calculate_hash(&description, &amount.to_string()), // hash: "".into(),
        };
        // trans.hash = transaction::calculate_hash(&trans);
        // trans.hash = encode([trans.get_cleaned_desc(), trans.amount.to_string()].concat());

        trans
    }
}

// impl Serialize for Transaction {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut state = serializer.serialize_struct("Transaction", 5)?;
//         state.serialize_field("uuid", &self.uuid)?;
//         state.serialize_field("date", &self.date)?;
//         state.serialize_field("desc", &self.desc)?;
//         state.serialize_field("amount", &self.amount)?;
//         state.serialize_field("hash", &self.hash)?;
//         state.end()
//     }
// }

fn get_cleaned_desc(desc: String) -> String {
    match regex_cleaner::remove_mm_slash_yy(&desc) {
        Ok(v) => v,
        Err(why) => panic!("{}", why),
    }
}

pub fn calculate_hash<S: Hash, T: Hash>(s: &S, t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    t.hash(&mut hasher);
    hasher.finish()
}

// impl Hash for Transaction {

//     fn hash<H: Hasher>(&self, state: &mut H) {
//         transaction::get_cleaned_desc(self.description).hash(state);
//         self.amount.to_string().hash(state);
//     }
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_new_transaction() {
        let date: String = "05/15/2022".into();
        let parsed = NaiveDate::parse_from_str(&date, "%m/%d/%Y").unwrap();
        let desc: String = "test description".into();
        let amount: f64 = 435.25;
        let transaction = Transaction::new(parsed, desc.clone(), amount);

        assert_eq!(transaction.date.to_string(), parsed.to_string());
        assert_eq!(transaction.desc, desc);
        assert_eq!(transaction.amount, amount);
    }
}
