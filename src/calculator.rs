const E96_VALUES: [f64; 192] = [
    //first round of values
    10.0, 10.2, 10.5, 10.7, 11.0, 11.3, 11.5, 11.8, 12.1, 12.4, 12.7, 13.0, 13.3, 13.7, 14.0, 14.3,
    14.7, 15.0, 15.4, 15.8, 16.2, 16.5, 16.9, 17.4, 17.8, 18.2, 18.7, 19.1, 19.6, 20.0, 20.5, 21.0,
    21.5, 22.1, 22.6, 23.2, 23.7, 24.3, 24.9, 25.5, 26.1, 26.7, 27.4, 28.0, 28.7, 29.4, 30.1, 30.9,
    31.6, 32.4, 33.2, 34.0, 34.8, 35.7, 36.5, 37.4, 38.3, 39.2, 40.2, 41.2, 42.2, 43.2, 44.2, 45.3,
    46.4, 47.5, 48.7, 49.9, 51.1, 52.3, 53.6, 54.9, 56.2, 57.6, 59.0, 60.4, 61.9, 63.4, 64.9, 66.5,
    68.1, 69.8, 71.5, 73.2, 75.0, 76.8, 78.7, 80.6, 82.5, 84.5, 86.6, 88.7, 90.9, 93.1, 95.3, 97.6,
    // Second round of values
    100.0, 102.0, 105.0, 107.0, 110.0, 113.0, 115.0, 118.0, 121.0, 124.0, 127.0, 130.0, 133.0,
    137.0, 140.0, 143.0, 147.0, 150.0, 154.0, 158.0, 162.0, 165.0, 169.0, 174.0, 178.0, 182.0,
    187.0, 191.0, 196.0, 200.0, 205.0, 210.0, 215.0, 221.0, 226.0, 232.0, 237.0, 243.0, 249.0,
    255.0, 261.0, 267.0, 274.0, 280.0, 287.0, 294.0, 301.0, 309.0, 316.0, 324.0, 332.0, 340.0,
    348.0, 357.0, 365.0, 374.0, 383.0, 392.0, 402.0, 412.0, 422.0, 432.0, 442.0, 453.0, 464.0,
    475.0, 487.0, 499.0, 511.0, 523.0, 536.0, 549.0, 562.0, 576.0, 590.0, 604.0, 619.0, 634.0,
    649.0, 665.0, 681.0, 698.0, 715.0, 732.0, 750.0, 768.0, 787.0, 806.0, 825.0, 845.0, 866.0,
    887.0, 909.0, 931.0, 953.0, 976.0,
];

// в Е24 есть уникальные от всех остальных номиналы сопротивления, которые в то же время имеют точность 1% как и Е96, соответственно чтобы сделать более 
// точный подбор стоит совместить вместе списке E96 и E24 и при необходимости можно запариться
// const R24_SPECIFIC_VALUES: [f64;] = [
// 12.0, 13.0

// 120.0
// ];

const MAXIMUM_ALLOWED_RATIO: f64 = 99.0;
const MINIMUM_ALLOWED_RATIO: f64 = 0.011;

//

/*
 @ Main formulas

         R2 * Vin
 Vout = ----------
         R1 + R2

  R1     Vin - Vout 
 ---- = ------------
  R2        Vout

   Если отношение сопротивлений больше номинального - ошибка меньше 0
   Если отношение сопротивлений меньше номинального - ошибка больше 0

   Нам предпочтительнее ошибка меньше ноля
*/
#[allow(dead_code)]
pub fn calculate_best_resistors_values(input_voltage: f64, output_voltage: f64) {
    if input_voltage < 0.0 || output_voltage < 0.0 || output_voltage >= input_voltage {
        println!("Wrong input voltage!");
        return;
    }

    let resistance_ratio = (input_voltage - output_voltage) / output_voltage;

    // For now, let assume, that if dividing ration is bigger then 50 or smaller than 0.02
    // it is too big to be real. Error handling should be made better in the future!!
    if resistance_ratio == 1.0 {
        println!("Resistors should have same value!");
        return;
    } else if resistance_ratio > MAXIMUM_ALLOWED_RATIO {
        println!("Ratio is too big!");
        return;
    } else if resistance_ratio < MINIMUM_ALLOWED_RATIO {
        println!("Ratio is too small!");
        return;
    }

    if resistance_ratio < 1.0 {
        // calculate_for_r1_smaller_then_r2(&resistance_ratio)
        println! ("Not ready yet");
    } else {
        calculate_for_r1_bigger_then_r2(&resistance_ratio, &input_voltage, &output_voltage);
    }

    return;
}

// This algorithm goes through double e96 range for r1 and e96/10 for r2 (r2 is smaller then r1)
fn calculate_for_r1_bigger_then_r2(resistance_ratio: &f64, input_voltage: &f64, output_voltage: &f64) {
    // Отношение для большего напряжение равно максимальному - отправная точка для уменьшения
    // Отношение для меньшего напряжения равно 1 - отправная точка для увеличения при r1 > r2
    let mut bigger_r1: f64 = MAXIMUM_ALLOWED_RATIO;
    let mut bigger_r2: f64 = 1.0;
    let mut current_bigger_ration = MAXIMUM_ALLOWED_RATIO;

    let mut smaller_r1: f64 = 1.0;
    let mut smaller_r2: f64 = 1.0;
    let mut current_smaller_ration = 1.0;

    let mut is_exact_ratio = false;

    let mut number_of_iterations: u32 = 0;
    let mut number_of_full_inputs: u32 = 0;

    let mut range_devider = 1.0;

    if *resistance_ratio > 20.0{
        range_devider = 10.0;
    }

    for resistor1 in E96_VALUES.iter() {
        // Для того, чтобы сократить количество операций деления выполняемых алгоритмом будем проводить деление только
        let one_rercent_zone: f64 = *resistor1 / (*resistance_ratio * 1.02);
        let max_value: f64 = *resistor1 / *resistance_ratio;

        // println!("\none percent {} at r1 = {}, max value {} ", one_rercent_zone, *resistor1, max_value);

        // let check_zone: f64 = *resistor1 / (current_smaller_ration);
        for resistor2 in E96_VALUES.iter() {
            let real_r2_value = resistor2 / range_devider;
            // Если значение резистора меньше, чем значение допустимой зоны - пропускаем значение
            if real_r2_value < one_rercent_zone {
                number_of_full_inputs += 1;
                continue;
            }
            // print! ("{}, ", real_r2_value);
            number_of_iterations += 1;

            let current_resistors_ratio = *resistor1 / real_r2_value;

            if is_exact_ratio {
                if (current_resistors_ratio - *resistance_ratio).abs() < 0.00001 {
                // if current_resistors_ratio == *resistance_ratio {
                    println!("R1: {}  \t R2: {} ", resistor1, real_r2_value);

                    // For the same r1 value there is no more r2 values possible, so we break
                    break;
                }

                if current_resistors_ratio < *resistance_ratio {
                    break;
                }
            } 
            else 
            {
                // Если первое точное совпадение
                // if current_resistors_ratio == *resistance_ratio {
                if (current_resistors_ratio - *resistance_ratio).abs() < 0.00001 {
                    println!( "Exact match was found! \nR1: {}  \t R2: {} ", resistor1, real_r2_value);
                    is_exact_ratio = true;
                    break;
                }

                // Если значение больше номинального надо проверить меньшее отношение и закончить итерацию
                if current_resistors_ratio < *resistance_ratio {
                    if *resistance_ratio - current_resistors_ratio < *resistance_ratio - current_smaller_ration {
                        smaller_r1 = *resistor1;
                        smaller_r2 = real_r2_value;
                        current_smaller_ration = current_resistors_ratio;
                    }
                    break;
                }
                
                bigger_r1 = *resistor1;
                bigger_r2 = real_r2_value;
                // current_bigger_ration = bigger_r1 / bigger_r2;

            }
        }
    }

    if !is_exact_ratio {
        println!("Desired ration is {} \n", resistance_ratio);
        println!("Bigger ratio");
        display_devider_information(bigger_r1, bigger_r2, input_voltage, output_voltage);
        
        println!("Smaller raio");
        display_devider_information(smaller_r1, smaller_r2, input_voltage, output_voltage);
    }

    
    let mut fast_number: u32 = 0;
    for i in 1..=192
    {
        fast_number += i;
    }
    println!("Expected number of multiplications: {}", fast_number);
    println!("Real number of multiplications: {}", number_of_iterations);
    println!("Real number of calculateions: {}", number_of_full_inputs);

}

// fn calculate_for_r1_smaller_then_r2(resistance_ratio: &f64) {
//     // // Изначально отношение будет равно 1, что всегда больше чем нужное нам, так как r1 > r2
//     // let mut bigger_r1: f64 = 1.0;
//     // let mut bigger_r2: f64 = 1.0;
//     // let mut smaller_r1: f64 = 1.0;
//     // let mut smaller_r2: f64 = 1.0;
//     // let mut is_exact_ratio = false;

//     // for resistor1 in E96_VALUES.iter()
//     // {
//     //     // To reduce the number of f64 divisions that should be executed every iteration we will only check values if their ratio is in a 1% zone of nominal
//     //     let one_rercent_zone: f64 = resistor1/(*resistance_ratio * 1.01);
//     //     for resistor2 in E96_VALUES.iter()
//     //     {
//     //         if (*resistor2 / 10.0) < one_rercent_zone
//     //         {
//     //             continue;
//     //         }

//     //         let current_resistors_ratio = *resistor1 / (*resistor2 / 10.0);

//     //         if is_exact_ratio
//     //         {
//     //             if current_resistors_ratio == *resistance_ratio
//     //             {
//     //                 println!("R1: {}\n R2: {} ", resistor1, resistor2);

//     //                 // For the same r1 value there is no more r2 values possible, so we break
//     //                 break;
//     //             }

//     //             if current_resistors_ratio > *resistance_ratio
//     //             {
//     //                 break;
//     //             }
//     //         }
//     //         else
//     //         {
//     //             if current_resistors_ratio == *resistance_ratio
//     //             {
//     //                 println!("Exact match was found! \n R1: {}\n R2: {} ", resistor1, resistor2);
//     //                 is_exact_ratio = true;
//     //                 break;
//     //             }

//     //             if current_resistors_ratio > *resistance_ratio
//     //             {
//     //                 if current_resistors_ratio - *resistance_ratio < bigger_r1 / bigger_r2 - *resistance_ratio

//     //                 bigger_r1 = *resistor1;
//     //             }
//     //         }

//     //         // First one bigger then we need can be closest with
//     //         if (*resistor2 / 10.0) >= *resistor1
//     //         {

//     //             break;
//     //         }
//     //         if is_exact_ratio
//     //         {
//     //             if resistor1 / (resistor2 / 10.0) == *resistance_ratio
//     //             {
//     //                 println!("R1: {}\n R2: {} ", resistor1, resistor2);
//     //                 // For the same r1 value there is no more r2 values possible, so we break
//     //                 break;
//     //             }
//     //             if resistor1/resistor2 > *resistance_ratio
//     //         }
//     //         else
//     //         {

//     //         }

//     //     }
//     // }
// }

fn display_devider_information(chosen_r1: f64, chosen_r2: f64, input_voltage: &f64, desired_output_voltage: &f64) 
{
    let final_output_voltage = chosen_r2*input_voltage/(chosen_r1 + chosen_r2);
    let absolute_mistake = final_output_voltage - desired_output_voltage; // in V
    let relative_mistake = (desired_output_voltage - final_output_voltage) / desired_output_voltage * 100.0; // in %

    println!("Resistance ratio: {}", chosen_r1/chosen_r2);
    println!("R1: {}\nR2: {}", chosen_r1, chosen_r2);
    println!("Real output voltage: {}", final_output_voltage);
    println!("Absolute mistake: {} V", absolute_mistake);
    println!("Relative mistake: {} %\n", relative_mistake);
}
