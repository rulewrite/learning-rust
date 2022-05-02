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

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn main() {
    println!("{}", value_on_cents(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // 3과 7외에 모든 수를 의마하며 이름이 꼭 other가 아니어도 된다.
        // 아울러 마지막 패턴은 나열하지 않은 가능한 모든 값을 의미하기에 마지막에 위치하며 없다면 컴파일 되지 않는다.
        // other => move_player(other),

        // 만약 굳이 값을 사용하지 않는다면 _를 사용한다.
        // _ => reroll(),

        // 만약 아무 일도 일어나지 않는다면 빈 튜플 타입을 사용하여 표현할 수 있다.
        // 이전에 언급한 패턴과 일치하지 않는 값을 사용하지 않을 것이며 어떤 코드도 실행하지 않는다고 러스트에게 명시적으로 알려줄 수 있다.
        _ => (),
    }
}
