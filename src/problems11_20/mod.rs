mod problem11;
mod problem12;
mod problem13;
mod problem14;
mod problem15;
mod problem16;
mod problem17;
mod problem18;
mod problem19;
mod problem20;

pub fn get_answer(num : i32) -> u64{
	match num {
		11 => problem11::go(),
		12 => problem12::go(),
		13 => problem13::go(),
		14 => problem14::go(),
		15 => problem15::go(),
		16 => problem16::go(),
		17 => problem17::go(),
		18 => problem18::go(),
		19 => problem19::go(),
		20 => problem20::go(),
		_ => 0,
	}
}
