use chrono::Local;

mod cli;
mod demoloop;
mod meeting;
mod mqtt;

const MAX_HEIGHT: u8 = 32 - 5;

fn main() {
    let matches = cli::build().get_matches();

    let retain = matches.is_present("retain");

    let mut sender = {
        let host = matches
            .value_of("MQTT Server")
            .expect("MQTT Host could not be read from command line");

        let port = matches
            .value_of("MQTT Port")
            .and_then(|s| s.parse::<u16>().ok())
            .expect("MQTT Port could not be read from command line");

        let base_topic = matches
            .value_of("MQTT Base Topic")
            .expect("MQTT Base Topic could not be read from command line");

        mqtt::Sender::new(host, port, base_topic)
    };

    if let Some(matches) = matches.subcommand_matches("brightness") {
        let brightness = matches
            .value_of("brightness")
            .and_then(|o| o.parse::<u8>().ok())
            .expect("brightness could not be read from the command line");

        sender.send("bri", brightness, retain);
    } else if let Some(matches) = matches.subcommand_matches("demoloop") {
        let burntime = matches
            .value_of("burntime")
            .and_then(|s| s.parse::<u64>().ok())
            .expect("Burntime could not be read from command line");

        demoloop::do_stuff(&mut sender, burntime, retain);
    } else if let Some(matches) = matches.subcommand_matches("meeting") {
        let verbose = matches.is_present("verbose");

        let start = matches
            .value_of("starttime")
            .and_then(cli::time_string_to_date_time)
            .expect("starttime could not be read from the command line");

        let mut end = matches
            .value_of("endtime")
            .and_then(cli::time_string_to_date_time)
            .expect("endtime could not be read from the command line");

        let now = Local::now();

        if end.timestamp() - start.timestamp() <= 0 || end.timestamp() - now.timestamp() <= 0 {
            end = end
                .checked_add_signed(chrono::Duration::days(1))
                .expect("failed to assume end date tomorrow");
        }

        println!("Now:   {}", now.to_string());
        println!("Start: {}", start.to_string());
        println!("End:   {}", end.to_string());

        meeting::do_stuff(&mut sender, &start, &end, retain, verbose);
    } else {
        unimplemented!("Subcommand not implemented");
    }
}
