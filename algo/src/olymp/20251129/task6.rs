#[allow(dead_code)]
#[allow(unused_mut)]
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
    #[allow(unused_mut)]
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
