fn fahrenheit_to_celsius(temp: f64) -> f64{
    (temp - 32.0) * 5.0 / 9.0
}
// fn celsius_to_fahrenheit(temp: f64) -> f64{
//     (temp * 9.0 / 5.0) + 32.0
// }

fn main(){
    let mut _f_temp:f64 = 85.0;

    let _c_temp = fahrenheit_to_celsius(_f_temp);
    println!("Fahrenheit to Celsius: {} -> {}", _f_temp, _c_temp);

    for _i in 0..5{
        _f_temp += 1.0;
        println!("{} -> {}", _f_temp, fahrenheit_to_celsius(_f_temp));
    }
}