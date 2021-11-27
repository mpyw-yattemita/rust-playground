pub fn main() {
    let mut numbers = [8, 1, 4, 2, 3, 2, 4];
    let mut numbers_vec = vec![8, 1, 4, 2, 3, 2, 4];

    println!("Initial:");
    println!("{:?}", numbers);
    println!("{:?}", numbers_vec);
    println!();

    // ascending
    numbers.sort();
    numbers_vec.sort();
    println!("Sort by asc:");
    println!("{:?}", numbers);
    println!("{:?}", numbers_vec);
    println!();
    assert_eq!([1, 2, 2, 3, 4, 4, 8], numbers);
    assert_eq!(vec![1, 2, 2, 3, 4, 4, 8], numbers_vec);

    // ascending
    numbers.sort_by(|a, b| a.cmp(b));
    numbers_vec.sort_by(|a, b| a.cmp(b));
    println!("Sort by asc:");
    println!("{:?}", numbers);
    println!("{:?}", numbers_vec);
    println!();
    assert_eq!([1, 2, 2, 3, 4, 4, 8], numbers);
    assert_eq!(vec![1, 2, 2, 3, 4, 4, 8], numbers_vec);

    // descending
    numbers.sort_by(|a, b| a.cmp(b).reverse());
    numbers_vec.sort_by(|a, b| a.cmp(b).reverse());
    println!("Sort by desc:");
    println!("{:?}", numbers);
    println!("{:?}", numbers_vec);
    println!();
    assert_eq!([8, 4, 4, 3, 2, 2, 1], numbers);
    assert_eq!(vec![8, 4, 4, 3, 2, 2, 1], numbers_vec);
}
