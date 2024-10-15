use std::vec;

pub fn merge(nums1: &mut Vec<i32>, mut m: i32, nums2: &mut Vec<i32>, mut n: i32) {
    let mut last = m+ n -1;
    while (m > 0 && n > 0){
        if (nums1[m as usize - 1] > nums2[n as usize - 1]){
            nums1[last as usize] = nums1[m as usize -1];
            m -= 1;
        }else{
            nums1[last as usize] = nums2[n as usize - 1];
            n -= 1
        }
        last -= 1;
    }
    while (n > 0){
        nums1[last as usize] = nums2[n as usize - 1];
        last -= 1;
        n -= 1;
    }
    println!("nums1 : {:?}", nums1);
}