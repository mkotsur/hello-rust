use std::io;
use std::sync::mpsc::channel;

const BELL_CHART: char = '';

fn main() {
    println!("Enter minutes:");

    let mut mins = String::new();

    let res = io::stdin().read_line(&mut mins);
    res.expect("Failed to read line");

    let mins: i64 = match mins.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };

    println!("Starting 🍅 for {} minutes", mins);

    let timer = timer::Timer::new();
    let (tx, rx) = channel();

    let _guard = timer.schedule_with_delay(chrono::Duration::minutes(mins), move || {
        // This closure is executed on the scheduler thread,
        // so we want to move it away asap.

        let _ignored = tx.send(()); // Avoid unwrapping here.
    });

    rx.recv().unwrap();
    println!("{} 🍅 🍅 🍅", BELL_CHART);
}
