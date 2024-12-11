use std::time::Instant;

mod first;
mod second;

fn main() {
    println!("PART_ONE: {}", first::run());

    let before = Instant::now();
    println!("PART_TWO: {}", second::run());
    println!("Elapsed time: {:.2?}", before.elapsed());
}
