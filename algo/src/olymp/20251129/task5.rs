#[allow(unused_imports)]
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::BinaryHeap;
use std::io::{self, Read};
use ordered_float::OrderedFloat;

#[test]
fn test_task5() {
    // чтение ввода
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let vel: f64 = it.next().unwrap().parse::<u64>().unwrap() as f64;

    let mut times = Vec::with_capacity(n);
    let mut g = Vec::with_capacity(n);

    for _ in 0..n {
        let ti = it.next().unwrap().parse::<u64>().unwrap() as f64;
        let gi = it.next().unwrap().parse::<u64>().unwrap() as f64;
        times.push(ti);
        g.push(gi);
    }

    // виртуальные требования
    let s: Vec<f64> = g.iter().map(|&x| x / vel).collect();

    let mut ans = vec![0.0_f64; n];

    // очередь активных работ: (вирт. дедлайн, id)
    let mut heap = BinaryHeap::<Reverse<(OrderedFloat<f64>, usize)>>::new();

    let mut v = 0.0_f64;
    let mut t = 0.0_f64;
    let mut k = 0_usize;
    let mut idx = 0_usize;

    while idx < n || k > 0 {
        let next_arrival = if idx < n { times[idx] } else { f64::INFINITY };

        let next_completion = if k > 0 {
            let &Reverse((d, _)) = heap.peek().unwrap();
            let d = d.0; // извлекаем f64
            t + (d - v) * (k as f64)
        } else {
            f64::INFINITY
        };

        if next_arrival <= next_completion {
            if k > 0 {
                v += (next_arrival - t) / (k as f64);
            }
            t = next_arrival;

            let d = v + s[idx];
            heap.push(Reverse((OrderedFloat(d), idx)));
            k += 1;
            idx += 1;
        } else {
            let Reverse((d, id)) = heap.pop().unwrap();
            let d = d.0;
            t = next_completion;
            v = d;
            ans[id] = t;
            k -= 1;
        }
    }

    // вывод
    let mut out = String::new();
    for x in ans {
        out.push_str(&format!("{:.12}\n", x));
    }
    print!("{out}");
}
