#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::io::Read;

#[test]
fn test_task2() {
 // Считываем весь ввод
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.lines();

    // n, m
    let first = it.next().unwrap().split_whitespace().collect::<Vec<_>>();
    let n: usize = first[0].parse().unwrap();
    let m: usize = first[1].parse().unwrap();

    // строки
    let lines: Vec<String> = it.map(|s| s.to_string()).collect();

    let mut answer = usize::MAX;

    for i in 0..n - m {
        let a = lines[i].as_bytes();
        let b = lines[i + m].as_bytes();

        let mut cnt = 0;

        // сравниваем с конца
        let mut ia = a.len();
        let mut ib = b.len();

        while ia > 0 && ib > 0 {
            if a[ia - 1] == b[ib - 1] {
                cnt += 1;
                ia -= 1;
                ib -= 1;
            } else {
                break;
            }
        }

        if cnt < answer {
            answer = cnt;
        }
    }

    // Если не было ни одной пары (теоретически невозможно при 1 ≤ m < n)
    if answer == usize::MAX {
        answer = 0;
    }

    println!("{}", answer);
}


