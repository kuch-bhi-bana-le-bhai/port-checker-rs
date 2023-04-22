# std::env for args to struct

For code in main which has the argparse implementation with struct and impl -> Output - 
```
❯ cargo run -- 192.168.1.1 
...
Running target/debug/port-checker-rs with :: Arguements(Arguements { flags: "", ipaddr: 192.168.1.1, threads: 4 })

❯ cargo run -- -j 100 192.168.1.1
...
Running target/debug/port-checker-rs with :: Arguements(Arguements { flags: "-j", ipaddr: 192.168.1.1, threads: 100 })

❯ cargo run -- -h
...
...
Usage: port-checker-rs [flags] <ipaddr>

Flags: -j <threads> <ipaddr> for the number of threads to use w/ ipaddr

       -h or --help for this help message
```
