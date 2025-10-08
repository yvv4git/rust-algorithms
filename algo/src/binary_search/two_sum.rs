/*
У тебя есть вектор чисел, и нужно вернуть индексы двух элементов, 
сумма которых равна заданному числу target.  
Но - нельзя использовать вложенные циклы (O(n^2)). Сделай решение за O(n log n) с сортировкой + бинарным поиском.

Подсказка:
- Отсортируй массив по значениям, сохрани оригинальные индексы
- Для каждого элемента ищи (target - num) бинарным поиском в отсортированном массиве
- Убедись, что индексы разные (не один элемент дважды)

Так можно избежать хэш-таблиц и использовать встроенный бинарный поиск.
*/
// Структура для хранения значения и его оригинального индекса
#[derive(PartialOrd, Ord, PartialEq, Eq, Clone)]
struct IndexedNum {
    value: i32,
    index: usize,
}

impl IndexedNum {
    fn new(value: i32, index: usize) -> Self {
        IndexedNum { value, index }
    }
}

fn main() {
    // Исходные данные: вектор чисел и целевая сумма
    // Time complexity: O(n log n) - сортировка + n * O(log n) поисков
    // Space complexity: O(n) - для хранения пар (значение, индекс)
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    
    // Создаём вектор структур (значение, оригинальный индекс)
    let mut indexed_nums: Vec<IndexedNum> = nums.iter().enumerate().map(|(i, &n)| IndexedNum::new(n, i)).collect();
    
    // Сортируем по значению O(n log n)
    indexed_nums.sort_unstable_by_key(|item| item.value);
    
    // Проходим по оригинальному массиву (чтобы сохранить порядок поиска)
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        
        // Бинарный поиск: ищем complement в отсортированном векторе
        // Используем std::slice::binary_search_by для поиска по value
        match indexed_nums.binary_search_by(|item| {
            if item.value == complement {
                std::cmp::Ordering::Equal
            } else if item.value < complement {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        }) {
            Ok(found_idx) => {
                let found_index = indexed_nums[found_idx].index;
                if found_index != i {  // Не тот же элемент
                    let indices = if i < found_index { format!("[{}, {}]", i, found_index) } else { format!("[{}, {}]", found_index, i) };
                    println!("Indices: {}", indices);
                    return;
                }
            }
            Err(_) => {}  // Не найдено, продолжаем
        }
    }
    
    // Если решение не найдено
    println!("No two sum solution found!");
}