fn main() {
    // 문자열 리터럴은 편하지만 불변이라 한계가 있다. 언제나 모든 문자열 값을 알 수 없다.
    // 만약 사용자 입력을 받아 저장하거나 할 때는 두번째 문자열 타입인 String을 사용한다.
    // 이 타입은 힙에 할당되므로 컴파일 타임에 알 수 없는(정해지지 않는) 양의 텍스트를 저장할 수 있다.
    let mut s = String::from("hello"); // ::은 네임스페이스 연산자

    s.push_str(", world!"); // 전달한 스트링 리터럴을 기존 뒤에 붙여준다.

    println!("{}", s);

    string_scope();

    move_value();

    clone_value();
}

fn string_scope() {
    let s = String::from("hello"); // s는 여기서부터 유효

    // s를 가지고 작업
    println!("{}", s);
} // 이 스코프는 끝났고, s는 더 이상 유효하지 않다. `drop()`

/**
 * 변수와 데이터가 상호작용하는 방법: 이동(move)
 * 특정 변수의 값을 다른 변수에 대입할 때.
 */
fn move_value() {
    let x = 5;
    // x의 값 5가 y로 복사(copy) 됨.
    let y = x;
    println!("{}, {}", x, y);

    let s1 = String::from("is move");
    // 얕은 복사가 아닌 이동(move)된다.
    let s2 = s1;
    // 따라서 아래는 에러로 러스트에서 무효한 참조를 막는다.
    // println!("{}", s1);
    println!("{}", s2);
} // 오직 s2만 유효하며 스코프 밖으로 넘어가면 s2만이 메모리가 해제된다.
  // 러스트는 결코 자동으로 데이터의 깊은 복사를 만들지 않는다. 따라서 모든 "자동"으로 이루어지는 복사는 실행 과정에서 효율적이라 가정할 수 있다.

/**
 * 변수와 데이터가 상호작용하는 방법: 복사(clone)
 * 힙 데이터의 깊은 복사를 수행할 수 있음.
 */
fn clone_value() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 명시적으로 비용이 많이 드는 코드가 실행중임을 알 수 있다.
    println!("s1 = {}, s2 = {}", s1, s2);
}
