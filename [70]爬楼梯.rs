/**
假设你正在爬楼梯。需要 n 阶你才能到达楼顶。 

 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？ 

 

 示例 1： 

 
输入：n = 2
输出：2
解释：有两种方法可以爬到楼顶。
1. 1 阶 + 1 阶
2. 2 阶 

 示例 2： 

 
输入：n = 3
输出：3
解释：有三种方法可以爬到楼顶。
1. 1 阶 + 1 阶 + 1 阶
2. 1 阶 + 2 阶
3. 2 阶 + 1 阶
 

 

 提示： 

 
 1 <= n <= 45 
 

 Related Topics 记忆化搜索 数学 动态规划 
 👍 2780 👎 0

 */
struct Solution {}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    /// 我们使用这样一个事实，即爬 n 级楼梯的方式数等于爬 n-1 级楼梯的方式数加上爬 n-2 级楼梯的方式数。
    /// 
    /// 我们可以使用这个事实来构建一个递归函数，该函数将返回爬 n 步楼梯的方法数。
    /// 
    /// 我们还可以使用这个事实来构建一个动态规划函数，该函数将返回爬 n 步楼梯的方法数。
    /// 
    /// 我们还可以使用这个事实来构建一个函数，该函数将返回使用封闭形式解决方案爬上 n 步楼梯的方法数。
    /// 
    /// 封闭形式的解决方案是最有效的解决方案。
    /// 
    /// 封闭形式的解决方案是最优雅的解决方案。
    /// 
    /// 封闭形式的解决方案是最漂亮的解决方案。
    /// 
    /// 封闭形式的解决方案是最数学化的解决方案。
    /// 
    /// 封闭形式的解决方案是
    /// 
    /// Arguments:
    /// 
    /// * `n`: 步数
    /// 
    /// Returns:
    /// 
    /// 爬 n 级楼梯的方法数。
    pub fn climb_stairs(n: i32) -> i32 {
        use std::collections::HashMap;
        /// 找出所有小于 n 的素数。
        let mut primes = vec![2];
        for i in 3..n + 1 {
            let mut flag = true;
            for j in 0..primes.len() {
                if i % primes[j] == 0 {
                    flag = false;
                    break;
                }
            }
            if flag {
                primes.push(i);
            }
        }
        let prime_num: usize = primes.len();
        /// 计算从 1 到 n 的所有数字的质因数分解。
        let mut powers_map = HashMap::new();
        powers_map.insert(1, vec![0; prime_num]);
        for i in 0..prime_num {
            let mut powers = vec![0; prime_num];
            powers[i] = 1;
            powers_map.insert(primes[i], powers);
        }
        for i in 3..n + 1 {
            let mut powers;
            let num = i;
            for j in 0..prime_num {
                let mut power;
                if num % primes[j] == 0 {
                    powers = powers_map.get(&(num / primes[j])).unwrap().clone();
                    power = powers[j];
                    power += 1;
                    powers[j] = power;
                    powers_map.insert(i, powers);
                    break;
                }
            }
        }
        /// 计算爬 n 个楼梯的方法数。
        let nums = n;
        let mut total = 0;
        for i in 0..nums / 2 + 1 {
            // println!("i: {}", i);
            if i == 0 {
                total += 1;
                continue;
            }
            let mut numerator_powers = vec![0; prime_num];
            for j in 0..i {
                let addend = powers_map.get(&(nums - i - j)).unwrap();
                for k in 0..prime_num {
                    numerator_powers[k] += addend[k];
                }
            }
            let mut denominator_powers = vec![0; prime_num];
            for j in 1..i + 1 {
                let addend = powers_map.get(&j).unwrap();
                for k in 0..prime_num {
                    denominator_powers[k] += addend[k];
                }
            }
            let mut quotient_powers = vec![0; prime_num];
            for j in 0..prime_num {
                quotient_powers[j] = numerator_powers[j] - denominator_powers[j];
            }
            let mut quotient = 1;
            for j in 0..prime_num {
                quotient *= primes[j].pow(quotient_powers[j]);
            }
            total += quotient
        }
        return total;
    }
}

//leetcode submit region end(Prohibit modification and deletion)
fn main() {
    // let v=vec![3,4];
    // println!("{:?}",v);
    // let v=vec![3;4];
    // println!("{:?}",v);
    // let i=1;
    // let num=i;
    // println!("{:?}",i);
    println!("{:?}",Solution::climb_stairs(1));
    println!("{:?}",Solution::climb_stairs(2));
    println!("{:?}",Solution::climb_stairs(3));
    println!("{:?}",Solution::climb_stairs(4));
}