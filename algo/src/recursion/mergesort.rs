// Сортировка слиянием (Merge Sort) - алгоритм сортировки, использующий принцип "разделяй и властвуй"
// Временная сложность: O(n log n) во всех случаях
// Пространственная сложность: O(n) из-за использования дополнительной памяти для временного хранения
//
// Алгоритм:
// 1. Разделить массив на две примерно равные части
// 2. Рекурсивно отсортировать каждую часть
// 3. Слить (merge) отсортированные части в один массив

fn main() {
    let mut arr = [5, 2, 4, 6, 1, 3];
    merge_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}

// Функция сортировки слиянием
// 
// # Аргументы
// * `arr` - изменяемый срез (slice) массива для сортировки
fn merge_sort(arr: &mut [i32]) {
    // Базовый случай: массив из 0 или 1 элемента уже отсортирован
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    // Создаем копии левой и правой половин для избежания конфликтов заимствования
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();
    
    // Рекурсивно сортируем копии половин
    merge_sort(&mut left);
    merge_sort(&mut right);
    
    // Сливаем отсортированные половины обратно в исходный массив
    let mut temp = Vec::with_capacity(arr.len());
    temp.extend_from_slice(&left);
    temp.extend_from_slice(&right);
    merge(&left, &right, &mut temp);
    
    arr.copy_from_slice(&temp);
}

// Функция слияния двух отсортированных массивов
// 
// # Аргументы
// * `left` - левая половина массива
// * `right` - правая половина массива
// * `result` - результирующий массив для слияния
fn merge(left: &[i32], right: &[i32], result: &mut [i32]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    // Слияние пока есть элементы в обеих половинах
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result[k] = left[i];
            i += 1;
        } else {
            result[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    // Копирование оставшихся элементов из левой половины
    if i < left.len() {
        result[k..].copy_from_slice(&left[i..]);
    }
    
    // Копирование оставшихся элементов из правой половины
    if j < right.len() {
        result[k..].copy_from_slice(&right[j..]);
    }
}

