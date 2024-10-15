mod merge_sorted_array;
mod remove_element;
mod remove_duplicates;
mod majority_element;
// mod longest_common_prefix;
mod reverse_words;
mod rotate_array;

fn main() {
    // println!("Hello, world!");
    // let mut vec1 = vec![1, 2, 3, 0 , 0, 0];
    // let mut vec2 = vec![2, 5, 6];
    // merge_sorted_array::merge(&mut vec1, 3, &mut vec2, 3);

    // let mut vec1 = vec![3, 2, 2, 3];
    // remove_element::remove_element(&mut vec1, 3);

    // let mut vec1 = vec![0,0,1,1,1,2,2,2,3,3,4];
    // vec1.dedup();
    // println!("dedup nums {:?}", vec1);
    // remove_duplicates::remove_duplicates_ii(&mut vec1);

    // let mut vec1 = vec![1, 1, 2, 1, 1, 2];
    // majority_element::majority_element(vec1);

    // let mut s = "  The Sky is    Blue".to_string();
    // reverse_words::reverse_words(s);

    let mut test = vec![1, 2, 3, 4, 5, 6, 7];
    // rotate_array::rotate_once(&mut test);
    rotate_array::rotate(&mut test, 3);
    
}
