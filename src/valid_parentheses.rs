use std::collections::HashMap;

pub fn valid_parentheses(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let mut map: HashMap<char,char> = HashMap::new();

    map.insert('(', ')');
    map.insert('{', '}');
    map.insert('[', ']');

    for ch in s.chars(){
        if map.contains_key(&ch){
            stack.push(ch);
        }else if let Some(&last) = stack.last(){
            if map.get(&last) == Some(&ch){ 
                stack.pop();
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    stack.is_empty()
}