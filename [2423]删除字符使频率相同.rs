/**
给你一个下标从 0 开始的字符串 word ，字符串只包含小写英文字母。你需要选择 一个 下标并 删除 下标处的字符，使得 word 中剩余每个字母出现 频率 
相同。 

 如果删除一个字母后，word 中剩余所有字母的出现频率都相同，那么返回 true ，否则返回 false 。 

 注意： 

 
 字母 x 的 频率 是这个字母在字符串中出现的次数。 
 你 必须 恰好删除一个字母，不能一个字母都不删除。 
 

 

 示例 1： 

 输入：word = "abcc"
输出：true
解释：选择下标 3 并删除该字母，word 变成 "abc" 且每个字母出现频率都为 1 。
 

 示例 2： 

 输入：word = "aazz"
输出：false
解释：我们必须删除一个字母，所以要么 "a" 的频率变为 1 且 "z" 的频率为 2 ，要么两个字母频率反过来。所以不可能让剩余所有字母出现频率相同。
 

 

 提示： 

 
 2 <= word.length <= 100 
 word 只包含小写英文字母。 
 

 Related Topics 哈希表 字符串 计数 
 👍 18 👎 0

 */
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut counts = vec![0; 26];
        for c in word.chars() {
            counts[c as usize - 'a' as usize] += 1;
        }
        counts = counts.into_iter().filter(|&x| x > 0).collect::<Vec<_>>();
        if counts.len() == 1 { return true; }
        let mut count_map = std::collections::HashMap::new();
        for count in counts {
            if count_map.get(&count).is_some() {
                count_map.insert(count, count_map.get(&count).unwrap() + 1);
            } else {
                count_map.insert(count, 1);
            }
        }
        if count_map.len() != 2 {
            if count_map.len() == 1 && count_map.contains_key(&1) {
                return true;
            }
            return false;
        }
        let mut keys = count_map.keys().collect::<Vec<_>>();
        keys.sort();
        if keys[0] == &1 && count_map.get(keys[0]).unwrap() == &1 {
            return true;
        }
        if (keys[1] - keys[0] == 1) && *count_map.get(keys[1]).unwrap() == 1 {
            return true;
        }
        false
    }
}

//leetcode submit region end(Prohibit modification and deletion)
fn main() {
    let word = "aaxdx".to_string();
    println!("{}", Solution::equal_frequency(word));
}