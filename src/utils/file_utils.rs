use crate::prelude::*;

/// # Usage
/// takes a string line and the path to a log file and creates a new
/// file if one does not exist, it appends if the file does exist. If we are 
/// successful, we write to the file.
pub fn write_to_log_file(line: &str, path: &str) {
    match std::fs::OpenOptions::new().read(true).create(true).append(true).open(path) {
        Ok(mut s) => s.write_all([line, "\n"].concat().as_bytes()).expect("Error writing to file"), 
        Err(why) => panic!("{}", why),
    }
}

/// # Usage
/// print_logs takes a path input and attempts to open the file. We use a 
/// match statement and set file with the results from Ok(). We panic if
/// there is a problem opening the file, dumping the reason for the panic
/// to the screen.
/// 
/// With the file value set, we them read it to a string and then output
/// to the screen, panicing and printing why if we fail.
pub fn print_logs(path: &std::path::Path) {
    let mut buffer = String::new();

    let mut file = match std::fs::OpenOptions::new().read(true).open(path) {
        Ok(file) => file,
        Err(why) => panic!("{}", why),
    };
    
    match file.read_to_string(&mut buffer) {
        Ok(_s) => println!("{}", buffer),
        Err(why) => panic!("{}", why),
    }
}

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

pub fn write_transactions_to_file_serde(transactions: Vec<Transaction>, outfile: String) -> std::io::Result<()> {
    let mut file = File::create(outfile)?;
    for t in transactions {
        file.write_all(serde_json::to_string(&t).unwrap().as_bytes())?;
    }

    Ok(())
}

// check our log if it exists
pub fn file_exists(path: &str) -> bool {
    Path::new(&path).exists()
}

pub fn read_file_to_vec(path: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file: File = File::open(&path)?;
    let reader: std::io::BufReader<File> = std::io::BufReader::new(file);
    let times: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        
    Ok(times)
}

pub fn load_file(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let mut results: String = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut results)?;

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_load_log_file() {
        match load_file(Path::new("./test_files/test.csv")) {
            Ok(v) => println!("{}", v),
            Err(why) => panic!("{}", why),
        }
    }

    #[test]
    pub fn test_serialize_transactions_to_file() {
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

        match write_transactions_to_file_serde(transactions, "./test_files/collected.json".to_string()) {
            Ok(_) => (),
            Err(why) => panic!("{}", why),
        }
    }
}