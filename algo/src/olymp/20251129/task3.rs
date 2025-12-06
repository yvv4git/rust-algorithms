use std::io::{self, Read};

#[allow(dead_code)]
fn main() {
    // ---------- чтение ввода ----------
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut x = vec![0i64; n];
    let mut y = vec![0i64; n];

    for i in 0..n {
        x[i] = it.next().unwrap().parse().unwrap();
        y[i] = it.next().unwrap().parse().unwrap();
    }

    // ---------- предварительный расчёт квадратов расстояний ----------
    let mut dist = vec![vec![0i64; n]; n];
    for i in 0..n {
        for j in 0..n {
            let dx = x[i] - x[j];
            let dy = y[i] - y[j];
            dist[i][j] = dx*dx + dy*dy;
        }
    }

    // ---------- симуляция для всех возможных стартов ----------
    let mut best = i64::MAX;

    for start in 0..n {
        let mut used = vec![false; n];
        used[start] = true;
        let mut cur = start;
        let mut time_sum = 0i64;

        for _ in 1..n {
            let mut nxt = None;
            let mut best_d = i64::MAX;

            // ищем ближайший непосещённый
            for j in 0..n {
                if !used[j] {
                    let d = dist[cur][j];
                    if d < best_d || (d == best_d && Some(j) < nxt) {
                        best_d = d;
                        nxt = Some(j);
                    }
                }
            }

            let j = nxt.unwrap();
            used[j] = true;
            time_sum += best_d;
            cur = j;
        }

        if time_sum < best {
            best = time_sum;
        }
    }

    // ---------- вывод ----------
    println!("{}", best);
}
