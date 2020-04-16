use colored::Colorize;

#[allow(dead_code)]
pub fn find_integrated(end_of_the_interval: i32)
{
    let mut previous_simple_number_found = 3;
    let mut next_value_should_be_colored: bool = false;
    let mut vector_of_numbers = Vec::new();
    vector_of_numbers.push(2);
    vector_of_numbers.push(3);
    print!("1, 2");
    
    for value_in_check in 4..=end_of_the_interval
    {
        for current_devider in &vector_of_numbers
        {
            if value_in_check % current_devider == 0
            {
                // Value is not simple
                break;
            }
            
            if value_in_check / (*current_devider)  < (*current_devider) 
            {
                // New simple number has been found
                vector_of_numbers.push(value_in_check);
                let previous_pushed_value_as_a_string: String = previous_simple_number_found.to_string();

                if value_in_check - previous_simple_number_found == 2
                {
                    print!(", {}", previous_pushed_value_as_a_string.blue());
                    next_value_should_be_colored = true;
                }
                else
                {
                    if next_value_should_be_colored == true 
                    {
                        print!(", {}", previous_pushed_value_as_a_string.blue());
                        next_value_should_be_colored = false;
                    }
                    else
                    {
                        print!(", {}", previous_pushed_value_as_a_string);
                    }
                }

                previous_simple_number_found = value_in_check;
                break;
            }
        }
    }
}


#[allow(dead_code)]
pub fn find(end_of_the_interval: i32,  vector_to_store: &mut Vec<i32>)
{
    vector_to_store.push(2);
    vector_to_store.push(3);
    
    for value_in_check in 4..=end_of_the_interval
    {
        for current_devider in vector_to_store.iter()
        {
            if value_in_check % *current_devider == 0
            {
                // Value is not simple
                break;
            }
            
            if value_in_check / (*current_devider)  < (*current_devider) 
            {
                // New simple number has been found
                vector_to_store.push(value_in_check);
                break;
            }
        }
    }
}

// Function won't print last number, can be fixed, but i'm lazy
#[allow(dead_code)]
pub fn print_from_vector(vector_to_print: &Vec<i32>)
{
    print!("1");
    let mut loop_skips = 0;
    

    for pointer in 1..vector_to_print.len()
    {
        if loop_skips != 0
        {
            loop_skips -= 1;
            continue;
        }
        
        if vector_to_print[pointer] - vector_to_print[pointer-1] == 2
        {
            //print 2 values and skip next
            print!(", {}, {}", vector_to_print[pointer - 1].to_string().blue(), vector_to_print[pointer].to_string().blue());
            loop_skips = 1;
            continue;
        }

        //print value
        print!(", {}", vector_to_print[pointer-1]);
    }

    
}