fn main() 
{
    let mut total = 0;
    let mut i = 0;
    while i < 1000
    {
        if i%3 == 0 || i%5 == 0
        {
            total += i;
        }
        i = i + 1;
    }
    println!("{}", total)
}
