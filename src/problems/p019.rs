//! Problem 19: Counting Sundays

solve!(expecting_answer: 171, with: || {
    const DAYS_IN_WEEK: u16 = 7;
    const DAYS_FROM_MONDAY_TO_SUNDAY: u16 = 6;
    const NUM_DAYS_PER_MONTH_PER_LEAP_CYCLE: [u16; 12 * 4] = [
        31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
        31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
        31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
        31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    ];

    // Skip 1900
    let mut day_counter: u16 = NUM_DAYS_PER_MONTH_PER_LEAP_CYCLE[0..12].iter().sum();

    let mut sunday_counter = 0;
    for month in 0..(12 * 100) {
        if day_counter % DAYS_IN_WEEK == DAYS_FROM_MONDAY_TO_SUNDAY {
            sunday_counter += 1;
        }

        day_counter += NUM_DAYS_PER_MONTH_PER_LEAP_CYCLE[month % (12 * 4)];
    }

    sunday_counter as u128
});
