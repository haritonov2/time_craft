mod clock;

use clock::Clock;

fn main() {
    let time: Clock = Clock::new(1, 20);

    println!("{}", time);
}
