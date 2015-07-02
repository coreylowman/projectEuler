mod problem51;
mod problem52;
mod problem53;
mod problem54;
mod problem55;
mod problem56;
mod problem57;
mod problem58;
mod problem59;
mod problem60;

pub fn get_answer(num : i32) -> u64 {
	match num {
		51 => problem51::go(),
		52 => problem52::go(),
		53 => problem53::go(),
		54 => problem54::go(),
		55 => problem55::go(),
		56 => problem56::go(),
		57 => problem57::go(),
		58 => problem58::go(),
		59 => problem59::go(),
		60 => problem60::go(),
		_ => 0,
	}
}