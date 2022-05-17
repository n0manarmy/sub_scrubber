use crate::prelude::*;

pub fn remove_mm_slash_yy(val: &str) -> Result<String, Box<dyn Error>> {
    let re = Regex::new("[0-9]{2}/[0-9]{2} ")?;
    let result = re.replace(&val, "").to_string();
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    pub fn test_remove_mm_slash_yy() {
        let test = "AMZ*The Washington P 04/10 PURCHASE help@washpost DC";
        let result = "AMZ*The Washington P PURCHASE help@washpost DC".to_string();
        
        assert_eq!(&result, &remove_mm_slash_yy(test.into()).unwrap());
        
        dbg!(transaction::calculate_hash(&result));
    }
}