pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut prev = 1;  // 第一个数
    let mut curr = 2;  // 第二个数
    let mut sum = 2;   // 从2开始，因为前两个数都要计入和

    // 当当前数不超过阈值时继续计算
    while curr <= threshold {
        // 计算下一个斐波那契数
        let next = prev + curr;
        prev = curr;
        curr = next;

        // 如果当前数不超过阈值，加入到和中
        if curr <= threshold && curr % 2 == 1 {
            sum += curr;
        }
    }

    sum
}
