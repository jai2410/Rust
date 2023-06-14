const MAX_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn main() {
    let (missiles, ready): (i32, i32) = (MAX_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles-ready);
}