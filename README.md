# Five Three One

A utility for generating 531 weightlifting plans

## Usage

```
five_three_one 0.1.0

USAGE:
    five_three_one <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    generate         Generate a 5/3/1 plan
    help             Prints this message or the help of the given subcommand(s)
    one-rep          Calculate a one rep max from a weight and reps
    weight-combos    Calculate all of the weights that can be provided by a set of plates, this is helpful since
                     unique combinations of weights can be expensive to calculate
```

### `generate`

```
five_three_one-generate 0.1.0
Generate a 5/3/1 plan

USAGE:
    five_three_one generate [FLAGS] [OPTIONS] --bench-max <bench-max> --dead-max <dead-max> --months <months> --ohp-max <ohp-max> --squat-max <squat-max>

FLAGS:
    -h, --help       Prints help information
    -n, --ninety     If the weights provided are already set to 90% (good for generating after you've started)
    -V, --version    Prints version information

OPTIONS:
    -b, --bench-max <bench-max>          Your known maximum 1 rep max bench press
    -d, --dead-max <dead-max>            Your known maximum 1 rep max dead lift
    -e, --extra-path <extra-path>        A path to a .toml, .json or .yaml file including all of the extra exercises you
                                         have planned for each workout, if not provided 4x45 1x35 1x25 2x10 1x5 1x2.5 is
                                         assumed
    -f, --file <file>                    The path of the html file you'd like the plan saved to
    -m, --months <months>                How many months you'd like to generate
    -o, --ohp-max <ohp-max>              Your known maximum 1 rep max overhead press
    -s, --squat-max <squat-max>          Your known maximum 1 rep max squat
    -w, --weights-path <weights-path>    A path to a .toml, .json or .yaml file including all of your plate sets This
                                         can be generated using the weight-combos command
```

### `one-rep`

```sh
five_three_one-one-rep 0.1.0
Estimate a one rep max from a weight and reps

USAGE:
    five_three_one one-rep --reps <reps> --weight <weight>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --reps <reps>        
    -w, --weight <weight>    
```

### `weight-combos`

```
five_three_one-weight-combos 0.1.0
Calculate all of the weights that can be provided by a set of plates, this is helpful since unique combinations of
weights can be expensive to calculate

USAGE:
    five_three_one weight-combos [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --format <format>         Format for printing, options include toml,json,yaml
    -o, --output <output>         Optionally if you'd like to have the values printed to a file defaults to stdout
    -w, --weights <weights>...    Weights you own, each -w flag should be formatted as <wt>[x<ct>] for example 45 would
                                  be 1 45 lb weight while 25x6 would be 6 25 lb weights
```

## Output

### `generate`

There are 2 options for output when running `generate`

### plain text

This format crates an ascii text layout describing each of your workouts for a single week in a column.

Each weight will be indented slightly from the that day's focus and will include a list of plates needed
on each side of the bar to reach the target weight. 

```
$ five_three_one generate -b 125 -d 200 -s 215 -o 95 -m 1
--------------------------------------------------------------------------------------------------------
Week 1: Reps 5            Week 2: Reps 3            Week 3: Reps 5/3/1        Week 4: Reps 5
--------------------------------------------------------------------------------------------------------
Bench                     Bench                     Bench                     Bench
  75(10 5)                  80(10 5 2.5)              85(10 10)                 60(5 2.5)
  85(10 10)                 90(10 10 2.5)            100(25 2.5)                60(5 2.5)
 100(25 2.5)               105(25 5)                 110(25 5 2.5)              60(5 2.5)
Squats                    Squats                    Squats                    Squats
 130(35 5 2.5)             140(45 2.5)               150(45 5 2.5)             100(25 2.5)
 150(45 5 2.5)             155(45 10)                165(35 25)                100(25 2.5)
 165(35 25)                175(45 10 10)             185(45 25)                100(25 2.5)
OHP                       OHP                       OHP                       OHP
  60(5 2.5)                 60(5 2.5)                 65(10)                    45()
  65(10)                    70(10 2.5)                75(10 5)                  45()
  75(10 5)                  80(10 5 2.5)              85(10 10)                 45()
Deads                     Deads                     Deads                     Deads
 120(35 2.5)               130(35 5 2.5)             135(45)                    90(10 10 2.5)
 135(45)                   145(45 5)                 155(45 10)                 90(10 10 2.5)
 155(45 10)                165(35 25)                175(45 10 10)              90(10 10 2.5)

```

### html

If a `-f` flag is provided to `generate` it will create an HTML file with your
workout plan. This fill is designed to have 2 days per 8.5"x11" page.

[This page]() was generated with the following arguments.

```sh$
five_three_one generate -b 125 -d 200 -s 215 -o 95 -m 1 -f ./docs/index.html
```

This page will have each of your workouts in sequential order from top to bottom.
If you are interested in printing out the plan as a booklet, you can use the hotkey
`alt-o` (`option+o`) to re-order the pages so they will be in the right order if folded in half
and nested within one another. The book order is probably not ideal for plans longer
than 2 months, at this time.

#### Supporting Work

In the html output, you can assign supporting workout names to be included in your list for each day.

The `-e` argument can point to a `json`, `yaml`, or `toml` file that has the following format.


```json
{
    "included_weeks": [1,2,3,4],
    "bench": [
        {
            "name": "Incline Bench Press"
        }
    ],
    "squat": [
        {
            "name": "Bulgarian Split Squats"
        }
    ],
    "dead": [
        {
            "name": "Gliding Hamstring Curls"
        }
    ],
    "ohp": [
        {
            "name": "Chin-ups"
        }
    ]
}
```

- included_weeks: This defines which weeks to include the values on. This is useful if you're changing your support work on a different schedule than your main workouts
- bench: An array of objects with a `name` property which will be include on your bench press days
- squat: An array of objects with a `name` property which will be include on your squat days
- dead: An array of objects with a `name` property which will be include on your dead lift days
- ohp: An array of objects with a `name` property which will be include on your over head press days

### Your weights

By default the program assumes you have 4 45lb plates, 1 35lb plate, 1 25lb plate, 2 10lb plates, 1 5lb plate and 1 2.5lb plate. With this setup you can reach every  weight possible at a 2.5lb interval from 45lbs to 580lb

If you have a different setup, you can provide the `-w` argument to `generate` which should point to a weights file.
This file can also be in the `json`, `yaml`, or `toml` format. This file should contain an object where the keys
are the total weight and the values are an array of plate increments that would need to be on each side of the bar
to achieve the total weight. [checkout default_weights.toml](./src/default_weights.toml) to get a better idea.

The easiest way to create one of these files is to use the `weight-combos` sub-command. As an example, if your home
setup only had 1 45lb weight instead of 4 your could generate a file with the following

```sh
five_three_one weight-combos -w45 -w35 -w25 -w10x2 -w5 -w2.5 -f toml -o ./weights.toml
```

## Calculating your one rep max

It can be difficult, especially if you are working alone, to figure out what your true
one rep max is. There is a mostly accurate way to estimate it by reaching near exhaustion
on a weight with a higher rep count. Ideally this would be between 3 and 5 reps as the
accuracy decreases the higher the reps.

The equation for this calculation is `(<weight> * <reps> * 0.0333) + <weight>)`.

Doing this manually can be a bit of a chore since it doesn't account for normal spacing
on the availability of plates. For example: `(130 * 3 * 0.333) + 130 = 142.987`

This program will do the math for you and also round to an appropriate 5lb increment.

```sh
five_three_one one-rep -w 130 -r 3
145
```
