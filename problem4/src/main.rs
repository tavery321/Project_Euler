fn is_palindrome(total: i32) -> bool
{
    let int_string = total.to_string();
    let half = int_string.len()/2;
    let pal = int_string.bytes().take(half).eq(int_string.bytes().rev().take(half));
    return pal
}

fn main() 
{
    let mut max = 0;
    for num1 in (100..1000).rev()
    {
        for num2 in (100..1000).rev()
        {
            let total = num1*num2;
            let ret = is_palindrome(total);
            if ret == true
            {
                if total > max
                {
                    max = total;
                }   
            }
        }
    }
    println!("{}", max);
}
