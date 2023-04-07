/**
实现 pow(x, n) ，即计算 x 的整数 n 次幂函数（即，xⁿ ）。 

 

 示例 1： 

 
输入：x = 2.00000, n = 10
输出：1024.00000
 

 示例 2： 

 
输入：x = 2.10000, n = 3
输出：9.26100
 

 示例 3： 

 
输入：x = 2.00000, n = -2
输出：0.25000
解释：2-2 = 1/22 = 1/4 = 0.25
 

 

 提示： 

 
 -100.0 < x < 100.0 
 -231 <= n <= 231-1 
 n 是一个整数 
 -104 <= xⁿ <= 104 
 

 Related Topics 递归 数学 
 👍 1088 👎 0

 */
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        if x == 1.0 {
            return 1.0;
        }
        let mut product = 1.0;
        return if n < 0 {
            for i in 0..-n {
                product *= x;
            }
            1.0 / product
        } else {
            for i in 0..n {
                product *= x;
            }
            product
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
