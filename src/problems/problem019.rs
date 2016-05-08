fn go() -> String {
    let days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut day_of_week = 1;
    let mut num_sundays = 0;
    for year in 1901..2001 {
        for month in 0..12 {
            let d = if month == 1 && year % 4 == 0 {
                days[month] + 1
            } else {
                days[month]
            };
            if day_of_week == 6 {
                num_sundays += 1;
            }
            for _ in 0..d {
                day_of_week = (day_of_week + 1) % 7;
            }
        }
    }
    num_sundays.to_string()
}

problem!(go, 171);
