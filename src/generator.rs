static SINGLE_LINE: &str = "DividendDetail,Data,Summary,USD,BND,43645828,US,20200806,20200803,14,,,2.22,2.22,2.22,-0.03,-0.03,-0.03\n";

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("large.csv");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let limit = 2_000_000_0;
    let mut index = 0;

    loop {
      index = index + 1;
      match file.write_all(SINGLE_LINE.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => (),
      }
      if index >= limit {
        break;
      }
    }
}