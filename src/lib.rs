#![allow(internal_features)]
#![feature(portable_simd)]
#![feature(core_intrinsics)]
#![feature(unbounded_shifts)]

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

pub mod day3;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day10;
#[allow(static_mut_refs, non_upper_case_globals)]
pub mod day11;
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod day13;
pub mod day14;
#[allow(non_snake_case)]
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;

aoc_lib!{ year = 2024 }