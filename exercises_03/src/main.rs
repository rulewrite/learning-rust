fn main() {
    let celsius = 20f64;
    println!("섭씨 {}도는 화씨로: {}", celsius, to_fahrenheit(celsius));

    let fahrenheit = 20.0;
    println!("화씨 {}도는 섭씨로: {}", fahrenheit, to_celsius(fahrenheit));

    let number = 55;
    println!(
        "{}번째 피보나치 수는: {}",
        number,
        get_fibonacci_number(number)
    );

    print_twelve_days_of_christmas_lyrics();
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

fn get_fibonacci_number(number: u64) -> u64 {
    if number == 0 {
        return 0;
    }
    if number == 1 {
        return 1;
    }
    let mut current = 0;
    let mut previous_of_previous = 0;
    let mut previous = 1;
    // 0, 1, 2, 3, 4, 5, 6, 7...
    // 0, 1, 1, 2, 3, 5, 8, 13...
    for _ in 2..number + 1 {
        current = previous_of_previous + previous;
        previous_of_previous = previous;
        previous = current;
    }
    return current;
}

const GIFTS_BY_DAY: [&str; 12] = [
    "partridge in a pear tree",
    "turtle-doves",
    "French hens",
    "calling birds",
    "golden rings",
    "geese a laying",
    "swans a swimming",
    "maids a-milking",
    "ladies dancing",
    "lords a-leaping",
    "pipers piping",
    "drummers drumming",
];
fn print_twelve_days_of_christmas_lyrics() {
    for today in 1..13 {
        println!("On the {} day of Christmas\nMy true love sent to me", today);

        for day in (1..today + 1).rev() {
            let gift = GIFTS_BY_DAY[day - 1];
            let quantity = day;
            let is_multiple_gifts = today > 1;

            if quantity == 1 {
                println!("{} {}", if is_multiple_gifts { "And a" } else { "A" }, gift);
                continue;
            }

            println!("{} {}", quantity, gift);
        }

        println!("");
    }
}
