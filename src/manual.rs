use askama::Template;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use structopt::StructOpt;

use crate::{
    index::{run_index, IndexArgs},
    DayName, RenderedDay, RenderedSupport, RenderedSupports, RenderedWeek, RenderedWeeks, Weight,
};

#[derive(Debug, Clone, StructOpt)]
enum Args {
    /// Initialize a configuration
    Init(InitArgs),
    /// Update an existing configuration for the next month
    Next(NextArgs),
    /// Generate an html file with a formatted plan
    Generate(GenerateArgs),
    /// Generate an Index.html file
    Index(IndexArgs),
}

#[derive(Debug, Clone, StructOpt)]
struct InitArgs {
    #[structopt(short, long)]
    /// Squat One Rep Max
    squat: f32,
    #[structopt(short, long)]
    /// Dead Lift One Rep Max
    dead_lift: f32,
    #[structopt(short, long)]
    /// Bench Press One Rep Max
    bench_press: f32,
    #[structopt(short, long)]
    /// Overhead Press One Rep Max
    overhead_press: f32,
    #[structopt(short, long)]
    /// If the values are already at 90%
    ninety: bool,
    #[structopt(long)]
    /// If provided, where to write the updated output. Defaults to stdout
    output: Option<PathBuf>,
}

#[derive(Debug, Clone, StructOpt)]
struct NextArgs {
    #[structopt(short, long)]
    /// The current TOML file
    input: PathBuf,
    #[structopt(short, long)]
    /// If provided, where to write the updated output. Defaults to stdout
    output: Option<PathBuf>,
    #[structopt(short, long)]
    /// If the update should clear the supporting exercises
    clear_supports: bool,
}

#[derive(Debug, Clone, StructOpt)]
struct GenerateArgs {
    #[structopt(short, long)]
    /// Path to the config file
    input: PathBuf,
    #[structopt(short, long)]
    /// Path to the html output
    output: PathBuf,
}

#[derive(Serialize, Deserialize, Default)]
struct ConfigFile {
    squat: [[f32; 3]; 4],
    dead_lift: [[f32; 3]; 4],
    bench_press: [[f32; 3]; 4],
    overhead_press: [[f32; 3]; 4],
    supports: Option<Supports>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Supports {
    included_weeks: Vec<u8>,
    bench_press: Vec<Support>,
    dead_lift: Vec<Support>,
    overhead_press: Vec<Support>,
    squat: Vec<Support>,
}
impl Default for Supports {
    fn default() -> Self {
        Self {
            included_weeks: vec![1, 2, 3, 4],
            bench_press: Vec::new(),
            dead_lift: Vec::new(),
            overhead_press: Vec::new(),
            squat: Vec::new(),
        }
    }
}

impl Supports {
    fn dead_lift_for(&self, week: u8) -> impl Iterator<Item = Support> {
        if !self.included_weeks.contains(&week) {
            return Vec::new().into_iter();
        }
        self.dead_lift.clone().into_iter()
    }
    fn squat_for(&self, week: u8) -> impl Iterator<Item = Support> {
        if !self.included_weeks.contains(&week) {
            return Vec::new().into_iter();
        }
        self.squat.clone().into_iter()
    }
    fn overhead_press_for(&self, week: u8) -> impl Iterator<Item = Support> {
        if !self.included_weeks.contains(&week) {
            return Vec::new().into_iter();
        }
        self.overhead_press.clone().into_iter()
    }
    fn bench_press_for(&self, week: u8) -> impl Iterator<Item = Support> {
        if !self.included_weeks.contains(&week) {
            return Vec::new().into_iter();
        }
        self.bench_press.clone().into_iter()
    }
}

fn render_supports(iter: impl Iterator<Item = Support>) -> [RenderedSupport; 13] {
    let mut ret = [
        RenderedSupport::default(),
        RenderedSupport::default(),
        RenderedSupport::default(),
        RenderedSupport::default(),
        RenderedSupport::default(),
        RenderedSupport::default(),
        RenderedSupport::default(),
        RenderedSupport::default(),
        RenderedSupport::default(),
        RenderedSupport::default(),
        RenderedSupport::default(),
        RenderedSupport::default(),
        RenderedSupport::default(),
    ];
    for (i, v) in iter.enumerate() {
        ret[i] = RenderedSupport {
            name: v.name,
            reps: v.reps.as_ref().map(ToString::to_string).unwrap_or_default(),
        };
    }
    ret
}

impl Into<RenderedSupports> for Supports {
    fn into(self) -> RenderedSupports {
        let mut ret = RenderedSupports::default();
        for (i, supp) in self.bench_press.into_iter().enumerate() {
            ret.bench[i] = supp.into();
        }
        for (i, supp) in self.overhead_press.into_iter().enumerate() {
            ret.ohp[i] = supp.into();
        }
        for (i, supp) in self.squat.into_iter().enumerate() {
            ret.squat[i] = supp.into();
        }
        for (i, supp) in self.dead_lift.into_iter().enumerate() {
            ret.dead[i] = supp.into();
        }
        ret.included_weeks = self.included_weeks;
        ret
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Support {
    name: String,
    #[serde(default)]
    reps: Option<u8>,
}

impl Support {
    fn placeholder() -> Self {
        Self {
            name: "Placeholder".to_string(),
            reps: None,
        }
    }
}

impl Into<RenderedSupport> for Support {
    fn into(self) -> RenderedSupport {
        RenderedSupport {
            name: self.name,
            reps: self
                .reps
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_default(),
        }
    }
}

impl Into<RenderedWeeks> for ConfigFile {
    fn into(self) -> RenderedWeeks {
        static WEEK_ONE_NAME: &str = "First Week";
        static WEEK_TWO_NAME: &str = "Second Week";
        static WEEK_THREE_NAME: &str = "Max Week";
        static WEEK_FOUR_NAME: &str = "De-load Week";
        static WEEK_NAMES: &[&str] = &[
            WEEK_ONE_NAME,
            WEEK_TWO_NAME,
            WEEK_THREE_NAME,
            WEEK_FOUR_NAME,
        ];
        let mut weeks = Vec::new();
        let supports = self.supports.unwrap_or_default();

        for i in 0..4 {
            let week = RenderedWeek {
                days: vec![
                    RenderedDay {
                        name: DayName::Deads,
                        exercises: self.dead_lift[i]
                            .iter()
                            .copied()
                            .map(|f| f.into())
                            .collect(),
                        supports: render_supports(supports.dead_lift_for((i + 1) as u8)),
                        reps: reps_for_week(i),
                    },
                    RenderedDay {
                        name: DayName::Squat,
                        exercises: self.squat[i].iter().copied().map(|f| f.into()).collect(),
                        supports: render_supports(supports.squat_for((i + 1) as u8)),
                        reps: reps_for_week(i),
                    },
                    RenderedDay {
                        name: DayName::Bench,
                        exercises: self.bench_press[i]
                            .iter()
                            .copied()
                            .map(|f| f.into())
                            .collect(),
                        supports: render_supports(supports.bench_press_for((i + 1) as u8)),
                        reps: reps_for_week(i),
                    },
                    RenderedDay {
                        name: DayName::OHP,
                        exercises: self.overhead_press[i]
                            .iter()
                            .copied()
                            .map(|f| f.into())
                            .collect(),
                        supports: render_supports(supports.overhead_press_for((i + 1) as u8)),
                        reps: reps_for_week(i),
                    },
                ],
                number: i as _,
                name: WEEK_NAMES[i],
            };
            weeks.push(week);
        }
        RenderedWeeks { weeks }
    }
}

type R<T> = Result<T, Box<dyn std::error::Error>>;

pub fn run() -> R<()> {
    let args = Args::from_args();
    match args {
        Args::Init(args) => run_init(args),
        Args::Next(args) => run_next(args),
        Args::Generate(args) => run_generate(args),
        Args::Index(args) => run_index(args),
    }
}

fn run_next(args: NextArgs) -> R<()> {
    let bytes = std::fs::read(&args.input)?;
    let mut config: ConfigFile = toml::from_slice(bytes.as_slice())?;
    if args.clear_supports {
        if let Some(supports) = config.supports.as_mut() {
            supports.bench_press.clear();
            supports.bench_press.push(Support::placeholder());
            supports.overhead_press.clear();
            supports.overhead_press.push(Support::placeholder());
            supports.squat.clear();
            supports.squat.push(Support::placeholder());
            supports.dead_lift.clear();
            supports.dead_lift.push(Support::placeholder());
        }
    }
    for item in config.bench_press.iter_mut() {
        for wt in item {
            *wt += 5.0;
        }
    }
    for item in config.overhead_press.iter_mut() {
        for wt in item {
            *wt += 5.0;
        }
    }
    for item in config.squat.iter_mut() {
        for wt in item {
            *wt += 10.0;
        }
    }
    for item in config.dead_lift.iter_mut() {
        for wt in item {
            *wt += 10.0;
        }
    }
    let text = toml::to_string_pretty(&config)?;
    if let Some(output) = args.output.as_ref() {
        std::fs::write(output, text.as_bytes())?;
    } else {
        println!("{}", text);
    }
    Ok(())
}

fn run_init(args: InitArgs) -> R<()> {
    let mut config = ConfigFile::default();
    log::debug!("Bench");
    config.bench_press = max_to_sets(args.bench_press, args.ninety);
    log::debug!("OHP");
    config.overhead_press = max_to_sets(args.overhead_press, args.ninety);
    log::debug!("Squat");
    config.squat = max_to_sets(args.squat, args.ninety);
    log::debug!("Dead");
    config.dead_lift = max_to_sets(args.dead_lift, args.ninety);
    config.supports = Some(Supports::default());
    let text = toml::to_string_pretty(&config)?;
    if let Some(output) = args.output.as_ref() {
        std::fs::write(output, text.as_bytes())?;
    } else {
        println!("{}", text);
    }
    Ok(())
}

fn run_generate(args: GenerateArgs) -> R<()> {
    let bytes = std::fs::read(&args.input)?;
    let config: ConfigFile = toml::from_slice(bytes.as_slice())?;
    let ctx: RenderedWeeks = config.into();
    std::fs::write(&args.output, ctx.render()?)?;
    Ok(())
}

fn max_to_sets(max: f32, ninety: bool) -> [[f32; 3]; 4] {
    const WEEK_ONE_PERCENTS: [f32; 3] = [0.65, 0.75, 0.85];
    const WEEK_TWO_PERCENTS: [f32; 3] = [0.70, 0.80, 0.90];
    const WEEK_THREE_PERCENTS: [f32; 3] = [0.75, 0.85, 0.95];
    const WEEK_FOUR_PERCENTS: [f32; 3] = [0.5, 0.5, 0.5];
    let base = if ninety { max } else { max * 0.9 };
    log::debug!("base: {}", base);
    let mut ret = [[0f32; 3], [0f32; 3], [0f32; 3], [0f32; 3]];
    log::debug!("week 1");
    for (i, &mul) in WEEK_ONE_PERCENTS.iter().enumerate() {
        let val = mul_fixed(base, mul);
        log::debug!("Set {}: {}", i, val);
        ret[0][i] = val;
    }
    log::debug!("week 2");
    for (i, &mul) in WEEK_TWO_PERCENTS.iter().enumerate() {
        let val = mul_fixed(base, mul);
        log::debug!("Set {}: {}", i, val);
        ret[1][i] = mul_fixed(base, mul);
    }
    log::debug!("week 3");
    for (i, &mul) in WEEK_THREE_PERCENTS.iter().enumerate() {
        let val = mul_fixed(base, mul);
        log::debug!("Set {}: {}", i, val);
        ret[2][i] = mul_fixed(base, mul);
    }
    log::debug!("week 4");
    for (i, &mul) in WEEK_FOUR_PERCENTS.iter().enumerate() {
        let val = mul_fixed(base, mul);
        log::debug!("Set {}: {}", i, val);
        ret[3][i] = mul_fixed(base, mul);
    }
    ret
}

fn mul_fixed(base: f32, mul: f32) -> f32 {
    log::trace!("mul_fixed {}, {}", base, mul);
    let mut ret = (base * mul).round();
    log::trace!("init: {}", ret);
    while ret % 5.0 != 0.0 {
        ret += 1.0;
    }
    log::trace!("ret: {}", ret);
    ret
}

impl From<f32> for Weight {
    fn from(raw: f32) -> Self {
        let value = format!("{: >3}", raw);
        let side = calculate_side(raw);
        let formatted_side = format_side(&side);
        Weight {
            value,
            side: formatted_side,
        }
    }
}

fn format_side(side: &[f32]) -> String {
    let mut s = String::from("(");
    for (i, f) in side.iter().enumerate() {
        if i > 0 {
            s.push(' ');
        }
        s.push_str(&f.to_string());
    }
    s.push(')');
    s
}

fn calculate_side(mut weight: f32) -> Vec<f32> {
    if weight <= 45.0 {
        return vec![];
    }
    weight -= 45.0;
    weight = weight / 2.0;
    let mut ret = Vec::new();
    while weight > 0.0 {
        if weight >= 45.0 {
            ret.push(45.0);
            weight -= 45.0;
        } else if weight >= 35.0 {
            ret.push(35.0);
            weight -= 35.0;
        } else if weight >= 25.0 {
            ret.push(25.0);
            weight -= 25.0;
        } else if weight >= 10.0 {
            ret.push(10.0);
            weight -= 10.0;
        } else if weight >= 5.0 {
            ret.push(5.0);
            weight -= 5.0;
        } else if weight >= 2.5 {
            ret.push(2.5);
            weight -= 2.5;
        }
    }
    ret
}

fn reps_for_week(week: usize) -> [u8; 3] {
    match week {
        0 | 3 => [5, 5, 5],
        1 => [3, 3, 3],
        2 => [5, 3, 1],
        _ => unreachable!(),
    }
}
