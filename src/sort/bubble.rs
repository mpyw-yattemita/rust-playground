use std::cmp::Ordering;

pub fn main() {
    let mut numbers = [8, 1, 4, 2, 3, 2, 4];
    let mut numbers_vec = vec![8, 1, 4, 2, 3, 2, 4];

    println!("Initial:");
    println!("{:?}", numbers);
    println!("{:?}", numbers_vec);
    println!();

    // ascending
    numbers.bubble_sort();
    numbers_vec.bubble_sort();
    println!("Sort by asc:");
    println!("{:?}", numbers);
    println!("{:?}", numbers_vec);
    println!();
    assert_eq!([1, 2, 2, 3, 4, 4, 8], numbers);
    assert_eq!(vec![1, 2, 2, 3, 4, 4, 8], numbers_vec);

    // ascending
    numbers.bubble_sort_by(|a, b| a.cmp(b));
    numbers_vec.bubble_sort_by(|a, b| a.cmp(b));
    println!("Sort by asc:");
    println!("{:?}", numbers);
    println!("{:?}", numbers_vec);
    println!();
    assert_eq!([1, 2, 2, 3, 4, 4, 8], numbers);
    assert_eq!(vec![1, 2, 2, 3, 4, 4, 8], numbers_vec);

    // descending
    numbers.bubble_sort_by(|a, b| a.cmp(b).reverse());
    numbers_vec.bubble_sort_by(|a, b| a.cmp(b).reverse());
    println!("Sort by desc:");
    println!("{:?}", numbers);
    println!("{:?}", numbers_vec);
    println!();
    assert_eq!([8, 4, 4, 3, 2, 2, 1], numbers);
    assert_eq!(vec![8, 4, 4, 3, 2, 2, 1], numbers_vec);
}

trait BubbleSort<T> {
    fn bubble_sort(&mut self);
    fn bubble_sort_by(&mut self, sort_by: fn(&T, &T) -> Ordering);
}

impl<T> BubbleSort<T> for [T] where T: Ord {
    fn bubble_sort(&mut self) {
        self.bubble_sort_by(|a, b| a.cmp(b))
    }
    fn bubble_sort_by(&mut self, sort_by: fn(&T, &T) -> Ordering) {
        let min = 0;
        let max = self.len() - 1;

        for i in min..=max {
            for j in (i + 1..=max).rev() {
                if sort_by(&self[j - 1], &self[j]).is_gt() {
                    self.swap(j, j - 1);
                }
            }
        }
    }
}
