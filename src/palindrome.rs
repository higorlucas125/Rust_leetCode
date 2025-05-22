pub fn palindrome(x:i64) -> bool{
    let mut index: i64 = x;
    let mut palindrome: i64 = 0;

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
