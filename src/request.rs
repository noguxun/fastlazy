
use fastly::{Body, Request, Error};
use fastly::http::{HeaderValue};


pub trait RequestLazy: Sized {
	fn set_header(self, header: &'static str, value: &str) -> Result<Self, Error>;
	fn set_path(&mut self, new_path: &str) -> Result<(), Error>;
}


impl RequestLazy for Request<Body> {
	fn set_header(mut self, header: &'static str, value: &str) -> Result<Self, Error> {
		self.headers_mut()
				.insert(header, HeaderValue::from_str(value)?);
				
		Ok(self)
	}

	fn set_path(&mut self, new_path: &str) -> Result<(), Error>{
		let mut parts = self.uri().clone().into_parts();
    let path_and_query = match self.uri().query() {
        Some(query) => format!("{}?{}", new_path, query),
        None => String::from(new_path),
    }
    .parse()?;
    parts.path_and_query = Some(path_and_query);
    let uri = fastly::http::Uri::from_parts(parts)?;
		*self.uri_mut() = uri;
		
		Ok(())
	}
}


