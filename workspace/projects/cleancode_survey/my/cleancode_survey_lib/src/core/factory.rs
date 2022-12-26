use crate::models::{Survey,Message, State};

use super::{*, error::CustomError};
use log::{info,trace};
use serde_json::Value;

#[allow(dead_code)]
#[allow(unused_mut)]

pub fn json_factory<F>(
    f: F,
) -> Result<(), CustomError>
where
    F: FnOnce() -> Result<serde_json::Value, CustomError>,
{
    let serde_values_records: serde_json::Value =
        serde_json::from_value(f().unwrap())?;//.unwrap();
    let value_fields: serde_json::Value = serde_values_records["survey_result_detail"].clone();

    trace!("{:?}",value_fields);

    let _count=value_fields.as_object().unwrap().into_iter().enumerate().count();

        value_fields.as_object().unwrap().into_iter().enumerate().for_each(|(_i, value_of_key)| {

            process_message_call(_i,&value_of_key.1);
            trace!("\n value_of_key {:?}-{:?}\n",_i,value_of_key);
        });

    Ok(())
}

pub fn process_message_call(index:usize,value_of_key:Value) {
        let mut state = State {
            quit: false,
            survey: todo!(),
            description: todo!(),
        };
        state.process(Message::Update(index,value_of_key.to_string()));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Quit);

        //assert_eq!(state. ,String::from("Simple Survey"));
        //assert_eq!(state.quit, true);
    }
