extern crate rouille;
use std::fs::File;

use rouille::Response;

fn main() {
	rouille::start_server("0.0.0.0:80", move |request| {
		println!("{}", request.url());

		// Return generic HTML files
		if let Some(request) = request.remove_prefix("/html") {
			return rouille::match_assets(&request, "./web/html");
		}
		// Return generic CSS files
		if let Some(request) = request.remove_prefix("/css") {
			return rouille::match_assets(&request, "./web/css");
		}
		// Return generic JS/WASM files
		if let Some(request) = request.remove_prefix("/wasm") {
			return rouille::match_assets(&request, "./web/wasm/pkg");
		}
		// Return generic asset files
		if let Some(request) = request.remove_prefix("/assets") {
			return rouille::match_assets(&request, "./web/assets");
		}

		if request.url() == "/favicon.ico" {
			let file = File::open("./web/assets/favicon.png")
				.expect("manually listed file should exist");
			return Response::from_file("image/png", file);
		}

		// Return single page application
		let file = File::open("./web/html/ztrix.html")
			.expect("manually listed file should exist");
		return Response::from_file("text/html", file);
	});
}