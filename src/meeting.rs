use crate::mqtt::Sender;
use crate::MAX_HEIGHT;
use chrono::{DateTime, Local, Timelike};
use std::thread::sleep;
use std::time::{Duration, Instant};

mod math;

pub const TIMEFORMAT: &str = "%_H:%M:%S";
const END_BLINK_DURATION_SECONDS: u64 = 60;
const END_BLINK_INTERVAL: Duration = Duration::from_millis(750);

pub fn do_stuff(
    sender: &mut Sender,
    start: &DateTime<Local>,
    end: &DateTime<Local>,
    end_blink: bool,
    retain: bool,
    verbose: bool,
) {
    if let Some(duration) = math::duration_until(&Local::now(), &start) {
        println!("wait till start");
        sleep(duration);
    }

    if retain {
        sender.send("height-percentage", "", true);
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

        sender.send("hue", hue, retain);
        sender.send("sat", "100", retain);
        sender.send("height", height, retain);
        sender.send("lit", "1", retain);
        sender.send("on", "1", retain);

        sleep_until_second(15);
    }

    if verbose {
        println!("{} end!", Local::now().format(TIMEFORMAT));
    }

    sender.send("height", 0, retain);
    sender.send("lit", 0, retain);
    sender.send("hue", "0", retain);
    sender.send("sat", "100", retain);

    if end_blink {
        let start = Instant::now();
        loop {
            let since_start = Instant::now().duration_since(start);
            if since_start.as_secs() > END_BLINK_DURATION_SECONDS {
                break;
            }

            #[allow(clippy::cast_precision_loss)]
            let bri = math::interpolate(
                5,
                255,
                since_start.as_secs_f64() / (END_BLINK_DURATION_SECONDS as f64),
            );

            if verbose {
                println!(
                    "{:5.1} / {:3} sec -> bri {:3}",
                    since_start.as_secs_f64(),
                    END_BLINK_DURATION_SECONDS,
                    bri
                );
            }

            sender.send("bri", bri, false);

            sender.send("height", MAX_HEIGHT, false);
            sleep(END_BLINK_INTERVAL);
            sender.send("height", 0, retain);
            sleep(END_BLINK_INTERVAL);
        }

        sender.send("bri", "5", false);
    }
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
