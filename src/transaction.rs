 use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub date: NaiveDate,
    pub description: String,
    pub amount: f64,
}

impl Transaction {

    pub fn new(date: NaiveDate, description: String, amount: f64) -> Self {
        Self {
            date,
            description,
            amount,
        }
    }

    fn get_cleaned_desc(&self) -> String {
        match regex_cleaner::remove_mm_slash_yy(&self.description) {
            Ok(v) => v,
            Err(why) => panic!("{}", why),
        }
    }
}

impl Hash for Transaction {
    
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_cleaned_desc().hash(state);
        self.amount.to_string().hash(state);
    }
}

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

        assert_eq!(transaction.date, parsed);
        assert_eq!(transaction.description, desc);
        assert_eq!(transaction.amount, amount);

    }
}