
use mylib;

pub fn main() {
    let line = mylib::read_serial_port();
    println!("{:?}", line);

}
