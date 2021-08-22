# bb8-skytable

[Skytable rust client](https://github.com/skytable/client-rust) support library for the [bb8](https://github.com/djc/bb8) connection pool. Heavily based on [bb8-redis](https://github.com/djc/bb8/tree/main/redis)

# Basic usage example

```rust
use bb8_skytable::{
    bb8,
    skytable:{actions::Actions},
    SkytableConnectionManager
};

#[tokio::main]
async fn main() {
	let manager = SkytableConnectionManager::new("127.0.0.1", 2003);
	let pool = bb8::Pool::builder().build(manager).await.unwrap();
  	let mut conn = pool.get().await.unwrap();
    conn.set("x", "100").await.unwrap();
}
```
