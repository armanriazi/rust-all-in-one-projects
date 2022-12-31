#![allow(dead_code, unused_variables, unused_imports)]

use array2d::{Array2D, Error};
use log::{debug, trace, info};



 #[derive(Debug,PartialEq,Clone)]
pub struct BoardSpace {
  default:(isize,isize),
  pub dimensions: (isize,isize),  
  pub cells:Array2D<isize>
}
impl BoardSpace {
     
    pub fn new(dimensions:&(isize,isize)) ->Self {
      BoardSpace{
          dimensions: dimensions.clone(),
          default: dimensions.clone(),
          cells : Array2D::filled_with(0, dimensions.0 as usize, dimensions.1 as usize)
      }
    }

  
    pub fn  get_dimensions(&self) -> (isize,isize) {
        return self.dimensions;
    }


    pub fn  set_dimensions(&mut self, dimensions:(isize,isize)) {
      self.dimensions = dimensions;
    }

    pub fn  can_occupy_xy(&self,xy:(isize,isize)) -> bool{
      if self.is_xy_not_occuped(xy) && self.is_xy_in_board(xy) {       
        return true;
      }
       false
    }

    pub fn  occupy_xy(&mut self,xy:(isize,isize)) -> bool {
      // if !self.cells[xy.0]{
      //    self.cells = []
      // }
      if !self.can_occupy_xy(xy) {
        return false;
      }
      self.cells[(xy.0 as usize,xy.1 as usize)] = 1;

      info!("{:?}",self.cells[(xy.0 as usize,xy.1 as usize)]);
      self.boardspace_report();
      
      true
    }


    pub fn boardspace_report(&self){
     
      for row_iter in self.cells.rows_iter() {
        for element in row_iter {
            trace!("\nBoardSpace_Report:{:?},", element);
        }
      }
    }

    /// [TODO]
    pub fn  is_xy_not_occuped(&self,xy:(isize,isize)) -> bool {
      let (x,y)=xy;
      // if !self.cells.row_iter(x).is_ok() {
      //   self.cells.set_row_major(5,0) = [];
      // }
      if !self.cells[(xy.0 as usize,xy.1 as usize)].gt(&1_isize) { 
        debug!(r#"{:?}"#,"true_is_xy_not_occupied");
        return true;
      }
      debug!(r#"{:?}"#,"false_is_xy_not_occupied");
       false
    }
    /// [TODO]
    pub fn  is_xy_not_occupied(&self,xy:(isize,isize)) -> bool {
      let (x,y)=xy;
      // if !self.cells.row_iter(x).is_ok() {
      //   self.cells.set_row_major(5,0) = [];
      // }
      if !self.cells[(xy.0 as usize,xy.1 as usize)].ge(&0_isize) { 
        debug!(r#"{:?}"#,"true_is_xy_not_occupied");
        return true;
      }
      debug!(r#"{:?}"#,"false_is_xy_not_occupied");
       false
    }

    pub fn  is_xy_in_board(&self,xy:(isize,isize))-> bool {
       trace!(r#"{:?}"#,self.default.0);
      if xy.0 < self.default.0 && xy.1 < self.default.1 && xy.0 >= 0 && xy.1 >= 0 {
         debug!(r#"{:?}"#,"true_is_xy_in_board");
        return true;
      }
       false
    }
    pub fn  unoccupy_xy(&mut self,xy:(isize,isize)) -> bool {
      // if !self.cells[xy.0] { 
      //   self.cells[xy.0] = [];
      // }
      self.cells[(xy.0 as usize, xy.1 as usize)] = 0;
      true
    }

}


//--------------

// #[cfg(test)]
// mod tests {
// use super::*;
// use pretty_assertions::assert_eq;
// use pretty_assertions::{assert_eq, assert_ne};
// use serde::de::Expected;
//     #[test]
//     fn new(){
//        let b= Board::new((5,5));
       
//        assert_eq!(&b.dimensions.0,5_isize);
//        assert_eq!(&b.dimensions.1,5_isize);
//     }
// }
