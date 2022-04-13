fn main() {
    let mut s1 = String::from("hello");

    // &는 참조자를 나타내며 소유권 대신 참조자를 넘김
    let len = calculator_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // 참조 생성 작업을 빌림(Borrowing)이라 하며 함수의 파라미터로 생성되는 것(calculator_length)도 마찬가지.
    let s2 = &mut s1;
    // 변수가 불변이듯 참조도 불변으로 빌린 것은 수정할 수 없다.
    s2.push_str(", world!");
    change(s2);
    println!("The length of '{}' is {}.", s2, s2.len());

    mutable_references();
}

// 매개변수로 &String 지정함. s는 String의 참조자
fn calculator_length(s: &String) -> usize {
    // 참조는 s -> s1 -> 힙데이터로 이루어짐
    s.len()
} // s는 스포크 밖으로 벗어났지만 값에 대한 소유권이 없어 아무 일도 일어나지 않는다.

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn mutable_references() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // 가변 참조자는 한번에 하나만 가질 수 있으므로 아래는 실패한다.
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    println!("{}", r1);

    // r1이 사용된 후 여기서는 가능하다.
    let r2 = &mut s;
    println!("{}", r2);
    // 이러한 제한 사항은 데이터 레이스를 방지할 수 있다.
    // 1. 두 개 이상의 포인터가 동시에 같은 데이터에 접근하여
    // 2. 그 중 적어도 하나의 포인터가 데이터를 쓴다(write).
    // 3. 데이터에 접근 간에 동기화하를 메커니즘이 없음.
    // 러스트는 데이터 레이스가 발생할 수 있는 코드는 컴파일 조차 안되어 문제의 발생을 막는다.

    let mut s2 = String::from("hello");
    {
        let r3 = &mut s2;
        r3.push_str("hello");
    } // r3은 스코프 밖으로 벗어나므로 새로운 참조자를 만들 수 있다.
    let r4 = &mut s2;
    r4.push_str("hello");
    println!("{}", s2); // hellohellohello

    // 가변 참조자와 불변 참조자를 혼용할 때도 비슷한 규칙이 있다.
    let mut s3 = String::from("hello");

    let r5 = &s3;
    let r6 = &s3;
    // 불변 참조자가 있는 동안 가변 참조자를 만들 수 없다.
    // 불변 참조자의 사용자는 사용 간에 값이 바뀌리라 예상하지 않기 때문에
    // let r7 = &mut s3; // 문제가 되는 코드
    // println!("{} {} {}", r5, r6, r7);

    println!("{} {}", r5, r6);

    // 참조의 범위는 사용된 마지막 시점까지 계속된다. r5, r6는 위에 사용 후 사용되지 않으므로 아래는 컴파일 가능하다.
    let r7 = &mut s3;
    println!("{}", r7);
}
