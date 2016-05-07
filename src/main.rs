extern crate num;
extern crate time;

use std::io;
use std::io::Write;
use std::fs;
use std::fs::File;

use time::{PreciseTime};

mod problems1_10;
mod problems11_20;
mod problems21_30;
mod problems31_40;
mod problems41_50;
mod problems51_60;
mod problems61_70;
mod prime_gen;

fn create_mod_contents(start:i32) -> String {
	let mut mod_contents = String::new();
	for i in 0..10 {
		mod_contents.push_str("mod problem");
		mod_contents.push_str(&(start + i).to_string());
		mod_contents.push_str(";\n");
	}
	mod_contents.push_str("\npub fn get_answer(num : i32) -> u64 {\n\tmatch num {\n");
	for i in 0..10 {
		mod_contents.push_str("\t\t");
		mod_contents.push_str(&(start + i).to_string());
		mod_contents.push_str(" => problem");
		mod_contents.push_str(&(start + i).to_string());
		mod_contents.push_str("::go(),\n");
	}
	mod_contents.push_str("\t\t_ => 0,\n\t}\n}");
	mod_contents
}

fn create_sol_contents() -> String {
	let mut sol_contents = String::new();
	sol_contents.push_str("pub fn go() -> u64 {\n\t0\n}");
	sol_contents
}

fn init_next_ten(){
	let start = 61;
	let mut fpath = String::new();
	fpath.push_str("src/problems");
	fpath.push_str(&start.to_string());
	fpath.push('_');
	fpath.push_str(&(start+9).to_string());
	
	let res = fs::create_dir(fpath.clone());
	
	match res { Err(_) => { println!("directory already exists"); return }, _ => () }
	
	let mut mod_path = String::new();
	mod_path.push_str(&fpath.clone());
	mod_path.push_str("/mod.rs");

	let mut modf = File::create(mod_path).ok().expect("file creation fail");	
	let _ = modf.write(create_mod_contents(start).as_bytes());

	let sol_contents = create_sol_contents();
	for i in 0..10 {
		let mut sol_path = String::new();
		sol_path.push_str(&fpath.clone());
		sol_path.push_str("/problem");
		sol_path.push_str(&(start + i).to_string());
		sol_path.push_str(".rs");
		
		let mut solf = File::create(sol_path).ok().expect("file creation fail");
		let _ = solf.write(sol_contents.as_bytes());
	}
}

fn test_times() {
	for pnum in 1..61 {
		let start = PreciseTime::now();
		match pnum {
			1...10 => problems1_10::get_answer(pnum),
			11...20 => problems11_20::get_answer(pnum),
			21...30 => problems21_30::get_answer(pnum) as u64,
			31...40 => problems31_40::get_answer(pnum),
			41...50 => problems41_50::get_answer(pnum),
			51...60 => problems51_60::get_answer(pnum),
			_ => 0
		};
		let dur = start.to(PreciseTime::now());
		let d = dur.num_milliseconds() as f32 / 1000f32;
		println!("{}-{}s",pnum,d);
	}
}

fn main() {
    println!("Which problem do you want the answer to?");
	
	let mut pnum = String::new();	
	
	io::stdin().read_line(&mut pnum)
		.ok()
		.expect("Failed to read line");
		
	let pnum : i32 = pnum.trim().parse()
		.ok()
		.expect("Please type a number!");
	
	let start = PreciseTime::now();
	
	match pnum {
		-1 => test_times(),
		0 => init_next_ten(),
		1...10 => println!("{}",problems1_10::get_answer(pnum)),
		11...20 => println!("{}",problems11_20::get_answer(pnum)),
		21...30 => println!("{}",problems21_30::get_answer(pnum)),
		31...40 => println!("{}",problems31_40::get_answer(pnum)),
		41...50 => println!("{}",problems41_50::get_answer(pnum)),
		51...60 => println!("{}",problems51_60::get_answer(pnum)),
		61...70 => println!("{}",problems61_70::get_answer(pnum)),
		_ => println!("that problem isn't done yet :("),
	}
	
	let dur = start.to(PreciseTime::now());
	let d_ms = dur.num_milliseconds();
	let d_s = d_ms as f32 / 1000f32;
	println!("ran in {}s ({}ms)",d_s,d_ms);
}
