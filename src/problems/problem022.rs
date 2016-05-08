use std::fs::File;
use std::io::prelude::*;

fn score(name:&str)->u64 {
	let mut score : u64 = 0;
	for c in name.chars() {
		score = score + (c as u64) - 64;
	}
	score
}

fn go() -> String {
	let mut f = File::open("data/p022_names.txt").ok().expect("file open fail");
	let mut s = String::new();
	f.read_to_string(&mut s).ok().expect("file read fail");
	let mut names : Vec<&str> = s.split(',').collect();
	names.sort();	
	let mut sum : u64 = 0;
	let mut i = 1;
	for x in names.iter() {
		sum += i * score(&x[1..(x.len()-1)]);
		i += 1;
	}
	sum.to_string()
}

problem!(go, 871198282);
