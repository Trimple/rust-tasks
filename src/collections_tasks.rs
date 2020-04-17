
use std::collections::HashMap;
use std::io;
use colored::Colorize;

const VOWELS: [char; 12] = ['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u', 'Y', 'y'];

// *** first task *** //
pub fn calculate_array_parameters(input_array: &[i32])
{
    let mut simple_vector: Vec<i32> = Vec::new();
    
    for value in input_array
    {
        simple_vector.push(*value);
    }
    
    println!("Here is an array i've got {:?}", simple_vector);
    println!("Here is average: {}", vector_average(&mut simple_vector));
    println!("Here is median: {}", vector_median(&mut simple_vector));
    println!("Here is mode: {}", vector_mode(&mut simple_vector));
}


fn vector_average(input_vector: &mut Vec<i32>) -> f64
{
    (input_vector.iter().sum::<i32>() as f64) / (input_vector.len() as f64)
}

fn vector_median(input_vector: &mut Vec<i32>) -> f64
{
    let input_vecotr_len = input_vector.len();
    
    if input_vecotr_len == 1
    {
        return input_vector[0] as f64;
    }

    input_vector.sort();
    println!("Sorted vector: {:?}", input_vector);

    if input_vecotr_len % 2 == 1
    {
        return input_vector[input_vecotr_len/2] as f64;
    }

    return  (input_vector[input_vecotr_len/2] as f64 + input_vector[input_vecotr_len/2 - 1] as f64) / 2.0;
}

fn vector_mode(input_vector: &mut Vec<i32>) -> i32
{
    let mut temp_hash_map: HashMap<i32, i32> = HashMap::new();
    
    for value in input_vector
    {
        let count = temp_hash_map.entry(*value).or_insert(1);
        *count += 1;
    }
    
    let mut key_with_maximum_entryies: i32 = 0;
    let mut maximum_number_of_entries: i32 = 0;
    
    for (key, value) in temp_hash_map
    {
        if value > maximum_number_of_entries
        {
            maximum_number_of_entries = value;
            key_with_maximum_entryies = key;
        }
    }
    
    return key_with_maximum_entryies;
}

// *** Second task *** //
// Work with strings
pub fn convert_string_to_pig_latin(input_string: &str)
{
    let mut new_string: String = String::new();
    
    for word in input_string.split(" ")
    {
        for character in word.chars()
        {
            if is_vowel(character)
            {
                new_string += &(format!("{}-hay ",word));
                break;
            }
            
            new_string += &format!("{}-{}ay ", &word[1..], &word[..1]);
            break;
            
        }
    }
    println!("Input string: {}", input_string);
    println!("Pig string: {}", new_string);
}


fn is_vowel(input_char: char) -> bool
{
    for vowel in VOWELS.iter()
    {
        if input_char == *vowel 
        {
            return true
        }
    }
    return false;
}

// *** Third task *** //

pub fn console_bot()
{
    println!("\n{} my little fellow user! It's me, a great text-interface program that can do stuff. 
What can I do you may ask? Oh, lots of things. Type {} to know more!", "Hello".cyan(), "help".green());
    
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();


    loop
    {
        let mut text_from_the_console = String::new();
        // print!(">");
        
        io::stdin().read_line(&mut text_from_the_console)
        .expect("Failed to read line");
    
        // println!("Your input: {}", text_from_the_console);
        
        let mut word = text_from_the_console.split_whitespace();
        
        let command_option = word.next();

        if command_option == None
        {
            println!("Type something!");
            continue;
        }

        let commnad = command_option.unwrap();

        if commnad == "help"
        {
            let help_command = format!("\n{} - shows this help;","help".green());
            let add_command = format!("\n{} {} to {} - adds person with specified name to the specified department. Only one word employe and department names are acceptable;","add".green(),"`person name`".cyan(), "`department name`".cyan());
            let show_command = format!("\n{0} {1} - shows all employees of the specified department. Shows all employees of all departments if the {1} is specified as {2};","show".green(),"`department name`".cyan(), "all".cyan().italic());
            let quit_command = format!("\n{} - stops the program.","quit".green());
            println!("Currently available commands:");
            println!("{}\n{}\n{}\n{}\n", help_command, add_command, show_command, quit_command);
        }
        else if commnad == "add"
        {
            let person_name = word.next(); 
            let to_word = word.next(); 
            let department_name = word.next(); 

            if person_name == None || to_word == None || department_name == None
            {
                println!("{} Not enough parameters for {} command.", "Warning:".red(), "add".green());
                continue;
            }

            let person_name = person_name.unwrap(); 
            let to_word = to_word.unwrap(); 
            let department_name = department_name.unwrap(); 

            if to_word != "to"
            {
                println!("{} Third word in /add command should be {}","Warning:".red(), "to".bold());
                continue;
            }

            if departments.contains_key(department_name)
            {
                departments.get_mut(department_name).unwrap().push(person_name.to_string());
            }
            else
            {
                departments.insert(department_name.to_string(), Vec::new());
                departments.get_mut(department_name).unwrap().push(person_name.to_string());
            }
            println!("{} {} added to {} department", person_name.cyan(), "successfully".yellow(), department_name.cyan());
        }
        else if commnad == "show"
        {
            println!();
            let destination = word.next();
            
            if destination == None
            {
                println!("{} Not enough parameters for {} command.", "Warning:".red(), "show".green());
                continue;
            }
            
            let destination = destination.unwrap();

            if destination == "all"
            {
                for (department, current_vector) in departments.iter()
                {
                    println!("{} department contains:", department.cyan());
                    for employe in current_vector
                    {
                        println!("{}",employe);
                    }
                }
                continue;
            }

            if departments.contains_key(destination)
            {
                println!("{} department contains:", destination.cyan());
                let employes_vector = departments.get(destination);
                for employe in employes_vector.unwrap()
                {
                    println!("{}", employe);
                }
            }
            else 
            {
                println!("{} There is no such department.","Warning:".red());
                continue;
            }
        }
        else if commnad == "quit"
        {
            return;
        }
        else
        {
            println!("{} Unknown command. Try {} command.", "Warning:".red(), "help".green());
        }
    }
}

