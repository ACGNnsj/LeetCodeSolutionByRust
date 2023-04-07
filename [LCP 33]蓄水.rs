use std::f32::{INFINITY, NAN};
use std::ptr::null;

//给定 N 个无限容量且初始均空的水缸，每个水缸配有一个水桶用来打水，第 `i` 个水缸配备的水桶容量记作 `bucket[i]`。小扣有以下两种操作：
//- 升级水桶：选择任意一个水桶，使其容量增加为 `bucket[i]+1`
//- 蓄水：将全部水桶接满水，倒入各自对应的水缸
//
//每个水缸对应最低蓄水量记作 `vat[i]`，返回小扣至少需要多少次操作可以完成所有水缸蓄水要求。
//
//注意：实际蓄水量 **达到或超过** 最低蓄水量，即完成蓄水要求。
//
//**示例 1：**
//
//> 输入：`bucket = [1,3], vat = [6,8]`
//>
//> 输出：`4`
//>
//> 解释：
//> 第 1 次操作升级 bucket[0]；
//> 第 2 ~ 4 次操作均选择蓄水，即可完成蓄水要求。
//> ![vat1.gif](https://pic.leetcode-cn.com/1616122992-RkDxoL-vat1.gif)
//
//**示例 2：**
//
//> 输入：`bucket = [9,0,1], vat = [0,2,2]`
//>
//> 输出：`3`
//>
//> 解释：
//> 第 1 次操作均选择升级 bucket[1]
//> 第 2~3 次操作选择蓄水，即可完成蓄水要求。
//
//**提示：**
//- `1 <= bucket.length == vat.length <= 100`
//- `0 <= bucket[i], vat[i] <= 10^4`
//
// Related Topics 贪心 数组 堆（优先队列） 👍 77 👎 0
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn store_water(mut bucket: Vec<i32>, vat: Vec<i32>) -> i32 {
        let num = bucket.len();
        let mut rest_times = vec![0; num];
        let mut min_upgrade_times = vec![0; num];
        let mut reduce_times = vec![0; num];
        let mut total_times = 0;
        for i in 0..num {
            if vat[i] == 0 {
                continue;
            }
            if bucket[i] == 0 {
                bucket[i] = 1;
                total_times += 1;
                rest_times[i] = vat[i];
            } else {
                rest_times[i] = Self::positive_ceiling_division(vat[i], bucket[i]);
            }
            if rest_times[i] == 1 {
                continue;
            }
            // (min_upgrade_times[i], reduce_times[i]) = Self::min_upgrade_times(bucket[i], vat[i], rest_times[i]);
            let tuple = Self::min_upgrade_times(bucket[i], vat[i], rest_times[i]);
            min_upgrade_times[i] = tuple.0;
            reduce_times[i] = tuple.1;
        }
        let mut possible_min_times = vec![];
        let mut loss_time = 0;
        loop {
            let (max_indexes, max, second_max) = Self::find_largest_two_num(rest_times.clone());
            let max_reduce_time = max - second_max;
            let mut actual_reduce_time = max_reduce_time;
            let mut total_upgrade_times = 0;
            for i in max_indexes.clone() {
                total_upgrade_times += min_upgrade_times[i];
                if reduce_times[i] < actual_reduce_time {
                    actual_reduce_time = reduce_times[i];
                }
            }
            // if total_upgrade_times >= actual_reduce_time {
            possible_min_times.push(total_times + max);
            // return total_times + max;
            // } else {
            if total_upgrade_times >= actual_reduce_time {
                loss_time += 1;
                if loss_time > 22 {
                    println!("possible_min_times: {:?}", possible_min_times);
                    return *possible_min_times.iter().min().unwrap();
                }
            }
            for i in &max_indexes {
                rest_times[*i] -= reduce_times[*i];
                bucket[*i] += min_upgrade_times[*i];
                // total_times += total_upgrade_times;
            }
            total_times += total_upgrade_times;
            for i in 0..num {
                if vat[i] == 0 {
                    continue;
                }
                rest_times[i] = Self::positive_ceiling_division(vat[i], bucket[i]);
                if rest_times[i] == 1 {
                    continue;
                }
                // (min_upgrade_times[i], reduce_times[i]) = Self::min_upgrade_times(bucket[i], vat[i], rest_times[i]);
                let tuple = Self::min_upgrade_times(bucket[i], vat[i], rest_times[i]);
                min_upgrade_times[i] = tuple.0;
                reduce_times[i] = tuple.1;
            }
            // }
        }
    }
    pub fn positive_ceiling_division(a: i32, b: i32) -> i32 {
        (a - 1) / b + 1
    }
    pub fn min_upgrade_times(bucket: i32, vat: i32, rest_time: i32) -> (i32, i32) {
        let next_bucket = Self::positive_ceiling_division(vat, rest_time - 1);
        let upgrade_times = next_bucket - bucket;
        let reduce_times = rest_time - Self::positive_ceiling_division(vat, next_bucket);
        (upgrade_times, reduce_times)
    }
    pub fn find_largest_two_num(nums: Vec<i32>) -> (Vec<usize>, i32, i32) {
        let mut max = 0;
        let mut second_max = 0;
        let mut max_indexes = vec![];
        for i in 0..nums.len() {
            if nums[i] > max {
                second_max = max;
                max = nums[i];
                max_indexes = vec![i];
            } else if nums[i] == max {
                max_indexes.push(i);
            } else if nums[i] > second_max {
                second_max = nums[i];
            }
        }
        (max_indexes, max, second_max)
    }
}

//leetcode submit region end(Prohibit modification and deletion)
fn main() {
    let vat = 36;
    let bucket = 4;
    let rest_time = Solution::positive_ceiling_division(vat, bucket);
    println!("rest_time: {}", rest_time);
    let (upgrade_times, reduce_times) = Solution::min_upgrade_times(bucket, vat, rest_time);
    println!("upgrade_times: {}, reduce_times: {}", upgrade_times, reduce_times);
    println!("{}", Solution::store_water(vec![1, 1, 1, 1, 1, 1, 1, 1], vec![80, 80, 80, 80, 79, 79, 79, 79]));
    //80 4+79=83 4+4+40=48 4+4+8+27=43 4+4+8+8+20=44
}