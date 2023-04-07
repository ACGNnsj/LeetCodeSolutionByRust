/**
请你实现三个 API append，addAll 和 multAll 来实现奇妙序列。 

 请实现 Fancy 类 ： 

 
 Fancy() 初始化一个空序列对象。 
 void append(val) 将整数 val 添加在序列末尾。 
 void addAll(inc) 将所有序列中的现有数值都增加 inc 。 
 void multAll(m) 将序列中的所有现有数值都乘以整数 m 。 
 int getIndex(idx) 得到下标为 idx 处的数值（下标从 0 开始），并将结果对 10⁹ + 7 取余。如果下标大于等于序列的长度，请返回 -
1 。 
 

 

 示例： 

 
输入：
["Fancy", "append", "addAll", "append", "multAll", "getIndex", "addAll", 
"append", "multAll", "getIndex", "getIndex", "getIndex"]
[[], [2], [3], [7], [2], [0], [3], [10], [2], [0], [1], [2]]
输出：
[null, null, null, null, null, 10, null, null, null, 26, 34, 20]

解释：
Fancy fancy = new Fancy();
fancy.append(2);   // 奇妙序列：[2]
fancy.addAll(3);   // 奇妙序列：[2+3] -> [5]
fancy.append(7);   // 奇妙序列：[5, 7]
fancy.multAll(2);  // 奇妙序列：[5*2, 7*2] -> [10, 14]
fancy.getIndex(0); // 返回 10
fancy.addAll(3);   // 奇妙序列：[10+3, 14+3] -> [13, 17]
fancy.append(10);  // 奇妙序列：[13, 17, 10]
fancy.multAll(2);  // 奇妙序列：[13*2, 17*2, 10*2] -> [26, 34, 20]
fancy.getIndex(0); // 返回 26
fancy.getIndex(1); // 返回 34
fancy.getIndex(2); // 返回 20
 

 

 提示： 

 
 1 <= val, inc, m <= 100 
 0 <= idx <= 10⁵ 
 总共最多会有 10⁵ 次对 append，addAll，multAll 和 getIndex 的调用。 
 

 Related Topics 设计 线段树 数学 
 👍 68 👎 0

 */

//leetcode submit region begin(Prohibit modification and deletion)
struct Fancy {
    length: usize,
    nums: Vec<u64>,
    operators: Vec<Vec<bool>>,
    operands: Vec<Vec<i32>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Fancy {
    fn new() -> Self {
        Fancy {
            length: 0,
            nums: vec![],
            operators: vec![],
            operands: vec![],
        }
    }

    fn append(&mut self, val: i32) {
        self.nums.push(val as u64);
        self.operators.push(vec![]);
        self.operands.push(vec![]);
        self.length += 1;
    }

    fn add_all(&mut self, inc: i32) {
        for i in 0..self.length {
            self.operators[i].push(true);
            self.operands[i].push(inc);
        }
    }

    fn mult_all(&mut self, m: i32) {
        for i in 0..self.length {
            self.operators[i].push(false);
            self.operands[i].push(m);
        }
    }

    fn get_index(&mut self, idx: i32) -> i32 {
        if idx as usize >= self.length {
            return -1;
        }
        let length = self.operators[idx as usize].len();
        let mut result = self.nums[idx as usize];
        let mut mult_times = 0;
        for i in 0..length {
            match self.operators[idx as usize][i] {
                true => {
                    result += self.operands[idx as usize][i] as u64;
                }
                false => {
                    mult_times += 1;
                    result *= self.operands[idx as usize][i] as u64;
                    if mult_times > 6 {
                        result %= 1000000007;
                        mult_times = 0;
                    }
                }
                _ => {}
            }
        }
        result %= 1000000007;
        self.nums[idx as usize] = result;
        self.operators[idx as usize].clear();
        self.operands[idx as usize].clear();
        result as i32
    }
}

/**
 * Your Fancy object will be instantiated and called as such:
 * let obj = Fancy::new();
 * obj.append(val);
 * obj.add_all(inc);
 * obj.mult_all(m);
 * let ret_4: i32 = obj.get_index(idx);
 */
//leetcode submit region end(Prohibit modification and deletion)
fn main() {
    let mut obj = Fancy::new();
    obj.append(2);
    obj.add_all(3);
    obj.append(7);
    obj.mult_all(2);
    let ret_4: i32 = obj.get_index(0);
    println!("{}", ret_4);
    obj.add_all(3);
    obj.append(10);
    obj.mult_all(2);
    let ret_4: i32 = obj.get_index(0);
    println!("{}", ret_4);
    let ret_4: i32 = obj.get_index(1);
    println!("{}", ret_4);
    let ret_4: i32 = obj.get_index(2);
    println!("{}", ret_4);
}