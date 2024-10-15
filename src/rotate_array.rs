use std::vec;

// use std::ptr;
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let length = nums.len();
    nums.reverse();
    nums[..k as usize].reverse();
    nums[k as usize..].reverse();
    println!("nums after k rotation = {:?}", nums);
    
}

pub fn rotate_once(nums: &mut Vec<i32>){
    let mut temp = nums[nums.len()-1];
    let mut lenght = nums.len();
    for i in (1..nums.len()).rev(){
        nums[i] = nums[i-1];
    }
    nums[0] = temp;
    println!("nums after 1 rotation = {:?}", nums);
}