use crate::prelude::*;

pub fn find_duplicates(transactions: &Vec<Transaction>) -> HashMap<u64, (Transaction, usize)>{

    let mut found: HashMap<u64, (Transaction, usize)> = HashMap::new();

    let dup = transactions.clone();
    for d in dup {
        for t in transactions {
            if (t.uuid != d.uuid) && (t.hash == d.hash) {
                // found.get_mut(&t.hash).unwrap().1 += 1;
                found.entry(d.hash).or_insert((d.clone(), 0)).1 += 1;
                // if found.contains_key(&t.hash) {
                //     let mut f = *found.get_mut(&t.hash).unwrap();
                //     f.1 += 1;
                // }
                // dbg!(&t);
                // dbg!(&d);
            }
        }
    }

    found
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_find_duplicates() {
        let mut transactions: Vec<Transaction> = Vec::new();

        let statements = std::path::Path::new("./statements");
        dbg!("reading statement path");
        let statement_load = statement_loader::load_statements(statements);
        dbg!("loading statements");
        for statement in statement_load {
            let imported_statement: String = bank_of_america_excel_import::import(Path::new(&statement));
            match csv_importer::import(imported_statement) {
                Ok(mut v) => transactions.append(&mut v),
                Err(why) => panic!("{}", why),
            }
        }
        dbg!("statement import complete");

        let found = find_duplicates(&transactions);

        for (_, values) in found {
            if values.1 > 6 {
                println!("Amount: {}, Count {}, Desc {}", values.0.amount, values.1, values.0.description);
            }
        }
    }
}