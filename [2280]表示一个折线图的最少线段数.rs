//给你一个二维整数数组 stockPrices ，其中 stockPrices[i] = [dayi, pricei] 表示股票在 dayi 的价格为 
//pricei 。折线图 是一个二维平面上的若干个点组成的图，横坐标表示日期，纵坐标表示价格，折线图由相邻的点连接而成。比方说下图是一个例子： 
// 请你返回要表示一个折线图所需要的 最少线段数 。 
//
// 
//
// 示例 1： 
//
// 
//
// 输入：stockPrices = [[1,7],[2,6],[3,5],[4,4],[5,4],[6,3],[7,2],[8,1]]
//输出：3
//解释：
//上图为输入对应的图，横坐标表示日期，纵坐标表示价格。
//以下 3 个线段可以表示折线图：
//- 线段 1 （红色）从 (1,7) 到 (4,4) ，经过 (1,7) ，(2,6) ，(3,5) 和 (4,4) 。
//- 线段 2 （蓝色）从 (4,4) 到 (5,4) 。
//- 线段 3 （绿色）从 (5,4) 到 (8,1) ，经过 (5,4) ，(6,3) ，(7,2) 和 (8,1) 。
//可以证明，无法用少于 3 条线段表示这个折线图。
// 
//
// 示例 2： 
//
// 
//
// 输入：stockPrices = [[3,4],[1,2],[7,8],[2,3]]
//输出：1
//解释：
//如上图所示，折线图可以用一条线段表示。
// 
//
// 
//
// 提示： 
//
// 
// 1 <= stockPrices.length <= 10⁵ 
// stockPrices[i].length == 2 
// 1 <= dayi, pricei <= 10⁹ 
// 所有 dayi 互不相同 。 
// 
//
// Related Topics 几何 数组 数学 数论 排序 👍 23 👎 0

struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn minimum_lines(mut stock_prices: Vec<Vec<i32>>) -> i32 {
        if stock_prices.len() <= 2 {
            return (stock_prices.len() - 1) as i32;
        }
        // println!("{:?}", stock_prices);
        stock_prices.sort_by(|a, b| a[0].cmp(&b[0]));
        // println!("{:?}", stock_prices);
        let interval_num = stock_prices.len() - 1;
        let mut segment_count = 1;
        let mut slope_ratio = (stock_prices[1][1] - stock_prices[0][1], stock_prices[1][0] - stock_prices[0][0]);
        // println!("({},{}),({},{}),{}", stock_prices[0][0], stock_prices[0][1], stock_prices[1][0], stock_prices[1][1], slope);
        for i in 1..interval_num {
            let current_slope = (stock_prices[i + 1][1] - stock_prices[i][1], stock_prices[i + 1][0] - stock_prices[i][0]);
            if !is_equal(slope_ratio, current_slope) {
                segment_count += 1;
                slope_ratio = current_slope;
            }
            // println!("({},{}),({},{}),{}", stock_prices[i][0], stock_prices[i][1], stock_prices[i + 1][0], stock_prices[i + 1][1], slope_ratio);
        }
        segment_count
    }
}

fn is_equal(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 * b.1 == a.1 * b.0
}

//leetcode submit region end(Prohibit modification and deletion)
fn main() {
    let array = [[83, 35], [79, 51], [61, 48], [54, 87], [44, 93], [22, 5], [87, 28], [64, 8], [89, 78], [62, 83], [58, 72], [48, 7], [97, 16], [27, 100], [65, 48], [11, 31], [29, 76], [93, 29], [72, 59], [73, 74], [9, 90], [66, 81], [12, 8], [86, 80], [84, 43], [36, 63], [80, 45], [81, 88], [95, 5], [40, 59]];
    let vec: Vec<Vec<i32>> = array.iter().map(|x| x.to_vec()).collect();
    let r = Solution::minimum_lines(vec);
    println!("{}", r);
    // let o=1.cmp(&2);
}