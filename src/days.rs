use crate::RunnerRepository;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub fn make_repo() -> RunnerRepository {
    let mut repo = RunnerRepository::new();
    repo.merge_day(1, day1::add_variants);
    repo.merge_day(2, day2::add_variants);
    repo.merge_day(3, day3::add_variants);
    repo.merge_day(4, day4::add_variants);
    repo.merge_day(5, day5::add_variants);
    repo.merge_day(6, day6::add_variants);
    repo.merge_day(7, day7::add_variants);
    repo
}
