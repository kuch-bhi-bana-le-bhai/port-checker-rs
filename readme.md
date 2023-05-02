# port-checker-rs
Check for open ports on an IP address.

Usages:
- `port-checker-rs -h`: help
- `port-checker-rs -j 100 192.168.1.1`: scan with defined number of threads
-  `port-checker-rs 192.168.1.1`: scan with default number of threads (10)

Sample Run:
```
❯ ./target/release/port-checker-rs -j 10 192.168.1.1
Debug :: Running ./target/release/port-checker-rs with :: Arguements(Arguements { flags: "-j", ipaddr: 192.168.1.1, threads: 10 })
....
port 53 is open!
port 80 is open!
port 443 is open!
port 8883 is open!
```
