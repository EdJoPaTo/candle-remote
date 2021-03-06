use crate::mqtt::Sender;
use crate::MAX_HEIGHT;
use std::thread::sleep;
use std::time::Duration;

const INCREASE_HEIGHT_DURATION: Duration = Duration::from_millis(100);
const MIN_HUE_SHIFT: f32 = 45.0;

fn get_next_hue(last: f32) -> f32 {
    let random_part = rand::random::<f32>() * (360.0 - (2.0 * MIN_HUE_SHIFT));
    let next = last + random_part + MIN_HUE_SHIFT;
    f32::round(next % 360.0)
}

pub fn do_stuff(sender: &mut Sender, burntime: u64, retain: bool) {
    let duration = Duration::from_millis(burntime);
    let mut hue = 0.0;

    if retain {
        sender.send("height-percentage", "", true);
        sender.send("height", "", true);
    }

    loop {
        hue = get_next_hue(hue);

        println!("new candle... hue: {}", hue);

        sender.send("lit", "0", retain);
        sender.send("hue", hue, retain);
        sender.send("sat", "100", retain);
        sender.send("on", "1", retain);
        sleep(duration);

        for height in 0..=MAX_HEIGHT {
            sender.send("height", height, false);
            sleep(INCREASE_HEIGHT_DURATION);
        }

        sleep(duration);
        sender.send("lit", 1, retain);
        sleep(duration);

        for height in 1..=MAX_HEIGHT {
            sender.send("height", MAX_HEIGHT - height, false);
            sleep(duration);
        }
    }
}
