//! Skytable support for the `bb8` connection pool.
//!
//! # Basic Example
//!
//! ```
//! use bb8_skytable::{
//!     bb8,
//!     skytable:{actions::Actions},
//!     SkytableConnectionManager
//! };
//!
//! #[tokio::main]
//! async fn main() {
//! 	let manager = SkytableConnectionManager::new("127.0.0.1", 2003);
//!		let pool = bb8::Pool::builder().build(manager).await.unwrap();

//!   	let mut conn = pool.get().await.unwrap();
//!     conn.set("x", "100").await.unwrap();
//! }
//! ```
#![allow(clippy::needless_doctest_main)]
#![deny(missing_docs, missing_debug_implementations)]

pub use bb8;
pub use skytable;

use async_trait::async_trait;

use skytable::aio::Connection;
use skytable::query;
use skytable::Element;

/// A `bb8::ManageConnection` for `skytable::aio::Connection`.

#[derive(Clone, Debug)]
pub struct SkytableConnectionManager {
	host: String,
	port: u16,
}

impl SkytableConnectionManager {
	/// Create a new `SkytableConnectionManager`.
	/// See `skytable::aio::Connection::new` for a description of the parameter types.
	pub fn new(host: &str, port: u16) -> SkytableConnectionManager {
		SkytableConnectionManager {
			host: host.to_string(),
			port,
		}
	}
}

#[async_trait]
impl bb8::ManageConnection for SkytableConnectionManager {
	type Connection = Connection;
	type Error = std::io::Error;

	async fn connect(&self) -> Result<Self::Connection, Self::Error> {
		Connection::new(&self.host, self.port).await
	}

	async fn is_valid(
		&self,
		conn: &mut bb8::PooledConnection<'_, Self>,
	) -> Result<(), Self::Error> {
		let query = query!("HEYA");
		let result = conn.run_simple_query(&query).await;
		let response = match result {
			Ok(response) => response,
			Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::NotConnected, e.to_string())),
		};

		match response {
			Element::String(s) => {
				if s == "HEY!" {
					Ok(())
				} else {
					Err(std::io::Error::new(
						std::io::ErrorKind::InvalidData,
						"Did not receive HEY!",
					))
				}
			}
			_ => Err(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"Did not receive a String!",
			)),
		}
	}

	fn has_broken(&self, _: &mut Self::Connection) -> bool {
		false
	}
}
