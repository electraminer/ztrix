extern crate rouille;
use std::fs::File;
use std::path::Path;
use rouille::Response;

fn main() {
	rouille::start_server("0.0.0.0:80", move |request| {
		println!("{}", request.url());

		// Return specific HTML files
		if let Some(name) = match request.url().as_str() {
			"/" => Some("ztrix.html"),
			_ => None,
		} {
			let path = Path::new("./web/html").join(name);
			let file = File::open(path).expect(
				"manually listed file should exist");
			return Response::from_file("text/html", file);
		}

		// Return generic HTML files
		if let Some(request) = request.remove_prefix("/html") {
			return rouille::match_assets(&request, "./web/html");
		}
		// Return generic HTML files
		if let Some(request) = request.remove_prefix("/css") {
			return rouille::match_assets(&request, "./web/css");
		}
		// Return generic JS/WASM files
		if let Some(request) = request.remove_prefix("/wasm") {
			return rouille::match_assets(&request, "./web/wasm/pkg");
		}

		return Response::empty_404();
	});
}