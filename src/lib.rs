pub mod mini_grep;
pub mod concurrent;

use std::io::Read;
pub use concurrent::concurrent;

pub fn mini_grep() {
  println!("{}", std::env::current_dir().unwrap().display());
  let mut file = std::fs::File::open("romeo.txt").unwrap();
  let mut buf = String::new();
  file.read_to_string(&mut buf).unwrap();
  let res = mini_grep::search_1("Romeo", buf.as_str());
  println!("{:?}", res);
}
