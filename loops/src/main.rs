fn main() {
    // 러스트가 제공하는 3가지 반복문: loop, while, for

    // 명시적으로 중지할 때까지 반복을 수행한다.
    loop {
        println!("again!");
        break;
    }

    let mut count = 0;
    // 루프에 레이블 지정
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up; // 'counting_up 루프를 멈춤
            }

            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
