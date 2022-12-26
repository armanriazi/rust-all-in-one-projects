#![allow(dead_code, unused_variables)]
use cleancode_survey_lib::core::factory::json_factory;
use cleancode_survey_lib::{core::error::CustomError,core::sample::*};
use log::{debug, error, log_enabled, info, Level};
use env_logger::{Builder, Target};
use std::{env, fs, path::Path};
use std::{fs::File, env::args};
use std::io::BufReader;


/// Main
///
/// ## Commands
///
///
/// ```RUST_LOG=INFO cargo run  -p cleancode_survey_bin --bin cleancode_survey_main_bin file workspace/projects/cleancode_survey/my/cleancode_survey_bin/data/1.json```
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


    survey_init_env_logger(true);

    info!("Starting Up...");


    let mut args: Vec<String> = env::args().collect();
    let mut mode = String::default();
    let mut file_name = String::default();

 if (&args).len() <= 1 {
        println!("** Please select a runner mode\n Help(file path transaction_list, or macrojson transaction_list)\n Default is cargo run macrojson **\n");
        args.push("macrojson".to_owned());
    } else {
        mode = (&args[1]).trim().to_lowercase();
    }
     info!("Mode: {:?}\n", &mode);

    if &mode == "file" {
        file_name = (&args[2]).trim().to_lowercase();
         info!("file name: {:?}\n", &file_name);

        let json = fs::read_to_string(file_name).unwrap();

        let file: serde_json::Value = serde_json::from_str(&json)?;

        json_factory( || {
            sample_json_data_from_file(file)
        })?;

    } else if &mode == "macrojson" {
        println!("Selected mode is macrojson\n");
        json_factory( || {
            sample_json_data_from_module()
        })?;

    } else if &mode == "stringjson" {
        println!("Selected mode is stringjson\n");
        json_factory( || {
            sample_json_data_from_string()
        })?;

    } else {
        println!("The mode is not selected! Default is macrojson\n");
        json_factory( || {
            sample_json_data_from_module()
        })?;

    }
    Ok(())
}

fn open_file(name: &str) -> std::io::Result<File> {
     info!("{:?}",&name);
    Ok(File::open(name)?)
}

pub fn survey_init_env_logger(is_enable:bool) {

    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    if is_enable{
       builder.init();
    }

    if log_enabled!(Level::Info) {
        info!("------------Welcome to env_logger------------");
    }
    else  {
        println!("----------env_logger have not been activated----------");
    }
}

pub fn sample_json_data_from_file(file: serde_json::Value) -> Result<serde_json::Value, CustomError> {
    println!("Selected mode is file!");
    //let reader = BufReader::new(file);
    //let serde_values = serde_json::from_reader(reader)?;

    return Ok(file);
}




// pub fn survey_sample_json_data_from_module() -> Result<serde_json::Value, CustomError> {
//     Ok(json!({
//     "blocks":[{
//         "block1":[{
//             "transactions":[{
//                 "transaction1":[{
//                         "inputs":[{

//                         }],
//                         "outputs":[{
//                             "to_addr": "Alice",
//                             "value": "50"
//                         },{
//                             "to_addr": "Bob",
//                             "value": "10"
//                         }]
//                 }]
//             }]
//         }],
//         "block2":[{
//             "transactions":[{
//                 "transaction1":[{
//                         "inputs":[{
//                         }],
//                         "outputs":[{
//                             "to_addr": "Alice",
//                             "value": "1000"
//                         },{
//                             "to_addr": "Bob",
//                             "value": "1000"
//                         }]
//                 }] ,
//                 "transaction2":[{
//                         "inputs":[{
//                             "to_addr": "Alice",
//                             "value": "50"
//                         },{
//                             "to_addr": "Bob",
//                             "value": "10"
//                         }],
//                         "outputs":[{
//                             "to_addr": "Alice",
//                             "value": "40"
//                         },{
//                             "to_addr": "Bob",
//                             "value": "20"
//                         }]
//                 }]
//             }]
//         }]
//       }]
//     }))
// }

// pub fn survey_sample_json_data_from_string() -> Result<serde_json::Value, CustomError> {
//     let json = r#"{
//     "blocks":[{
//         "block1":[{
//             "transactions":[{
//                     "transaction1":[{
//                         "inputs":[{
//                         }],
//                         "outputs":[{
//                             "to_addr": "Alice",
//                             "value": "1000"
//                         },{
//                             "to_addr": "Bob",
//                             "value": "1000"
//                         }]
//                     }]
//             }]
//         }]
//     }]
//     }
//     "#;

//     let js = serde_json::from_str(json).unwrap();

//     Ok(js)
// }
