use std::env;
use std::net::{IpAddr, TcpStream, SocketAddr};
use std::io::{self, Write};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use std::time::Duration;

const MAX_PORT: u16 = 65535;

#[derive(Debug)]
struct Arguements {
    flags: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguements {

    fn new(args: &[String]) -> Result<Arguements, &'static str> {
      // Result<Ok, Err> ->
      //   Ok() for Arguements ;
      //   static lifetime so we can send back Err message to main fn for Err()
      if args.len() < 2 {
          // min = port-checker-rs <ipaddr> OR port-checker-rs -h
          return Err("not enough Arguements");
      } else if args.len() > 4 {
          // max = port-checker-rs -j 100 <ipaddr>
          return Err("too many Arguements");
      }

      let f = args[1].clone();

      if let Ok(ipaddr) = IpAddr::from_str(&f) {
        // if called with only ipaddr and parsed as valid ip
        return Ok(Arguements { flags: String::from(""), ipaddr, threads: 4 });
      } else {
        // if called with flags
        let flags = args[1].clone();
        if flags.contains("-h") || flags.contains("--help") && args.len() == 2 {
            println!("Usage: port-checker-rs [flags] <ipaddr>
            \r\nFlags: -j <threads> <ipaddr> for the number of threads to use w/ ipaddr
            \r\n       -h or --help for this help message");
            return Err("help");
        } else if flags.contains("-h") || flags.contains("--help") {
            return Err("too many Arguements");
        } else if flags.contains("-j") {
            let ipaddr = match IpAddr::from_str(&args[3]) {
                Ok(ipaddr) => ipaddr,
                Err(_) => return Err("invalid ipaddr: must be ipv4 or ipv6"),
            };
            let threads = match args[2].parse::<u16>() {
                Ok(threads) => threads,
                Err(_) => return Err("invalid threads: must be a number"),
            };
            return Ok(Arguements { flags, ipaddr, threads });
            } else {
                return Err("invalid syntax");
            }
        }
    }
}


fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        let sock_addr = SocketAddr::new(addr, port);
        let timeout: Duration = Duration::from_millis(200);
        match TcpStream::connect_timeout(&sock_addr, timeout) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {
                // print!("x");
                // io::stdout().flush().unwrap();
            }
        };

        if MAX_PORT - port <= num_threads {
            break;
        }

        port += num_threads;

    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguements = Arguements::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing Arguements: {}", program, err);
            process::exit(1);
        }
    });
    println!("Debug :: Running {} with :: Arguements({:?})", program, arguements);

    let num_of_threads: u16 = arguements.threads;
    let addr = arguements.ipaddr;

    let (tx, rx) = channel();

    for i in 0 .. num_of_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr, num_of_threads);
        });
    }

    let mut out: Vec<u16> = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!();
    out.sort();
    for v in out {
        println!("port {} is open!", v);
    }

}
