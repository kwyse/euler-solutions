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
    // Skip 1900
    let mut day_counter: u16 = NUM_DAYS_PER_MONTH_PER_LEAP_CYCLE[0..12].iter().sum();

    let mut sunday_counter = 0;
    for month in 0..(12 * 100) {
        if day_counter % DAYS_IN_WEEK == DAYS_FROM_MONDAY_TO_SUNDAY {
            sunday_counter += 1;
        }

        day_counter += NUM_DAYS_PER_MONTH_PER_LEAP_CYCLE[month % (12 * 4)];
    }

    sunday_counter
}

const DAYS_IN_WEEK: u16 = 7;
const DAYS_FROM_MONDAY_TO_SUNDAY: u16 = 6;
const NUM_DAYS_PER_MONTH_PER_LEAP_CYCLE: [u16; 12 * 4] = [
    31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
];
