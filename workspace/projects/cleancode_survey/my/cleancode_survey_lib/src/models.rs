pub struct Survey<T>{
    pub survey_result_detail: T,
    pub name: T,
    pub url: T,
    pub participant_count: T,
    pub response_rate: T,
    pub submitted_response_count: T,
}

impl<T> Survey<T>{
    pub fn new(&self, survey:Self)-> Self{
        Survey{
            survey_result_detail :survey.survey_result_detail ,
            name: survey.name,
            url: survey.url,
            participant_count: survey.participant_count,
            response_rate: survey.response_rate,
            submitted_response_count: survey.submitted_response_count,
        }
    }
    pub fn set_survey_result_detail(&mut self,survey_result_detail:T) {
        self.survey_result_detail = survey_result_detail;
    }
    pub fn set_name(&mut self,name:T) {
        self.name = name;
    }
    pub fn set_url(&mut self,url:T) {
        self.url = url;
    }
    pub fn set_participant_count(&mut self,participant_count:T) {
        self.participant_count = participant_count;
    }
}
pub enum Message {
    Update(usize,String),
    Read(),
    Echo(String),
    Quit,
}

pub struct State<T>{
    pub survey: Survey<T>,
    pub description: T,
    pub quit: T,
}

impl<T>  State<T> {

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn particle_setter_survey(&self, index: T, value: T) {
        match index {
            0 => self.survey.set_survey_result_detail(value),
            1 => self.survey.set_name(value),
            2 => self.survey.set_url(value),
            3 => self.survey.set_participant_count(value),
        }
    }


    pub  fn process(&mut self, message: Message) {
        match message {
            Message::Update(index,value) => {
                self.particle_setter_survey(index,value);
            }
            Message::Echo(s) => self.echo(s),
            Message::Quit => self.quit(),
            Message::Read() => todo!(),
        }
    }
}
