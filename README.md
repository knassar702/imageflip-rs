imageflip.com parser crate 🦀

```rust
use imgflipparser::start;

fn main() {
    for i in start(Some(10)) {
        for (key, value) in i {
            println!("{}:{}", key, value);
        }
    }
}
```
