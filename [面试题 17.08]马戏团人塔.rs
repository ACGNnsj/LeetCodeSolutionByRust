//有个马戏团正在设计叠罗汉的表演节目，一个人要站在另一人的肩膀上。出于实际和美观的考虑，在上面的人要比下面的人矮一点且轻一点。已知马戏团每个人的身高和体重，请
//编写代码计算叠罗汉最多能叠几个人。 
//
// 示例： 
//
// 
//输入：height = [65,70,56,75,60,68] weight = [100,150,90,190,95,110]
//输出：6
//解释：从上往下数，叠罗汉最多能叠 6 层：(56,90), (60,95), (65,100), (68,110), (70,150), (75,190) 
//
//
// 提示： 
//
// 
// height.length == weight.length <= 10000 
// 
//
// Related Topics 数组 二分查找 动态规划 排序 👍 102 👎 0

struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use rand::Rng;
use std::collections::HashSet;

impl Solution {
    pub fn best_seq_at_index(height: Vec<i32>, weight: Vec<i32>) -> i32 {
        let mut people = Vec::new();
        for i in 0..height.len() {
            people.push((height[i], weight[i]));
        }
        people.sort_by(|a, b| {
            if a.0 == b.0 {
                a.1.cmp(&b.1)
            } else {
                a.0.cmp(&b.0)
            }
        });
        println!("people: {:?}", people);
        let origin = people[0];
        let mut origins = vec![];
        for person in &people {
            if !(person.0 > origin.0 && person.1 > origin.1) && *person != origin {
                origins.push(*person);
            }
        }
        println!("origins: {:?}", origins);
        let mut to_remove = HashSet::new();
        for i in 0..origins.len() {
            let origin = origins[i];
            for j in 0..origins.len() {
                let person = origins[j];
                if person.0 >= origin.0 && person.1 >= origin.1 && person != origin {
                    to_remove.insert(j);
                    // push_unrepeated_into_ordered_asc(&mut to_remove, j);
                }
            }
        }
        let mut ordered_to_remove: Vec<_> = to_remove.into_iter().collect();
        ordered_to_remove.sort();
        ordered_to_remove.reverse();
        println!("ordered_to_remove: {:?}", ordered_to_remove);
        // println!("to_remove: {:?}", to_remove);
        for i in ordered_to_remove {
            origins.remove(i);
        }
        println!("origins: {:?}", origins);
        let end = *people.last().unwrap();
        let mut ends = vec![];
        for person in &people {
            if !(person.0 > end.0 && person.1 > end.1) {
                ends.push(*person);
            }
        }
        println!("ends: {:?}", ends);
        to_remove = HashSet::new();
        for i in 0..ends.len() {
            let end = ends[i];
            for j in 0..ends.len() {
                let person = ends[j];
                if person.0 <= end.0 && person.1 <= end.1 && person != end {
                    to_remove.insert(j);
                }
            }
        }
        ordered_to_remove = to_remove.into_iter().collect();
        ordered_to_remove.sort();
        ordered_to_remove.reverse();
        println!("ordered_to_remove: {:?}", ordered_to_remove);
        for i in ordered_to_remove {
            ends.remove(i);
        }
        println!("ends: {:?}", ends);
        1
    }
}

fn push_unrepeated_into_ordered_asc<T: PartialOrd + Copy + Ord>(vec: &mut Vec<T>, element: T) {
    let len = vec.len();
    if len == 0 {
        vec.push(element);
    } else {
        let last = vec[len - 1];
        if last < element {
            vec.push(element);
        } else if last > element {
            vec.push(element);
            vec.sort();
        }
    }
}

//leetcode submit region end(Prohibit modification and deletion)
fn main() {
    let mut rng = rand::thread_rng();
    let mut heights = vec![];
    let mut weights = vec![];
    for i in 0..100 {
        let height = rng.gen_range(100..150);
        let weight = rng.gen_range(100..150);
        heights.push(height);
        weights.push(weight);
    }

    // let height = vec![65, 69, 68, 70, 80, 75, 77, 60, 68, 70];
    // let weight = vec![100, 89, 88, 90, 80, 190, 77, 95, 110, 70];
    let res = Solution::best_seq_at_index(heights, weights);
    println!("res: {}", res);
}