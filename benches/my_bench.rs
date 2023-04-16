use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::*;
use rustcirkeln::*;

mod perf;

pub fn search_1(c: &mut Criterion) {
  let file_path = "romeo.txt";
  let file = fs::File::open(file_path).unwrap();
  let all = fs::read_to_string(file_path).unwrap();
  c.bench_function("search_1", |b| b.iter(|| search("thou", all.as_str())));
}

criterion_group!(
  name = benches;
  // This can be any expression that returns a `Criterion` object.
  config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100));
  targets = search_1
);

criterion_main!(benches);

//	//#[test]
//	#[bench]
//	fn do_test(bencher: &mut Bencher) -> impl Termination {
//		let file_path = "romeo.txt";
//		let file = File::open(file_path).unwrap();
//		let all = fs::read_to_string(file_path).unwrap();
//		let res = search("thou", all.as_str());
//		//let buf: BufReader<File> = BufReader::new(file);
//		//let content = buf.lines();
//		//let res = search("thou", content.into_iter());
//		println!("Number of rows: {}", res.len());
//    assert!(res.len() > 0);
//  }