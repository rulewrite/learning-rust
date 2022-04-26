// 사용자 정의 데이터 타입을 정의하는 구조체 말고 또다른 방법 열거형
enum IpAddrKind {
    V4, // 열거형의 variants
    V6,
}

enum IpAddr {
    // variants에 값을 갖도록 해 IP 주소를 함께 저장할 수 있다.
    V4(u8, u8, u8, u8), // 각 variants 별로 데이터 타입을 다르게 지정할 수 있다.
    V6(String),
}

// 각 variants에 다양한 타입이 설정
enum Message {
    Quit,                       // 연관 데이터가 없음
    Move { x: i32, y: i32 },    // 익명 구조체
    Write(String),              // String
    ChangeColor(i32, i32, i32), // 세개의 i32
}

// 열거형도 메소드를 정의할 수 있다.
impl Message {
    fn call(&self) {
        // 메소드 내용 정의
    }
}

// 위 Message 열거형은 다음과 같이 분리될 수 있다. 하지만 모든 variants가 `Message` 타입으로 그룹화되며
// 아래 처럼 각기 다른 타입을 갖는 여러 구조체를 사용한다면 메시지 중 어떤 한가지를 인자로 받는 함수를 정의하기 어렵다.
struct QuitMessage; // 유닛 구조체
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 튜플 구조체
struct ChangeColorMessage(i32, i32, i32); // 튜플 구조체

fn main() {
    // 열거형의 variants는 식별자 IpAddrKind에 의한 네임 스페이스가 생성된다.
    let four = IpAddrKind::V4; // 열거형의 인스턴스를 생성
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    // 열거형 variants의 이름도 인스턴스를 생성하는 함수가 된다.
    // 열거형을 정의하면 이 생성자 함수가 자동으로 정의된다.
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));
    m.call();

    // 표준 라이브러리에서 정의된 열거형 Option
    // Option 타입은 값이 있거나 없는 흔히 발생할 수 있는 은한 케이스를 나타낼 수 있어 많이 사용된다.
    // 타입 시스템 관점에서 보면 컴파일러가 발생할 수 있는 모든 경우를 처리했는지 체크할 수 있다.

    /*
    러스트에서는 null이 없다.
    대신에 값의 존재나 부재의 개념을 표현할 수 있는 열거형인 Option<T>가 다음과 같이 정의되어 있다.
    enum Option<T> {
      Some(T),
      None,
    }
    */

    // Option<T>는 기본적으로 포함되어 있어 명시적으로 스코프로 가져오지 않아도 사용할 수 있으며
    // Option 열거형의 variants도 동일해 Option::을 붙이지 않고 Some과 None을 바로 사용할 수 있다.
    // 다음은 숫자와 문자열 타입을 갖는 Option 값을 할당하는 예이다.
    let some_number = Some(5); // 타입은 Option<i32>
    let some_string = Some("a string"); // 타입은 Option<&str>
                                        // Some variants에 값을 지정했기에 타입을 추론할 수 있다.

    // None을 대입하는 경우 타입 추론이 불가해 Option 타입에 어노테이션이 필요하다.
    let absent_number: Option<i32> = None;

    // Some 값이 있다면 값이 존재하고 Some variants가 갖고 있는 값에 대해 알 수 있다.
    // None 값을 사용하면 어떤 면에서는 null과 같이 유효한 값을 갖고 있지 않다는 걸 알 수 있다.
    // 하지만 Option<T>를 사용하는 것이 null보다 나은 것은 간단히 말해
    // Option<T>와 T(T는 어떤 타입이든 될 수 있다.)는 다른 타입이며 컴파일러는 Option<T>의 값을
    // 확실한 유효 값처럼 사용하지 못하게 하기 때문이다. 예를 들어 다음 코드는 Option<i8>에 i8을 더하려 하기 때문에 컴파일 되지 않는다.
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;
    // 컴파일 에러 메시지는 Option<i8>과 i8을 어떻게 더해야 하는지 이해하지 못한다고 나오며 이는 둘이 다른 타입이기 때문이다.
    // 즉 덧셈을 수행하기 전에 Option<T>를 T로 변환해야 한다.
    // null이 아닐 거라느 가정을 놓치는 경우가 발생하지 않아 안전하다.
    // Option<T>에서 Some varian의 T 값을 가져오기 위해선 Option<T>의 다양한 메소드들을 사용할 수 있다.
}

// 열거형 타입을 받는 함수를 정의할 수 있다.
fn route(ip_kind: IpAddrKind) {}
