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
}
