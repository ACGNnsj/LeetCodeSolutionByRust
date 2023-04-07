/**
给你一个整数 x ，如果 x 是一个回文整数，返回 true ；否则，返回 false 。 

 回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。 

 
 例如，121 是回文，而 123 不是。 
 

 

 示例 1： 

 
输入：x = 121
输出：true
 

 示例 2： 

 
输入：x = -121
输出：false
解释：从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
 

 示例 3： 

 
输入：x = 10
输出：false
解释：从右向左读, 为 01 。因此它不是一个回文数。
 

 

 提示： 

 
 -2³¹ <= x <= 2³¹ - 1 
 

 

 进阶：你能不将整数转为字符串来解决这个问题吗？ 

 Related Topics 数学 
 👍 2344 👎 0

 */
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut digits = Vec::new();
        let mut x = x;
        while x > 0 {
            digits.push(x % 10);
            x /= 10;
        }
        let length = digits.len();
        for i in 0..length / 2 {
            if digits[i] != digits[length - 1 - i] {
                return false;
            }
        }
        true
    }
}

//leetcode submit region end(Prohibit modification and deletion)
fn main() {
    let mut digits = Vec::new();
    let mut x = 13033;
    while x > 0 {
        println!("{}", x);
        println!("{}", x % 10);
        digits.push(x % 10);
        x /= 10;
    }
    println!("{:?}", digits);
}