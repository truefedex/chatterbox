use super::super::*;
use std::fs;
use std::io;
use std::cmp;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use rustc_serialize::*;
use std::collections::BTreeMap;
use hound::WavReader;

pub struct PatternBased {
	patterns: Vec<PatternCollection>,
}

pub struct PatternCollection {
	sounds: BTreeMap<String, Vec<i16>>,
	max_chars: i16,
}

#[derive(RustcDecodable)]
pub struct PatternCollectionDescription {
	path: String,
	max_chars: i16,
	sounds: BTreeMap<String, String>,
}

impl PatternBased {
	fn read_pattern_collection(path: &str) -> Result<PatternCollection, SynthError> {
		let mut file = try!(File::open(path).map_err(SynthError::Io));
		let mut data = String::new();
		try!(file.read_to_string(&mut data).map_err(SynthError::Io));
		let collection_desc: PatternCollectionDescription = try!(json::decode(&data).map_err(SynthError::PatternCollectionDecode));		
		debug!("Decoded path of collection: {}, patterns count: {}", collection_desc.path, collection_desc.sounds.len());
		
		let mut sounds: BTreeMap<String, Vec<i16>> = BTreeMap::new();
		let relative_patterns_path = &collection_desc.path;
		for (chars, file_name) in collection_desc.sounds.iter() {
			let collection_path = Path::new(path).parent();
			if collection_path.is_none() {
				continue;
			}
			let pattern_path = collection_path.unwrap().join(relative_patterns_path).join(file_name);
			debug!("Pattern path: {}", pattern_path.to_str().unwrap());
			if let Ok(reader) = WavReader::open(&pattern_path) {
				let spec = reader.spec();
				if spec.channels != 1 {
					warn!("Unsupported pattern format - only mono sources supported: {}", pattern_path.to_str().unwrap());
					continue;
				}
				if spec.sample_rate != 44100 {
					warn!("Unsupported pattern format - supported samplerate is 44100 : {}", pattern_path.to_str().unwrap());
					continue;
				}
				if spec.bits_per_sample != 16 {
					warn!("Unsupported pattern format - only 16 bit audio data supported : {}", pattern_path.to_str().unwrap());
					continue;
				}
				let samples: Vec<i16> = reader.into_samples().filter_map(|sample|{ sample.ok() }).collect();
				sounds.insert(chars.clone(), samples);
			}
		}
		Ok(PatternCollection{sounds: sounds, max_chars: collection_desc.max_chars})
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
		
		PatternBased::new(collections)
	}
	
	pub fn new(patterns: Vec<PatternCollection>) -> PatternBased {
		PatternBased{patterns: patterns}
	}
}

impl Backend for PatternBased {
    fn synth(&self, input : &str, out: &mut Output) {
		let mut max_chars = 1;
		for collection in &self.patterns {
			if max_chars < collection.max_chars {
				max_chars = collection.max_chars;
			}
		}
	
        for (i, _) in input.chars().enumerate() {
			let mut max_scan_chars_count = cmp::min(max_chars, (input.len() - i) as i16);
			debug!("i {} max_scan_chars_count {}", i, max_scan_chars_count);
			for scan_chars_count in (1..max_scan_chars_count).rev() {
				debug!("scan_chars_count {}", scan_chars_count);
				let str_to_search = &input[i..i + scan_chars_count as usize];
				debug!("Str i: {} str_to_search: {}", i, str_to_search);
				for collection in &self.patterns {
					
				}
			}
        }
    }       
}
