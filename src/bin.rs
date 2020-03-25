
use mylib;

pub fn main() {
    loop {
        let line = mylib::read_serial_port();
        println!("{:?}", line);
    }
}
