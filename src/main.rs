fn main() {
    println!("Hello, world!");
}

pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let mut dp: Vec<f32> = vec![f32::INFINITY; (amount + 1) as usize];
        dp[0] = 0.0;

        for coin in coins {
            for i in coin..=amount {
                dp[i as usize] = if dp[i as usize] < dp[(i - coin) as usize] + 1.0 {
                    dp[i as usize]
                } else {
                    dp[(i - coin) as usize] + 1.0
                };      
            }
        }

        let result: i32 = if dp[amount as usize] != f32::INFINITY {
            dp[amount as usize] as i32
        } else {
            -1 
        };

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_change_1() {
        let coins = vec![1, 2, 5];
        let amount = 11;
        assert_eq!(Solution::coin_change(coins, amount), 3); // 11 = 5 + 5 + 1
    }

    #[test]
    fn test_coin_change_2() {
        let coins = vec![2];
        let amount = 3;
        assert_eq!(Solution::coin_change(coins, amount), -1); // No combination can make 3
    }

    #[test]
    fn test_coin_change_3() {
        let coins = vec![1];
        let amount = 0;
        assert_eq!(Solution::coin_change(coins, amount), 0);
    }

    #[test]
    fn test_coin_change_4() {
        let coins = vec![1];
        let amount = 2;
        assert_eq!(Solution::coin_change(coins, amount), 2);
    }

    #[test]
    fn test_coin_change_5() {
        let coins = vec![1, 2, 5];
        let amount = 0;
        assert_eq!(Solution::coin_change(coins, amount), 0); 
    }

    #[test]
    fn test_coin_change_6() {
        let coins = vec![1, 2, 5];
        let amount = 100;
        assert_eq!(Solution::coin_change(coins, amount), 20);
    }
}
