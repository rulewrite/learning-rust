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

    // `while`로 컬렉션을 순회할 수 있다.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }
    // 하지만 위와 같은 경우,
    // 에러가 발생하기 쉬우며 (배열의 인덱스를 정확히 알아야 함.)
    // 느리다. (컴파일러가 반복될 때마다 index가 배열 범위에 포함되는지 조건 검사를 런타임 코드에 추가하기 때문)

    // 따라서 for 문으로 순회하는게 좋다.
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {}", element);
    }

    // 특정 횟수만큼 실행하는 상황에서도 표준 라이브러리에 제공되는 Range를 활용한 for문이 깔끔하다.
    // Range는 한 숫자에서 다른 숫자의 "전"까지 모든 숫자를 차례로 생성한다.
    for number in (1..4).rev() {
        println!("{}!", number); // 3, 2, 1
    }
    println!("LIFTOFF!!!");
}
