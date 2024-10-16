use std::collections::HashSet;

fn square(num: i32) -> i32{
    num * num
}
fn sum_of_squares(mut n:i32) -> i32{
    let mut sqre = 0;
    while n>0 {
        sqre += square(n%10);
        n = n/10;
    }
    sqre
}

pub fn is_happy(mut n: i32) -> bool {
    let mut result = 0;
    let mut test_set: HashSet<i32> = HashSet::new();

    while !test_set.contains(&n){
        result = sum_of_squares(n);
        println!("result = {}", result);
        test_set.insert(n);
        if result == 1 {
            println!("Happy number!");
            return true;
        }
        n = result;
    }
    if test_set.contains(&result){
        println!("Unhappy number!");
        return false;
    } 
    return true;
}