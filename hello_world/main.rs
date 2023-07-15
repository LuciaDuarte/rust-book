
/* The main function is special: it is always the first code that runs
 in every executable Rust program. */
fn main() {
  /* println! calls a Rust macro. If it had called a function instead, 
  it would be entered as println (without the !).
   */
    println!("Hello, world!");
}

/* 
Before running a Rust program, you must compile it using the 
Rust compiler by entering the rustc command and passing it the
name of your source file, like this:
$ rustc main.rs
*/

