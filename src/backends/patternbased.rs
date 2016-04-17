extern crate rustc_serialize;

use Backend;
use Output;
use std::fs;
use std::io;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use rustc_serialize::json;
use std::collections::BTreeMap;

pub struct PatternBased {
	patterns: Vec<PatternCollection>,
}

#[derive(RustcDecodable)]
pub struct PatternCollection {
	path: String,
	sounds: BTreeMap<String, String>,
}

impl PatternBased {
	fn read_pattern_collection(path: &str) -> Result<PatternCollection, io::Error> {
		let mut file = File::open(path).unwrap();
		let mut data = String::new();
		file.read_to_string(&mut data).unwrap();

		let json = Json::from_str(&data).unwrap();
		println!("We here: {}", path);
		Ok(PatternCollection)
	}
	
	pub fn from_patterns_path(patterns_path: &str) -> PatternBased {
		let mut collections: Vec<PatternCollection> = Vec::new();
		
		if let Ok(entrys) = fs::read_dir(patterns_path) {			
			for entry_result in entrys {
				if let Ok(entry) = entry_result {
					if entry.file_type().unwrap().is_file() { 
						if let Some(ext) = entry.path().as_path().extension() {
							if ext.to_str().unwrap().to_string().to_lowercase() == "json" {
								if let Some(path_str) = entry.path().as_path().to_str() {
									if let Ok(collection) = PatternBased::read_pattern_collection(path_str) {
										collections.push(collection);
									}
								}
							}
						}
					}					
				}
			}
		}
		
		PatternBased{patterns: collections}
	}
	
	pub fn new(patterns: Vec<PatternCollection>) -> PatternBased {
		PatternBased{patterns: patterns}
	}
}

impl Backend for PatternBased {
    fn synth(&self, input : &str, out: &mut Output) {
        for chr in input.chars() {
            //self.synth_char(chr, out)
        }
    }       
}
