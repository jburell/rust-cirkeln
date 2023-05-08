#![allow(unused)]
#![feature(test)]
use std::error::Error;
use std::fs;
use std::io::Read;

pub struct Config {
	pub query: String,
	pub file_path: String,
	pub ignore_case: bool,
}

pub fn mini_grep() {
  println!("{}", std::env::current_dir().unwrap().display());
  let mut file = std::fs::File::open("romeo.txt").unwrap();
  let mut buf = String::new();
  file.read_to_string(&mut buf).unwrap();
  let res = search_1("Romeo", buf.as_str());
  println!("{:?}", res);
}

pub fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
	let contents = fs::read_to_string(config.file_path)?;

	let results = if config.ignore_case {
		search_case_insensitive(&config.query, &contents)
	} else {
		search_1(&config.query, &contents)
	};

	Ok(results)
}

pub fn search_1(query: &str, contents: &str) -> Vec<String> {
	contents
		.lines()
		.filter(|line| line.contains(query))
		.map(|line| line.to_string())
		.collect()
}

pub fn search_case_insensitive(
	query: &str,
	contents: &str,
) -> Vec<String> {
	let query = query.to_lowercase();

	contents
		.lines()
		.filter(|line| line.to_lowercase().contains(&query))
		.map(|line| line.to_string())
		.collect()
}

pub fn search_2(query: &str, contents: Vec<Result<String, std::io::Error>>) -> Vec<Result<String, std::io::Error>> {
	contents
		.into_iter()
		.fold(Vec::new(), |mut acc, r|
			match r {
				| Ok(n) => if n.contains(query) {
					acc.push(Ok(n.to_owned()));
					acc
				} else { 
					acc
				},
				| Err(e) => vec![Err(e)],
			}
		)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn do_test() {
		let file_path = "romeo.txt";
		let all = fs::read_to_string(file_path).unwrap();
		let res = search_1("thou", all.as_str());
		println!("Number of rows: {}", res.len());
    assert!(res.len() > 0);
  }
}