# timelog

logs `$PWD` to influxdb every time your bash prompt is displayed.

## requirements

- influxdb
- curl

## usage

#### customize:

tweak hard-coded values like host, database and measurement name in `src/main.rs`.

#### build:

`$ cargo build --release`

#### "install":

`$ cp target/release/timelog ~/.cargo/bin`

#### add trigger in `.bashrc`:

`export PROMPT_COMMAND=$HOME/.cargo/bin/timelog`

## troubleshooting

run with "debug" feature enabled: `$ cargo run --features debug`

## why not use a bash script?

because it was fun to tinker around with this.
