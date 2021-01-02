# Candle Remote

Made to work with the [esp-mqtt-neomatrix-candle](https://github.com/EdJoPaTo/esp-mqtt-neomatrix-candle)

## Usage

### Demoloop

```plaintext
candle-demoloop
Show an endless loop to demonstrate the candle actively

USAGE:
    candle demoloop [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --base-topic <STRING>    MQTT Root Topic of the candle matrix to publish to [default: espMatrixCandle]
    -p, --port <INT>             Port on which the MQTT Broker is running [default: 1883]
    -h, --host <HOST>            Host on which the MQTT Broker is running [default: localhost]
    -b, --burntime <INT>         Time it takes for each layer to burn down (milliseconds) [default: 1000]
```

### Meeting

```plaintext
candle-meeting
Show a candle burning down until the end of a meeting

USAGE:
    candle meeting [FLAGS] [OPTIONS] <STARTTIME> <ENDTIME>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Show each time tick on stdout

OPTIONS:
    -t, --base-topic <STRING>    MQTT Root Topic of the candle matrix to publish to [default: espMatrixCandle]
    -p, --port <INT>             Port on which the MQTT Broker is running [default: 1883]
    -h, --host <HOST>            Host on which the MQTT Broker is running [default: localhost]

ARGS:
    <STARTTIME>    Start time of the Meeting. From then the remaining time is published.
    <ENDTIME>      End time of the Meeting. Until then the remaining time is published.
```
