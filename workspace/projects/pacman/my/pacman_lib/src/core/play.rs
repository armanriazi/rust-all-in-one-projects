
use log::info;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

use super::error::CustomError;
use super::pacman;
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
        
    pub fn arrange_statepacman(boardspace:BoardSpace)-> Pacman{

        let _pacman= Pacman::new(&boardspace);
        let return_pacman=_pacman.clone();
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
    return_pacman

    }

    #[tokio::main]
    pub async fn play(filepath:&str,pacman:&mut Pacman) -> io::Result<bool>{
        info!("Playing...");
              
      let mut f = File::open(filepath).await?;
      //let mut buffer = [0; 1024];
      let mut buffer=String::new();
      // read up to 1024 bytes
      //let n = f.read(&mut buffer).await?;
      f.read_to_string(&mut buffer).await?;

      let u= buffer.split("\n").filter(|x| *x != "").collect::<Vec<_>>();

      u.into_iter().for_each(|command| {
          info!("{:?}",&command);
          pacman.run(command.to_owned());
        });

      Ok(true)
    }
}