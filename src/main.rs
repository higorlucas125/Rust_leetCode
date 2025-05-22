use ChallangeLeetcode::{
    palindrome::palindrome,
    romanNumerals::romanNumerals,
    longest_common_prefix::logest_prefix,
    valid_parentheses::valid_parentheses
};

fn main() {

//    let newData = twoSum::two_sum(vec![2,7,11,15], 9);
//    let newDataThree = three_sum(vec![3,4,7,3,7,3], 9);
   let is_palindrome: bool = palindrome(121);
   let string_now = logest_prefix(vec!["aaa".to_string(), "aa".to_string(), "aaa".to_string()]);
   let palindrome = valid_parentheses("({})".to_string());

   println!(" > {:?}", palindrome);
//   let string_now = logest_prefix(vec!["a".to_string()]);
//   let string_now = logest_prefix(vec!["aa".to_string(),"aa".to_string()]);


    // println!(" > {:?}", newData);
    // println!(" > {:?} ", newDataThree);
    println!(" > {} ", is_palindrome);
    println!(" > {} ", string_now);

    // let  roman = romanNumerals("MCMXCIV".to_string());
    // let roman_perfomac = romanNumerals::roman_numbers_perfomac("MCMXCIV".to_string());

    // println!(" {} ", roman);
    // println!(" roman_perfomac -> {} ", roman_perfomac);
}
