#![allow(dead_code, unused_variables, unused_imports)]


use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

pub struct FileReader;

impl FileReader{
  #[tokio::main]
  pub async fn file_reader(filepath:&str) -> io::Result<()> {
      let mut f = File::open(filepath).await?;
      let mut buffer = [0; 10];

      // read up to 10 bytes
      let n = f.read(&mut buffer[..]).await?;

      println!("The bytes: {:?}", &buffer[..n]);
      Ok(())
  }
}
// use std::fs;

// pub let fileReader = async (fileName) => {
  
//   const filePath = "./commands/${fileName}.txt";
//   if (await fs.existsSync(filePath)) {
//     return await fs
//       .readFileSync(filePath, 'utf8')
//       .split('\n')
//       .filter((line) => line !== '')
//       .map((line) => line.trim());
//   }
//    None
// }


