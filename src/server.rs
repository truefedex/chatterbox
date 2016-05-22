use chatterbox::*;
use output::WavChatterboxOutput;
use iron::prelude::*;
use iron::status;
use staticfile::Static;
use mount::Mount;
use router::Router;
use url::percent_encoding::percent_decode;
use iron::headers::{ContentLength, ContentType};
use iron::mime::{Mime, TopLevel, SubLevel};
use iron::modifiers::Header;
use std::path::Path;
use std::io::Cursor;
use std::sync::Arc;


fn synth(req: &mut Request, backend: Arc<Box<Backend + Send + Sync>>) -> IronResult<Response> {	
	let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("");
	let decoded_query = percent_decode(query.as_bytes()).decode_utf8().unwrap();
	
	let mut buf = Cursor::new(Vec::new());
	{
		let mut output = WavChatterboxOutput::new_for_mem(&mut buf);
		backend.synth(&decoded_query, &mut output);
	}
	let raw_result = buf.into_inner();
	let len = raw_result.len();
	let mut res = Response::with((status::Ok, raw_result ));
	res.set_mut(Header(ContentLength(len as u64)));
	res.set_mut(Header(ContentType(Mime(TopLevel::Audio, SubLevel::Ext("wav".to_string()), vec![]))));
	Ok(res)
}

pub fn run(backend: Arc<Box<Backend + Send + Sync>>) {
    let mut router = Router::new();
    
    router
        .get("/synth/:query", move |req: &mut Request| synth(req, backend.clone()));

    let mut mount = Mount::new();
    mount
        .mount("/api/", router)
        .mount("/", Static::new(Path::new("www")));

    println!("Server running on http://localhost:3000/");

    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}
