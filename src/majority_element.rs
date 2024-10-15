use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut count_map:HashMap<i32, u32> = HashMap::new();
    for (i, num) in nums.iter().enumerate(){
        match count_map.get_mut(num) {
            Some(val) =>{
                *val += 1;
            }
            None => {
                count_map.insert(*num, 1);
            }
        }
    }
    println!("generated hashmap : {:?}", count_map);
    let max_key = count_map.iter().max_by_key(|x| x.1).unwrap();
    println!("Max occurance {:?}", max_key.0);
    return 0;
}