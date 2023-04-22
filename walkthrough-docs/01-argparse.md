# std::env for args

For - 
```rs
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```

Output - 
```
❯❯❯ cargo run
   Compiling port-checker-rs v0.1.0 (/Users/jaydihenkar/work/port-checker-rs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.99s
     Running `target/debug/port-checker-rs`
["target/debug/port-checker-rs"]

[~/w/port-checker-rs] (09:45:32) master
❯❯❯ cargo run -- -h
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/port-checker-rs -h`
["target/debug/port-checker-rs", "-h"]

[~/w/port-checker-rs] (09:45:38) master
❯❯❯ cargo run -- -j 100 192.168.1.1
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/port-checker-rs -j 100 192.168.1.1`
["target/debug/port-checker-rs", "-j", "100", "192.168.1.1"]
```
