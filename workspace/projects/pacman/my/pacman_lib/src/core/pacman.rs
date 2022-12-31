#![allow(dead_code, unused_variables, unused_imports)]

use super::boradspace::BoardSpace;

const FACES: [&str; 4] = ["NORTH", "EAST", "SOUTH", "WEST"];

 #[derive(Debug,PartialEq)]
pub struct Pacman {
  pub board :BoardSpace,
  pub xy:(isize,isize),
  pub face:String,
  pub is_placed :bool
}

impl Pacman{
  pub fn new(board:&BoardSpace) -> Self {
      Pacman{
        board : board.clone(),
        xy:(0_isize,0_isize),
        face : String::default(),
        is_placed : false,
      }
    }

  pub fn move_run(&mut self) -> bool{
      if !self.is_placed {
        return false;
      }
      let (dx,dy)=self.xy; //destruct x,y
      match self.face.as_str() {        
        "EAST"=>{
          if !self.board.can_occupy_xy((dx+1,dy)){
            return false;
          }
          self.board.unoccupy_xy((dx,dy));
          self.board.occupy_xy((dx+1,dy));
          self.xy.0 += 1;
          return true;
        }
        "WEST"=>{
          if !self.board.can_occupy_xy((dx-1,dy)){
            return false;
          }
          self.board.unoccupy_xy((dx,dy));
          self.board.occupy_xy((dx-1,dy));
          self.xy.0 -= 1;
          return true;
        }
        "NORTH"=> {
          
          if !self.board.can_occupy_xy((dx,dy + 1 )){
            return false;
          }
          self.board.unoccupy_xy((dx,dy));
          self.board.occupy_xy((dx,dy + 1 ));
          self.xy.1 += 1;
          return true;
        }
        "SOUTH"=>{
          if !self.board.can_occupy_xy((dx,dy - 1 )){
            return false;
          }
          self.board.unoccupy_xy((dx,dy));
          self.board.occupy_xy((dx,dy - 1 ));
          self.xy.1 -= 1;
          return true;
        }   
        &_ =>{
          return false;
        }     
      }
    }

  pub fn left(&mut self) -> bool{
      if !self.is_placed{ 
        return false;
      }
      
      let mut face_iter_index = FACES.iter().enumerate().filter(|(i,&s)| s.eq(self.face.as_str()));
        if let Some(face_index)=face_iter_index.next(){
          let found_index=face_index.0.to_string().parse::<isize>().unwrap();
          if found_index - 1 < 0_isize {
             self.face = FACES[3].to_string();
          } else {
            self.face = FACES[face_index.0 - 1].to_string()
          }
        }            
     true    
  }

  pub  fn right(&mut self) ->bool {
        
    if !self.is_placed{ 
        return false;
      }
      
      let mut face_iter_index = FACES.iter().enumerate().filter(|(i,&s)| s.eq(self.face.as_str()));
        if let Some(face_index)=face_iter_index.next(){
          let found_index=face_index.0.to_string().parse::<isize>().unwrap();
          if found_index + 1 > 3_isize {
             self.face = FACES[0].to_string();
          } else {
            self.face = FACES[face_index.0 + 1].to_string()
          }
        }            
     true      
  }

  pub fn report(&mut self)-> bool {
      if !self.is_placed {
        return false;
      }      
    println!("report: {:?},{:?},{:?}", self.xy.0, self.xy.1, self.face);
      //state    
      true
    }

  pub  fn place(&mut self,xy:(isize,isize), face:String ) ->bool{
    if !self.board.can_occupy_xy(xy) {
      return false;
    }
    self.xy=xy;
    self.face = face;
    self.is_placed = true;
    self.board.occupy_xy(xy);
    true
  }

  pub  fn run(&mut self,cmd:String) ->bool{
  
      if cmd.clone().starts_with("PLACE") {
        let replaced=cmd.as_str().replace("PLACE ", "");
        let xy_face:Vec<&str> = replaced.split(",").collect();
        self.place((xy_face[0].parse::<isize>().unwrap(), xy_face[1].parse::<isize>().unwrap()), xy_face[2].to_string());
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

