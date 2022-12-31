// #![allow(dead_code, unused_variables, unused_imports)]


// use std::error::Error;

// use log::info;
// use tokio::fs::File;
// use tokio::io::{self, AsyncReadExt};

// use super::error::CustomError;

//  #[derive(Debug,PartialEq)]
// pub struct FileReader;

// impl FileReader{
//   #[tokio::main]
//   pub async fn file_reader(filepath:&str) -> Result<Vec<&str>,dyn Error> {
//       let mut f = File::open(filepath).await?;
//       //let mut buffer = [0; 1024];
//       let mut buffer=String::new();
//       // read up to 1024 bytes
//       //let n = f.read(&mut buffer).await?;
//       f.read_to_string(&mut buffer).await?;

//       let u= buffer.split("\n").filter(|x| *x != "").collect::<Vec<_>>();
     
//       Ok(u)       
      
//   }
// }
//   // #[tokio::main]
//   // pub async fn file_reader<T: AsRef<str>>(filepath:T) -> Result<Vec<&'static str>,CustomError>{
//   //     let mut f = File::open(filepath.as_ref()).await?;
//   //     //let mut buffer = [0; 1024];
//   //     let mut buffer=String::new();
//   //     // read up to 1024 bytes
//   //     //let n = f.read(&mut buffer).await?;
//   //     f.read_to_string(&mut buffer).await?;

//   //     let vec_buffer= buffer.split("\n").filter(|x| *x != "").collect::<Vec<_>>();
     
//   //     Ok(vec_buffer)
      
//   // }



// // use std::fs;

// // pub let fileReader = async (fileName) => {
  
// //   const filePath = "./commands/${fileName}.txt";
// //   if (await fs.existsSync(filePath)) {
// //     return await fs
// //       .readFileSync(filePath, 'utf8')
// //       .split('\n')
// //       .filter((line) => line !== '')
// //       .map((line) => line.trim());
// //   }
// //    None
// // }


