mod transaction;
mod importers;
mod utils;

mod prelude {  
    pub use chrono::prelude::*;
    pub use csv::Reader;
    pub use std::path::Path;
    pub use std::error::Error;
    pub use regex::Regex;
    pub use std::collections::hash_map::DefaultHasher;
    pub use std::hash::{Hash, Hasher};

    use crate::transaction;
    
    pub use crate::utils::*;
    pub use crate::importers::{
        csv_importer, 
        regex_cleaner,
    };
}

use prelude::*;

fn main() {
    println!("Hello, world!");
}
