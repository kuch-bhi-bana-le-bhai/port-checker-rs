# Error Handling in Rust

## recoverable Errors using `Result` type

The `Result` type which is defined as followings is used to handle errors in rust - 
```rs
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Example - 1: Matching the Error using the Result enum type.
```rs
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    // debug type of `f`
    println!("f={:?}", f);

    match f {
        Ok(file) => file,
        Err(ref error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

Output where file doesn't exists - 
```
f=Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:11:27
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### Example 2 - Advanced Error Handling mechanisms

```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() { 
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_errors => {
                panic!("Problem opening the file: {:?}", other_errors)
            },
        },
    };

    // debug type of `f`
    println!("f={:?}", f);
}
```

Here we use `ErrorKind::NotFound` to ensure that the file is not found - and handle it by creating the file.

Output - 
```
f=File { fd: 3, path: "/Users/jaydihenkar/work/port-checker-rs/walkthrough-docs/error-handling-rs/hello.txt", read: false, write: true }
```

### Example 3 - Rewriting the above error handling with `unwrap_or_else( |err| )`
```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else( |err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else( |err| {
                panic!("Problem creating the file: {:?}", err);
            })
        } else {
            panic!("Problem opening the file: {:?}", err);
        }
    });

    // debug type of `f`
    println!("f={:?}", f);
}
```

Here `unwrap_or_else( |err| {} )` is a closure that makes the error available for handling.

### Example 4 - One can use `unwrap()` to extract the result

```rs
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    // debug type of `f`
    println!("f={:?}", f);
}
```

Here, the unwrap() basically tries to return the result Ok() instead of result type.

Output - 
```
# If file is present OP - 
f=File { fd: 3, path: "/Users/jaydihenkar/work/port-checker-rs/walkthrough-docs/error-handling-rs/hello.txt", read: true, write: false }

# If file is not present OP - 
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:4:37
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### Example 5 - One can use `expect()` to expect error and react accordingly
```rs
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")
        .expect("custom :: File not found!");
    // debug type of `f`
    println!("f={:?}", f);
}
```

this help returning the custom message in the error, though it'd still panic the thread - 
```
thread 'main' panicked at 'custom :: File not found!: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:5:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### Example 6 - Propogating the error to the caller
```rs
use std::fs::File;
use std::io::Error;

fn openfile(filename: String) -> Result<File, Error> {
    let f = File::open(filename);
    match f {
        Ok(file) => Ok(file),
        Err(e) => Err(e),
    }
}

fn main() {
    let f = openfile(String::from("hello.txt"));

    // debug type of `f`
    println!("f={:?}", f);
}
```

Here the error is propogated to the caller using the Result return type.

### Example 7 - Using the `?` Operator as a shorthand for returning Results
```rs
use std::fs::File;
use std::io::Error;

fn openfile(filename: String) -> Result<File, Error> {
    let f = File::open(filename)?;
    Ok(f)
}

fn main() {
    let f = openfile(String::from("hello.txt"));

    // debug type of `f`
    println!("f={:?}", f);
}
```

## `panic` macro
Rust sets an exit code of 101 when the process panicked.

example for following code - 
```rs
fn main() {
    panic!("crash and burn");
}
```

Output - 
```
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/error-handling-rs`
thread 'main' panicked at 'crash and burn', src/main.rs:3:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

❯ RUST_BACKTRACE=1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/error-handling-rs`
thread 'main' panicked at 'crash and burn', src/main.rs:3:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/panicking.rs:575:5
   1: core::panicking::panic_fmt
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/panicking.rs:64:14
   2: error_handling_rs::main
             at ./src/main.rs:3:5
   3: core::ops::function::FnOnce::call_once
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/ops/function.rs:507:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

❯ RUST_BACKTRACE=full cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/error-handling-rs`
thread 'main' panicked at 'crash and burn', src/main.rs:3:5
stack backtrace:
   0:        0x102f39706 - std::backtrace_rs::backtrace::libunwind::trace::h310cbd77a7d2ae59
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:        0x102f39706 - std::backtrace_rs::backtrace::trace_unsynchronized::h5768bae568840507
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:        0x102f39706 - std::sys_common::backtrace::_print_fmt::hd104a205649a2ffb
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/sys_common/backtrace.rs:65:5
   3:        0x102f39706 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h521420ec33f3769d
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/sys_common/backtrace.rs:44:22
   4:        0x102f4d1da - core::fmt::write::h694a0d7c23f57ada
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/fmt/mod.rs:1208:17
   5:        0x102f3765c - std::io::Write::write_fmt::h1920a3973ad439e5
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/io/mod.rs:1682:15
   6:        0x102f394ea - std::sys_common::backtrace::_print::h75582c4ed1a04abb
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/sys_common/backtrace.rs:47:5
   7:        0x102f394ea - std::sys_common::backtrace::print::hef1aa4dbdc07ee06
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/sys_common/backtrace.rs:34:9
   8:        0x102f3ab43 - std::panicking::default_hook::{{closure}}::h529701a1070b4ce0
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/panicking.rs:267:22
   9:        0x102f3a898 - std::panicking::default_hook::hfeeab2c667b2d7c2
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/panicking.rs:286:9
  10:        0x102f3b273 - std::panicking::rust_panic_with_hook::h1b5245192f90251d
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/panicking.rs:688:13
  11:        0x102f3b003 - std::panicking::begin_panic_handler::{{closure}}::h3658f3a9566379d4
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/panicking.rs:577:13
  12:        0x102f39ba8 - std::sys_common::backtrace::__rust_end_short_backtrace::h9e01645d962f8882
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/sys_common/backtrace.rs:137:18
  13:        0x102f3ad0d - rust_begin_unwind
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/panicking.rs:575:5
  14:        0x102f53e03 - core::panicking::panic_fmt::h0097ad8ec0b07517
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/panicking.rs:64:14
  15:        0x102f1bb1d - error_handling_rs::main::ha22c6c282bbdea36
                               at /Users/jaydihenkar/work/port-checker-rs/walkthrough-docs/error-handling-rs/src/main.rs:3:5
  16:        0x102f1b9ae - core::ops::function::FnOnce::call_once::hb9dbc1457f270dd7
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/ops/function.rs:507:5
  17:        0x102f1b931 - std::sys_common::backtrace::__rust_begin_short_backtrace::he72dc5ece57955c4
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/sys_common/backtrace.rs:121:18
  18:        0x102f1b904 - std::rt::lang_start::{{closure}}::h9eb0d68c845357ff
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/rt.rs:166:18
  19:        0x102f35664 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::h2302f1d25ef2ca9b
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/ops/function.rs:606:13
  20:        0x102f35664 - std::panicking::try::do_call::h6695e32a593de2cc
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/panicking.rs:483:40
  21:        0x102f35664 - std::panicking::try::hd4a93095627721a9
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/panicking.rs:447:19
  22:        0x102f35664 - std::panic::catch_unwind::he41b3dba63feca94
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/panic.rs:137:14
  23:        0x102f35664 - std::rt::lang_start_internal::{{closure}}::hbf45583011495a61
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/rt.rs:148:48
  24:        0x102f35664 - std::panicking::try::do_call::ha3e6b3edab7da449
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/panicking.rs:483:40
  25:        0x102f35664 - std::panicking::try::hd4e0f354bf7022b9
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/panicking.rs:447:19
  26:        0x102f35664 - std::panic::catch_unwind::h1035b163871a4269
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/panic.rs:137:14
  27:        0x102f35664 - std::rt::lang_start_internal::hd56d2fa7efb2dd60
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/rt.rs:148:20
  28:        0x102f1b8d7 - std::rt::lang_start::h0ae865179ef05c24
                               at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/rt.rs:165:17
  29:        0x102f1bb38 - _main

❯ echo $status
101
```

### `panic = abort`
In case when we want the OS to handle the panic - we can mention the `panic = abort` in `Cargo.toml`.

example in toml - 
```toml
[profile.release]
panic = 'abort'
```

Output -
```
❯ cargo run --release
    Finished release [optimized] target(s) in 0.02s
     Running `target/release/error-handling-rs`
thread 'main' panicked at 'crash and burn', src/main.rs:3:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
fish: Job 1, 'cargo run --release' terminated by signal SIGABRT (Abort)

❯ echo $status
134
# Exit code 134 means that the program was aborted by a SIGABRT signal

```