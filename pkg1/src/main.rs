use chrono;
use chrono::prelude::*;

fn main() {
    let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    print!("{}\n",dt)
    // try using the `chrono` crate here
}