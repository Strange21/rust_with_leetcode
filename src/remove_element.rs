use core::num;



pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    // let mut k = 0;
    // nums.iter_mut().filter(|x| {**x == val});
    // let k = nums.iter().filter(|x|{**x == val}).count();
    // nums.retain(|x|
    //     {
    //         *x != val 
    //     });
    
    // println!("nums : {:?}", nums);
    // return k as i32;
    let mut j= 0;
    for i in 0..nums.len(){
        if nums[i] != val{
            nums[j] = nums[i];
            j += 1;
        }
    }
    return j as i32;
}