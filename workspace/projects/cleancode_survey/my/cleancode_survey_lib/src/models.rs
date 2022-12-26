use log::{trace, info};
use serde_json::Value;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Survey {
    pub name: String,
    pub url: String,
    pub participant_count: i32,
    pub response_rate: f64,
    pub submitted_response_count: i32,
}

impl Survey {
    pub fn new(
        name: String,
        url: String,
        participant_count: i32,
        response_rate: f64,
        submitted_response_count: i32,
    ) -> Self {
        Survey {
            name: name,
            url: url,
            participant_count: participant_count,
            response_rate: response_rate,
            submitted_response_count: submitted_response_count,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }
    pub fn set_response_rate(&mut self, response_rate: f64) {
        self.response_rate = response_rate;
    }
    pub fn set_participant_count(&mut self, participant_count: i32) {
        self.participant_count = participant_count;
    }
    pub fn set_submitted_response_count(&mut self, submitted_response_count: i32) {
        self.submitted_response_count = submitted_response_count;
    }
}

#[derive(Debug, PartialEq)]
pub enum Message {
    Update(usize, String, Value),
    Echo(String),
    Completed,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct State {
    pub survey: Survey,
    pub description: String,
    pub completed: bool,
}

impl State {
    fn completed(&mut self) {
        self.completed = true;
        self.read_survey();
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    /// serde_json has a issue to url keyword-sensetive word, so be careful
    fn particle_setter_survey(&mut self, key: Option<&str>, value: Value) {

        match key {
            Some("name") => &self.survey.set_name(value.to_string()),
            Some("participant_count") => &self.survey.set_participant_count(value.to_string().parse::<i32>().unwrap_or(-1)),
            Some("response_rate") => &self.survey.set_response_rate(value.to_string().parse::<f64>().unwrap_or(-1.0)),
            Some("iurl") | Some("url") | Some("rl") => &self.survey.set_url(value.to_string()),
            Some("submitted_response_count") => &self.survey.set_submitted_response_count(value.to_string().parse::<i32>().unwrap_or(-1)),
            Some(&_) => &self.process(Message::Completed),
            None => &self.process(Message::Completed)
        };
    }
    fn read_survey(&self) {
        trace!("{:?}\n", &self);
    }

    pub fn process(&mut self, message: Message) {
        match message {
            Message::Update(index, key, value) => {

                self.particle_setter_survey(Some(key.as_str()), value);
            }
            Message::Echo(s) => self.echo(s),
            Message::Completed => self.completed(),
        }
    }
}
