extern crate num;

use std::io;

mod problems1_10;
mod problems11_20;
mod problems21_30;
mod problems31_40;
mod prime_gen;

fn main() {
    println!("Which problem do you want the answer to?");
	
	let mut pnum = String::new();	
	
	io::stdin().read_line(&mut pnum)
		.ok()
		.expect("Failed to read line");
		
	let pnum : i32 = pnum.trim().parse()
		.ok()
		.expect("Please type a number!");
		
	match pnum {
		1...10 => println!("{}",problems1_10::get_answer(pnum)),
		11...20 => println!("{}",problems11_20::get_answer(pnum)),
		21...30 => println!("{}",problems21_30::get_answer(pnum)),
		31...40 => println!("{}",problems31_40::get_answer(pnum)),
		_ => println!("that problem isn't done yet :("),
	}
}
