mod problem31;
mod problem32;
mod problem33;
mod problem34;
mod problem35;
mod problem36;
mod problem37;
mod problem38;
mod problem39;
mod problem40;

pub fn get_answer(num : i32) -> u64{
	match num {
		31 => problem31::go(),
		32 => problem32::go(),
		33 => problem33::go(),
		34 => problem34::go(),
		35 => problem35::go(),
		36 => problem36::go(),
		37 => problem37::go(),
		38 => problem38::go(),
		39 => problem39::go(),
		40 => problem40::go(),
		_ => 0,
	}
}
