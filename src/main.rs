#![no_main]

use std::{str::FromStr, thread, time::Duration};

// pub fn main() {
//     loop {
//         println!("Performing recurring task...");
//         thread::sleep(Duration::from_secs(5));
//     }
// }

#[no_mangle]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
