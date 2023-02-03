#[allow(dead_code)]
fn greeter() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn do_test() {
		greeter();
    assert!(true)
  }
}