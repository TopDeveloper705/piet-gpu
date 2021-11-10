// Copyright 2021 The piet-gpu authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Also licensed under MIT license, at your choice.

//! Tests for piet-gpu shaders and GPU capabilities.

mod clear;
mod config;
mod prefix;
mod prefix_tree;
mod runner;
mod test_result;

use clap::{App, Arg};
use piet_gpu_hal::InstanceFlags;

use crate::config::Config;
use crate::runner::Runner;
use crate::test_result::{ReportStyle, TestResult};

fn main() {
    let matches = App::new("piet-gpu-tests")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Verbose reporting of results"),
        )
        .arg(
            Arg::with_name("groups")
                .short("g")
                .long("groups")
                .help("Groups to run")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .help("Size of tests")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("n_iter")
                .short("n")
                .long("n_iter")
                .help("Number of iterations")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dx12")
                .long("dx12")
                .help("Prefer DX12 backend"),
        )
        .get_matches();
    let style = if matches.is_present("verbose") {
        ReportStyle::Verbose
    } else {
        ReportStyle::Short
    };
    let config = Config::from_matches(&matches);
    unsafe {
        let report = |test_result: &TestResult| {
            test_result.report(style);
        };
        let mut flags = InstanceFlags::empty();
        if matches.is_present("dx12") {
            flags |= InstanceFlags::DX12;
        }
        let mut runner = Runner::new(flags);
        report(&clear::run_clear_test(&mut runner, &config));
        if config.groups.matches("prefix") {
            report(&prefix::run_prefix_test(&mut runner, &config));
            report(&prefix_tree::run_prefix_test(&mut runner, &config));
        }
    }
}
