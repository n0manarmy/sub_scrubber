use crate::prelude::*;

pub fn find_duplicates(transactions: &Vec<Transaction>) -> HashMap<u64, (Transaction, usize)>{

    let mut collected: HashMap<u64, (Transaction, usize)> = HashMap::new();

    for t in transactions {
        collected.insert(t.hash.clone(), (t.clone(), 0));
    }

    dbg!("collected size: {}", collected.len());
    dbg!("transactions size: {}", transactions.len());

    for t in transactions {
        if collected.contains_key(&t.hash) {
            collected.get_mut(&t.hash).unwrap().1 += 1;
        }
    }

    // while transaction_pos < transactions.len() {
    //     let transaction = &transactions[transaction_pos];
    //     for t in transactions {
    //         if (t.uuid != transaction.uuid) && (t.hash == transaction.hash) {
    //             found.entry(transaction.hash.clone()).or_insert((transaction.clone(), 0)).1 += 1;
    //         }
    //     }
    //     transaction_pos += 1;
    // }

    // let dup = transactions.clone();
    // for d in dup {
    //     for t in transactions {
    //         if (t.uuid != d.uuid) && (t.hash == d.hash) {
    //             // found.get_mut(&t.hash).unwrap().1 += 1;
    //             found.entry(d.hash).or_insert((d.clone(), 0)).1 += 1;
    //             // if found.contains_key(&t.hash) {
    //             //     let mut f = *found.get_mut(&t.hash).unwrap();
    //             //     f.1 += 1;
    //             // }
    //             // dbg!(&t);
    //             // dbg!(&d);
    //         }
    //     }
    // }

    collected
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_find_duplicates() {
        let mut ledger: Ledger = Ledger::new();

        let statements = std::path::Path::new("./statements");
        dbg!("reading statement path");
        let statement_load = statement_loader::load_statements(statements);
        dbg!("loading statements");
        for statement in statement_load {
            let imported_statement: String = bank_of_america_excel_import::import(Path::new(&statement));
            match csv_importer::import(imported_statement) {
                Ok(mut v) => ledger.transactions.append(&mut v),
                Err(why) => panic!("{}", why),
            }
        }
        dbg!("statement import complete");

        let found = find_duplicates(&ledger.transactions);

        for (_, values) in found {
            if values.1 > 5 {
                println!("Amount: {}, Count {:.0}, Desc {}", values.0.amount, values.1, values.0.desc);
            }
        }
    }

    #[test]
    pub fn test_find_saved_ledger_duplicates() {
        let saved_ledger = "./test_files/collected.json";
        dbg!("reading statement path");
        let  ledger: Ledger = file_utils::read_transactions_from_file_serde(saved_ledger).expect("Error loading saved ledger");
        dbg!("loading statements");
        let found: HashMap<u64, (Transaction, usize)> = find_duplicates(&ledger.transactions);

        for (_, values) in found {
            if values.1 > 3 && values.0.amount < 0.0 {
                println!("Amount: ${}, Count {:.0}, Desc {}", values.0.amount, values.1, values.0.desc);
            }
        }
    }
}