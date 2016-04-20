use Backend;
use Output;
use std::fs;
use std::io;
use std::ffi::OsStr;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use rustc_serialize::*;
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
		/*let mut file = File::open(path).unwrap();
		let mut data = String::new();
		file.read_to_string(&mut data).unwrap();

		let json = Json::from_str(&data).unwrap();
		println!("We here: {}", path);*/
		Ok(PatternCollection{path: "".to_string(), sounds: BTreeMap::new()})
	}
	
	pub fn from_patterns_path(patterns_path: &str) -> PatternBased {
		let mut collections: Vec<PatternCollection> = Vec::new();
		
		const PATTERN_COLLECTION_CONFIG_EXT :  &'static str = "json";
		if let Ok(entrys) = fs::read_dir(patterns_path) {			
			for entry_result in entrys {
				if let Ok(entry) = entry_result {
					if let Ok(file_type) = entry.file_type() {
						if file_type.is_file() {							
							if let Some(ext) = entry.path().as_path().extension() {
								if let Some(ext_str) = ext.to_str() {
									if ext_str.to_string().to_lowercase() == PATTERN_COLLECTION_CONFIG_EXT {
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
