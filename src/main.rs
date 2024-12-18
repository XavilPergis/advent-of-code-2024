#![feature(portable_simd)]

use std::{
    collections::HashMap,
    fmt::Display,
    path::PathBuf,
    time::{Duration, Instant},
};

use reqwest::header::{HeaderMap, HeaderValue};
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
struct RunConfig {
    #[structopt(long, short)]
    /// The file path of the input to use.
    pub input: Option<String>,
    #[structopt(long, short = "s", default_value = "1")]
    pub sample_count: usize,
    #[structopt(long, short, default_value = "60")]
    pub rerun_time_limit_s: f64,
    #[structopt(subcommand)]
    pub subcommand: RunCommand,
}

#[derive(Clone, Debug, StructOpt)]
enum RunCommand {
    Run { variant: Option<String> },
    Compare { variant1: String, variant2: String },
    List,
}

pub struct RunContext<'a> {
    pub input: &'a str,
    pub input_scratch: &'a mut [u8],
    write_output: bool,
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
    days: HashMap<u32, HashMap<String, Box<dyn Fn(&mut RunContext)>>>,
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

    pub fn add_variant<T, F>(&mut self, name: &'static str, runner: F)
    where
        F: Fn(&mut RunContext) -> eyre::Result<T> + 'static,
        T: Display,
    {
        let variants = self
            .days
            .entry(self.current_day)
            .or_insert_with(|| Default::default());
        variants.insert(
            name.into(),
            Box::new(move |ctx| {
                ctx.begin_timestamp = Some(Instant::now());
                let res = runner(ctx);
                ctx.complete_timestamp = Some(Instant::now());
                if ctx.write_output {
                    match res {
                        Ok(value) => println!("{value}"),
                        Err(err) => println!("\x1b[31merror\x1b[0m:\n{err:?}"),
                    }
                }
            }),
        );
    }
}


pub mod bitset;

// i fucking hate that macros are sensitive to declaration order. why is it like this.
macro_rules! as_display {
    ($($arg:tt)*) => {{
        $crate::WrapAsDisplay(move |f: &mut std::fmt::Formatter<'_>| write!(f, $($arg)*))
    }};
}

mod days;

fn run_variant(
    repo: &RunnerRepository,
    config: &RunConfig,
    day: u32,
    variant: &str,
) -> eyre::Result<Vec<Sample>> {
    if !repo.days.contains_key(&day) {
        eyre::bail!("day {day} does not exist");
    }

    let variants = &repo.days[&day];
    let Some(part) = variants.get(variant) else {
        eyre::bail!("day{day}/{} was not found", variant);
    };

    let file_path = config
        .input
        .clone()
        .unwrap_or_else(|| format!("inputs/day{day}.txt"));

    if !std::fs::exists(&file_path)? {
        let Ok(session_token) = std::env::var("AOC_TOKEN") else {
            eyre::bail!("AOC_TOKEN env var not specified, could not fetch missing input.");
        };

        let file_path = PathBuf::from(&file_path);
        let Some(parent_dir) = file_path.parent() else {
            eyre::bail!("not a valid file path.");
        };

        let year = 2024;

        let mut headers = HeaderMap::new();
        let mut cookie = HeaderValue::from_str(&session_token)
            .map_err(|_| eyre::eyre!("invalid session token: non-ascii"))?;
        cookie.set_sensitive(true);
        headers.insert("Cookie", cookie);

        println!("\x1b[31mfetching missing input for day {day}\x1b[0m");

        let res = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()?
            .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
            .send()?;

        std::fs::create_dir_all(parent_dir)?;
        std::fs::write(&file_path, &res.bytes()?[..])?;
    }

    let input = std::fs::read_to_string(&file_path)?;
    let mut scratch = vec![0u8; input.len()];

    println!(
        "\x1b[32mrunning\x1b[0m [\x1b[34m{} iters\x1b[0m] day{day}/{}",
        config.sample_count, variant
    );
    let mut ctx = RunContext {
        input: &input,
        input_scratch: &mut scratch,
        write_output: true,
        begin_timestamp: None,
        parsed_timestamp: None,
        complete_timestamp: None,
    };

    let mut samples = Vec::with_capacity(config.sample_count);
    let loop_start = Instant::now();
    for _ in 0..config.sample_count {
        ctx.input_scratch.clone_from_slice(input.as_bytes());

        part(&mut ctx);
        ctx.write_output = false;

        let (start, end) = (
            ctx.begin_timestamp.unwrap(),
            ctx.complete_timestamp.unwrap(),
        );
        samples.push(Sample {
            full: end.duration_since(start),
            // parse: ctx.parsed_timestamp.map(|ts| ts.duration_since(start)),
        });

        if loop_start.elapsed() > Duration::from_secs_f64(config.rerun_time_limit_s) {
            break;
        }
    }

    Ok(samples)
}

fn parse_variant(variant: &str) -> eyre::Result<(u32, &str)> {
    let Some((day, variant)) = variant.split_once('.') else {
        eyre::bail!("invalid variant '{variant}'");
    };
    let Ok(day) = day.trim_start_matches('d').parse::<u32>() else {
        eyre::bail!("invalid day '{day}'");
    };
    Ok((day, variant))
}

#[derive(Copy, Clone, Debug)]
struct SampleSummary {
    pub count: usize,
    pub mean: Duration,
    pub median: Duration,
    pub min: Duration,
    pub max: Duration,
}

impl SampleSummary {
    pub fn summarize(samples: &[Sample]) -> SampleSummary {
        let mut total = Duration::ZERO;
        let mut min = Duration::MAX;
        let mut max = Duration::ZERO;
        for sample in samples {
            total += sample.full;
            min = Duration::min(min, sample.full);
            max = Duration::max(max, sample.full);
        }
        let mut samples = samples.iter().copied().collect::<Vec<_>>();
        samples.sort_unstable_by_key(|sample| sample.full);
        let median = samples[(samples.len() - 1) / 2].full;

        SampleSummary {
            count: samples.len(),
            mean: total / samples.len() as u32,
            median,
            min,
            max,
        }
    }
}

fn main() -> eyre::Result<()> {
    dotenv::dotenv().ok();

    let config = RunConfig::from_args_safe()?;
    let repo = days::make_repo();

    match &config.subcommand {
        RunCommand::Run { variant } => {
            let (day, variant) = if let Some(variant) = variant {
                parse_variant(&variant)?
            } else {
                let max_day = repo.days.keys().max().copied().unwrap();
                let part_name = match &variant {
                    Some(name) => &*name,
                    None => {
                        if repo.days[&max_day].contains_key("part2") {
                            "part2"
                        } else if repo.days[&max_day].contains_key("part1") {
                            "part1"
                        } else {
                            eyre::bail!("day {max_day} has no 'part1' or 'part2' specified")
                        }
                    }
                };
                (max_day as u32, part_name)
            };

            let samples = run_variant(&repo, &config, day, variant)?;
            let summary = SampleSummary::summarize(&samples);

            println!(
                "[\x1b[32msamples\x1b[0m {}] [\x1b[32mmean\x1b[0m {}] [\x1b[32mmedian\x1b[0m {}] [\x1b[32mextrema\x1b[0m {} - {}]",
                summary.count,
                DisplayDuration(summary.mean),
                DisplayDuration(summary.median),
                DisplayDuration(summary.min),
                DisplayDuration(summary.max)
            );
        }
        RunCommand::Compare { variant1, variant2 } => {
            // it might be cool to benchmark by continually starting child processes and using ipc
            // to get the results, so that funky stuff like code pages being better or worse aligned
            // doesnt muddy the results as much. but idk how significant the effect of that stuff is.

            let (day1, part1) = parse_variant(&variant1)?;
            let (day2, part2) = parse_variant(&variant2)?;
            let samples1 = run_variant(&repo, &config, day1, part1)?;
            let samples2 = run_variant(&repo, &config, day2, part2)?;

            let summary1 = SampleSummary::summarize(&samples1);
            let summary2 = SampleSummary::summarize(&samples2);

            fn hl(x: bool) -> &'static str {
                match x {
                    true => "\x1b[32m",
                    false => "\x1b[31m",
                }
            }

            println!(
                "[\x1b[32msamples\x1b[0m {}] [\x1b[32mmean\x1b[0m {}{}\x1b[0m] [\x1b[32mmedian\x1b[0m {}{}\x1b[0m] [\x1b[32mextrema\x1b[0m {} - {}] \x1b[34m{variant1}\x1b[0m",
                summary1.count,
                hl(summary1.mean < summary2.mean),
                DisplayDuration(summary1.mean),
                hl(summary1.median < summary2.median),
                DisplayDuration(summary1.median),
                DisplayDuration(summary1.min),
                DisplayDuration(summary1.max)
            );

            println!(
                "[\x1b[32msamples\x1b[0m {}] [\x1b[32mmean\x1b[0m {}{}\x1b[0m] [\x1b[32mmedian\x1b[0m {}{}\x1b[0m] [\x1b[32mextrema\x1b[0m {} - {}] \x1b[34m{variant2}\x1b[0m",
                summary2.count,
                hl(summary2.mean < summary1.mean),
                DisplayDuration(summary2.mean),
                hl(summary2.median < summary1.median),
                DisplayDuration(summary2.median),
                DisplayDuration(summary2.min),
                DisplayDuration(summary2.max)
            );
        }
        RunCommand::List => {
            println!("Available Variants:");
            let mut days: Vec<_> = repo.days.keys().copied().collect();
            days.sort_unstable();
            for day in days {
                let mut variants: Vec<_> = repo.days[&day].keys().collect();
                variants.sort_unstable();
                for variant in variants {
                    println!("\t- d{day}.{variant}");
                }
            }
        }
    }

    Ok(())
}

#[derive(Copy, Clone, Debug)]
struct Sample {
    pub full: Duration,
    // pub parse: Option<Duration>,
}

#[derive(Copy, Clone, Debug)]
pub struct DisplayDuration(pub Duration);

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

pub struct WrapAsDisplay<F>(F);
impl<F> std::fmt::Display for WrapAsDisplay<F>
where
    F: Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0(f)
    }
}
