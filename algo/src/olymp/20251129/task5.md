# E. Загрузка бэкапов

**Ограничение времени:** 2 секунды
**Ограничение памяти:** 256 Мб
**Ввод:** стандартный ввод или input.txt
**Вывод:** стандартный вывод или output.txt

Загрузка бэкапов на сервер происходит по расписанию. Всего будет загружено n бэкапов, i-й бэкап занимает g_i гигабайт и начинает загружаться на сервер в момент времени t_i. Скорость загрузки данных на сервер равна v гигабайт в секунду. Данные загружаются параллельно, т.е. если в какой-то момент одновременно загружаются k бэкапов, то загрузка каждого из них происходит со скоростью v/k гигабайт в секунду.

Для каждого из бэкапов определите, в какой момент закончится его загрузка на сервер.

## Формат ввода
В первой строке вводятся два целых числа n, v (1 ≤ n ≤ 100000, 1 ≤ v ≤ 10^9) — количество бэкапов и скорость загрузки на сервер в гигабайтах в секунду.

В следующих n строках вводится информация о бэкапах: в i+1-й строке вводится два целых числа t_i, g_i (1 ≤ t_i, g_i ≤ 10^9) — время начала загрузки i-го бэкапа в секундах и его размер в гигабайтах.

Гарантируется, что для всех 1 ≤ i ≤ n−1 верно, что t_i ≤ t_{i+1}.

## Формат вывода
Выведите n чисел, i-е из которых означает время окончания загрузки i-го бэкапа в секундах.

Ответ будет считаться верным, если он имеет относительную или абсолютную погрешность не более 10^{-6}.

## Система оценивания
Решения, верно работающие при n ≤ 100 и если все загрузки начинаются в момент времени 1, будут набирать не менее 24 баллов.

Решения, верно работающие при n ≤ 1000, будут набирать не менее 70 баллов.

## Примеры

### Пример 1
**Ввод**
2 2
1 1
3 1

**Вывод**
1.50000000000000000
3.50000000000000000

### Пример 2
**Ввод**
2 1
1 2
2 2

**Вывод**
4.00000000000000000
5.00000000000000000



## Solution
```
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};
use ordered_float::OrderedFloat;

fn main() {
    // чтение ввода
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let v: f64 = it.next().unwrap().parse::<u64>().unwrap() as f64;

    let mut t = Vec::with_capacity(n);
    let mut g = Vec::with_capacity(n);

    for _ in 0..n {
        let ti = it.next().unwrap().parse::<u64>().unwrap() as f64;
        let gi = it.next().unwrap().parse::<u64>().unwrap() as f64;
        t.push(ti);
        g.push(gi);
    }

    // виртуальные требования
    let mut s: Vec<f64> = g.iter().map(|&x| x / v).collect();

    let mut ans = vec![0.0_f64; n];

    // очередь активных работ: (вирт. дедлайн, id)
    let mut heap = BinaryHeap::<Reverse<(OrderedFloat<f64>, usize)>>::new();

    let mut V = 0.0_f64;
    let mut T = 0.0_f64;
    let mut k = 0_usize;
    let mut idx = 0_usize;

    while idx < n || k > 0 {
        let next_arrival = if idx < n { t[idx] } else { f64::INFINITY };

        let next_completion = if k > 0 {
            let &Reverse((d, _)) = heap.peek().unwrap();
            let d = d.0; // извлекаем f64
            T + (d - V) * (k as f64)
        } else {
            f64::INFINITY
        };

        if next_arrival <= next_completion {
            if k > 0 {
                V += (next_arrival - T) / (k as f64);
            }
            T = next_arrival;

            let d = V + s[idx];
            heap.push(Reverse((OrderedFloat(d), idx)));
            k += 1;
            idx += 1;
        } else {
            let Reverse((d, id)) = heap.pop().unwrap();
            let d = d.0;
            T = next_completion;
            V = d;
            ans[id] = T;
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
```

## Solution-2
```
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Key {
    g: f64,
    id: usize,
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.g == other.g
    }
}
impl Eq for Key {}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let c = self.g.partial_cmp(&other.g);
        if c == Some(Ordering::Equal) {
            Some(self.id.cmp(&other.id))
        } else {
            c
        }
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    // чтение
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let v: f64 = it.next().unwrap().parse::<u64>().unwrap() as f64;

    let mut t = vec![0.0_f64; n];
    let mut g = vec![0.0_f64; n];
    for i in 0..n {
        t[i] = it.next().unwrap().parse::<u64>().unwrap() as f64;
        g[i] = it.next().unwrap().parse::<u64>().unwrap() as f64;
    }

    let mut ans = vec![0.0_f64; n];
    let mut g_left = g.clone();

    let mut active = BTreeSet::<Key>::new();

    let mut T = 0.0_f64;
    let mut idx = 0usize;

    while idx < n || !active.is_empty() {
        let next_arrival = if idx < n { t[idx] } else { f64::INFINITY };

        // ближайшее завершение
        let next_finish = if let Some(first) = active.first() {
            let gmin = first.g;
            let k = active.len() as f64;
            T + gmin * k / v
        } else {
            f64::INFINITY
        };

        if next_arrival <= next_finish {
            // моделируем до прихода
            let dt = next_arrival - T;
            if !active.is_empty() {
                let k = active.len() as f64;
                let dec = dt * v / k;

                // уменьшаем у всех, пересобираем дерево
                let vkeys: Vec<Key> = active.iter().cloned().collect();
                active.clear();
                for k0 in vkeys {
                    let newg = k0.g - dec;
                    g_left[k0.id] = newg;
                    active.insert(Key { g: newg, id: k0.id });
                }
            }
            T = next_arrival;
            active.insert(Key { g: g_left[idx], id: idx });
            idx += 1;
        } else {
            // моделируем до завершения
            let dt = next_finish - T;
            let kf = active.len() as f64;
            let dec = dt * v / kf;

            let vkeys: Vec<Key> = active.iter().cloned().collect();
            active.clear();
            for k0 in vkeys {
                let newg = k0.g - dec;
                g_left[k0.id] = newg;
                active.insert(Key { g: newg, id: k0.id });
            }

            T = next_finish;

            // снимаем минимальный
            let first = *active.first().unwrap();
            active.remove(&first);
            ans[first.id] = T;
        }
    }

    for x in ans {
        println!("{:.12}", x);
    }
}
```


## Solution-3
```
package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"os"
)

type Task struct {
	deadline float64
	index    int
}

// Min-heap
type TaskHeap []Task

func (h TaskHeap) Len() int            { return len(h) }
func (h TaskHeap) Less(i, j int) bool  { return h[i].deadline < h[j].deadline }
func (h TaskHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *TaskHeap) Push(x interface{}) { *h = append(*h, x.(Task)) }
func (h *TaskHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

func main() {
	reader := bufio.NewReader(os.Stdin)
	var n int
	var v float64
	fmt.Fscanf(reader, "%d %f\n", &n, &v)

	t := make([]float64, n)
	g := make([]float64, n)
	for i := 0; i < n; i++ {
		fmt.Fscanf(reader, "%f %f\n", &t[i], &g[i])
	}

	ans := make([]float64, n)
	h := &TaskHeap{}
	heap.Init(h)

	T := 0.0 // реальное время
	V := 0.0 // виртуальное время
	idx := 0

	for idx < n || h.Len() > 0 {
		nextArrival := 1e18
		if idx < n {
			nextArrival = t[idx]
		}

		nextCompletion := 1e18
		if h.Len() > 0 {
			nextCompletion = T + ((*h)[0].deadline-V)*float64(h.Len())
		}

		if nextArrival <= nextCompletion {
			if h.Len() > 0 {
				V += (nextArrival - T) / float64(h.Len())
			}
			T = nextArrival
			heap.Push(h, Task{deadline: V + g[idx]/v, index: idx})
			idx++
		} else {
			top := heap.Pop(h).(Task)
			T = nextCompletion
			V = top.deadline
			ans[top.index] = T
		}
	}

	for _, x := range ans {
		fmt.Printf("%.12f\n", x)
	}
}

```