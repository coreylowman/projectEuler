mod problem41;
mod problem42;
mod problem43;
mod problem44;
mod problem45;
mod problem46;
mod problem47;
mod problem48;
mod problem49;
mod problem50;

pub fn get_answer(num : i32) -> u64 {
	match num {
		41 => problem41::go(),
		42 => problem42::go(),
		43 => problem43::go(),
		44 => problem44::go(),
		45 => problem45::go(),
		46 => problem46::go(),
		47 => problem47::go(),
		48 => problem48::go(),
		49 => problem49::go(),
		50 => problem50::go(),
		_ => 0,
	}
}