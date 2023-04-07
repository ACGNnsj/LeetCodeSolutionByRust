/**
给定两个整数 n 和 k，返回范围 [1, n] 中所有可能的 k 个数的组合。 

 你可以按 任何顺序 返回答案。 

 

 示例 1： 

 
输入：n = 4, k = 2
输出：
[
  [2,4],
  [3,4],
  [2,3],
  [1,2],
  [1,3],
  [1,4],
] 

 示例 2： 

 
输入：n = 1, k = 1
输出：[[1]] 

 

 提示： 

 
 1 <= n <= 20 
 1 <= k <= n 
 

 Related Topics 回溯 
 👍 1232 👎 0

 */
struct Solution;
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    /// 从一组 n 个元素生成 k 个元素的所有组合的函数。
    /// 
    /// Arguments:
    /// 
    /// * `n`: 集合中元素的数量
    /// * `k`: 每个子集中的元素数
    /// 
    /// Returns:
    /// 
    /// 整数向量的向量。
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut power = vec![];
        let mut ele = vec![];
        power.push(ele);
        for i in 0..n {
            for j in 0..power.len() {
                ele = power[j].clone();
                /// 避免向量长度大于k的情况。
                if ele.len() == k as usize { continue; }
                ele.push(i + 1);
                power.push(ele);
            }
        }
        power.retain(|x| x.len() == k as usize);
        power
    }
}

//leetcode submit region end(Prohibit modification and deletion)
fn main() {
    let vec=vec![1;4];
    println!("{:?}",&vec.len());
    let i=3;
    println!("{:?}",&i>&1);
    println!("{:?}",&i<&1);
}