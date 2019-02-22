//! Problem 3: Largest prime factor

solve!(expecting_answer: 6857, with: || {
    let mut target = 600_851_475_143;
    let mut generator = (2..3).chain((3..target).step_by(2));
    let limit = |n| (n as f64).sqrt() as u128;

    let mut last_factor = 2;
    while let Some(factor) = generator.next().filter(|&n| n <= limit(target)) {
        while target % factor == 0 { target /= factor }
        last_factor = factor;
    }

    if target == 1 { last_factor }
    else { target }
});
