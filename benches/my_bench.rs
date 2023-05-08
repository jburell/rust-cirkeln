use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::*;

use rustcirkeln::*;
mod perf;

pub fn search_1(c: &mut Criterion) {
  let file_path = "romeo.txt";
  let all = fs::read_to_string(file_path).unwrap();
  //let all_as_res_lines = 
  //  all
  //  .clone()
  //  .as_str()
  //  .lines()
  //  .fold(Vec::new(), |mut acc, l| {
  //    acc.push(Ok(l.to_string())); 
  //    acc
  //  });

  c.bench_function("search_1", |b| b.iter(|| mini_grep::search_1("thou", black_box(all.as_str()))));
  c.bench_function("search_ci", |b| b.iter(|| mini_grep::search_case_insensitive("thou", black_box(all.as_str()))));
  //c.bench_function("search_2", |b| b.iter(|| mini_grep::search_2("thou", black_box(all_as_res_lines))));
}

criterion_group!(
  name = benches;
  config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(1000));
  targets = search_1
);
criterion_main!(benches);