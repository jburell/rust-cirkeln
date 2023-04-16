#![feature(test)]
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

pub fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    Ok(results)
}

pub fn search(query: &str, contents: &str) -> Vec<String> {
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

#[cfg(test)]
mod test {
	//use std::io::*;
	use std::fs::*;
	use std::process::Termination;
	use super::*;

	#[test]
	fn do_test() {
		let file_path = "romeo.txt";
		let file = File::open(file_path).unwrap();
		let all = fs::read_to_string(file_path).unwrap();
		let res = search("thou", all.as_str());
		//let buf: BufReader<File> = BufReader::new(file);
		//let content = buf.lines();
		//let res = search("thou", content.into_iter());
		println!("Number of rows: {}", res.len());
    assert!(res.len() > 0);
  }
}