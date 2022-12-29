#![allow(dead_code, unused_variables, unused_imports)]

use array2d::{Array2D, Error};
pub struct XY {
  x:usize,
  y:usize,
}
pub struct Board {
  dimensions: (usize,usize),  
  cells:Array2D<usize>
}
impl Board {
     
    pub fn new(&mut self,dimensions:(usize,usize)) {
      self.dimensions= dimensions;
      self.cells = Array2D::filled_with(0, dimensions.0, dimensions.1);          
    }

  
    pub fn  get_dimensions(&self) -> (usize,usize) {
        return self.dimensions;
    }


    pub fn  set_dimensions(&self, dimensions:(usize,usize)) {
      self.dimensions = dimensions;
    }


    pub fn  occupy_xy(&self,xy:(usize,usize)) -> bool {
      // if !self.cells[xy.0]{
      //    self.cells = []
      // }
      if !self.can_occupy_xy(xy) {
        return false;
      }
      self.cells[(xy.0,xy.1)] = 1;
      true
    }


    pub fn  vacant_xy(&self,xy:(usize,usize)) -> bool {
      // if !self.cells[xy.0] { 
      //   self.cells[xy.0] = [];
      // }
      self.cells[(xy.0,xy.1)] = 0;
      true
    }


    pub fn  is_xy_not_occupied(&self,xy:(usize,usize)) -> bool {
      // if !self.cells[xy.0] {
      //   self.cells[xy.0] = [];
      // }

      if !self.cells[(xy.0,xy.1)].count_zeros()<=0 {
        return true;
      }
       false
    }


    pub fn  is_xy_in_board(&self,xy:(usize,usize))-> bool {
      if xy.0 < 5 && xy.1 < 5 && xy.0 >= 0 && xy.1 >= 0 {
        return true;
      }
       false
    }


    pub fn  can_occupy_xy(&self,xy:(usize,usize)) -> bool{
      if self.is_xy_not_occupied(xy) && self.is_xy_in_board(xy) {
        return true;
      }
       false
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
       
//        assert_eq!(&b.dimensions.0,5_usize);
//        assert_eq!(&b.dimensions.1,5_usize);
//     }
// }
