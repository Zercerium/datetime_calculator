# datetime_calculator

simple cmd tool for calculate a date in the future or a duration.

# Installation

1. install Rust, see [here](https://www.rust-lang.org/tools/install)
2. clone the repo `git clone https://github.com/Zercerium/datetime_calculator.git`
3. `cd datetime_calculator`
4. install the project `cargo install --path .`

# Usage

- if you don't apply a sign the last applied sign will be used for all following durations; the default sign is `+`
  - `10d-20m60s` means +10 days, -20 minutes, -60 (**minus**) seconds

TODO

## examples

- `dtc today 20w`
- `dtc today -7d`
- `dtc 21.04.2024 20w`

# Development

## Roadmap

## Ideas

### higher prio

- [ ] possibility to specify output format
- [ ] allow different input formats
- [ ] use the same output format as default which was used for the input

### normal prio

- [ ] better error messages (for example if the time format parsing from time doesn't work, how to pass the value on to nom?)

### lower prio

- [ ] use proptest for testing
- [ ] test coverage badge in readme
- [ ] version badge in readme
- [ ] extract core functionality into lib
- [ ] allow spaces in duration (in the cmd the string has to be `"`-escaped in this case e.g. `"20m 10s"`)
