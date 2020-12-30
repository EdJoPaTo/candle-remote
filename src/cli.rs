use chrono::{DateTime, Local, NaiveTime};
use clap::{App, AppSettings, Arg, SubCommand};

pub fn build() -> App<'static, 'static> {
    App::new("Candle Remote")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .global_setting(AppSettings::ColoredHelp)
        .setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::with_name("MQTT Server")
                .short("h")
                .long("host")
                .global(true)
                .value_name("HOST")
                .takes_value(true)
                .help("Host on which the MQTT Broker is running")
                .default_value("localhost"),
        )
        .arg(
            Arg::with_name("MQTT Port")
                .short("p")
                .long("port")
                .global(true)
                .value_name("INT")
                .takes_value(true)
                .help("Port on which the MQTT Broker is running")
                .default_value("1883"),
        )
        .arg(
            Arg::with_name("MQTT Base Topic")
                .short("t")
                .long("base-topic")
                .global(true)
                .value_name("STRING")
                .takes_value(true)
                .help("MQTT Root Topic of the candle matrix to publish to")
                .default_value("espMatrixCandle"),
        )
        .subcommand(
            SubCommand::with_name("meeting")
                .visible_aliases(&["m"])
                .about("Show a candle burning down until the end of a meeting")
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("Show each time tick on stdout"),
                )
                .arg(
                    Arg::with_name("starttime")
                        .required(true)
                        .value_name("STARTTIME")
                        .help(
                            "Start time of the Meeting. From then the remaining time is published.",
                        ),
                )
                .arg(
                    Arg::with_name("endtime")
                        .required(true)
                        .value_name("ENDTIME")
                        .help(
                            "End time of the Meeting. Until then the remaining time is published.",
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("demoloop")
                .visible_aliases(&["demo", "loop", "d", "l"])
                .about("Show an endless loop to demonstrate the candle actively")
                .arg(
                    Arg::with_name("burntime")
                        .short("b")
                        .long("burntime")
                        .value_name("INT")
                        .takes_value(true)
                        .help("Time it takes for each layer to burn down (milliseconds)")
                        .default_value("1000"),
                ),
        )
}

pub fn time_string_to_date_time(timestring: &str) -> Option<DateTime<Local>> {
    let today = chrono::offset::Local::now().date();
    let fmt = if timestring.len() > 5 {
        "%H:%M:%S"
    } else {
        "%H:%M"
    };
    NaiveTime::parse_from_str(timestring, fmt)
        .ok()
        .and_then(|t| today.and_time(t))
}
