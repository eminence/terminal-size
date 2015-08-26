
#[derive(Debug)]
pub struct Width(u16);
#[derive(Debug)]
pub struct Height(u16);

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::terminal_size;


#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::terminal_size;


#[test]
fn it_works() {
    let x = terminal_size();
    println!("{:?}", x);
}
