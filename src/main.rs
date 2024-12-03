use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use structopt::StructOpt;

mod day1;
mod day2;
mod day3;

#[derive(Clone, Debug, StructOpt)]
struct RunConfig {
    #[structopt(long, short)]
    pub day: Option<u32>,
    #[structopt(long, short)]
    pub variant: Option<String>,
    #[structopt(long, short)]
    /// The file path of the input to use.
    pub input: Option<String>,
}

pub struct RunContext<'a> {
    pub input: &'a str,
    begin_timestamp: Option<Instant>,
    parsed_timestamp: Option<Instant>,
    complete_timestamp: Option<Instant>,
}

impl RunContext<'_> {
    pub fn mark_parse_complete(&mut self) {
        self.parsed_timestamp = Some(Instant::now());
    }
}

pub struct RunnerRepository {
    current_day: u32,
    days: HashMap<u32, HashMap<String, VariantRunner>>,
}

impl RunnerRepository {
    pub fn new() -> Self {
        Self {
            current_day: 0,
            days: Default::default(),
        }
    }
}

impl RunnerRepository {
    pub fn merge_day(&mut self, day: u32, register: fn(&mut RunnerRepository)) {
        self.current_day = day;
        register(self)
    }

    pub fn add_variant(&mut self, name: &'static str, runner: VariantRunner) {
        let variants = self
            .days
            .entry(self.current_day)
            .or_insert_with(|| Default::default());
        variants.insert(name.into(), runner);
    }
}

pub type VariantRunner = fn(&mut RunContext) -> eyre::Result<()>;

fn make_repo() -> RunnerRepository {
    let mut repo = RunnerRepository::new();
    repo.merge_day(1, day1::add_variants);
    repo.merge_day(2, day2::add_variants);
    repo.merge_day(3, day3::add_variants);
    repo
}

fn main() -> eyre::Result<()> {
    let config = RunConfig::from_args_safe()?;
    let repo = make_repo();

    let day = match config.day {
        Some(day) => day,
        None => repo.days.keys().max().copied().unwrap(),
    };

    if !repo.days.contains_key(&day) {
        eyre::bail!("day {day} does not exist");
    }

    let variants = &repo.days[&day];
    let part_name = match &config.variant {
        Some(name) => &*name,
        None => {
            if variants.contains_key("part2") {
                "part2"
            } else if variants.contains_key("part1") {
                "part1"
            } else {
                eyre::bail!("day {day} has no 'part1' or 'part2' specified")
            }
        }
    };

    let Some(part) = variants.get(part_name) else {
        eyre::bail!("day{day}/{} was not found", part_name);
    };

    let file_path = config
        .input
        .unwrap_or_else(|| format!("inputs/day{day}.txt"));
    let input = std::fs::read_to_string(file_path)?;

    println!("running day{day}/{}", part_name);
    if let Err(_) = std::panic::catch_unwind(|| {
        let mut ctx = RunContext {
            input: &input,
            begin_timestamp: None,
            parsed_timestamp: None,
            complete_timestamp: None,
        };

        ctx.begin_timestamp = Some(Instant::now());
        let res = part(&mut ctx);
        ctx.complete_timestamp = Some(Instant::now());

        if let Err(err) = res {
            println!("\x1b[31mpart returned error:\x1b[0m {err:?}");
        }

        let elapsed = ctx
            .complete_timestamp
            .unwrap()
            .duration_since(ctx.begin_timestamp.unwrap());
        println!("finished in {}", DisplayDuration(elapsed));
    }) {
        println!("\x1b[31m=== part panicked! ===\x1b[0m");
    }

    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct DisplayDuration(pub Duration);

impl std::fmt::Display for DisplayDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dur = self.0;
        if dur.as_nanos() < 1000 {
            write!(f, "{} ns", dur.as_nanos())?;
        } else if dur.as_micros() < 1000 {
            write!(f, "{}.{:0>3} µs", dur.as_micros(), dur.as_nanos() % 1000)?;
        } else if dur.as_millis() < 1000 {
            write!(f, "{}.{:0>3} ms", dur.as_millis(), dur.as_micros() % 1000)?;
        } else {
            write!(f, "{}.{:0>3} s", dur.as_secs(), dur.as_millis() % 1000)?;
        }
        Ok(())
    }
}
