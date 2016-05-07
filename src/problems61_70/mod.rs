mod problem61;
mod problem62;
mod problem63;
mod problem64;
mod problem65;
mod problem66;
mod problem67;
mod problem68;
mod problem69;
mod problem70;

pub fn get_answer(num : i32) -> u64 {
	match num {
		61 => problem61::go(),
		62 => problem62::go(),
		63 => problem63::go(),
		64 => problem64::go(),
		65 => problem65::go(),
		66 => problem66::go(),
		67 => problem67::go(),
		68 => problem68::go(),
		69 => problem69::go(),
		70 => problem70::go(),
		_ => 0,
	}
}