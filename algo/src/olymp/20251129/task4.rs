use std::io::{self, Read};

#[allow(dead_code)]
fn main() {
    // Чтение входа
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let mut years: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        years.push(it.next().unwrap().parse::<i64>().unwrap());
    }

    if n == 1 {
        // Только год 1 — достаточно одной кометы с периодом > 0 (например > X-1)
        println!("1");
        return;
    }

    // Сделаем массив b = years - 1, но исключим ноль (год 1 -> 0), т.к. мы работаем с положительными периодами
    let mut b: Vec<i64> = years.iter().map(|&y| y - 1).filter(|&x| x > 0).collect();
    // сортировать на всякий случай (вход уже возрастающий, но после вычитания 1 порядок сохраняется)
    b.sort_unstable();

    let m = b.len();
    let mut covered = vec![false; m];
    let mut ans = 0usize;

    for i in 0..m {
        if covered[i] {
            continue;
        }
        // Нужен новый период = b[i]
        let p = b[i];
        ans += 1;
        // Пометить все элементы, кратные p
        for j in i..m {
            if !covered[j] && b[j] % p == 0 {
                covered[j] = true;
            }
        }
    }

    println!("{}", ans);
}
