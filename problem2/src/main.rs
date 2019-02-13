fn main() 
{
    let mut sum = 2;
    let mut start = 1;
    let mut next = 2;
    let mut total = start + next;
    while total < 4000000
    {
        if total % 2 == 0
        {
            sum = sum + total;
        }
        start = next;
        next = total;
        total = start + next;
    }
    println!("{}", sum);
}
