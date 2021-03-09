use std::{cmp::Ordering, collections::HashMap, convert::{TryFrom, TryInto}, fmt::Debug, fs::write, iter, ops::{Add, Deref, Mul}, path::PathBuf, str::FromStr};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

static DEFAULT_WEIGHTS: &str = include_str!("default_weights.toml");

#[derive(Debug, StructOpt)]
pub enum Args {
    /// Generate a 5/3/1 plan
    Generate(GenerateArgs),
    /// Calculate a one rep max from a weight and reps
    OneRep(OneRepArgs),
    /// Calculate all of the weights that can be provided
    /// by a set of plates, this is helpful since unique combinations
    /// of weights can be expensive to calculate
    WeightCombos(WeightComboArgs),
}
#[derive(Debug, StructOpt)]
pub struct GenerateArgs {
    /// Your known maximum 1 rep max squat
    #[structopt(short, long)]
    squat_max: f32,

    /// Your known maximum 1 rep max dead lift
    #[structopt(short, long)]
    dead_max: f32,

    /// Your known maximum 1 rep max bench press
    #[structopt(short, long)]
    bench_max: f32,

    /// Your known maximum 1 rep max overhead press
    #[structopt(short, long)]
    ohp_max: f32,

    /// How many months you'd like to generate
    #[structopt(short, long)]
    months: u32,

    /// A path to a .toml, .json or .yaml file including all of your plate sets
    /// This can be generated using the weight-combos command
    #[structopt(short, long)]
    weights_path: Option<PathBuf>,
    
    /// A path to a .toml, .json or .yaml file including all of the extra
    /// exercises you have planned for each workout, if not provided 4x45 1x35 1x25 2x10 1x5 1x2.5 is assumed
    #[structopt(short, long)]
    extra_path: Option<PathBuf>,

    /// If the weights provided are already set to 90% (good for generating after you've started)
    #[structopt(short, long)]
    ninety: bool,

    /// The path of the html file you'd like the plan saved to
    #[structopt(short, long)]
    file: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub struct OneRepArgs {
    #[structopt(short, long)]
    weight: f32,
    #[structopt(short, long)]
    reps: u8,
}
#[derive(Debug, StructOpt)]
pub struct WeightComboArgs {
    /// Weights you own, each -w flag should be formatted as
    /// <wt>[x<ct>] for example 45 would be 1 45 lb weight while
    /// 25x6 would be 6 25 lb weights
    #[structopt(short, long)]
    weights: Vec<WeightArg>,
    /// Optionally if you'd like to have the values printed to a file
    /// defaults to stdout
    #[structopt(short, long)]
    output: Option<PathBuf>,
    /// Format for printing, options include toml,json,yaml
    #[structopt(short, long)]
    format: Option<WeightsFormat>,
}

#[derive(Debug)]
pub enum WeightsFormat {
    Toml,
    Json,
    Yaml,
    Text,
}

impl FromStr for WeightsFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ret = match s.to_lowercase().as_str() {
            "toml" => Self::Toml,
            "json" => Self::Json,
            "yaml" => Self::Yaml,
            "text" => Self::Text,
            _ => return Err(format!("Unknown output format: {:?}", s)),
        };
        Ok(ret)
    }
}

#[derive(Debug)]
struct WeightArg {
    weight: f32,
    count: u8,
}

impl FromStr for WeightArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('x');
        let weight = parts
            .next()
            .ok_or_else(|| format!("Invalid weight expected at least one value {:?}", s))?;
        let parsed: f32 = weight
            .parse()
            .map_err(|_| format!("Weight must be a number: {:?}", s))?;
        let count: u8 = if let Some(count) = parts.next() {
            if count == "" {
                1
            } else {
                count
                    .parse()
                    .map_err(|_| format!("Count must be a number: {:?}", s))?
            }
        } else {
            1
        };
        Ok(Self {
            weight: parsed,
            count,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Week {
    number: u32,
    squat: [f32; 3],
    dead: [f32; 3],
    bench: [f32; 3],
    ohp: [f32; 3],
    reps: [u8; 3],
}

impl Week {
    fn new(number: u32,
        squat: [f32; 3],
        dead: [f32; 3],
        bench: [f32; 3],
        ohp: [f32; 3],
        reps: u8,) -> Self {
            Self {
                number,
                squat,
                dead,
                bench,
                ohp,
                reps: [reps; 3],
            }
    }

    fn new_week_three(number: u32,
        squat: [f32; 3],
        dead: [f32; 3],
        bench: [f32; 3],
        ohp: [f32; 3],) -> Self {
            Self {
                number,
                squat,
                dead,
                bench,
                ohp,
                reps: [5, 3, 1],
            }
        }
}



#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Supports {
    bench: Vec<Support>,
    dead: Vec<Support>,
    ohp: Vec<Support>,
    squat: Vec<Support>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Support {
    name: String
}

#[derive(Debug, Serialize, Clone)]
pub struct RenderedWeek {
    number: u32,
    squat: Vec<Weight>,
    dead: Vec<Weight>,
    bench: Vec<Weight>,
    ohp: Vec<Weight>,
    reps: [u8; 3],
}

#[derive(Debug, Serialize, Clone)]
struct Weight {
    value: String,
    side: String,
}

impl Week {
    pub fn as_rendered(&self, available: &HashMap<Float, Vec<f32>>) -> RenderedWeek {
        let squat = self.squat.iter().map(|f| {
            Weight {
                value: format!("{: >3}", f),
                side: format_side(f, &available)
            }
        }).collect();
        let dead = self.dead.iter().map(|f| Weight {
            value: format!("{: >3}", f),
            side: format_side(f, &available)
        }).collect();
        let bench = self.bench.iter().map(|f| Weight {
            value: format!("{: >3}", f),
            side: format_side(f, &available)
        }).collect();
        let ohp = self.ohp.iter().map(|f| Weight {
            value: format!("{: >3}", f),
            side: format_side(f, &available)
        }).collect();
        RenderedWeek {
            number: self.number,
            squat,
            dead,
            bench,
            ohp,
            reps: self.reps
        }

    }
}

const WEEK_ONE_PERCENTS: [f32; 3] = [0.65, 0.75, 0.85];
const WEEK_TWO_PERCENTS: [f32; 3] = [0.70, 0.80, 0.90];
const WEEK_THREE_PERCENTS: [f32; 3] = [0.75, 0.85, 0.95];
const WEEK_FOUR_PERCENTS: [f32; 3] = [0.5, 0.5, 0.5];
fn main() {
    let args: Args = Args::from_args();
    match args {
        Args::Generate(gen_args) => generate(gen_args),
        Args::OneRep(one_rep_args) => one_rep(one_rep_args),
        Args::WeightCombos(combos) => weight_combos(combos),
    }
}

fn weight_combos(combos: WeightComboArgs) {
    let mut map = HashMap::new();
    for combo in combos.weights {
        map.entry(combo.weight.into())
            .and_modify(|i| *i += combo.count)
            .or_insert(combo.count);
    }
    let results = calculate_all_weights_from(&map);
    let format = combos.format.unwrap_or(WeightsFormat::Toml);
    let mut keys: Vec<Float> = results.0.keys().copied().collect();
    keys.sort();
    let mut rekeyed = indexmap::IndexMap::new();
    for key in keys {
        rekeyed.insert(key.0.to_string(), results.0[&key].clone());
    }
    let text = match format {
        WeightsFormat::Toml => toml::to_string_pretty(&rekeyed).unwrap(),
        WeightsFormat::Json => serde_json::to_string_pretty(&rekeyed).unwrap(),
        WeightsFormat::Yaml => serde_yaml::to_string(&rekeyed).unwrap(),
        WeightsFormat::Text => {
            let mut s = String::new();
            for (key, value) in rekeyed {
                s.push_str(&format!("{: >3} {:?}\n", key, value));
            }
            s
        }
    };
    if let Some(path) = combos.output {
        std::fs::write(&path, text.as_bytes()).unwrap();
    } else {
        println!("{}", text);
    }
}

fn one_rep(args: OneRepArgs) {
    let OneRepArgs { weight, reps } = args;
    let rounded = round_weight((weight * reps as f32 * 0.0333) + weight);
    println!("{}", rounded);
}

static HTML: &str = include_str!("templates/plan.html");

fn generate(gen_args: GenerateArgs) {
    let mut squat_base = gen_args.squat_max;
    let mut dead_base = gen_args.dead_max;
    let mut bench_base = gen_args.bench_max;
    let mut ohp_base = gen_args.ohp_max;
    if !gen_args.ninety {
        squat_base = (gen_args.squat_max * 0.9).ceil();
        dead_base = (gen_args.dead_max * 0.9).ceil();
        bench_base = (gen_args.bench_max * 0.9).ceil();
        ohp_base = (gen_args.ohp_max * 0.9).ceil();
    }
    let mut weeks = Vec::with_capacity(gen_args.months as _);
    for _month in 0..gen_args.months {
        for week in 0..4 {
            let number = week + 1;
            match week {
                0 => weeks.push(Week::new(
                    number,
                    sets_from(squat_base, WEEK_ONE_PERCENTS),
                    sets_from(dead_base, WEEK_ONE_PERCENTS),
                    sets_from(bench_base, WEEK_ONE_PERCENTS),
                    sets_from(ohp_base, WEEK_ONE_PERCENTS),
                    5,
                )),
                1 => weeks.push(Week::new(
                    number,
                    sets_from(squat_base, WEEK_TWO_PERCENTS),
                    sets_from(dead_base, WEEK_TWO_PERCENTS),
                    sets_from(bench_base, WEEK_TWO_PERCENTS),
                    sets_from(ohp_base, WEEK_TWO_PERCENTS),
                    3,
                )),
                2 => weeks.push(Week::new_week_three(number, sets_from(squat_base, WEEK_THREE_PERCENTS), sets_from(dead_base, WEEK_THREE_PERCENTS), sets_from(bench_base, WEEK_THREE_PERCENTS), sets_from(ohp_base, WEEK_THREE_PERCENTS))),
                3 => weeks.push(Week::new(
                    number,
                    sets_from(squat_base, WEEK_FOUR_PERCENTS),
                    sets_from(dead_base, WEEK_FOUR_PERCENTS),
                    sets_from(bench_base, WEEK_FOUR_PERCENTS),
                    sets_from(ohp_base, WEEK_FOUR_PERCENTS),
                    5,
                )),
                _ => unreachable!(),
            }
        }
        squat_base += 10.0;
        dead_base += 10.0;
        bench_base += 5.0;
        ohp_base += 5.0;
    }
    let available_weights = read_weights(gen_args.weights_path);
    if let Some(html_path) = gen_args.file {
        let mut ctx = tera::Context::new();
        ctx.insert("weeks", &weeks.iter().map(|w| w.as_rendered(&available_weights.0)).collect::<Vec<_>>());
        ctx.insert("supports", &read_supports(gen_args.extra_path));
        let out = tera::Tera::one_off(HTML, &ctx, false).unwrap();
        write(&html_path, out).unwrap();
    } else {
        print_plan_to_terminal(&weeks, &available_weights.0)
    }
    
}

fn print_plan_to_terminal(weeks: &[Week], available_weights: &HashMap<Float, Vec<f32>>) {
    let mut week_strs = Vec::new();
    for week in weeks {
        let mut s = String::new();
        s.push_str("--------------------------\n");
        if week.number == 3 {
            s.push_str(&format!("Week {}: Reps 5/3/1\n", week.number));
        } else {
            s.push_str(&format!("Week {}: Reps {}\n", week.number, week.reps[0]));
        }
        s.push_str("--------------------------\n");
        s.push_str("Bench\n");
        for set in &week.bench {
            s.push_str(&format!(" {}{}\n", format!("{: >3}", set), format_side(set, &available_weights)));
        }
        s.push_str("Squats\n");
        for set in &week.squat {
            s.push_str(&format!(" {}{}\n", format!("{: >3}", set), format_side(set, &available_weights)));
        }
        s.push_str("OHP\n");
        for set in &week.ohp {
            s.push_str(&format!(" {}{}\n", format!("{: >3}", set), format_side(set, &available_weights)));
        }
        s.push_str("Deads\n");
        for set in &week.dead {
            s.push_str(&format!(" {}{}\n", format!("{: >3}", set), format_side(set, &available_weights)));
        }
        week_strs.push(s);
    }
    let longest_line = week_strs
        .iter()
        .flat_map(|l| l.lines().map(|l| l.len()))
        .max()
        .unwrap();
    for chunk in week_strs.chunks(4) {
        let week1 = &chunk[0];
        if let Some(week2) = chunk.get(1) {
            if let Some(week3) = chunk.get(2) {
                if let Some(week4) = chunk.get(3) {
                    for (lhs, ((lmid, rmid), rhs)) in week1
                        .lines()
                        .zip(week2.lines().zip(week3.lines()).zip(week4.lines()))
                    {
                        let padding_lhs = " ".repeat(longest_line.saturating_sub(lhs.len()));
                        let padding_lmid = " ".repeat(longest_line.saturating_sub(lmid.len()));
                        let padding_rmid = " ".repeat(longest_line.saturating_sub(rmid.len()));
                        println!(
                            "{}{}{}{}{}{}{}",
                            lhs, padding_lhs, lmid, padding_lmid, rmid, padding_rmid, rhs
                        );
                    }
                } else {
                    for (lhs, (mid, rhs)) in week1.lines().zip(week2.lines().zip(week3.lines())) {
                        let padding_lhs = " ".repeat(longest_line.saturating_sub(lhs.len()));
                        let padding_mid = " ".repeat(longest_line.saturating_sub(mid.len()));
                        println!("{}{}{}{}{}", lhs, padding_lhs, mid, padding_mid, rhs);
                    }
                }
            } else {
                for (lhs, rhs) in week1.lines().zip(week2.lines()) {
                    let padding = " ".repeat(longest_line.saturating_sub(lhs.len()));
                    println!("{}{}{}", lhs, padding, rhs);
                }
            }
        } else {
            println!("{}", week1);
        }
    }
}

fn format_side(weight: &f32, available_weights: &HashMap<Float, Vec<f32>>) -> String {
    let mut s = String::new();
    if let Some(plates) = available_weights.get(&Float(*weight)) {
        s.push('(');
        s.push_str(&plates.iter().map(|f| f.to_string()).join(" "));
        s.push(')');
    } else {
        s.push_str("(??)")
    }
    s
}

fn sets_from(base: f32, percents: [f32; 3]) -> [f32; 3] {
    let mut ret = [0f32; 3];
    for i in 0..3 {
        ret[i] = round_weight(percents[i] * base);
    }
    ret
}

fn round_weight(v: f32) -> f32 {
    let mut v = v.round();
    while v % 10.0 != 0.0 && v % 10.0 != 5.0 {
        v += 1.0;
    }
    v
}

fn read_weights(path: Option<PathBuf>) -> WeightsMap {
    if let Some(p) = &path {
        if let Ok(raw) = std::fs::read_to_string(p) {
            match p.extension().map(|s| s.to_str()) {
                Some(Some("toml")) => {
                    if let Ok(ser_map) = toml::from_str::<SeralizedWeightsMap>(&raw) {
                        ser_map.try_into().unwrap_or_else(|_| default_weights())
                    } else {
                        default_weights()
                    }
                }
                Some(Some("json")) => {
                    if let Ok(ser_map) = serde_json::from_str::<SeralizedWeightsMap>(&raw) {
                        ser_map.try_into().unwrap_or_else(|_| default_weights())
                    } else {
                        default_weights()
                    }
                }
                Some(Some("yaml")) => {
                    if let Ok(ser_map) = serde_yaml::from_str::<SeralizedWeightsMap>(&raw) {
                        ser_map.try_into().unwrap_or_else(|_| default_weights())
                    } else {
                        default_weights()
                    }
                }
                _ => default_weights(),
            }
        } else {
            default_weights()
        }
    } else {
        default_weights()
    }
}
fn read_supports(path: Option<PathBuf>) -> Supports {
    if let Some(p) = &path {
        if let Ok(raw) = std::fs::read_to_string(p) {
            match p.extension().map(|s| s.to_str()) {
                Some(Some("toml")) => {
                    if let Ok(ser_map) = toml::from_str::<Supports>(&raw) {
                        ser_map.try_into().unwrap_or_default()
                    } else {
                        Default::default()
                    }
                }
                Some(Some("json")) => {
                    if let Ok(ser_map) = serde_json::from_str::<Supports>(&raw) {
                        ser_map.try_into().unwrap_or_default()
                    } else {
                        Default::default()
                    }
                }
                Some(Some("yaml")) => {
                    if let Ok(ser_map) = serde_yaml::from_str::<Supports>(&raw) {
                        ser_map.try_into().unwrap_or_default()
                    } else {
                        Default::default()
                    }
                }
                _ => Default::default(),
            }
        } else {
            Default::default()
        }
    } else {
        Default::default()
    }
}

fn calculate_all_weights_from(available: &HashMap<Float, u8>) -> WeightsMap {
    let flattened: Vec<Float> = available
        .iter()
        .flat_map(|(weight, count)| iter::repeat_with(move || *weight).take(*count as _))
        .collect();
    weights_from_flattened_list(&flattened)
}

fn default_weights() -> WeightsMap {
    let swm = toml::from_str::<SeralizedWeightsMap>(&DEFAULT_WEIGHTS).expect("Invalid default weights toml");
    swm.try_into().expect("Invalid defaults weights")
}

const BAR: f32 = 45f32;
const SIDES: f32 = 2f32;
fn weights_from_flattened_list<'a>(plates: &[Float]) -> WeightsMap {
    let mut ret = HashMap::new();
    ret.insert(Float(BAR), Vec::new());
    for i in 0..plates.len() {
        for mut set in plates.iter().copied().permutations(i + 1) {
            set.sort_by(|l, r| r.cmp(l));
            let (total_weight, plates) = sum_and_side(&set);
            insert_or_swap_if_fewer(total_weight.into(), plates, &mut ret);
        }
    }
    WeightsMap(ret)
}

fn sum_and_side(plates: &[Float]) -> (Float, Vec<f32>) {
    let side: f32 = plates.iter().fold(0.0, |acc, f| acc + f.0);
    let plates: Vec<f32> = plates.iter().map(|f| f.0).collect();
    let total_weight = (side * SIDES) + BAR;
    (total_weight.into(), plates)
}

fn insert_or_swap_if_fewer(
    weight: Float,
    mut plates: Vec<f32>,
    current: &mut HashMap<Float, Vec<f32>>,
) {
    current
        .entry(weight)
        .and_modify(|current| {
            if current.len() > plates.len() {
                std::mem::swap(current, &mut plates);
            }
        })
        .or_insert(plates);
}
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct SeralizedWeightsMap(pub HashMap<String, Vec<f32>>);
pub struct WeightsMap(pub HashMap<Float, Vec<f32>>);

impl TryFrom<SeralizedWeightsMap> for WeightsMap {
    type Error = String;

    fn try_from(value: SeralizedWeightsMap) -> Result<Self, Self::Error> {
        let mut ret = HashMap::new();
        for (key, value) in value.0 {
            let key: Float = key.parse()?;
            ret.insert(key, value);
        }
        Ok(Self(ret))
    }
}

impl Into<SeralizedWeightsMap> for WeightsMap {
    fn into(self) -> SeralizedWeightsMap {
        let mut ret = HashMap::new();
        for (key, value) in self.0 {
            ret.insert(key.to_string(), value);
        }
        SeralizedWeightsMap(ret)
    }
}

#[derive(PartialOrd, Clone, Copy, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Float(pub f32);

impl FromStr for Float {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f: f32 = s.parse().map_err(|e| format!("{}", e))?;
        Ok(Self(f))
    }
}

impl From<f32> for Float {
    fn from(f: f32) -> Self {
        Self(f)
    }
}

impl Debug for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Into<f32> for Float {
    fn into(self) -> f32 {
        self.0
    }
}
impl Deref for Float {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => return std::cmp::Ordering::Equal,
            (false, true) => return std::cmp::Ordering::Less,
            (true, false) => return std::cmp::Ordering::Greater,
            _ => {}
        }
        match (self.0.is_infinite(), other.0.is_infinite()) {
            (true, true) => return std::cmp::Ordering::Equal,
            (false, true) => return std::cmp::Ordering::Greater,
            (true, false) => return std::cmp::Ordering::Less,
            _ => {}
        }
        let left: u64 = self.into();
        let right: u64 = other.into();
        left.partial_cmp(&right).unwrap()
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        let left: u64 = self.into();
        let right: u64 = other.into();
        matches!(left.cmp(&right), Ordering::Equal)
    }
}

impl Eq for Float {}

impl std::hash::Hash for Float {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.into());
    }
}

impl Into<u64> for &Float {
    fn into(self) -> u64 {
        (self.0 * 10.0).floor() as u64
    }
}

impl Add<Float> for &Float {
    type Output = Float;
    fn add(self, other: Float) -> Self::Output {
        Float(self.0.add(other.0))
    }
}
impl Add<f32> for Float {
    type Output = Float;
    fn add(self, other: f32) -> Self::Output {
        Float(self.0.add(other))
    }
}
impl Mul<f32> for &Float {
    type Output = Float;
    fn mul(self, other: f32) -> Self::Output {
        Float(self.0.mul(other))
    }
}
impl Mul<f32> for Float {
    type Output = Float;
    fn mul(self, other: f32) -> Self::Output {
        Float(self.0.mul(other))
    }
}
