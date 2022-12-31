#![allow(dead_code, unused_variables, unused_imports)]

use array2d::{Array2D, Error};

 #[derive(Debug,PartialEq)]
pub struct XY {
  x:isize,
  y:isize,
}

 #[derive(Debug,PartialEq,Clone)]
pub struct Board {
  default:(isize,isize),
  pub dimensions: (isize,isize),  
  pub cells:Array2D<isize>
}
impl Board {
     
    pub fn new(dimensions:&(isize,isize)) ->Self {
      Board{
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
      if self.is_xy_not_occupied(xy) && self.is_xy_in_board(xy) {
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
      true
    }



    pub fn  is_xy_not_occupied(&self,xy:(isize,isize)) -> bool {
      // if !self.cells[xy.0] {
      //   self.cells[xy.0] = [];
      // }

      if !self.cells[(xy.0 as usize,xy.1 as usize)].count_zeros()<=0 {
        return true;
      }
       false
    }


    pub fn  is_xy_in_board(&self,xy:(isize,isize))-> bool {
      if xy.0 < self.default.0 && xy.1 < self.default.1 && xy.0 >= 0 && xy.1 >= 0 {
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
