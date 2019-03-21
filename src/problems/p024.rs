//! Problem 24: Lexicographic permutations

solve!(expecting_answer: "2783915460", with: || {
    let factorial = |n| (2..=n).product::<usize>();

    let mut target = 999_999;
    let mut possibilities = (0..10).collect::<Vec<_>>();
    let mut permutation = [0; 10];
    for i in 0..10 {
        let fact = factorial(10 - i - 1);
        permutation[i] = possibilities.remove(target / fact);
        target = target % fact;
    }

    permutation.iter().map(ToString::to_string).collect::<String>()
});
