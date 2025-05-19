use crate::twoSum::three_sum;
use crate::palindrome::palindrome;
use crate::romanNumerals::romanNumerals;
mod twoSum; 
mod palindrome;
mod romanNumerals;

fn main() {
   let newData = twoSum::two_sum(vec![2,7,11,15], 9);
   let newDataThree = three_sum(vec![3,4,7,3,7,3], 9);
   let is_palindrome: bool = palindrome(121);

    println!(" > {:?}", newData);
    println!(" > {:?} ", newDataThree);
    println!(" > {} ", is_palindrome);

    let  roman = romanNumerals("MCMXCIV".to_string());
    let roman_perfomac = romanNumerals::roman_numbers_perfomac("MCMXCIV".to_string());

    println!(" {} ", roman);
    println!(" roman_perfomac -> {} ", roman_perfomac);
}
