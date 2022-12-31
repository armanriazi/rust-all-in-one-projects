
#[cfg(test)]
mod tests {
use super::*;
use pacman_lib::core::{boradspace::BoardSpace, play::Play};
use pretty_assertions::{assert_eq, assert_ne};


    #[test]
    fn play_new_dimensions_5_test(){
       let board_arranged:BoardSpace =  Play::arrange_stateboard(&(5isize,5isize));
       let board_main =  BoardSpace::new(&(5isize,5isize));
       
        assert_eq!(board_arranged.dimensions.0,board_main.dimensions.0);
        assert_eq!(board_arranged.dimensions.0,board_main.dimensions.1);
        //        
    }
}