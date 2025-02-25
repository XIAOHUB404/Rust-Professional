pub fn dp_rec_mc(amount: u32) -> u32 {
    let mut memo = vec![None; (amount + 1) as usize];
    dp_rec_mc_helper(amount, &mut memo)
}

fn dp_rec_mc_helper(amount: u32, memo: &mut Vec<Option<u32>>) -> u32 {
    let coins = vec![100, 50, 30, 20, 10, 5, 2, 1];
    
    // 如果金额为0，返回0
    if amount == 0 {
        return 0;
    }
    
    // 检查是否已经计算过这个金额
    if let Some(cached) = memo[amount as usize] {
        return cached;
    }
    
    let mut min_coins = u32::MAX;
    
    // 遍历所有可能的硬币
    for &coin in coins.iter() {
        if coin <= amount {
            // 递归计算剩余金额所需的最小硬币数
            let sub_problem = dp_rec_mc_helper(amount - coin, memo);
            println!("{sub_problem}");
            // 如果子问题有解，更新最小硬币数
            if sub_problem != u32::MAX {
                min_coins = min_coins.min(1 + sub_problem);
            }
        }
    }
    
    // 存储计算结果到缓存
    memo[amount as usize] = Some(min_coins);
    min_coins
}
