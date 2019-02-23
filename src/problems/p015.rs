//! Problem 15: Lattice paths

solve!(expecting_answer: 137_846_528_820, with: || {
    let mut paths: u128 = 1;
    for i in 0..20 {
        paths = paths * (40 - i) / (i + 1);
    }

    paths
});
