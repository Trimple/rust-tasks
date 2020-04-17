/*
    Пара идей для приближения этого очень чернового приложения к уровню реального приложения.
 */

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

// Most of the E24 values are unique in comparison to E48-E192 which i really important to get more variants in finished program
#[allow(dead_code)]
const E24_VALUES: [f64; 48] = [
    // First round of values
    10.0,	11.0,	12.0,	13.0,	15.0,	16.0,
    18.0,	20.0,	22.0,	24.0,	27.0,	30.0,
    33.0,	36.0,	39.0,	43.0,	47.0,	51.0,
    56.0,	62.0,	68.0,	75.0,	82.0,	91.0,
    // Second round of values
    100.0,	110.0,	120.0,	130.0,	150.0,	160.0,
    180.0,	200.0,	220.0,	240.0,	270.0,	300.0,
    330.0,	360.0,	390.0,	430.0,	470.0,	510.0,
    560.0,	620.0,	680.0,	750.0,	820.0,	910.0
];

// Enum for future use if i will be brage enougf to remake all arrays into Enum values to code flexible, so separate usage of E24, E96, E192 and E24 + E96
//  can be achived
// E96 is actually E192 with each second value used starting from 100
#[allow(dead_code)]
enum ResistorsTypes{
    E24 (f64),
    E192 (f64)
}

const MAXIMUM_ALLOWED_RATIO: f64 = 99.0;
const MINIMUM_ALLOWED_RATIO: f64 = 0.011;

//

/*
    @note пока что нет ни времени ни смысла докумментировать данный код, по этому пока ну буду 

 @ Main formulas

         R2 * Vin
 Vout = ----------
         R1 + R2

  R1     Vin - Vout 
 ---- = ------------
  R2        Vout


    Очень важная пометка для понимания того, что вообще в коде происходит
    Если отношение сопротивлений больше номинального - ошибка меньше 0
    Если отношение сопротивлений меньше номинального - ошибка больше 0
    Нам предпочтительнее ошибка меньше ноля
*/
#[allow(dead_code)]
pub fn calculate_resistors_from_voltages(input_voltage: f64, output_voltage: f64) {
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
        calculate_for_r1_smaller_then_r2(&resistance_ratio, &input_voltage);
    } 
    else
    {
        calculate_for_r1_bigger_then_r2(&resistance_ratio, &input_voltage);
    }

    return;
}

/*
    Существуют ситуации, в которых если невозможно получить точного значения напряжения предпочтительнее получать напряжение
    меньше номинального, даже если абсолютное значение этой погрешности больше, чем у лучшего варианта с напряжение выше 
    номинального. По этой причине данный алгоритм рассчитывает сразу оба варианта делителей если не находит точного совпадения.
 */
// Алгоритм для расчета делителя если отношение сопротивлений больше 1
fn calculate_for_r1_bigger_then_r2(resistance_ratio: &f64, input_voltage: &f64) {
    // Отношение для большего напряжение равно максимальному при r1 > r2 - отправная точка для уменьшения
    let mut bigger_r1: f64 = MAXIMUM_ALLOWED_RATIO;
    let mut bigger_r2: f64 = 1.0;
    let mut current_bigger_ratio = MAXIMUM_ALLOWED_RATIO;
    
    // Отношение для меньшего напряжения равно 1 - меньше минимального возможного значения при r1 > r2
    //   отправная точка для увеличения.
    let mut smaller_r1: f64 = 1.0;
    let mut smaller_r2: f64 = 1.0;
    let mut current_smaller_ration = 1.0;

    // Алгоритм вычисления при точном совпадении отличается от алгоритма без совппадения, соответствено нужен флаг
    let mut is_exact_ratio = false;

    // Переменные для проверки эффективности алгоритма, удалю, когда посчитаю, что алгоритм закончен.
    let mut number_of_full_algorithm_calls: u32 = 0;
    let mut number_of_continues: u32 = 0;

    // Некий способ оптимизации алгоритма если разница между значениями довольно большая, однако на самом деле данны метд дает не столько
    // серьезное улучшение итогового результата, сколько серьезное улучшение вычислительной нагрузки, может быть удалю со временем.
    let mut range_divider = 1.0;
    if *resistance_ratio > 20.0{
        range_divider = 10.0;
    }

    for resistor1 in E96_VALUES.iter() {
        // Для того, чтобы сократить количество операций деления выполняемых алгоритмом будем проводить деление только если сопротивление r2 
        // находится в 2% от нужного. Шаг выбран с оглядкой на точность резисторов - 1%, соответственно резистора подобраны так, чтобы 
        // максимальное значение с учетом погрешности одного резистора по возможности минимально пересекалось с минимальным с учетом погрешности
        // значением следующего. Соответственно интервал в погрешность*2 - хороший шаг для выбора резистора, данный шаг точно обеспечит по крайней 
        // 1 значение для сравнения для 
        let minimum_r2_value: f64 = *resistor1 / (*resistance_ratio * 1.02);

        for resistor2 in E96_VALUES.iter() {
            let real_r2_value = resistor2 / range_divider;

            // Если значение резистора меньше, чем значение допустимой зоны - пропускаем значение
            if real_r2_value < minimum_r2_value {
                number_of_continues += 1;   // Для оценки эффективности алгоритма
                continue;
            }
            
            number_of_full_algorithm_calls += 1; // Для оценки эффективности алгоритма

            let current_resistors_ratio = *resistor1 / real_r2_value;

            if is_exact_ratio {
                if (current_resistors_ratio - *resistance_ratio).abs() < 0.000001    // You should not compare floating point units with ==
                {
                    println!("R1: {}  \t R2: {} ", resistor1, real_r2_value);       // Пока если есть точное совпадение, то выводятся все 

                    // For the same r1 value there is no more r2 values possible, so we break
                    break;
                }

                if current_resistors_ratio < *resistance_ratio
                {
                    break;
                }
            } 
            else 
            {
                // Если первое точное совпадение. Потом поменяю то, как конкретно это выводится на экран
                if (current_resistors_ratio - *resistance_ratio).abs() < 0.000001
                {
                    println!( "Exact match was found! \nR1: {}  \t R2: {} ", resistor1, real_r2_value);
                    is_exact_ratio = true;
                    break;
                }

                // Если значение меньше номинального надо проверить меньшее отношение и закончить итерацию
                if current_resistors_ratio < *resistance_ratio {
                    if *resistance_ratio - current_resistors_ratio < *resistance_ratio - current_smaller_ration {
                        smaller_r1 = *resistor1;
                        smaller_r2 = real_r2_value;
                        current_smaller_ration = current_resistors_ratio;
                    }
                    break;
                }
                
                if current_resistors_ratio - *resistance_ratio < current_bigger_ratio - *resistance_ratio
                {
                    bigger_r1 = *resistor1;
                    bigger_r2 = real_r2_value;
                    current_bigger_ratio = current_resistors_ratio;
                }
            }
        }
    }

    if !is_exact_ratio {    // Если совпадение не точное, выведем дополнительную инфорамцию об отдельных делителях
        let desired_output_voltage: f64 = input_voltage/(1.0 + resistance_ratio);

        println!("No exact matches were found!");

        println!("Desired output voltage: {:.4} V", desired_output_voltage);

        println!("\nBigger ratio\n");
        let bigger_ratio_output_voltage = display_divider_information(bigger_r1, bigger_r2, input_voltage);
        println!("Absolute mistake: {:.4} V", bigger_ratio_output_voltage - desired_output_voltage);
        println!("Relative mistake: {:.4} %", (bigger_ratio_output_voltage - desired_output_voltage) * 100.0 / desired_output_voltage);
        
        println!("\nSmaller ratio\n");
        let smaller_ratio_output_voltage = display_divider_information(smaller_r1, smaller_r2, input_voltage);
        println!("Absolute mistake: {:.4} V", smaller_ratio_output_voltage - desired_output_voltage);
        println!("Relative mistake: {:.4} %", (smaller_ratio_output_voltage - desired_output_voltage) * 100.0 / desired_output_voltage);
    }

    // calculation to check effectivness of the algorithm
    let mut fast_number: u32 = 0;
    for i in 1..=192
    {
        fast_number += i;
    }
    println!("\nAlgorithms efficiency stuff!");
    println!("Expected number of multiplications: {}", fast_number);
    println!("Real number of multiplications: {}", number_of_full_algorithm_calls);
    println!("Real number of continue calls: {}", number_of_continues);

}

//*******************
// Наглый копипаст и переделывание функции сверху
#[allow(dead_code)]
fn calculate_for_r1_smaller_then_r2(resistance_ratio: &f64, input_voltage: &f64) {
    // Большее отношение равно 1 при r1 < r2 - отправная точка для уменьшения
    let mut bigger_r1: f64 = 1.0;
    let mut bigger_r2: f64 = 1.0;
    let mut current_bigger_ratio = 1.0;
    
    // Меньшее отношение минимальному при r1 > r2 - отправная точка для увеличения.
    let mut smaller_r1: f64 = MINIMUM_ALLOWED_RATIO;
    let mut smaller_r2: f64 = 1.0;
    let mut current_smaller_ration = MINIMUM_ALLOWED_RATIO;

    // Алгоритм вычисления при точном совпадении отличается от алгоритма без совппадения, соответствено нужен флаг
    let mut is_exact_ratio = false;

    // Переменные для проверки эффективности алгоритма, удалю, когда посчитаю, что алгоритм закончен.
    let mut number_of_full_algorithm_calls: u32 = 0;
    let mut number_of_continues: u32 = 0;

    // Некий способ оптимизации алгоритма если разница между значениями довольно большая, однако на самом деле данны метд дает не столько
    // серьезное улучшение итогового результата, сколько серьезное улучшение вычислительной нагрузки, может быть удалю со временем.
    let mut range_divider = 1.0;
    if *resistance_ratio < 0.05
    {
        range_divider = 10.0;
    }

    for resistor2 in E96_VALUES.iter() 
    {
        // Для того, чтобы сократить количество операций деления выполняемых алгоритмом будем проводить деление только если сопротивление r2 
        // находится в 2% от нужного. Шаг выбран с оглядкой на точность резисторов - 1%, соответственно резистора подобраны так, чтобы 
        // максимальное значение с учетом погрешности одного резистора по возможности минимально пересекалось с минимальным с учетом погрешности
        // значением следующего. Соответственно интервал в погрешность*2 - хороший шаг для выбора резистора, данный шаг точно обеспечит по крайней 
        // 1 значение для сравнения для 
        let minimum_r1_value: f64 = *resistor2 * (*resistance_ratio * 0.97);

        // println! (" Current r2 value:{}\nMinimum r1 value: {}\n", *resistor2, minimum_r1_value);

        for resistor1 in E96_VALUES.iter() {
            let real_r1_value = resistor1 / range_divider;

            // Если значение резистора меньше, чем значение допустимой зоны - пропускаем значение
            if real_r1_value < minimum_r1_value {
                number_of_continues += 1;   // Для оценки эффективности алгоритма
                continue;
            }
            
            number_of_full_algorithm_calls += 1; // Для оценки эффективности алгоритма

            let current_resistors_ratio = real_r1_value / *resistor2;

            if is_exact_ratio {
                if (current_resistors_ratio - *resistance_ratio).abs() < 0.000001    // You should not compare floating point units with ==
                {
                    println!("R1: {}  \t R2: {} ", real_r1_value, resistor2);       // Пока если есть точное совпадение, то выводятся все 

                    // For the same r1 value there is no more r2 values possible, so we break
                    break;
                }

                if current_resistors_ratio > *resistance_ratio
                {
                    break;
                }
            } 
            else 
            {
                // Если первое точное совпадение. Потом поменяю то, как конкретно это выводится на экран
                if (current_resistors_ratio - *resistance_ratio).abs() < 0.000001
                {
                    println!( "Exact match was found! \nR1: {}  \t R2: {} ", real_r1_value, resistor2);
                    is_exact_ratio = true;
                    break;
                }

                // Если значение больше номинального надо проверить меньшее отношение и закончить итерацию
                if current_resistors_ratio > *resistance_ratio {
                    if current_resistors_ratio - *resistance_ratio < current_bigger_ratio - *resistance_ratio {
                        bigger_r1 = real_r1_value;
                        bigger_r2 = *resistor2;
                        current_bigger_ratio = current_resistors_ratio;
                    }
                    break;
                }

                if *resistance_ratio - current_resistors_ratio < *resistance_ratio - current_smaller_ration
                {
                    smaller_r1 = real_r1_value;
                    smaller_r2 = *resistor2;
                    current_smaller_ration = current_resistors_ratio;
                }

            }
        }
    }

    if !is_exact_ratio {    // Если совпадение не точное, выведем дополнительную инфорамцию об отдельных делителях
        let desired_output_voltage: f64 = input_voltage/(1.0 + resistance_ratio);

        println!("No exact matches were found!");

        println!("Desired output voltage: {:.4} V", desired_output_voltage);

        println!("\nBigger ratio\n");
        let bigger_ratio_output_voltage = display_divider_information(bigger_r1, bigger_r2, input_voltage);
        println!("Absolute mistake: {:.4} V", bigger_ratio_output_voltage - desired_output_voltage);
        println!("Relative mistake: {:.4} %", (bigger_ratio_output_voltage - desired_output_voltage) * 100.0 / desired_output_voltage);
        
        println!("\nSmaller ratio\n");
        let smaller_ratio_output_voltage = display_divider_information(smaller_r1, smaller_r2, input_voltage);
        println!("Absolute mistake: {:.4} V", smaller_ratio_output_voltage - desired_output_voltage);
        println!("Relative mistake: {:.4} %", (smaller_ratio_output_voltage - desired_output_voltage) * 100.0 / desired_output_voltage);
    }

    // calculation to check effectivness of the algorithm
    let mut fast_number: u32 = 0;
    for i in 1..=192
    {
        fast_number += i;
    }
    println!("\nAlgorithms efficiency stuff!");
    println!("Expected number of multiplications: {}", fast_number);
    println!("Real number of multiplications: {}", number_of_full_algorithm_calls);
    println!("Real number of continue calls: {}", number_of_continues);
}

/*
В дальнешем в эту функцию стоит будет добавить отображения рассчетной точности итогового делителя, как сделано в python версии программы.
При этом точность может определяться с помощью типа резистора.
Также важно добавить в функцию вычисление параметров делителей: протекающий ток и падающую мощность, возможно со временем можно добавить
отображение чего-то типа уровня шума.
*/
// Displays particular divider parameters and returns 
fn display_divider_information(r1_value: f64, r2_value: f64, input_voltage: &f64) -> f64
{
    let output_voltage = (r2_value * input_voltage) / (r1_value + r2_value);

    println!("Devider information:");
    println!("Input voltage: {} V", input_voltage);
    println!("R1: {} Ohms\nR2: {} Ohms", r1_value, r2_value);
    println!("=========================================");
    println!("Output voltage: {:.4} V", output_voltage);

    return output_voltage;
}

// *** Stuff for possible future use *** // 
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn calculate_r1(input_voltage: f64, output_voltage:f64, r2_value: f64)
{
    
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn calculate_r2(input_voltage: f64, output_voltage:f64, r1_value: f64)
{

}



