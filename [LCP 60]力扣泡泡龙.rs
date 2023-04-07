/**
欢迎各位勇者来到力扣城，本次试炼主题为「力扣泡泡龙」。

游戏初始状态的泡泡形如二叉树 `root`，每个节点值对应了该泡泡的分值。勇者最多可以击破一个节点泡泡，要求满足：
- 被击破的节点泡泡 **至多** 只有一个子节点泡泡
- 当被击破的节点泡泡有子节点泡泡时，则子节点泡泡将取代被击破泡泡的位置

 > 注：即整棵子树泡泡上移

请问在击破一个节点泡泡操作或无击破操作后，二叉泡泡树的最大「层和」是多少。

**注意：**
- 「层和」为同一高度的所有节点的分值之和

**示例 1：**

> 输入：`root = [6,0,3,null,8]`
>
> 输出：`11`
>
> 解释：勇者的最佳方案如图所示
> ![image.png](https://pic.leetcode-cn.com/1648180809-XSWPLu-image.png){:height=
"100px"}

**示例 2：**

> 输入：`root = [5,6,2,4,null,null,1,3,5]`
>
> 输出：`9`
>
> 解释：勇者击破 6 节点，此时「层和」最大为 3+5+1 = 9
> ![image.png](https://pic.leetcode-cn.com/1648180769-TLpYop-image.png){:height=
"200px"}

**示例 3：**

> 输入：`root = [-5,1,7]`
>
> 输出：`8`
>
> 解释：勇者不击破节点，「层和」最大为 1+7 = 8

**提示**：
- `2 <= 树中节点个数 <= 10^5`
- `-10000 <= 树中节点的值 <= 10000`

 Related Topics 树 动态规划 二叉树 
 👍 10 👎 0

 */
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn get_max_layer_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut layers: Vec<Vec<Option<Rc<RefCell<TreeNode>>>>> = vec![vec![root.clone()]];
        let mut layer_sums: Vec<i32> = vec![root.as_ref().unwrap().borrow().val];
        let mut children_indices_list: Vec<Vec<Vec<usize>>> = vec![];
        let mut vals_list: Vec<Vec<i32>> = vec![vec![root.as_ref().unwrap().borrow().val]];
        // if root.as_ref().unwrap().borrow().left.is_none() || root.as_ref().unwrap().borrow().right.is_none() {
        //     children_indices_list.push(vec![vec![0]]);
        // } else {
        //     children_indices_list.push(vec![vec![0, 1]]);
        // }
        let mut layer_num = 0;
        loop {
            let mut layer = vec![];
            let mut children_indices = vec![];
            let mut index = 0;
            let mut vals = vec![];
            for node in layers.last().unwrap() {
                // if node.is_none() {
                //     continue;
                // }
                let mut children_index = vec![];
                let node = node.as_ref().unwrap().borrow();
                if node.left.is_some() {
                    layer.push(node.left.clone());
                    vals.push(node.left.as_ref().unwrap().borrow().val);
                    children_index.push(index);
                    index += 1
                }
                if node.right.is_some() {
                    layer.push(node.right.clone());
                    vals.push(node.right.as_ref().unwrap().borrow().val);
                    children_index.push(index);
                    index += 1;
                }
                children_indices.push(children_index);
            }
            if layer.is_empty() {
                break;
            }
            vals_list.push(vals);
            children_indices_list.push(children_indices);
            layers.push(layer);
            layer_num += 1;
        }
        println!("{:?}", vals_list);
        println!("{:?}", layers);
        println!("{:?}", children_indices_list);
        32
    }
}

//leetcode submit region end(Prohibit modification and deletion)
struct Solution;

fn main() {
    let mut root = TreeNode::new(6);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(8))));
    root.right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    // // println!("{:?}", Some(Rc::new(RefCell::new(root))));
    println!("{:?}", Solution::get_max_layer_sum(Some(Rc::new(RefCell::new(root)))));
    let mut i = 1;
}