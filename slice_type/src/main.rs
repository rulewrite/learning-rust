fn main() {
    // 슬라이스는 컬렉션의 연속된 요소들을 참조하는 타입이다.

    let s = String::from("hello world");

    // String의 일부분에 대한 참조자
    // start..end는 start부터 end전까지를 포함한다.
    // 내부적으로 슬라이스 데이터 구조는 시작위치와 길이를 지정하며 ending_index - start_index이다.
    // let hello = &s[0..5];
    let hello = &s[..5]; // 앞에 0은 생략 가능하다.

    // let world = &s[6..11];
    // let world = &s[6..s.len()];
    let world = &s[6..]; // 마찬가지로 마지막은 생략가능하다.

    // let hello_world = &s[0..s.len()];
    let hello_world = &s[..]; // 전체 슬라이스를 만드려면 앞뒤 전부 생략 가능하다.

    println!("{}", hello);
    println!("{}", world);
    println!("{}", hello_world);
}
