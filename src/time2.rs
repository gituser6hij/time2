use chrono::{DateTime, Local};
use std::time::SystemTime;

fn main() {
    let current_time = SystemTime::now();
    let datetime: DateTime<Local> = current_time.into();
    println!("Current time: {}", datetime.format("%Y-%m-%d %H:%M:%S"));
}
