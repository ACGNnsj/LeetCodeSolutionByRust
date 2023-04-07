/**
三步问题。有个小孩正在上楼梯，楼梯有n阶台阶，小孩一次可以上1阶、2阶或3阶。实现一种方法，计算小孩有多少种上楼梯的方式。结果可能很大，你需要对结果模10000
00007。 

 示例1: 

 
 输入：n = 3 
 输出：4
 说明: 有四种走法
 

 示例2: 

 
 输入：n = 5
 输出：13
 

 提示: 

 
 n范围在[1, 1000000]之间 
 

 Related Topics 记忆化搜索 数学 动态规划 
 👍 94 👎 0

 */

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn ways_to_step(n: i32) -> i32 {
        let mut series: Vec<u64> = vec![1, 2, 4];
        if n < 4 {
            return series[(n - 1) as usize] as i32;
        }
        for i in 3..n as usize {
            series.push((series[i - 1] + series[i - 2] + series[i - 3]) % 1000000007);
        }
        series[n as usize - 1] as i32
    }
}

//leetcode submit region end(Prohibit modification and deletion)
fn main() {}