use crate::mqtt::Sender;
use crate::MAX_HEIGHT;
use std::thread::sleep;
use std::time::Duration;

const INCREASE_HEIGHT_DURATION: Duration = Duration::from_millis(100);

pub fn do_stuff(sender: &mut Sender, burntime: u64) {
    let duration = Duration::from_millis(burntime);

    loop {
        let hue = f32::round(rand::random::<f32>() * 360.0);

        println!("new candle... hue: {}", hue);

        sender.send("lit", "0");
        sender.send("hue", hue);
        sender.send("sat", "100");
        sender.send("on", "1");
        sleep(duration);

        for height in 0..=MAX_HEIGHT {
            sender.send("height", height);
            sleep(INCREASE_HEIGHT_DURATION);
        }

        sleep(duration);
        sender.send("lit", 1);
        sleep(duration);

        for height in 1..=MAX_HEIGHT {
            sender.send("height", MAX_HEIGHT - height);
            sleep(duration);
        }
    }
}
