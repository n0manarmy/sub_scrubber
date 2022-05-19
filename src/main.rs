pub mod transaction;
pub mod ledger;
mod transaction_search;
mod importers;
mod utils;

mod prelude {  
    pub use chrono::prelude::*;
    pub use csv::Reader;
    pub use std::path::Path;
    pub use std::error::Error;
    pub use std::io::prelude::*;
    pub use std::fs::File;
    pub use std::io::Read;
    pub use std::io;
    pub use std::fs::{self, DirEntry};

    pub use regex::Regex;
    pub use serde::ser::{Serializer, SerializeStruct};
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::hash_map::DefaultHasher;
    pub use std::collections::HashMap;
    pub use std::hash::{Hash, Hasher};
    pub use uuid::Uuid;
    pub use base64::encode;

    pub use crate::transaction;
    pub use crate::ledger::Ledger;
    pub use crate::transaction::Transaction;
    pub use crate::importers::bank_of_america_excel_import;
    
    pub use crate::utils::*;
    pub use crate::importers::{
        csv_importer, 
        regex_cleaner,
        statement_loader,
    };
}

use prelude::*;

fn main() {
    println!("Hello, world!");
}
