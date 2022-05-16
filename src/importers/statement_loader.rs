use crate::prelude::*;

pub fn load_statements(path: &Path) -> Vec<String> {
    let mut file_list: Vec<String> = Vec::new();
    for entry in path.read_dir().expect("Error reading directory") {
        if let Ok(entry) = entry {
            if entry.path().is_file() {
                file_list.push(String::from(entry.path().to_str().unwrap()));
            }
        }
    }
    
    file_list
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_load_statements() {
        let path_str = "./statements/";
        let path: &Path = Path::new(path_str);
        dbg!(load_statements(&path));
    }
}