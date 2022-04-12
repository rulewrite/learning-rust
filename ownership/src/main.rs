fn main() {
    // 문자열 리터럴은 편하지만 불변이라 한계가 있다. 언제나 모든 문자열 값을 알 수 없다.
    // 만약 사용자 입력을 받아 저장하거나 할 때는 두번째 문자열 타입인 String을 사용한다.
    // 이 타입은 힙에 할당되므로 컴파일 타임에 알 수 없는(정해지지 않는) 양의 텍스트를 저장할 수 있다.
    let mut s = String::from("hello"); // ::은 네임스페이스 연산자

    s.push_str(", world!"); // 전달한 스트링 리터럴을 기존 뒤에 붙여준다.

    println!("{}", s);

    string_scope();
}

fn string_scope() {
    let s = String::from("hello"); // s는 여기서부터 유효

    // s를 가지고 작업
    println!("{}", s);
} // 이 스코프는 끝났고, s는 더 이상 유효하지 않다. `drop()`
