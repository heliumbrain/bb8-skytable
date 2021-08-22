# bb8-skytable

[Skytable rust client](https://github.com/skytable/client-rust) support library for the [bb8](https://github.com/djc/bb8) connection pool.

# Basic usage example

```rust
use bb8_skytable::{
    bb8,
    skytable:{actions::Actions, Query},
    SkytableConnectionManager
};

#[tokio::main]
async fn main() {
	let manager = SkytableConnectionManager::new("127.0.0.1", 2003);
	let pool = bb8::Pool::builder().build(manager).await.unwrap();
	let manager = SkytableConnectionManager::new("127.0.0.1", 2003);
	let pool = bb8::Pool::builder().build(manager).await.unwrap();
	let pool = bb8::Pool::builder().build(manager).await
  	let mut conn = pool.get().await.unwrap();!
    conn.set(x, "2".to_string()).await.unwrap();
}
```
