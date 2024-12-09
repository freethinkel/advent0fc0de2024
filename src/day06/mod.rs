use std::time::Instant;

mod first;
mod second;

fn main() {
    let before = Instant::now();
    println!("PART_ONE: {}", first::run());
    println!("PART_TWO: {}", second::run());
    println!("Elapsed time: {:.2?}", before.elapsed());
}
