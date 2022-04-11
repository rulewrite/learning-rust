fn main() {
    let number = 3;

    // 조건과 관련된 코드 블럭은 match arms와 같이 arms라 부름
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 조건은 반드시 bool 타입이어야 한다. 묵시적 형변환이 없음.
    // if number {
    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    // if도 표현식이기에 let 구문 우축에 사용이 가능함.
    let number = if condition { 5 } else { 6 }; // 코드 블럭은 마지막 표현식으로 평가되고 숫자 자체도 표현식임을 기억하자.
    println!("The value of number is: {}", number); // 5

    // 아울러 if에 속한 각 arms의 결과는 반드시 같은 타입이어야 한다. 즉 아래는 실패한다.
    // let number = if condition { 5 } else { "six" };
    // 변수는 단일 타입을 가져야 하고 러스트는 컴파일 시에 `number` 변수의 타입이 뭔지 정확히 알아야 하기 때문.
    // 그래야 `number`가 사용되는 모든 곳에서 유효한지 검증이 가능하다.
}
