#[allow(unused_imports)]
use std::io;

#[test]
fn test_task1() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<u128> = input
        .split_whitespace()
        .map(|x| x.parse::<u128>().unwrap())
        .collect();

    let (p, m, c) = (nums[0], nums[1], nums[2]);

    let t1 = p / 2;                 // ограничение по программистам
    let t2 = m;                     // ограничение по математикам
    let t3 = (p + m - c) / 3;       // чтобы осталось >= C человек

    let result = t1.min(t2).min(t3);

    println!("{}", result);
}

