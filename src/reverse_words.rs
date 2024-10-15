pub fn reverse_words(mut s: String){
    let mut res = String::new();
    for each_string in s.chars().rev().collect::<String>().split(" "){
        if each_string.is_empty(){
            continue;
        }
        res.extend(each_string.trim().chars().rev());
        res += " "; 
    }
    println!("res : {:?}",res.as_str().trim().to_string());
}