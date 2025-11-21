use std::io::{self, Read};

fn count_two_sum(n: i128, k: i128) -> i128 {
    // Вычисляем границы допустимых a
    let a_min = std::cmp::max(1, k - n);
    let a_max = std::cmp::min(n, (k - 1) / 2);

    if a_max >= a_min {
        (a_max - a_min + 1) as i128
    } else {
        0
    }
}


fn main() {
    // Считываем n и k
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: i128 = it.next().unwrap().parse().unwrap();
    let k: i128 = it.next().unwrap().parse().unwrap();

    let ans = count_two_sum(n, k);

    println!("{}", ans);
}


// cd algo && cargo test --bin combinatorics_two_sum_counting

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_two_sum_basic() {
        assert_eq!(count_two_sum(5, 6), 2); // (1,5), (2,4)
        assert_eq!(count_two_sum(10, 11), 5); // (1,10), (2,9), (3,8), (4,7), (5,6)
        assert_eq!(count_two_sum(2, 3), 1); // (1,2)
    }

    #[test]
    fn test_count_two_sum_edge_cases() {
        assert_eq!(count_two_sum(1, 2), 0); // нет пар, так как a <= b <=1, a+b=2 невозможно
        assert_eq!(count_two_sum(1, 1), 0); // аналогично
        assert_eq!(count_two_sum(3, 4), 1); // (1,3)
        assert_eq!(count_two_sum(4, 5), 2); // (1,4), (2,3)
    }

    #[test]
    fn test_count_two_sum_no_pairs() {
        assert_eq!(count_two_sum(1, 3), 0);
        assert_eq!(count_two_sum(2, 5), 0);
    }
}