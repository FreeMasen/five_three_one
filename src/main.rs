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
}

#[derive(Debug, Serialize, Clone)]
pub struct RenderedWeek {
    number: u32,
    name: &'static str,
    days: Vec<RenderedDay>,
}

#[derive(Debug, Serialize, Clone)]
pub enum DayName {
    Bench,
    OHP,
    Squat,
    Deads,    
}

impl std::fmt::Display for DayName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Bench => write!(f, "Bench"),
            Self::OHP => write!(f, "OHP"),
            Self::Squat => write!(f, "Squat"),
            Self::Deads => write!(f, "Deads"),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct RenderedDay {
    name: DayName,
    exercises: Vec<Weight>,
    reps: [u8; 3],
    supports: [RenderedSupport; 13],
}

#[derive(Debug, Serialize, Clone)]
struct Weight {
    value: String,
    side: String,
}

fn main() {
    pretty_env_logger::init();
    manual::run().unwrap();
}
