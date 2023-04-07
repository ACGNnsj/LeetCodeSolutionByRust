//给你一个在 XY 平面上的 width x height 的网格图，左下角 的格子为 (0, 0) ，右上角 的格子为 (width - 1, 
//height - 1) 。网格图中相邻格子为四个基本方向之一（"North"，"East"，"South" 和 "West"）。一个机器人 初始 在格子 (0, 0) ，方
//向为 "East" 。 
//
// 机器人可以根据指令移动指定的 步数 。每一步，它可以执行以下操作。 
//
// 
// 沿着当前方向尝试 往前一步 。 
// 如果机器人下一步将到达的格子 超出了边界 ，机器人会 逆时针 转 90 度，然后再尝试往前一步。 
// 
//
// 如果机器人完成了指令要求的移动步数，它将停止移动并等待下一个指令。 
//
// 请你实现 Robot 类： 
//
// 
// Robot(int width, int height) 初始化一个 width x height 的网格图，机器人初始在 (0, 0) ，方向朝 
//"East" 。 
// void move(int num) 给机器人下达前进 num 步的指令。 
// int[] getPos() 返回机器人当前所处的格子位置，用一个长度为 2 的数组 [x, y] 表示。 
// String getDir() 返回当前机器人的朝向，为 "North" ，"East" ，"South" 或者 "West" 。 
// 
//
// 
//
// 示例 1： 
//
// 
//
// 输入：
//["Robot", "move", "move", "getPos", "getDir", "move", "move", "move", 
//"getPos", "getDir"]
//[[6, 3], [2], [2], [], [], [2], [1], [4], [], []]
//输出：
//[null, null, null, [4, 0], "East", null, null, null, [1, 2], "West"]
//
//解释：
//Robot robot = new Robot(6, 3); // 初始化网格图，机器人在 (0, 0) ，朝东。
//robot.move(2);  // 机器人朝东移动 2 步，到达 (2, 0) ，并朝东。
//robot.move(2);  // 机器人朝东移动 2 步，到达 (4, 0) ，并朝东。
//robot.getPos(); // 返回 [4, 0]
//robot.getDir(); // 返回 "East"
//robot.move(2);  // 朝东移动 1 步到达 (5, 0) ，并朝东。
//                // 下一步继续往东移动将出界，所以逆时针转变方向朝北。
//                // 然后，往北移动 1 步到达 (5, 1) ，并朝北。
//robot.move(1);  // 朝北移动 1 步到达 (5, 2) ，并朝 北 （不是朝西）。
//robot.move(4);  // 下一步继续往北移动将出界，所以逆时针转变方向朝西。
//                // 然后，移动 4 步到 (1, 2) ，并朝西。
//robot.getPos(); // 返回 [1, 2]
//robot.getDir(); // 返回 "West"
//
// 
//
// 
//
// 提示： 
//
// 
// 2 <= width, height <= 100 
// 1 <= num <= 10⁵ 
// move ，getPos 和 getDir 总共 调用次数不超过 10⁴ 次。 
// 
//
// Related Topics 设计 模拟 👍 20 👎 0


//leetcode submit region begin(Prohibit modification and deletion)
struct Robot {
    x_max: i32,
    y_max: i32,
    x: i32,
    y: i32,
    period: i32,
    step: i32,
    dir: String,
    coordinate_updated: bool,
    dir_updated: bool,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Robot {
    fn new(width: i32, height: i32) -> Self {
        Robot {
            x_max: width - 1,
            y_max: height - 1,
            x: 0,
            y: 0,
            period: 2 * (width + height - 2),
            step: 0,
            dir: "East".to_string(),
            coordinate_updated: true,
            dir_updated: true,
        }
    }

    fn step(&mut self, num: i32) {
        self.step += num;
        self.coordinate_updated = false;
        self.dir_updated = false;
    }

    fn get_pos(&mut self) -> Vec<i32> {
        if !self.coordinate_updated {
            self.refresh_coordinate()
        }
        vec![self.x, self.y]
    }

    fn get_dir(&mut self) -> String {
        if !self.dir_updated {
            self.refresh_dir()
        }
        self.dir.clone()
    }

    fn refresh_dir(&mut self) {
        if !self.coordinate_updated {
            self.refresh_coordinate()
        }
        self.dir_updated = true;
        let x = self.x;
        let y = self.y;
        let x_max = self.x_max;
        let y_max = self.y_max;
        self.dir = if y == 0 && x > 0 && x <= x_max {
            "East".to_string()
        } else if x == x_max && y > 0 && y <= y_max {
            "North".to_string()
        } else if y == y_max && x >= 0 && x < x_max {
            "West".to_string()
        } else {
            "South".to_string()
        }
    }

    fn refresh_coordinate(&mut self) {
        self.coordinate_updated = true;
        self.step %= self.period;
        let step = self.step;
        let x_max = self.x_max;
        let y_max = self.y_max;
        if step >= 0 && step < x_max {
            self.x = step;
            self.y = 0;
        } else if step >= x_max && step < x_max + y_max {
            self.x = x_max;
            self.y = step - x_max;
        } else if step >= 2 * x_max + y_max && step < self.period {
            self.x = 0;
            self.y = 2 * (x_max + y_max) - step;
        } else {
            self.x = 2 * x_max + y_max - step;
            self.y = y_max;
        }
    }
}

/**
 * Your Robot object will be instantiated and called as such:
 * let obj = Robot::new(width, height);
 * obj.step(num);
 * let ret_2: Vec<i32> = obj.get_pos();
 * let ret_3: String = obj.get_dir();
 */
//leetcode submit region end(Prohibit modification and deletion)
fn main() {
    let mut robot = Robot::new(6, 3);
    robot.step(2);
    robot.step(2);
    println!("{:?}", robot.get_pos());
    println!("{:?}", robot.get_dir());
    robot.step(2);
    robot.step(1);
    robot.step(4);
    println!("{:?}", robot.get_pos());
    println!("{:?}", robot.get_dir());
}