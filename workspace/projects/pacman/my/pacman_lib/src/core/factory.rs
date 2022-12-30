#![allow(dead_code, unused_variables, unused_imports)]


pub fn process_message_call(state: &mut State, index: usize, key: String, value: Value) {
    state.process(Message::Update(index, key.clone(), value));
    state.process(Message::Echo(String::from(format!(
        "Making model(updated key successfuly):{:?}",
        key
    ))));
    state.process(Message::IsCompleted(false));
}