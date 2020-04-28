mod resistor_divider;
mod simple_numbers;
mod collections_tasks;


fn main() {
    

    // ****** Voltage divider implementation ****** //
/*
    resistor_divider::calculate_resistors_from_voltages(5.0, 4);
    resistor_divider::calculate_resistors_from_voltages(5.0, 3);
    resistor_divider::calculate_resistors_from_voltages(5.0, 2);
    resistor_divider::calculate_resistors_from_voltages(5.0, 1);
    resistor_divider::calculate_resistors_from_voltages(5.0, 0.5);
    resistor_divider::calculate_resistors_from_voltages(5.0, 4.5);
*/

    // ****** Chapter 8 tasks ****** //
/*
    println!("Chapter 8 tasks: \n");
    let integers_array: [i32; 14] = [123, 412, -213, 85, 43, -422, -1, 0, 2, 5, 33, 0 ,-12, 11];
    
    collections_tasks::calculate_array_parameters(&integers_array);
    
    let string_to_input = "Hello world in here some more words with vowels apple orange onion";
    collections_tasks::convert_string_to_pig_latin(string_to_input);
    
    // I'm quite happy with what I came up with after 4 hours. Won't develop it further though.
    // To check how it works type help and then play for a while
    collections_tasks::console_bot();
*/
    // ****** Simple numbers implementation ****** //

    println!("Simple numbers algorithm:\n");

    // First way to solve task
    let mut vector_of_numbers: Vec<u32> = Vec::new();
    simple_numbers::find(1_000_000, &mut vector_of_numbers);
    simple_numbers::print_from_vector(&vector_of_numbers);

    // Second way to solve task
    // simple_numbers::find_integrated(100);  

    // ************************************* //
}
