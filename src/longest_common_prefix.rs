pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let temp = String::new();
    for i in 0..strs[0].len(){
        for s in strs{
            if s[i] != strs[0][i]{
                return temp;
            }
        }

    }
    return "";
}