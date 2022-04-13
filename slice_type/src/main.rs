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

    // let mut s2 = String::from("hello world");
    let s2 = String::from("hello world");
    let word = first_word(&s2);
    // s2.clear(); // 앞서 불변 참조자를 만들었으니 가변 참조자는 불가하다. clear 내에서 가변 참조자를 갖기 위한 시도를 하기 때문에 해당 코드는 컴파일이 실패한다.
    println!("{}", word);

    // 문자열 리터럴은 슬라이스다. (타입: &str)
    // 문자열 리터럴은 컴파일된 실행파일 내에 존재하며 특정 지점을 가리킨다. &str은 불변이므로 스트링 리터럴은 불변이다.
    let s3 = "hello, world";
    println!("{}", s3);

    let my_string = String::from("hello world");
    // 문자열의 전체 또는 일부에서도 동작
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // 문자열에 대한 참조에서도 작동하며 전체 슬라이스를 의미함. (암시적 변환)
    let word = first_word(&my_string);

    let my_string_literal = "hello world";
    // 전체 또는 부분 문자열 리터럴에서도 동작
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);
    // 문자열 리터럴은 이미 문자열 슬라이스이기 때문에 슬라이스 구문 없이도 동작
    let word = first_word(my_string_literal);
}

// 반환값은 슬라이스 시작 위치 참조와 요소 수로 이루어짐 (슬라이스 타입)
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // 바이트 배열로 변환

    for (i, &item) in bytes // 튜플 구조 해제
        // 바이트 배열의 반복자를 생성
        .iter()
        // 반복 결과를 (index, 요소에 대한 참조값) 튜플로 만들어 반환
        .enumerate()
    {
        // 바이트 리터럴 문법으로 공백 문자를 나타내는 바이트 값을 알아냄
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
