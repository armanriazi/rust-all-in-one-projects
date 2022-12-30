#![allow(dead_code, unused_variables, unused_imports)]

// use crate::models::{Survey,Message, State};
// use super::{*, error::CustomError};
// use log::{info,trace};

use super::pacman::Pacman;
use super::boradspace::Board;



 #[derive(Debug,PartialEq)]
pub enum Message {
    NewBorad(isize,isize),    
    Echo(String),    
    IsCompleted(bool),
}


//   pub fn make_new_borad(dimensions:(isize,isize)) ->  Box<Board> { 
//      Box::<Board>::from(Board::new(&dimensions))
//   }
//    pub fn make_new_pacman(board_obj:Board) -> Box<Pacman> {    
//      Box::<Pacman>::from(Pacman::new(&board_obj))
//   }


  // pub async fn play(commandsFileName) -> bool{
  //   const comands = await fileReader(commandsFileName);
  //   await comands.forEach((command) => {
  //     self.pacmanObj.run(command);
  //   });
  //  true
  // }
  
 #[derive(Debug,PartialEq)]
pub struct StateGame {
  pub board:Board,
  pub pacman:Pacman,
  pub IsCompleted: bool,  
  pub completed: bool,
}

impl StateGame{
  
    fn completed(&mut self) {
        self.completed = true;        
    }

    fn incomplete(&mut self) {
        self.completed = false;
    }

    fn make_new_borad(&mut self,dimensions:(isize,isize)) { 
         self.board=Board::new(&dimensions);
         self.make_new_pacman();
    }

    fn make_new_pacman(&mut self)  {    
         self.pacman=Pacman::new(&self.board);
    }

    fn echo(&self,s: String) {
        println!("{}", s);
    }

   pub fn process(&mut self, message: Message) {
        match message {
            Message::IsCompleted(false) =>  self.incomplete(),            
            Message::NewBorad(dx,dy) => self.make_new_borad((dx,dy)),                        
            Message::Echo(s) => self.echo(s),       
            Message::IsCompleted(true) =>  self.completed(),            
        }
    }
}