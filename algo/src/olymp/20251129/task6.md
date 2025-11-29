# F. Взлом процессора

**Ограничение времени:** 6 секунд
**Ограничение памяти:** 1024 Мб
**Ввод:** стандартный ввод или input.txt
**Вывод:** стандартный вывод или output.txt

Вася изучает процессор с N 32-битными регистрами, занумерованными числами от 1 до N. Каждый регистр может хранить числа от 0 до 2^32 − 1. Архитектура процессора поддерживает две операции:

1 K M — циклически сдвинуть биты регистра с номером K на M позиций вправо. Для примера рассмотрим не 32-битный регистр, а 8-битный. Если значение в регистре было 00010111 и он был циклически сдвинут вправо на 2, то его новое значение будет 11000101.

2 I J — посчитать побитовое исключающее или для регистров с номерами I и J и отправить его на системную шину.

Вася знает, какая программа исполнялась и получил доступ ко всем результатам операций второго типа. Помогите ему определить возможные значения регистров до начала выполнения программы.

## Формат ввода
В первой строке содержится два числа N и Q (1 ≤ N, Q ≤ 100000) — количество регистров процессора и операций в программе соответственно.

В следующих Q строках содержится описание операций, как в условии задачи. При этом гарантируется, что все описания корректны, сдвиг возможен не более чем на 31 бит. После каждой операции второго типа следует неотрицательное число в десятичной системе счисления — результат выполнения операции исключающего или для указанных регистров.

## Формат вывода
Выведите N чисел в десятичной системе счисления — возможные значения регистров. Если возможных ответов несколько — выведите любой. Если ни один набор значений регистров процессора не может дать таких результатов, то выведите число −1.

## Система оценивания
Решения, верно работающие при N, Q ≤ 1000, будут получать не менее 40 баллов.

## Примеры

### Пример 1
**Ввод**
3 3
2 1 2
1
2 1 3
2
2 2 3
3

**Вывод**
0 1 2

### Пример 2
**Ввод**
4 6
2 4 2
3
2 4 1
6
1 3 1
2 3 1
2
1 2 2
2 2 3
7

**Вывод**
5 0 14 3

### Пример 3
**Ввод**
5 6
2 4 2
10
2 5 3
2
2 2 3
1
2 1 4
3
1 3 1
2 3 4
2147483663

**Вывод**
15 6 7 12 5



## Solution
```
use std::io::{self, Read};

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        Self { buf: s.into_bytes(), idx: 0 }
    }
    fn ws(b: u8) -> bool { b == b' ' || b == b'\n' || b == b'\r' || b == b'\t' }
    fn next_u64(&mut self) -> u64 {
        while self.idx < self.buf.len() && Self::ws(self.buf[self.idx]) { self.idx += 1; }
        let mut v = 0u64;
        while self.idx < self.buf.len() && !Self::ws(self.buf[self.idx]) {
            v = v * 10 + (self.buf[self.idx] - b'0') as u64;
            self.idx += 1;
        }
        v
    }
    fn next_usize(&mut self) -> usize { self.next_u64() as usize }
}

struct XorDsu {
    p: Vec<usize>,
    x: Vec<u8>,
    sz: Vec<usize>,
}
impl XorDsu {
    fn new(n: usize) -> Self {
        let mut p = vec![0; n];
        for i in 0..n { p[i] = i; }
        Self { p, x: vec![0; n], sz: vec![1; n] }
    }
    // найти корень и xor до него
    fn find(&mut self, mut v: usize) -> (usize, u8) {
        let mut xr = 0u8;
        let mut u = v;
        while self.p[u] != u {
            xr ^= self.x[u];
            u = self.p[u];
        }
        let r = u;
        // сжатие пути
        let mut cur = v;
        let mut acc = 0u8;
        while self.p[cur] != cur {
            let par = self.p[cur];
            let old = self.x[cur];
            self.p[cur] = r;
            self.x[cur] = xr ^ acc;
            acc ^= old;
            cur = par;
        }
        (r, xr)
    }
    // уравнение: val[u] xor val[v] = w
    fn unite(&mut self, u: usize, v: usize, w: u8) -> bool {
        let (ru, xu) = self.find(u);
        let (rv, xv) = self.find(v);
        if ru == rv { return (xu ^ xv) == w; }
        let mut ru = ru;
        let mut rv = rv;
        let mut xu = xu;
        let mut xv = xv;
        if self.sz[ru] > self.sz[rv] {
            std::mem::swap(&mut ru, &mut rv);
            std::mem::swap(&mut xu, &mut xv);
        }
        self.p[ru] = rv;
        self.x[ru] = xu ^ xv ^ w;
        self.sz[rv] += self.sz[ru];
        true
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.next_usize();
    let q = sc.next_usize();
    let mut dsu = XorDsu::new(n * 32);
    let mut off = vec![0usize; n]; // накопленные циклические сдвиги
    let mut ok = true;

    for _ in 0..q {
        let t = sc.next_usize();
        if t == 1 {
            let k = sc.next_usize() - 1;
            let m = sc.next_usize() % 32;
            off[k] = (off[k] + m) % 32;
        } else {
            let i = sc.next_usize() - 1;
            let j = sc.next_usize() - 1;
            let x = sc.next_u64();
            for p in 0..32 {
                let a = i * 32 + (p + off[i]) % 32;
                let b = j * 32 + (p + off[j]) % 32;
                let bit = ((x >> p) & 1) as u8;
                if !dsu.unite(a, b, bit) { ok = false; break; }
            }
            if !ok { break; }
        }
    }

    if !ok {
        println!("-1");
        return;
    }

    let mut bits = vec![0u8; n * 32];
    for v in 0..n * 32 {
        let (_, xr) = dsu.find(v);
        bits[v] = xr;
    }

    let mut out = String::new();
    for r in 0..n {
        let mut val = 0u64;
        for b in 0..32 {
            if bits[r * 32 + b] == 1 { val |= 1u64 << b; }
        }
        out.push_str(&format!("{}{}", if r > 0 { " " } else { "" }, val));
    }
    println!("{}", out);
}
```