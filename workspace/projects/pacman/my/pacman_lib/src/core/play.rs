
use log::info;

use super::{boradspace::BoardSpace, pacman::Pacman};
use super::stateboard::{StateBoard, Message as MessageStateBoard};
use super::statepacman::{StatePacman, Message as MessageStatePacman};


pub struct Play;

impl Play{

    pub fn arrange_stateboard(dimension:&(isize,isize))-> BoardSpace{

        let _board= BoardSpace::new(&dimension);
        let return_board=_board.clone();
        let mut _stateboard: StateBoard = StateBoard {         
            completed: false,
            board:_board,        
        };

        _stateboard.process(MessageStateBoard::IsCompleted(false));
        _stateboard.process(MessageStateBoard::NewBorad(*dimension));    
        _stateboard.process(MessageStateBoard::Echo(String::from(format!(
            "Make arrange board state(set dimension successfuly):{:?}",
            &dimension
        ))));
        _stateboard.process(MessageStateBoard::IsCompleted(true));

    return_board
    }
        
    pub fn arrange_statepacman(boardspace:BoardSpace){

        let _pacman= Pacman::new(&boardspace);
        let mut _statepacman: StatePacman = StatePacman {         
            completed: false,            
            pacman: _pacman,        
        };

        _statepacman.process(MessageStatePacman::IsCompleted(false));
        _statepacman.process(MessageStatePacman::NewPacman(boardspace));    
        _statepacman.process(MessageStatePacman::Echo(String::from(format!(
            "Make arrange pacman state(updated successfuly)"            
        ))));
        _statepacman.process(MessageStatePacman::IsCompleted(true));

    }

    pub fn play() ->bool{
        info!("Playing...");
    //           async play(commandsFileName) {
    //     const comands = await fileReader(commandsFileName)
    //     await comands.forEach((command) => {
    //       this.pacmanObj.run(command)
    //     })
    //     return true
    //   }
        true
    }
}