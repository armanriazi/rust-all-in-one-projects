#![allow(dead_code, unused_variables, unused_imports)]

// use crate::models::{Survey,Message, State};
// use super::{*, error::CustomError};
// use log::{info,trace};

use super::pacman::Pacman;
use super::boradspace::BoardSpace;



 #[derive(Debug,PartialEq)]
pub enum Message {
    NewPacman(BoardSpace),    
    Echo(String),    
    IsCompleted(bool),
}

  
 #[derive(Debug,PartialEq)]
pub struct StatePacman {
  pub pacman:Pacman,  
  pub completed: bool,
}

impl StatePacman{

    fn completed(&mut self) {
        self.completed = true;        
    }

    fn incomplete(&mut self) {
        self.completed = false;
    }

    fn make_new_pacman(&mut self, board:BoardSpace)  {    
         self.pacman=Pacman::new(&board);
    }

    fn echo(&self,s: String) {
        println!("{}", s);
    }

   pub fn process(&mut self, message: Message) {
        match message {
            Message::IsCompleted(false) =>  self.incomplete(),            
            Message::NewPacman(board) => self.make_new_pacman(board),                        
            Message::Echo(s) => self.echo(s),       
            Message::IsCompleted(true) =>  self.completed(),            
        }
    }
}