 use crate::prelude::*;

#[derive(Debug)]
pub struct Transaction {
    pub date: Date<Local>,
    pub description: String,
    pub amount: f64,
}

impl Transaction {

    pub fn new(date: Date<Local>, description: String, amount: f64) -> Self {
        Self {
            date,
            description,
            amount,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_new_transaction() {
        let date: Date<Local> = Local::now();
        let desc: String = "test description".into();
        let amount: f64 = 435.25;
        let transaction = Transaction::new(date, desc.clone(), amount);

        assert_eq!(transaction.date, date);
        assert_eq!(transaction.description, desc);
        assert_eq!(transaction.amount, amount);

    }
}