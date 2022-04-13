fn main() {
    let s1 = String::from("hello");

    // &는 참조자를 나타내며 소유권 대신 참조자를 넘김
    let len = calculator_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // 참조 생성 작업을 빌림(Borrowing)이라 하며 함수의 파라미터로 생성되는 것(calculator_length)도 마찬가지.
    let s2 = &s1;
    // 변수가 불변이듯 참조도 불변으로 빌린 것은 수정할 수 없다.
    // s2.push_str(", world!");
    println!("The length of '{}' is {}.", s2, s2.len());
}

// 매개변수로 &String 지정함. s는 String의 참조자
fn calculator_length(s: &String) -> usize {
    // 참조는 s -> s1 -> 힙데이터로 이루어짐
    s.len()
} // s는 스포크 밖으로 벗어났지만 값에 대한 소유권이 없어 아무 일도 일어나지 않는다.
