

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::terminal_size;




#[test]
fn it_works() {
    let x = terminal_size();
    println!("{:?}", x);
}
