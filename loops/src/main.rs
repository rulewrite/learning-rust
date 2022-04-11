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

    // 스레드가 작업을 완료했는지 확인하거나, 실패할 수 있는 작업을 재시도 할 때 사용한다.
    // 또한 해당 작업의 결과를 루프 외부로 전달할 수 있다.
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // 반환 값 명시
        }
    };
    println!("The result is {}", result); // 20

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}
