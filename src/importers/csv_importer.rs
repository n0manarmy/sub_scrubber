use crate::{prelude::*, transaction::Transaction};

pub fn import(path: &Path) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let mut reader = Reader::from_path(&path)?;
    let mut transactions: Vec<Transaction> = Vec::new();
    for record in reader.records() {
        let r = record?;
        let date: DateTime<Local> = Local::from(r[0]).into().unwrap();
        transactions.push(Transaction::new( r[0].parse::<DateTime<Local>>().unwrap(), r[1].into(), r[2].parse::<f64>().unwrap()))
    }

    Ok(transactions)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_import() {
        let statement = std::path::Path::new("./statements/test.csv");
        
        let results = import(statement).unwrap();
        dbg!(results);
    }
}