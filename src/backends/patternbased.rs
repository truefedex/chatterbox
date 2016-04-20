use super::super::*;
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
	fn read_pattern_collection(path: &str) -> Result<PatternCollection, SynthError> {
		let mut file = try!(File::open(path).map_err(SynthError::Io));
		let mut data = String::new();
		try!(file.read_to_string(&mut data).map_err(SynthError::Io));
		let collection: PatternCollection = try!(json::decode(&data).map_err(SynthError::PatternCollectionDecode));		
		info!("Decoded path of collection: {}, patterns count: {}", collection.path, collection.sounds.len());
		Ok(collection)
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
											match PatternBased::read_pattern_collection(path_str) {
												Ok(collection) => collections.push(collection),
												Err(err) => warn!("Unable to read pattern collection {}", err),
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
