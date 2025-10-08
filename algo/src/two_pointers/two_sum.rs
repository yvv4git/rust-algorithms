/*
У тебя есть вектор чисел, и нужно вернуть индексы двух элементов, 
сумма которых равна заданному числу target.  
Но - нельзя использовать вложенные циклы (O(n^2)). Сделай решение за O(n log n) с two pointers.

Подсказка:
- Сначала отсортируй массив по значениям, но сохрани оригинальные индексы
- Используй два указателя: left на начало, right на конец
- Двигай указатели в зависимости от суммы

Так можно пройтись по массиву эффективно после сортировки.
*/


fn main() {
    // Исходные данные: вектор чисел и целевая сумма
    // Time complexity: O(n log n) - из-за сортировки + O(n) на two pointers
    // Space complexity: O(n) - для хранения пар (значение, индекс)
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    
    // Создаём вектор пар (значение, оригинальный индекс)
    let mut indexed_nums: Vec<(i32, usize)> = nums.iter().enumerate().map(|(i, &n)| (n, i)).collect();
    
    // Сортируем по значению O(n log n)
    indexed_nums.sort_by_key(|&(val, _)| val);
    
    // Два указателя
    let mut left = 0;
    let mut right = indexed_nums.len() - 1;
    
    // Пока указатели не встретились
    while left < right {
        // Вычисляем сумму
        let sum = indexed_nums[left].0 + indexed_nums[right].0;
        
        if sum == target {
            // Если сумма равна target - выводим отсортированные индексы и завершаем программу
            let idx1 = indexed_nums[left].1;
            let idx2 = indexed_nums[right].1;
            let indices = if idx1 < idx2 { format!("[{}, {}]", idx1, idx2) } else { format!("[{}, {}]", idx2, idx1) };
            println!("Indices: {}", indices);
            return;
        } else if sum < target {
            // Если сумма меньше - двигаем left вправо, чтобы увеличить сумму
            left += 1;
        } else {
            // Если сумма больше - двигаем right влево, чтобы уменьшить сумму
            right -= 1;
        }
    }
    
    // Если решение не найдено
    println!("No two sum solution found!");
}