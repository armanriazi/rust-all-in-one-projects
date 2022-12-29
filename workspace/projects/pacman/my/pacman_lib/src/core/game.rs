#![allow(dead_code, unused_variables, unused_imports)]

// use crate::models::{Survey,Message, State};
// use super::{*, error::CustomError};
// use log::{info,trace};

use super::pacman::Pacman;
use super::boradspace::Board;
struct Game {
  boardObj:Board,
  pacmanObg:Pacman,
}

impl Game{

  pub fn new(dimensions:(usize,usize)) {
    self.boardObj = Board::new( dimensions);
    self.pacmanObj = Pacman:new({ boardObj: self.boardObj });
  }

  pub async play(commandsFileName) -> bool{
    const comands = await fileReader(commandsFileName);
    await comands.forEach((command) => {
      self.pacmanObj.run(command);
    })
    return true;
  }
}

