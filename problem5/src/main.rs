fn main() {
    let mut found = false;
    //Can jump by 10
    for num in (2520..).step_by(10)
    {
        if found == true
        {
            break;
        }
        //1-10 are guaranteed to be factors if all factors from 11-20 are factors
        for factor in 11..21
        {
            if num % factor != 0
            {
                break;
            }
            if factor == 20 && num % factor == 0
            {
                println!("{}", num);
                found = true;
            }
        }
    }
}
