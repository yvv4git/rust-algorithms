use std::io::{self, BufRead};

#[allow(dead_code)]
fn weird_algorithm(n: u64) -> Vec<u64> {
    let mut current = n;
    let mut result = vec![current];

    while current != 1 {
        if current % 2 == 0 {
            current /= 2;
        } else {
            current = current * 3 + 1;
        }
        result.push(current);
    }

    result
}

#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    let n: u64 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let result = weird_algorithm(n);

    for (i, &num) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", num);
    }
    println!();
}

#[allow(dead_code)]
#[test]
fn test_task1_Weird_Algorithm() {
    assert_eq!(weird_algorithm(3), vec![3, 10, 5, 16, 8, 4, 2, 1]);
    assert_eq!(weird_algorithm(1), vec![1]);
    assert_eq!(weird_algorithm(2), vec![2, 1]);
}