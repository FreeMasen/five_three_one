use askama::Template;
use serde::Serialize;
use std::fmt::Debug;

mod manual;

#[derive(Debug, Clone, Default, Serialize)]
pub struct RenderedSupports {
    included_weeks: Vec<u8>,
    bench: [RenderedSupport; 13],
    dead: [RenderedSupport; 13],
    ohp: [RenderedSupport; 13],
    squat: [RenderedSupport; 13],
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct RenderedSupport {
    name: String,
    reps: String,
}

#[derive(Debug, Serialize, Clone, Template)]
#[template(path = "plan.html")]
pub struct RenderedWeeks {
    weeks: Vec<RenderedWeek>,
    supports: RenderedSupports,
}

#[derive(Debug, Serialize, Clone)]
pub struct RenderedWeek {
    number: u32,
    name: &'static str,
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

fn main() {
    manual::run().unwrap();
}
