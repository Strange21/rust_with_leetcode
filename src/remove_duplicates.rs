pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // nums.dedup();
    // println!("dedup nums {:?}", nums);
    // return nums.len() as i32;
    // let mut i =0;
    let mut j = 2;
    for i in 2..nums.len(){
        if nums[i] != nums[i -1]{
            
            nums[j] = nums[i];
            j+= 1;
        }
    }
    println!("nums {:?}", nums);
    return j as i32;

}

pub fn min(a:i32, b:i32) -> i32{
    if a < b {a} else {b}
}

pub fn remove_duplicates_ii(nums: &mut Vec<i32>) -> i32{
    let mut l  = 0;
    let mut r = 0;
    while r < nums.len(){
        let mut count  = 1;
        while (r+1 < nums.len() - 1) && (nums[r] == nums[r+1]) {
            r += 1;
            count += 1;
        }
        // println!("count for {} is {}", nums[r], count);
        for i in 0..(std::cmp::min(2, count)){
            // println!("interation {} for value {}", i, nums[r]);
            nums[l] = nums[r];
            l+=1;
        }
        r += 1;
    }
    // println!("nums {:?}", nums);
    return l as i32;
}