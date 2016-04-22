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

pub struct PatternCollection {
	sounds: BTreeMap<String, String>,
}

#[derive(RustcDecodable)]
pub struct PatternCollectionDescription {
	path: String,
	sounds: BTreeMap<String, String>,
}

impl PatternBased {
	fn read_pattern_collection(path: &str) -> Result<PatternCollection, SynthError> {
		let mut file = try!(File::open(path).map_err(SynthError::Io));
		let mut data = String::new();
		try!(file.read_to_string(&mut data).map_err(SynthError::Io));
		let collection_desc: PatternCollectionDescription = try!(json::decode(&data).map_err(SynthError::PatternCollectionDecode));		
		debug!("Decoded path of collection: {}, patterns count: {}", collection_desc.path, collection_desc.sounds.len());
		
		let mut sounds = BTreeMap::new();
		let relative_patterns_path = &collection_desc.path;
		for (letter, file_name) in collection_desc.sounds.iter() {
			let collection_path = Path::new(path).parent();
			if collection_path.is_none() {
				continue;
			}
			let pattern_path = collection_path.unwrap().join(relative_patterns_path).join(file_name);
			debug!("Pattern path: {}", pattern_path.to_str().unwrap());
			//let mut reader = hound::WavReader::open("testsamples/pop.wav").unwrap();
		}
		Ok(PatternCollection{sounds: sounds})
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
