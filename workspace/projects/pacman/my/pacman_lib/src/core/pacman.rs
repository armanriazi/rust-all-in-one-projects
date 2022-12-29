#![allow(dead_code, unused_variables, unused_imports)]

use super::boradspace::Board;

const FACES: [&str; 4] = ["NORTH", "EAST", "SOUTH", "WEST"];

pub struct Pacman {
  boardObj :Board,
  xy:(usize,usize),
  face:String,
  isPlaced :bool
}

impl Pacman{
  pub fn new(&mut self,boardObj:Board) {
      self.boardObj = boardObj;
      self.xy=(0,0);
      self.face = String::default();
      self.isPlaced = false;
    }

  pub  fn place(&mut self,xy:(usize,usize), face:String ) ->bool{
      if !self.boardObj.can_occupy_xy(xy) {
        return false;
      }
      self.xy=xy;
      self.face = face;
      self.isPlaced = true;
      self.boardObj.occupy_xy(xy);
      true
    }

  pub fn move_run(&mut self) -> bool{
      if !self.isPlaced {
        return false;
      }
      match self.face.as_str() {
        "NORTH"=> {
          if !self.boardObj.can_occupy_xy({ x: self.xy.x, y: self.y + 1 }){
            return false;
          }
          self.boardObj.vacant_xy({ x: self.x, y: self.y });
          self.boardObj.occupy_xy({ x: self.x, y: self.y + 1 });
          self.y += 1;
          return true;
        }
        "SOUTH"=>{
          if !self.boardObj.can_occupy_xy({ x: self.x, y: self.y - 1 }){
            return false;
          }
          self.boardObj.vacant_xy({ x: self.x, y: self.y });
          self.boardObj.occupy_xy({ x: self.x, y: self.y - 1 });
          self.xy.1 -= 1;
          return true;
        }
        "EAST"=>{
          if !self.boardObj.can_occupy_xy({ x: self.x + 1, y: self.y }){
            return false;
          }
          self.boardObj.vacant_xy({ x: self.x, y: self.y });
          self.boardObj.occupy_xy({ x: self.x + 1, y: self.y });
          self.xy.0 += 1;
          return true;
        }
        "WEST"=>{
          if !self.boardObj.can_occupy_xy({ x: self.x - 1, y: self.y }){
            return false;
          }
          self.boardObj.vacant_xy({ x: self.x, y: self.y });
          self.boardObj.occupy_xy({ x: self.x - 1, y: self.y });
          self.xy.0 -= 1;
          return true;
        }
      }
    }

  pub fn left(&mut self) -> bool{
      if !self.isPlaced{ 
        return false;
      }
      const faceIndex = FACES[face] => face == self.face;
      if (faceIndex - 1 < 0) {
        self.face = FACES[3];
      } else {
        self.face = FACES[faceIndex - 1]
      }
      return true    
  }

  pub  fn right(&mut self) ->bool {
      if !self.isPlaced{
        return false;
      }
      const faceIndex = FACES[face] => face == self.face;
      if (faceIndex + 1 > 3) {
        self.face = FACES[0];
      } else {
        self.face = FACES[faceIndex + 1];
      }
      true
    }

  pub fn report(&mut self)-> bool {
      if !self.isPlaced {
        return false;
      }
      const state = { x: self.x, y: self.y, face: self.face };
      println!("report", state);
      state    
    }

  pub  fn run(&mut self,cmd:String) ->bool{

      if cmd.starts_with("PLACE") {
        const xyFace = cmd.replace("PLACE ", "").split(",");
        self.place({ x: xyFace[0], y: xyFace[1], face: xyFace[2] })
      }
      if cmd.contains("MOVE") {
        self.move_run();
      }
      if cmd.contains("LEFT") {
        self.left();
      }
      if cmd.contains("RIGHT") {
        self.right();
      }
      if cmd.contains("REPORT") {
        self.report();
      }
      false
    }
}

