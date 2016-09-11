//! Tests for problems 11 through 20

#[test]
fn p011() {
    use resource::Resource;
    use mat::Direction;

    let buffer: String = Resource::new().get_with_newlines("p011");
    let matrix = buffer.parse::<::Matrix<u32>>().unwrap();

    let directions = [Direction::Right, Direction::DownRight, Direction::Down, Direction::DownLeft];
    let mut products: Vec<u32> = Vec::new();

    for (index, item) in matrix.clone().enumerate() {
        for direction in &directions {
            if let Some(adjacent_indexes) = matrix.as_ref().adjacent_indexes(index, 3, direction) {
                if let Some(adjacent_items) = matrix.items_at_indexes(adjacent_indexes) {
                    products.push(adjacent_items.iter().product::<u32>() * item);
                }
            }
        }
    }

    assert_eq!(&70_600_674, products.iter().max().unwrap());
}

#[test]
fn p012() {
    let divisors = |num: u64| {
        let mut vec = (1..(num / 2 + 1)).filter(|i| num % i == 0).collect::<Vec<u64>>();
        vec.push(num);
        vec
    };

    let answer = ::TriangleSequence::new()
        .map(|i| (i, divisors(i)))
        .find(|&(_, ref divisors)| divisors.len() > 500)
        .map(|(i, _)| i)
        .unwrap();

    assert_eq!(76_576_500, answer);
}
