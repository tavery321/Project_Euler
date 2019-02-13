fn main() 
{
    let mut number: i64 = 600851475143;
    let mut max_prime = -1;
    let max: i64 = (number as f64).sqrt() as i64 + 1;
    for factor in (3..max).step_by(2)
    {
        loop
        {
            if number % factor == 0
            {
                max_prime = factor;
                number = number / factor;
            }
            else
            {
                break;
            }
        }
       
    }
    println!("{}", max_prime);
}
