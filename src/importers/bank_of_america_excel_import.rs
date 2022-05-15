use crate::prelude::*;

pub static SKIP_LIST: [&'static str;5] = [
    "Description,,Summary Amt.",
    "Total credits",
    "Total debits",
    "Ending balance as of",
    "Beginning balance as of",
    ];

pub fn import(path: &Path) -> String {

    let mut results: String = String::new();

    let load = match file_utils::load_file(path) {
        Ok(v) => v,
        Err(why) => panic!("{}", why),
    };

    let split: Vec<&str> = load.split("\r\n").collect();

    for line in split {
        if SKIP_LIST.iter().any(|x|line.contains(x)) {
            continue;
        } else {
            results += &[line, "\r\n"].concat();
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_import_bofa() {
        let path = Path::new("./statements/stmt.csv");
        println!("{}", import(path));
    }
}