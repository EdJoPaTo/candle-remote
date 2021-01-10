use crate::mqtt::Sender;
use crate::MAX_HEIGHT;
use chrono::{DateTime, Local, Timelike};
use std::thread::sleep;
use std::time::Duration;

mod math;

pub const TIMEFORMAT: &str = "%_H:%M:%S";

pub fn do_stuff(
    sender: &mut Sender,
    start: &DateTime<Local>,
    end: &DateTime<Local>,
    verbose: bool,
) {
    if let Some(duration) = math::duration_until(&Local::now(), &start) {
        println!("wait till start");
        sleep(duration);
    }

    loop {
        let now = Local::now();
        let remaining_seconds = end.timestamp() - now.timestamp();
        let remaining_minutes = remaining_seconds / 60;
        if remaining_seconds <= 0 {
            break;
        }

        let position =
            math::calc_relative_position(start.timestamp(), end.timestamp(), now.timestamp());
        let hue = math::interpolate(80, 0, position);
        let height = math::interpolate(i64::from(MAX_HEIGHT), 0, position);

        if verbose {
            println!(
                "{} {:6} sec {:4} min {:2.2}% height: {:>2}",
                now.format(TIMEFORMAT),
                remaining_seconds,
                remaining_minutes,
                position * 100.0,
                height
            );
        }

        sender.send("hue", hue);
        sender.send("sat", "100");
        sender.send("height", height);
        sender.send("lit", "1");
        sender.send("on", "1");

        sleep_until_second(15);
    }

    if verbose {
        println!("{} end!", Local::now().format(TIMEFORMAT));
    }

    sender.send("height", 0);
    sender.send("lit", 0);
}

fn sleep_until_second(modulo: u32) {
    let now = Local::now();
    let remaining_nanoseconds = 1_000_000_000 - now.nanosecond();

    let current_second = now.second();
    let remaining_seconds = modulo - (current_second % modulo) - 1;

    sleep(Duration::new(
        u64::from(remaining_seconds),
        remaining_nanoseconds,
    ));
}
