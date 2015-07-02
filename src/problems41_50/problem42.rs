use std::fs::File;
use std::io::prelude::*;
use prime_gen::contains;

fn t(n:u64)->u64 {
	n * (n + 1) / 2
}

fn score(name:&str) -> u64 {	
	let nums : Vec<u8> = name.bytes().collect();
	let mut sum = 0;
	for n in nums {
		if 65 <= n && n <= 90 {
			sum += (n - 64) as u64;
		}
	}
	sum
}

pub fn go() -> u64 {
	let mut f = File::open("p042_words.txt").ok().expect("file open fail");
	let mut s = String::new();
	f.read_to_string(&mut s).ok().expect("file read fail");
	let names : Vec<&str> = s.split(',').collect();	
	let mut ts : Vec<u64> = Vec::new();
	for i in 1..10_000 {
		ts.push(t(i));
	}
	
	let mut num = 0;
	for name in names {
		let s = score(name);
		if contains(s,&ts) {
			num+=1;
		}
	}
	num
}