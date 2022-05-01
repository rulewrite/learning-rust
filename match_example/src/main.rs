#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny, // variants
    Nickel,
    Dime,
    Quarter(UsState), // 내부에 UsState 값을 갖도록 추가
}

fn value_on_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5, // arms
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // 러스트는 특정 케이스를 다루지 않으면 컴파일이 되지 않는다.
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("{}", value_on_cents(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
