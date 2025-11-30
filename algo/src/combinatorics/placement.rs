/// Генерация всех размещений k элементов из n без повторений.
///
/// Размещение - это упорядоченный набор из k различных элементов,
/// выбранных из множества из n элементов.
/// Число размещений: P(n,k) = n! / (n-k)!
/// Эта функция рекурсивно генерирует все возможные размещения.
///
/// # Аргументы
/// * `items` - Срез элементов, из которых выбираем
/// * `k` - Количество элементов в каждом размещении
///
/// # Возвращает
/// Вектор векторов, где каждый внутренний вектор - одно размещение.
///
/// # Примеры
/// ```
/// let placements = placements(&[1, 2, 3], 2);
/// // placements будет содержать: [1,2], [1,3], [2,1], [2,3], [3,1], [3,2]
/// ```
fn placements<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut current = Vec::with_capacity(k);
    let mut used = vec![false; items.len()];
    generate_placements(items, k, 0, &mut current, &mut used, &mut result);
    result
}

/// Вспомогательная рекурсивная функция для генерации размещений.
///
/// Использует backtracking: на каждом шаге выбирает следующий элемент,
/// который еще не использован, и рекурсивно продолжает.
///
/// # Аргументы
/// * `items` - Исходный срез элементов
/// * `k` - Общее количество элементов в размещении
/// * `depth` - Текущая глубина рекурсии (сколько элементов уже выбрано)
/// * `current` - Текущий набор выбранных элементов
/// * `used` - Массив флагов, какие элементы уже использованы
/// * `result` - Вектор для сбора результатов
fn generate_placements<T: Clone>(
    items: &[T],
    k: usize,
    depth: usize,
    current: &mut Vec<T>,
    used: &mut [bool],
    result: &mut Vec<Vec<T>>,
) {
    if depth == k {
        result.push(current.clone());
        return;
    }

    for i in 0..items.len() {
        if !used[i] {
            used[i] = true;
            current.push(items[i].clone());
            generate_placements(items, k, depth + 1, current, used, result);
            current.pop();
            used[i] = false;
        }
    }
}

#[test]
fn test_placement() {
    // Тест для k = 0
    assert_eq!(placements(&[1, 2, 3], 0), vec![vec![]]);

    // Тест для k = 1
    let mut p1 = placements(&[1, 2, 3], 1);
    p1.sort();
    assert_eq!(p1, vec![vec![1], vec![2], vec![3]]);

    // Тест для k = 2 из 3 элементов
    let mut p2 = placements(&[1, 2, 3], 2);
    p2.sort();
    let expected = vec![
        vec![1, 2], vec![1, 3], vec![2, 1], vec![2, 3], vec![3, 1], vec![3, 2]
    ];
    assert_eq!(p2, expected);

    // Тест для k = n (должно совпадать с перестановками)
    let mut p3 = placements(&[1, 2, 3], 3);
    p3.sort();
    let expected_full = vec![
        vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3],
        vec![2, 3, 1], vec![3, 1, 2], vec![3, 2, 1]
    ];
    assert_eq!(p3, expected_full);

    // Тест для k > n (должно быть пусто)
    assert_eq!(placements(&[1, 2], 3), vec![] as Vec<Vec<i32>>);
}