use crate::RunnerRepository;

macro_rules! add_days {
    ($($day:expr, $mod_name:ident;)*) => {
        $(pub mod $mod_name;)*

        pub fn make_repo() -> $crate::RunnerRepository {
            let mut repo = RunnerRepository::new();
            $(repo.merge_day($day, $mod_name::add_variants);)*
            repo
        }

    };
}

add_days! {
    1, day1;
    2, day2;
    3, day3;
    4, day4;
    5, day5;
    6, day6;
    7, day7;
    8, day8;
    9, day9;
    10, day10;
    11, day11;
    12, day12;
    13, day13;
}
