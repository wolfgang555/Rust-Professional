pub fn dp_rec_mc(amount: u32) -> u32 {
    if amount == 0 {
        return 0;
    }

    let coins = [1, 2, 5, 10, 20, 30, 50, 100];
    let amount = amount as usize;

    // dp[i] 表示金额 i 的最少硬币数量
    let mut dp = vec![amount + 1; amount + 1];
    dp[0] = 0;

    // 对每个金额，尝试每个硬币
    for i in 1..=amount {
        for &coin in coins.iter() {
            if coin as usize <= i {
                dp[i] = dp[i].min(dp[i - coin as usize] + 1);
            }
        }
    }

    if dp[amount] > amount {
        return 0;
    }
    dp[amount] as u32
}
