mod problem21;
mod problem22;
mod problem23;
mod problem24;
mod problem25;
mod problem26;
mod problem27;
mod problem28;
mod problem29;
mod problem30;

//stupid #27 having a negative number as an answer XD
pub fn get_answer(num : i32) -> i64{
	match num {
		21 => problem21::go() as i64,
		22 => problem22::go() as i64,
		23 => problem23::go() as i64,
		24 => problem24::go() as i64,
		25 => problem25::go() as i64,
		26 => problem26::go() as i64,
		27 => problem27::go(),
		28 => problem28::go() as i64 ,
		29 => problem29::go() as i64,
		30 => problem30::go() as i64,
		_ => 0,
	}
}
