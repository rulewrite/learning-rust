fn main() {
    let celsius = 20f64;
    println!("섭씨 {}도는 화씨로: {}", celsius, to_fahrenheit(celsius));

    let fahrenheit = 20.0;
    println!("화씨 {}도는 섭씨로: {}", fahrenheit, to_celsius(fahrenheit));
}

const FREEZING_POINT_OF_WATER_IN_FAHRENHEIT: f64 = 32.0;
const FAHRENHEIT_AND_CELSIUS_PROPORTION: f64 = 180.0 / 100.0;

// 섭씨를 화씨로 변환
fn to_fahrenheit(celsius: f64) -> f64 {
    (celsius * FAHRENHEIT_AND_CELSIUS_PROPORTION) + FREEZING_POINT_OF_WATER_IN_FAHRENHEIT
}

// 화씨를 섭씨로 변환
fn to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - FREEZING_POINT_OF_WATER_IN_FAHRENHEIT) / FAHRENHEIT_AND_CELSIUS_PROPORTION
}
