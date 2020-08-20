use super::*;
use test::Bencher;

#[bench]
fn bench_100_untyped(b: &mut Bencher) {	
	let data = read_from_file("data/data100.txt").expect("error reading file");
    b.iter(|| x(&data, true));
}

#[bench]
fn bench_100_typed(b: &mut Bencher) {	
	let data = read_from_file("data/data100.txt").expect("error reading file");
	b.iter(|| x(&data, false));
}

#[bench]
fn bench_1000_untyped(b: &mut Bencher) {	
	let data = read_from_file("data/data1K.txt").expect("error reading file");
    b.iter(|| x(&data, true));
}

#[bench]
fn bench_1000_typed(b: &mut Bencher) {	
	let data = read_from_file("data/data1K.txt").expect("error reading file");
	b.iter(|| x(&data, false));
}