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
    let mut state = State {
            completed: false,
            survey: Survey::new(String::default(),String::default(),0,0.0,0),
            description: String::default(),
        };

        value_fields.as_object().unwrap().into_iter().enumerate().for_each(|(_i, value_of_key)| {

            process_message_call(&mut state,_i,value_of_key.0.to_string(), value_of_key.1.to_owned());
            trace!("\n value_of_key {:?}-{:?}\n",_i,value_of_key);
        });

    Ok(())
}

pub fn process_message_call(state: &mut State,index:usize,key:String,value:Value) {

        state.process(Message::Update(index,key.clone(),value));
        state.process(Message::Echo(String::from(format!("Making model(updated key successfuly):{:?}",key))));
        state.process(Message::Completed);
}
