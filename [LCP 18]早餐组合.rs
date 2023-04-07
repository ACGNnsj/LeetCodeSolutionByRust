/**
小扣在秋日市集选择了一家早餐摊位，一维整型数组 `staple` 中记录了每种主食的价格，一维整型数组 `drinks` 中记录了每种饮料的价格。小扣的计划选择
一份主食和一款饮料，且花费不超过 `x` 元。请返回小扣共有多少种购买方案。

注意：答案需要以 `1e9 + 7 (1000000007)` 为底取模，如：计算初始结果为：`1000000008`，请返回 `1`

**示例 1：**

> 输入：`staple = [10,20,5], drinks = [5,5,2], x = 15`
>
> 输出：`6`
>
> 解释：小扣有 6 种购买方案，所选主食与所选饮料在数组中对应的下标分别是：
> 第 1 种方案：staple[0] + drinks[0] = 10 + 5 = 15；
> 第 2 种方案：staple[0] + drinks[1] = 10 + 5 = 15；
> 第 3 种方案：staple[0] + drinks[2] = 10 + 2 = 12；
> 第 4 种方案：staple[2] + drinks[0] = 5 + 5 = 10；
> 第 5 种方案：staple[2] + drinks[1] = 5 + 5 = 10；
> 第 6 种方案：staple[2] + drinks[2] = 5 + 2 = 7。

**示例 2：**

> 输入：`staple = [2,1,1], drinks = [8,9,5,1], x = 9`
>
> 输出：`8`
>
> 解释：小扣有 8 种购买方案，所选主食与所选饮料在数组中对应的下标分别是：
> 第 1 种方案：staple[0] + drinks[2] = 2 + 5 = 7；
> 第 2 种方案：staple[0] + drinks[3] = 2 + 1 = 3；
> 第 3 种方案：staple[1] + drinks[0] = 1 + 8 = 9；
> 第 4 种方案：staple[1] + drinks[2] = 1 + 5 = 6；
> 第 5 种方案：staple[1] + drinks[3] = 1 + 1 = 2；
> 第 6 种方案：staple[2] + drinks[0] = 1 + 8 = 9；
> 第 7 种方案：staple[2] + drinks[2] = 1 + 5 = 6；
> 第 8 种方案：staple[2] + drinks[3] = 1 + 1 = 2；

**提示：**
+ `1 <= staple.length <= 10^5`
+ `1 <= drinks.length <= 10^5`
+ `1 <= staple[i],drinks[i] <= 10^5`
+ `1 <= x <= 2*10^5`

 Related Topics 数组 双指针 二分查找 排序 
 👍 76 👎 0

 */
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    /// 我们使用两个向量来存储主食和饮料向量中每个数字的频率。然后我们遍历 staple 向量，对于每个数字，我们在 drinks 向量中添加数字的频率，使总和等于 x
    ///
    /// Arguments:
    ///
    /// * `staple`: 表示主食的整数向量
    /// * `drinks`: [1,1,1,1,1]
    /// * `x`: 早餐的总热量
    ///
    /// Returns:
    ///
    /// 可以用给定的主食和饮料组合制作的早餐组合的数量。
    pub fn breakfast_number(staple: Vec<i32>, drinks: Vec<i32>, x: i32) -> i32 {
        let mut staple_freq = vec![0; x as usize];
        let mut drinks_freq = vec![0; x as usize];
        for s in staple {
            if s < x { staple_freq[s as usize] += 1; }
        }
        for d in drinks {
            if d < x { drinks_freq[d as usize] += 1; }
        }
        for i in 1..x {
            drinks_freq[i as usize] += drinks_freq[(i - 1) as usize];
        }
        let mut staple_num;
        let mut drink_num;
        let mut product;
        let mut total: u64 = 0;
        for i in 1..x {
            staple_num = staple_freq[i as usize];
            if staple_num == 0 { continue; }
            drink_num = drinks_freq[(x - i) as usize];
            product = staple_num * drink_num;
            total += product;
        }
        (total % 1000000007) as i32
    }
}

//leetcode submit region end(Prohibit modification and deletion)
fn main() {
    let staple = vec![10, 20, 5];
    let drinks = vec![5, 5, 2];
    let x = 15;
    let result = Solution::breakfast_number(staple, drinks, x);
    println!("{}", result);
}