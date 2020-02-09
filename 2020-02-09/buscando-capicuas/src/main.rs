fn main() {
    let days1 = vec![0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let days2 = vec![0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for year in 1..10_000 {
        for (month, days) in (if is_leap(year) { &days1 } else { &days2 }).iter().enumerate().take(12+1).skip(1) {
            for day in 1..=*days {
                if check_zero_filled(year, month, day as usize) {
                    println!(
                        "year={}, month={}, day={}, f1: {}, f2: {}, f3: {}",
                        year,
                        month,
                        day,
                        format_date_1(year, month, day),
                        format_date_2(year, month, day),
                        format_date_1(year, day as usize, month)
                    );
                }
                else if check_unfilled(year, month, day) {
                    println!(
                        "year={}, month={}, day={} f1: {}, f2: {}, f3: {}",
                        year,
                        month,
                        day,
                        format_date_3(day, month, year),
                        format_date_3(month, day, year),
                        format_date_3(year, month, day)
                    );
                }
            }
        }
    }
}

fn is_leap(year: usize) -> bool {
    year % 4 == 0 && year % 100 != 0 || year % 400 == 0
}

fn check_zero_filled(year: usize, month: usize, day: usize) -> bool {
    let s = format_date_1(year, month, day);
    if s == s.chars().rev().collect::<String>() {
        let t = format_date_2(year, month, day);
        if t == t.chars().rev().collect::<String>() {
            let r = format_date_2(year, day, month);
            if r == r.chars().rev().collect::<String>() {
                return true;
            }
        }
    }
    false
}

fn check_unfilled(year: usize, month: usize, day: usize) -> bool {
    let s = format_date_3(year, month, day);
    if s == s.chars().rev().collect::<String>() {
        let t = format_date_3(day, month, year);
        if t == t.chars().rev().collect::<String>() {
            let r = format_date_3(month, day, year);
            if r == r.chars().rev().collect::<String>() {
                return true;
            }
        }
    }
    false
}

fn format_date_1(year: usize, month: usize, day: usize) -> String {
    if year < 100 {
        format!("{:02}{:02}{:02}", year, month, day)
    } else if year < 1000 {
        format!("{:03}{:02}{:02}", year, month, day)
    } else {
        format!("{:04}{:02}{:02}", year, month, day)
    }
}

fn format_date_2(year: usize, month: usize, day: usize) -> String {
    if year < 100 {
        format!("{:02}{:02}{:02}", day, month, year)
    } else if year < 1000 {
        format!("{:02}{:02}{:03}", day, month, year)
    } else {
        format!("{:02}{:02}{:04}", day, month, year)
    }
}

fn format_date_3(year: usize, month: usize, day: usize) -> String {
    format!("{}{}{}", year, month, day)
}
