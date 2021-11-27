use std::cmp::Ordering;

pub fn main() {
    let mut numbers = vec![22, 14, 16, 4, 30, 14, 29, 13, 16, 8, 5, 15, 22, 30, 24, 14, 14, 8, 11, 19, 30, 19, 12, 22, 7, 2, 12, 24, 17, 26];

    println!("Initial:");
    println!("{:?}", numbers);
    println!();

    // ascending
    numbers.quick_sort();
    println!("Sort by asc:");
    println!("{:?}", numbers);
    println!();
    assert_eq!(vec![2, 4, 5, 7, 8, 8, 11, 12, 12, 13, 14, 14, 14, 14, 15, 16, 16, 17, 19, 19, 22, 22, 22, 24, 24, 26, 29, 30, 30, 30], numbers);

    // ascending
    numbers.quick_sort_by(|a, b| a.cmp(b));
    println!("Sort by asc:");
    println!("{:?}", numbers);
    println!();
    assert_eq!(vec![2, 4, 5, 7, 8, 8, 11, 12, 12, 13, 14, 14, 14, 14, 15, 16, 16, 17, 19, 19, 22, 22, 22, 24, 24, 26, 29, 30, 30, 30], numbers);

    // descending
    numbers.quick_sort_by(|a, b| a.cmp(b).reverse());
    println!("Sort by desc:");
    println!("{:?}", numbers);
    println!();
    assert_eq!(vec![30, 30, 30, 29, 26, 24, 24, 22, 22, 22, 19, 19, 17, 16, 16, 15, 14, 14, 14, 14, 13, 12, 12, 11, 8, 8, 7, 5, 4, 2], numbers);
}

trait QuickSort<T> {
    fn quick_sort(&mut self);
    fn quick_sort_by(&mut self, sort_by: fn(&T, &T) -> Ordering);
}

impl<T> QuickSort<T> for [T] where T: Ord + Clone {
    fn quick_sort(&mut self) {
        self.quick_sort_by(|a, b| a.cmp(b))
    }
    fn quick_sort_by(&mut self, sort_by: fn(&T, &T) -> Ordering) {
        let mut stack = vec![0..self.len()];

        while let Some(range) = stack.pop() {
            // 終了条件: 比較対象なし
            if range.len() < 2 {
                continue;
            }

            let items = &mut self[range];

            let left_min = 0;
            let right_max = items.len() - 1;
            let pivot = items[(left_min + right_max) >> 1].clone();

            let mut left_cursor = left_min;
            let mut right_cursor = right_max;

            loop {
                // 境界値「未満」の間，左カーソルが右側に進む
                while left_cursor < right_max && sort_by(&items[left_cursor], &pivot).is_lt() {
                    left_cursor += 1;
                }
                // 境界値「超過」の間，右カーソルが左側に進む
                while left_min < right_cursor && sort_by(&pivot, &items[right_cursor]).is_lt() {
                    right_cursor -= 1;
                }
                // カーソルが重なったら境界値に到達した判定
                if right_cursor <= left_cursor {
                    break;
                }
                // 入れ替えてからカーソルを次に進める
                items.swap(left_cursor, right_cursor);
                if left_cursor < right_max {
                    left_cursor += 1;
                }
                if left_min < right_cursor {
                    right_cursor -= 1;
                }
            }

            // 境界値「未満」の集合に対してループ
            stack.push(left_min..left_cursor);
            // 境界値「超過」の集合に対してループ
            stack.push(right_cursor + 1..right_max + 1);
        }
    }
}
