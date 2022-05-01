enum Coin {
    Penny, // variants
    Nickel,
    Dime,
    Quarter,
}

fn value_on_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5, // arms
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("{}", value_on_cents(Coin::Penny));
}
