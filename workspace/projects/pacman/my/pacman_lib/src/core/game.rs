#![allow(dead_code, unused_variables, unused_imports)]

// use crate::models::{Survey,Message, State};
// use super::{*, error::CustomError};
// use log::{info,trace};

use super::pacman::Pacman;
use super::boradspace::Board;



 #[derive(Debug,PartialEq)]
pub enum Message {
    NewBorad(isize,isize),
    NewPacman(Board),
    Echo(String),    
    Quit,
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
pub struct Game {
  pub boardObj:Board,
  pub pacmanObj:Pacman,
  pub quit: bool,
}
impl Game{
  
    fn make_new_borad(dimensions:(isize,isize)) { 
         self.board=Board::new(&dimensions);
    }

    fn make_new_pacman(board:Board)  {    
         self.pacman=Pacman::new(&board);
    }
    
    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(s: String) {
        println!("{}", s);
    }

   pub fn process(&mut self, message: Message) {
        match message {
            Message::NewBorad(dimensions) => self.make_new_borad(dimensions),            
            Message::NewPacman(board) => self.make_new_borad(board),
            Message::Echo(s) => self.echo(s),       
            Message::Quit => self.quit(),            
        }
    }
}