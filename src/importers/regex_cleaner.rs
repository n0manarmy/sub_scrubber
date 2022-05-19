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
        let test1 = "AMZ*The Washington P 04/10 PURCHASE help@washpost DC";
        let test1_result = "AMZ*The Washington P PURCHASE help@washpost DC".to_string();
        let test2 = "NETFLIX COM 12/28 PURCHASE LOS GATOS CA";
        let test3 = "NETFLIX COM 11/28 PURCHASE LOS GATOS CA";
        let test4 = "NETFLIX COM 01/28 PURCHASE LOS GATOS CA";
        let test2_result = "NETFLIX COM PURCHASE LOS GATOS CA";
        let test_amount = 45.50;
        
        assert_eq!(&test1_result, &remove_mm_slash_yy(test1.into()).unwrap());
        assert_eq!(&test2_result, &remove_mm_slash_yy(test2.into()).unwrap());
        assert_eq!(&remove_mm_slash_yy(test2.into()).unwrap(), &remove_mm_slash_yy(test3.into()).unwrap());
        assert_eq!(&remove_mm_slash_yy(test2.into()).unwrap(), &remove_mm_slash_yy(test4.into()).unwrap());
        
        // dbg!(transaction::calculate_hash(&result, &test_amount.to_string()));
    }
}