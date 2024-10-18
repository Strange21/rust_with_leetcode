// Design a stack the supports push, pop, top, and get_min in constant time.
// Use two stacks. One stack for push, pop, top. A second stack for get_min which tracks the min for the stack.
struct MinStack {
    stk: Vec<i32>,
    min_stk: Vec<i32>,
}
impl MinStack {
    fn new() -> Self {
        MinStack {
            stk: Vec::new(),
            min_stk: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stk.push(val);
        let cur_min: i32 = match self.min_stk.last() {
            Some(&min_val) => min_val.min(val),
            None => val,
        };
        self.min_stk.push(cur_min);
    }

    fn pop(&mut self) {
        self.stk.pop();
        self.min_stk.pop();
    }

    fn top(&self) -> i32 {
        *self.stk.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stk.last().unwrap()
    }
}