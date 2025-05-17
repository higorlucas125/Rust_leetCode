pub fn palindrome(x:i32) -> bool{
    let mut index = x;
    let mut palindrome = 0;

    if x < 0 {
        return false
    }

    while index != 0 {
        let mut value = index % 10;
        palindrome = palindrome * 10 + value;
        index = index / 10;
    }

    if palindrome == x {
        return true;
    }

    false

}
