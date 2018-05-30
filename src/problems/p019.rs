/// You are given the following information, but you may prefer to do some
/// research for yourself.
///
/// 1 Jan 1900 was a Monday.
/// Thirty days has September,
/// April, June and November.
/// All the rest have thirty-one,
/// Saving February alone,
/// Which has twenty-eight, rain or shine.
/// And on leap years, twenty-nine.
///
/// A leap year occurs on any year evenly divisible by 4, but not on a century
/// unless it is divisible by 400. How many Sundays fell on the first of the
/// month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

#[test]
fn test() {
    assert_eq!(problem(), 171);
}

fn problem() -> u16 {
    let mut day_counter = NUM_DAYS_PER_MONTH_PER_LEAP_CYCLE[0..12].iter().sum();

    let mut sundays_counter = 0;
    for month in 0..(12 * 100) {
        if weekday_from_days_since_1900(day_counter) == Weekday::Sunday {
            sundays_counter += 1;
        }

        day_counter += NUM_DAYS_PER_MONTH_PER_LEAP_CYCLE[month % (12 * 4)];
    }

    sundays_counter
}

const NUM_DAYS_PER_MONTH_PER_LEAP_CYCLE: [u16; 12 * 4] = [
    31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
];

#[derive(PartialEq)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn weekday_from_days_since_1900(days: u16) -> Weekday {
    match days % 7 {
        0 => Weekday::Monday,
        1 => Weekday::Tuesday,
        2 => Weekday::Wednesday,
        3 => Weekday::Thursday,
        4 => Weekday::Friday,
        5 => Weekday::Saturday,
        6 => Weekday::Sunday,
        _ => unreachable!(),
    }
}
