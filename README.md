# Five Three One

A utility for generating 5/3/1 weightlifting plans

## Usage

```sh
five_three_one 0.1.0

USAGE:
    five_three_one <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    generate    Generate an html file with a formatted plan
    help        Prints this message or the help of the given subcommand(s)
    init        Initialize a configuration
    next        Update an existing configuration for the next month
```

```sh
five_three_one-generate 0.1.0
Generate an html file with a formatted plan

USAGE:
    five_three_one generate --input <input> --output <output>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>      Path to the config file
    -o, --output <output>    Path to the html output
```

```sh
five_three_one-init 0.1.0
Initialize a configuration

USAGE:
    five_three_one init [FLAGS] [OPTIONS] --bench-press <bench-press> --dead-lift <dead-lift> --overhead-press <overhead-press> --squat <squat>

FLAGS:
    -h, --help       Prints help information
    -n, --ninety     If the values are already at 90%
    -V, --version    Prints version information

OPTIONS:
    -b, --bench-press <bench-press>          Bench Press One Rep Max
    -d, --dead-lift <dead-lift>              Dead Lift One Rep Max
        --output <output>                    If provided, where to write the updated output. Defaults to stdout
    -o, --overhead-press <overhead-press>    Overhead Press One Rep Max
    -s, --squat <squat>                      Squat One Rep Max
```

```sh
five_three_one-next 0.1.0
Update an existing configuration for the next month

USAGE:
    five_three_one next [FLAGS] [OPTIONS] --input <input>

FLAGS:
    -c, --clear-supports    If the update should clear the supporting exercises
    -h, --help              Prints help information
    -V, --version           Prints version information

OPTIONS:
    -i, --input <input>      The current TOML file
    -o, --output <output>    If provided, where to write the updated output. Defaults to stdout
```