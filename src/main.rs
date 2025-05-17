use crate::twoSum::three_sum;
use crate::palindrome::palindrome;
mod twoSum; 
mod palindrome;

fn main() {
   let newData = twoSum::two_sum(vec![2,7,11,15], 9);
   let newDataThree = three_sum(vec![3,4,7,3,7,3], 9);
   let is_palindrome: bool = palindrome(121);

    println!(" > {:?}", newData);
    println!(" > {:?} ", newDataThree);
    println!(" > {} ", is_palindrome);
}
