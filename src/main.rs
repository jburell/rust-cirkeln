use std::io::Read;

use rustcirkeln::*;

fn main() {
  println!("{}", std::env::current_dir().unwrap().display());
  let mut file = std::fs::File::open("romeo.txt").unwrap();
  let mut buf = String::new();
  file.read_to_string(&mut buf).unwrap();
  let res = search_1("Romeo", buf.as_str());
  println!("{:?}", res);
}