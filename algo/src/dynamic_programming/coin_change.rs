/// # Задача о размене монет (Coin Change Problem)
///
/// ## Формулировка задачи
/// У вас есть набор монет различных номиналов и сумма, которую нужно разменять.
/// Необходимо найти минимальное количество монет, необходимое для размена заданной суммы.
/// Монеты можно использовать неограниченное количество раз.
///
/// ## Пример
/// Монеты: [1, 2, 5], сумма: 11
/// Минимальное количество: 3 (5 + 5 + 1)
///
/// ## Подход с динамическим программированием
/// Используем массив dp, где dp[i] - минимальное количество монет для суммы i.
///
/// Инициализация: dp[0] = 0, dp[i] = INF для i > 0
/// Для каждой монеты coin:
///   Для каждой суммы i от coin до amount:
///     dp[i] = min(dp[i], dp[i - coin] + 1)
///
/// ## Варианты задачи
/// - Количество способов размена (Coin Change 2)
/// - Ограниченное количество монет каждого типа
///
/// Решение задачи о размене монет с использованием динамического программирования.
///
/// # Аргументы
/// * `coins` - вектор номиналов монет
/// * `amount` - сумма для размена
///
/// # Возвращает
/// Минимальное количество монет или -1, если размен невозможен
///
/// # Временная сложность
/// O(amount * len(coins)) — два вложенных цикла: внешний по монетам, внутренний по суммам от coin до amount.
///
/// # Пространственная сложность
/// O(amount) — массив dp размером amount+1.
#[allow(dead_code)]
fn coin_change(coins: &[usize], amount: usize) -> i32 {
    // println!("coins: {:?}", coins);

    const INF: i32 = i32::MAX / 2; // Избегаем переполнения
    let mut dp = vec![INF; amount + 1];
    dp[0] = 0;
    // println!("{:?}", dp);

    for &coin in coins {
        for i in coin..=amount {
            if dp[i - coin] != INF {
                dp[i] = dp[i].min(dp[i - coin] + 1);
                // println!("{:?}", dp);
            }
        }
    }

    if dp[amount] == INF { -1 } else { dp[amount] }
}

/// Вариант: количество способов размена
/// # Временная сложность
/// O(amount * len(coins)) — аналогично.
///
/// # Пространственная сложность
/// O(amount) — массив dp размером amount+1.
#[allow(dead_code)]
fn coin_change_ways(coins: &[usize], amount: usize) -> usize {
    let mut dp = vec![0; amount + 1];
    dp[0] = 1; // Один способ разменять 0

    for &coin in coins {
        for i in coin..=amount {
            dp[i] += dp[i - coin];
        }
    }

    dp[amount]
}

#[test]
fn test_coin_change_dp_debug() {
    let coins = vec![1, 2, 5];
    let amount = 11;
    assert_eq!(coin_change(&coins, amount), 3);
}


/// Тесты для функций решения задачи о размене монет
#[test]
fn test_coin_change_dp() {
    // Пример 1: стандартный случай
    // Монеты: [1, 2, 5], сумма: 11
    // Минимально: 3 монеты (5+5+1)
    let coins = vec![1, 2, 5];
    let amount = 11;
    assert_eq!(coin_change(&coins, amount), 3);
    assert_eq!(coin_change_ways(&coins, amount), 11); // 11 способов с монетами 1,2,5 для суммы 11

    // Пример 2: сумма 0
    assert_eq!(coin_change(&coins, 0), 0);
    assert_eq!(coin_change_ways(&coins, 0), 1);

    // Пример 3: невозможно разменять
    let coins = vec![2];
    let amount = 3;
    assert_eq!(coin_change(&coins, amount), -1);
    assert_eq!(coin_change_ways(&coins, amount), 0);

    // Пример 4: одна монета подходит
    let coins = vec![1, 5];
    let amount = 5;
    assert_eq!(coin_change(&coins, amount), 1);
    assert_eq!(coin_change_ways(&coins, amount), 2); // 5 или 1+1+1+1+1
}

