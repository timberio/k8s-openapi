#![warn(rust_2018_idioms)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
)]

use k8s_openapi::{http, url};

pub enum UriBuilder {
	WithSchemeAndAuthority { scheme: Option<http::uri::Scheme>, authority: Option<http::Uri::Authority> },
	Custom(Box<Fn(&http:::Uri) -> Result<http::Uri, Box<dyn std::error::Error + Send + Sync>>>),
}

impl UriBuilder {
	fn apply(&self, request_uri: &http::Uri) -> Result<http::Uri, Box<dyn std::error::Error + Send + Sync>>> {
		match self {
			UriBuilder::WithSchemeAndAuthority { scheme, authority } => {
				let mut parts = request_uri.clone().into_parts();
				parts.scheme = scheme.clone();
				parts.authority = authority.clone();
				let result = http::Uri::from_parts(parts)?;
				Ok(result)
			},

			UriBuilder::Custom(f) => f(request_uri),
		}
	}
}

#[cfg(feature = "reqwest-blocking")]
pub struct ReqwestBlockingClient {
	inner: reqwest::blocking::Client,
	uri_builder: UriBuilder,
}

#[cfg(feature = "reqwest-blocking")]
impl ReqwestBlockingClient {
	pub fn new(
		inner: reqwest::blocking::Client,
		uri_builder: UriBuilder,
	) -> Self {
		ReqwestBlockingClient {
			inner,
			uri_builder,
		}
	}

	pub fn inner(&self) -> &reqwest::blocking::Client {
		&self.inner
	}

	pub fn inner_mut(&mut self) -> &mut reqwest::blocking::Client {
		&mut self.inner
	}

	pub fn execute<T>(
		&self,
		request: Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::RequestBody<T>), k8s_openapi::RequestError>,
	) -> Result<T, ReqwestBlockingError> {
		self.execute_stream(request).next()
	}

	pub fn execute_stream<T>(
		&self,
		request: Result<(http::Request<Vec<u8>>, fn(http::StatusCode) -> k8s_openapi::RequestBody<T>), k8s_openapi::RequestError>,
	) -> impl Iterator<Item = Result<T, ReqwestBlockingError>> {
		let (request, response_body) = request.map_err(ReqwestBlockingError::Request)?;

		let (parts, body) = request.into_parts();

		let mut partial_uri_parts: http::uri::Parts = parts.uri.into();
		let mut uri_parts: http::uri::Parts = self.base_uri.clone().into();
		uri_parts.path_and_query = partial_uri_parts.path_and_query;
		let uri = http::Uri::from_parts(uri_parts);

		let request = self.inner.client();
	}
}

#[derive(Debug)]
enum ReqwestBlockingError {
	Request(k8s_openapi::RequestError),
	Response(k8s_openapi::ResponseError),
}

impl std::fmt::Display for ReqwestBlockingError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ReqwestBlockingError::Request(_) => f.write_str("could not create request"),
			ReqwestBlockingError::Response(_) => f.write_str("could not create response"),
		}
	}
}

impl std::error::Error for ReqwestBlockingError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			ReqwestBlockingError::Request(err) => Some(err),
			ReqwestBlockingError::Response(err) => Some(err),
		}
	}
}
