
#[cfg(test)]
mod tests {
use super::*;
use pacman_lib::core::boradspace::BoardSpace;
//use pretty_assertions::{assert_eq, assert_ne};


    #[test]
    fn board_space_new_dimensions_5_test(){
       let board =  BoardSpace::new(&(5isize,5isize));
        assert_eq!(board.dimensions.0,5);
        assert_eq!(board.dimensions.1,5);
    }
        
    #[test]
    fn board_space_get_dimention5_test(){
       let board =  BoardSpace::new(&(5isize,5isize));        
        assert_eq!(board.get_dimensions().0,5);
        assert_eq!(board.get_dimensions().1,5);
    }
     #[test]
     //#[ignore]
    fn board_space_get_dimention6_test(){
       let board =  BoardSpace::new(&(5isize,5isize));        
        assert_ne!(board.get_dimensions().0,6);
        assert_ne!(board.get_dimensions().1,6);
    }
    //
    #[test]
    fn board_space_occupy_xy4_test(){
       let mut board =  BoardSpace::new(&(5isize,5isize));  
       assert_eq!(board.occupy_xy((4isize,4isize)),true);      
    }
    
    #[test]
    fn board_space_occupy_unoccupy_xy4_test(){
       let mut board =  BoardSpace::new(&(5isize,5isize));  
       board.occupy_xy((4isize,4isize));      
       assert_eq!(board.unoccupy_xy((4isize,4isize)),true); 
    }

       #[test]
    fn board_space_is_xy_not_occupied_xy4_test(){
       let mut board =  BoardSpace::new(&(5isize,5isize));  
       assert_eq!(board.can_occupy_xy((4isize,4isize)),true);      
    }
            
    #[test]
    fn board_space_is_xy_in_board_xy4_test(){
       let mut board =  BoardSpace::new(&(5isize,5isize));  
       assert_eq!(board.is_xy_in_board((4isize,4isize)),true);      
    }

    #[test]
    //#[ignore]
    fn board_space_is_xy_in_board_norange_xy_4_6_test(){
       let mut board =  BoardSpace::new(&(5isize,5isize));  
       assert_eq!(board.is_xy_in_board((4isize,6isize)),false);      
    }

    #[test]
    fn board_space_can_occupy_xy_4test(){
       let mut board =  BoardSpace::new(&(5isize,5isize));  
       assert_eq!(board.can_occupy_xy((4isize,4isize)),true);      
    }

      
    #[test]
    fn board_space_can_occupy_xy4_test(){
       let mut board =  BoardSpace::new(&(5isize,5isize));  
        board.occupy_xy((4isize,4isize));
       assert_eq!(board.is_xy_not_occupied((4isize,4isize)),false);      
    }

}
