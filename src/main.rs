#[cfg(windows)]
const EOL: &'static str = "\r\n";
#[cfg(not(windows))]
const EOL: &'static str = "\n";

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;

fn main() -> io::Result<()> {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  if filename.len() > 0 {
    let mut f = File::open(filename)?;
    let mut buffer = [0; 1000];
    let mut prev_line: String = String::from("");
    let mut current_line: String = String::from("");
    let mut total_count = 0i8;

    println!("Ticker,ExDate,Net,Gross,Tax");

    loop {
      f.read(&mut buffer[..])?;
      let text = String::from_utf8(buffer.to_vec()).expect("cant do shit");
      
      if text == prev_line {
        break;
      }
      prev_line = text.clone();
      for c in text.chars() {
        if c.to_string() != EOL {
          current_line.push_str(&c.to_string());
        } else {
          // println!("{:?}", current_line);

          let cells: Vec<String> = current_line.split(",").map(str::to_string).collect();

          // println!("{:?}", cells);

          if cells[1] == "Total" {
            total_count += 1;
          }

          if total_count == 0 && cells[1] == "Data" && cells[2] == "Summary" {
            println!(
              "{},{},{},{},{}",
              cells[4], // symbol
              cells[8], // date
              cells[12].parse::<f64>().unwrap().abs() - cells[15].parse::<f64>().unwrap().abs(),
              cells[12], // total income
              cells[15] // tax
            );
          }
          
          current_line = String::from("");
        }
      }
    }
  }
  Ok(())
}