fn permutation(chars: Vec<char>) -> Vec<String> {
    (0..chars.len())
        .flat_map(|drain_at| {
            let mut chars = chars.clone();
            let drained = chars.drain(drain_at..drain_at + 1).last().unwrap();
            if chars.is_empty() {
                vec![drained.to_string()]
            } else {
                permutation(chars)
                    .iter()
                    .map(|rest| drained.to_string() + rest)
                    .collect::<Vec<_>>()
            }
        })
        .collect::<Vec<_>>()
}

pub fn main() {
    let mut chars = "GAKKOU"
        .chars()
        .collect::<Vec<_>>();
    chars.sort();

    let mut result = permutation(chars);
    result.sort();
    result.dedup();

    for (idx, item) in result.iter().enumerate() {
        println!("{}: {}", idx + 1, item);
        if idx == 99 {
            break;
        }
    }
}
