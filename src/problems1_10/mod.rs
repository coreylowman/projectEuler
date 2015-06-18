mod problem1;
mod problem2;
mod problem3;
mod problem4;
mod problem5;
mod problem6;
mod problem7;
mod problem8;
mod problem9;
mod problem10;

pub fn get_answer(num : i32) -> u64{
	match num {
		1 => problem1::go(),
		2 => problem2::go(),
		3 => problem3::go(),
		4 => problem4::go(),
		5 => problem5::go(),
		6 => problem6::go(),
		7 => problem7::go(),
		8 => problem8::go(),
		9 => problem9::go(),
		10 => problem10::go(),
		_ => 0,
	}
}
