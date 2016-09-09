//! Tests for problems 11 through 20

#[test]
fn p011() {
    use resource::Resource;
    use mat::Direction;

    let buffer: String = Resource::new().get_with_newlines("p011");
    let matrix = buffer.parse::<::Matrix<u32>>().unwrap();

    // let c = matrix.get_n_adjacent_indexes(16, 3, Direction::Right).unwrap();
    // let e = matrix.get_elements(c);
    // assert_eq!("ss", format!("{:?}", e));

    // let iter = matrix.enumerate();
    let iter = matrix.clone().enumerate();
    let mut vecs: Vec<Vec<u32>> = Vec::new();
    let directions = vec![Direction::Right, Direction::DownRight, Direction::Down, Direction::DownLeft];
    for (index, val) in iter {
        for direction in &directions {
            if let Some(adj_values) = matrix.as_ref().get_n_adjacent_indexes(index, 3, direction) {
                if let Some(mut vals) = matrix.get_elements(adj_values) {
                    vals.push(val);
                    vecs.push(vals);
                }
            }
        }
    }

    let max: u32 = vecs.iter().map(|ref vec| vec.iter().product()).max().unwrap();

    assert_eq!(70600674, max);

    // assert_eq!(vec![26, 47, 68], c);
    // assert_eq!(vec![97, 38, 15], e);
}
