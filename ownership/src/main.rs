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

    copy_value_in_stack();

    function_and_ownership();

    return_and_scope();
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

/**
 * 스택에만 있는 데이터: 복사(copy)
 */
fn copy_value_in_stack() {
    // 정수형과 같이 컴파일 타임에 결정된 크기의 타입은 스택에 모두 저장되어 값이 빠르게 복사될 수 있다.
    let x = 5;
    // 즉 y가 생성된 후에 x를 해제할 이유가 없다.
    // 바꿔말하면 깊은 복사와 얕은 복사간에 차이가 없다.
    let y = x;
    println!("{}, {}", x, y);

    // 러스트에서는 정수형과 같이 스택에 저장되는 타입에 대해 Copy 트레잇이라는 특별한 어노테이션을 갖는다.
    // 만일 어떤 타입이 Copy 트레잇을 갖는다면 대입 후에도 이전 변수를 계속 사용할 수 있다.
    // 만약 Drop 트레잇이 구현되어 있다면 Copy 트레잇은 어노테이션 할 수 없다.
    // Copy되는 타입은 일반적인 규칙으로 스칼라 값들은 가능하며 할당이나 특정 리소스의 경우 불가함.
    // - u32와 같은 정수형
    // - boolean
    // - f64 같은 부동소수점
    // - char
    // - Copy가 가능한 타입만으로 구성된 튜플 ex. (i32, i32)는 가능
}

/**
 * 함수에 값을 넘기는 것 또한 변수 대입과 유사하므로 move되거나 copy될 것임.
 */
fn function_and_ownership() {
    let s = String::from("hello"); // s가 스코프 내에 선언됨.
    takes_ownership(s); // s의 값이 takes_ownership 함수 안으로 이동함. 더이상 유효하지 않음.

    let x = 5; // x가 스코프 안에 선언됨.
    makes_copy(x); // x를 함수로 보냈지만 i32는 copy되므로 x를 이후에 계속 사용 가능

    println!("stay: {}", x);
    // println!("it's gone: {}", s);
} // x는 스코프 밖으로 나가고 s도 나감, 하지만 s는 이미 이동되었으므로 아무런 일이 발생하지 않는다.

fn takes_ownership(some_string: String) {
    // some_string이 스코프내로 들어옴
    println!("{}", some_string);
} // some_string이 스코프 밖으로 벗어났고 some_string에 대한 drop이 호출되어 메모리가 해제됨.

fn makes_copy(some_integer: i32) {
    // some_integer가 스코프내로 들어옴
    println!("{}", some_integer);
} // some_integer가 스코프 밖으로 벗어났지만 별다른 일은 발생하지 않는다.

/**
 * 값의 반환도 마찬가지로 소유권을 이동시킨다.
 */
fn return_and_scope() {
    let s1 = gives_ownership(); // gives_ownership은 반환 값을 s1으로 이동시킨다.

    let s2 = String::from("hello"); // s2가 스코프 안에 들어옴.

    let s3 = takes_and_gives_back(s2); // s2는 takes_and_gives_back 안으로 이동되며
                                       // 반환값은 s3로 이동됨

    println!("{},{}", s1, s3);
} // s1, s3는 스코프 밖을 벗어나 drop이 호출되지만 s2는 이동됐으므로 아무런 일이 일어나지 않는다.

fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string이 스코프 안에 들어옴
    some_string // some_string은 호출한 함수로 이동한다.
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string이 스코프 안으로 들어옴
    a_string // a_string은 반환되고, 호출한 쪽의 함수로 이동됩니다.
}
