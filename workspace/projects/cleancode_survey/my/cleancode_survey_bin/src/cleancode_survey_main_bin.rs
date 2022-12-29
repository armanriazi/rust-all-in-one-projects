#![allow(dead_code, unused_variables, unused_imports)]
use cleancode_survey_lib::core::factory::json_factory;
use cleancode_survey_lib::{core::error::CustomError, core::sample::*};
use env_logger::{Builder, Target};
use log::{debug, error, info, log_enabled, Level};
use std::io::BufReader;
use std::process;
use std::{env, fs, path::Path};
use std::{env::args, fs::File};

/// Main
///
/// ## Commands
///
///
/// ```RUST_LOG=INFO cargo run  -p cleancode_survey_bin --bin 1 cleancode_survey_main_bin file workspace/projects/cleancode_survey/my/cleancode_survey_bin/data/1.json workspace/projects/cleancode_survey/my/cleancode_survey_bin/data/2.json```
///
/// ```cargo doc  --package cleancode_survey_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package cleancode_survey_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// `nothing`
///

/// cargo run
/// RUST_LOG=INFO time cargo run cleancode_survey ../data/1.json
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn main() -> Result<(), CustomError> {
    init_app();

    survey_init_env_logger(true);

    info!("Starting Up...");

    let mut args: Vec<String> = env::args().collect();

    let mut mode = String::default();
    let mut user_id: u32 = 0;
    let mut file_name = String::default();

    if (&args).len() <= 1 {
        info!("** Please select a runner mode\n Help(file path transaction_list, or macrojson transaction_list)\n Default is cargo run macrojson **\n");
        args.push("macrojson".to_owned());
    } else {
        user_id = (&args[1]).trim().parse::<u32>().unwrap_or(0);
        mode = (&args[2]).trim().to_lowercase();
    }
    info!("UserId: {:?}\n", user_id.clone());
    info!("Mode: {:?}\n", mode.clone());

    if &user_id <= &0 {
        dbg!("Select user id > 0");
        process::exit(1);
    }
    if &mode == "file" {
        for (i, arg) in args.iter().enumerate() {
            if arg.find("json").is_some() {
                //file_name = arg.to_string();
                let json = fs::read_to_string(arg).unwrap();

                let file: serde_json::Value = serde_json::from_str(&json)?;

                json_factory(|| sample_json_data_from_file(file), user_id)?;
            }
        }
    } else if &mode == "macrojson" {
        info!("Selected mode is macrojson\n");
        json_factory(|| sample_json_data_from_module(), user_id)?;
    } else if &mode == "stringjson" {
        info!("Selected mode is stringjson\n");
        json_factory(|| sample_json_data_from_string(), user_id)?;
    } else {
        info!("The mode is not selected! Default is macrojson\n");
        json_factory(|| sample_json_data_from_module(), user_id)?;
    }
    Ok(())
}

pub fn survey_init_env_logger(is_enable: bool) {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    if is_enable {
        builder.init();
    }

    if log_enabled!(Level::Info) {
        info!("------------Welcome to env_logger------------");
    } else {
        info!("-------env_logger have not been activated-------");
    }
}

/// # Another way to read from file
/// ```rust,no_run
///let reader = BufReader::new(file);
///let serde_values = serde_json::from_reader(reader)?;
/// ```
pub fn sample_json_data_from_file(
    file: serde_json::Value,
) -> Result<serde_json::Value, CustomError> {
    info!("Selected mode is file!");

    return Ok(file);
}

fn init_app() -> impl std::process::Termination {
    let machine_kind = if cfg!(unix) {
        println!("I'm running on a {} machine!", "unix");
        std::process::ExitCode::SUCCESS
    } else if cfg!(windows) {
        println!("I'm running on a {} machine!", "windows");
        std::process::ExitCode::SUCCESS
    } else {
        println!("I'm running on a {} machine!", "unknown");
        std::process::ExitCode::FAILURE
    };
}
