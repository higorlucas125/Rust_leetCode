use std::collections::HashMap;

pub fn romanNumerals(s:String) -> i32{
    // Regras III = 3
    // Regas IIII não é 4, 4 é IV
    // Nove é IX não XIIII 
    
    let mut map = HashMap::new();
    map.insert('I', 1);
    map.insert('V', 5);
    map.insert('X', 10);
    map.insert('L', 50);
    map.insert('C', 100);
    map.insert('D', 500);
    map.insert('M', 1000);

    let mut valueNow:i32 = 0;

    let  char_vec: Vec<char> = s.chars().collect();
    let mut i = 0;
    
    while i < char_vec.len(){

        let curr = map[&char_vec[i]];
        let next = if i + 1 < char_vec.len(){
            map[&char_vec[i + 1]]
        } else {
            0
        };

        // regra para os intervalos 
        if curr < next {
            valueNow += next - curr;
            i +=2;
        } else {
            valueNow += curr;
            i += 1;
        }
        
        
        
    }
    

    valueNow
}

pub fn roman_numbers_perfomac (s: String ) -> i32 {
    let mut map = HashMap::new();
        map.insert('I', 1); 
        map.insert('V', 5); 
        map.insert('X', 10); 
        map.insert('L', 50); 
        map.insert('C', 100); 
        map.insert('D', 500); 
        map.insert('M', 1000); 

        let chars: Vec<char> = s.chars().collect();
        let mut total = 0;
        let mut prev = 0;

        for c in chars.into_iter().rev() {
            let value = *map.get(&c).unwrap_or(&0);
            if value < prev {
                total -= value;
            } else {
                total += value;
            }
            prev = value;
        }

        total
}