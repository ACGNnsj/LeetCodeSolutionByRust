use std::cmp::min;

/**
某乐团的演出场地可视作 `num * num` 的二维矩阵 `grid`（左上角坐标为 `[0,0]`)，每个位置站有一位成员。乐团共有 `9` 种乐器，乐器编
号为 `1~9`，每位成员持有 `1` 个乐器。

为保证声乐混合效果，成员站位规则为：自 `grid` 左上角开始顺时针螺旋形向内循环以 `1，2，...，9` 循环重复排列。例如当 num = `5` 时，站
位如图所示

![image.png](https://pic.leetcode-cn.com/1616125411-WOblWH-image.png)

请返回位于场地坐标 [`Xpos`,`Ypos`] 的成员所持乐器编号。

**示例 1：**

> 输入：`num = 3, Xpos = 0, Ypos = 2`
>
> 输出：`3`
>
> 解释：
> ![image.png](https://pic.leetcode-cn.com/1616125437-WUOwsu-image.png)

**示例 2：**

> 输入：`num = 4, Xpos = 1, Ypos = 2`
>
> 输出：`5`
>
> 解释：
> ![image.png](https://pic.leetcode-cn.com/1616125453-IIDpxg-image.png)

**提示：**
- `1 <= num <= 10^9`
- `0 <= Xpos, Ypos < num`

 Related Topics 数学 
 👍 69 👎 0

 */
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    /// It returns the number of the seat in the orchestra.
    /// 
    /// Arguments:
    /// 
    /// * `num`: the number of rows/columns in the orchestra
    /// * `x_pos`: the x-coordinate of the musician
    /// * `y_pos`: the row number, starting from 0.
    /// 
    /// Returns:
    /// 
    /// The number of the seat in the orchestra.
    pub fn orchestra_layout(num: i32, x_pos: i32, y_pos: i32) -> i32 {
        let num = num as u64;
        let x_pos = x_pos as u64;
        let y_pos = y_pos as u64;
        use std::cmp::min;
        let round = min(min(x_pos, y_pos), min(num - 1 - x_pos, num - 1 - y_pos));
        let mut start = 4 * round * (num - round) % 9 + 1;
        if x_pos == round {
            start += y_pos - round;
        } else if y_pos == num - 1 - round {
            start += num - 1 - 2 * round;
            start += x_pos - round;
        } else if x_pos == num - 1 - round {
            start += 2 * (num - 1 - 2 * round);
            start += num - 1 - round - y_pos;
        } else if y_pos == round {
            start += 3 * (num - 1 - 2 * round);
            start += num - 1 - round - x_pos;
        }
        start %= 9;
        if start != 0 {
            start as i32
        } else {
            9
        }
    }
}

//leetcode submit region end(Prohibit modification and deletion)
fn main() {
    let num = 888888888;
    let x_pos = 111111111;
    let y_pos = 444444444;
    let result = Solution::orchestra_layout(num, x_pos, y_pos);
    println!("result: {}", result);
}