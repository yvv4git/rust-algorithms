use std::io::{self, Read};
use std::collections::HashSet;

fn select_cars(_n: usize, k: usize, a: Vec<i64>) -> Vec<i64> {
    let mut used: HashSet<i64> = HashSet::new();
    let mut ans: Vec<i64> = Vec::with_capacity(k);

    // Сначала по одному представителю каждого цвета (в порядке появления)
    for &val in &a {
        if ans.len() == k { break; }
        if !used.contains(&val) {
            used.insert(val);
            ans.push(val);
        }
    }

    // Если нужно — добираем до k любыми машинами (повторы разрешены)
    if ans.len() < k {
        for &val in &a {
            if ans.len() == k { break; }
            // можно добавлять даже уже использованные цвета
            ans.push(val);
        }
    }

    ans
}

fn main() {
    // Чтение входа
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();

    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        a.push(x);
    }

    let ans = select_cars(n, k, a);

    // Вывод ровно k чисел
    for (i, v) in ans.iter().enumerate() {
        if i > 0 { print!(" "); }
        print!("{}", v);
    }
    println!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        let a = vec![1, 2, 1, 2, 1];
        let result = select_cars(5, 3, a);
        assert_eq!(result.len(), 3);
        // Проверим, что уникальных цветов 2
        let unique: HashSet<i64> = result.iter().cloned().collect();
        assert_eq!(unique.len(), 2);
        assert!(unique.contains(&1));
        assert!(unique.contains(&2));
    }

    #[test]
    fn test_example_2() {
        let a = vec![2, 1, 8, 8, 8, 8, 8, 8, 8, 8];
        let result = select_cars(10, 4, a);
        assert_eq!(result.len(), 4);
        let unique: HashSet<i64> = result.iter().cloned().collect();
        assert_eq!(unique.len(), 3); // 1,2,8
        assert!(unique.contains(&1));
        assert!(unique.contains(&2));
        assert!(unique.contains(&8));
    }

    #[test]
    fn test_all_unique() {
        let a = vec![1, 2, 3, 4, 5];
        let result = select_cars(5, 3, a);
        assert_eq!(result.len(), 3);
        let unique: HashSet<i64> = result.iter().cloned().collect();
        assert_eq!(unique.len(), 3);
    }

    #[test]
    fn test_fewer_unique_than_k() {
        let a = vec![1, 1, 1, 1, 1];
        let result = select_cars(5, 3, a);
        assert_eq!(result.len(), 3);
        let unique: HashSet<i64> = result.iter().cloned().collect();
        assert_eq!(unique.len(), 1);
        assert!(unique.contains(&1));
    }

    #[test]
    fn test_k_equals_n() {
        let a = vec![1, 2, 3];
        let result = select_cars(3, 3, a);
        assert_eq!(result.len(), 3);
        let unique: HashSet<i64> = result.iter().cloned().collect();
        assert_eq!(unique.len(), 3);
    }
}