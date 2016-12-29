extern crate chrono;

fn main() {

    use chrono::{Local, DateTime};
    let local: DateTime<Local> = Local::now();

    println!("{}",local);
}
