mod resistor_divider;
mod simple_numbers;

fn main() {

    // resistor_divider::calculate_resistors_from_voltages(5.0, 1.22);

    // *** Simple numbers implementation *** //

    // First way to solve task
    let mut vector_of_numbers: Vec<i32> = Vec::new();
    simple_numbers::find(2000, &mut vector_of_numbers);
    simple_numbers::print_from_vector(&vector_of_numbers);

    // Second way to solve task
    // simple_numbers::find_integrated(100);

    // ************************************* //
}
