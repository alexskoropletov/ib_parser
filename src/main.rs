use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  if filename.len() > 0 {
    if let Ok(lines) = read_lines(filename) {
      let mut total_count = 0i8;
      println!("Ticker,ExDate,Net,Gross,Tax");
      for line in lines {
        if let Ok(ip) = line {
          let cells: Vec<String> = ip.split(",").map(str::to_string).collect();

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
        }
      }
    }
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}