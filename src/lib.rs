#![warn(rust_2018_idioms)]
// The following are enabled by clippy::pedantic
// #![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::implicit_hasher)]
#![allow(clippy::explicit_iter_loop)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::if_not_else)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::match_on_vec_items)]
#![allow(clippy::needless_pass_by_value)]
#![warn(clippy::trivially_copy_pass_by_ref)]
#![warn(clippy::inline_always)]
#![warn(clippy::cast_possible_truncation)]
#![warn(clippy::cast_possible_wrap)]
#![warn(clippy::cast_sign_loss)]
#![warn(clippy::unreadable_literal)]
#![warn(clippy::unseparated_literal_suffix)]
#![warn(clippy::manual_filter_map)]
#![warn(clippy::manual_find_map)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::similar_names)]
#![warn(clippy::unused_self)]

#[macro_use]
extern crate aoc_runner_derive;

pub mod common;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

aoc_lib! { year = 2020 }
