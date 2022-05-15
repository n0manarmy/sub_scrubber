use crate::prelude::*;

pub fn load_statements(path: &Path) -> Vec<&Path> {
    let mut file_list: Vec<&Path> = Vec::new();
    for entry in path.read_dir().expect("Error reading directory") {
        if let Ok(entry) = entry {
            if entry.path().is_file() {
                file_list.push(&entry.path());
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
        let path: &Path = Path::new("./statements/");
        load_statements(&path);
    }
}