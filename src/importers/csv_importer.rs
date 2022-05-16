use crate::prelude::*;

pub fn import(val: String) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let mut reader = Reader::from_reader(val.as_bytes());
    let mut transactions: Vec<Transaction> = Vec::new();
    
    for record in reader.records() {
        let r = record?;

        let date: NaiveDate = NaiveDate::parse_from_str(&r[0], "%m/%d/%Y")?;
        let desc: String = r[1].into();
        let amount: f64 = r[2].parse::<f64>().unwrap_or(0.0);
        transactions.push(Transaction::new(date, desc, amount));
    }

    Ok(transactions)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_import() {
        let statement = std::path::Path::new("./statements/stmt.csv");
        let imported_statement: String = bank_of_america_excel_import::import(statement);
        let results = import(imported_statement).unwrap();
        dbg!(results);
        // dbg!(results.get(0).hash());
    }

    #[test]
    pub fn test_bulk_import() {
        let mut transactions: Vec<Transaction> = Vec::new();

        let statements = std::path::Path::new("./statements");
        let statement_load = statement_loader::load_statements(statements);
        for statement in statement_load {
            let imported_statement: String = bank_of_america_excel_import::import(Path::new(&statement));
            match import(imported_statement) {
                Ok(mut v) => transactions.append(&mut v),
                Err(why) => panic!("{}", why),
            }
        }
        
        
        dbg!(transactions.len());
    }
}