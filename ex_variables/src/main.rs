const STARTING_MISSILES :i32 = 8;
const READY_AMOUNT :i32 = 8;

fn main() {
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles. {} missiles lift...", ready, missiles, missiles - ready);
}

