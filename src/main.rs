// use std::io;



fn main()
{
    // let var: i32 = 12;
    // let var2: i32 = 5;

    simple_numbers_generator(20);

}

fn simple_numbers_generator(end_of_the_interval: i32)
{
    // let mut previous_simple_number_found = 3;
    let mut vector_of_numbers = Vec::new();
    vector_of_numbers.push(2);
    vector_of_numbers.push(3);
    print!("1, 2, 3");

    
    for value_in_check in 4..=end_of_the_interval
    {
        for current_devider in &vector_of_numbers
        {
            if value_in_check % current_devider == 0
            {
                // Value is not simple
                break;
            }
            
            if value_in_check /(*current_devider) > (*current_devider)
            {
                // New simple number was found
                vector_of_numbers.push(value_in_check);
                print!(", {}", value_in_check);
                break;
            }
        }
    }
}