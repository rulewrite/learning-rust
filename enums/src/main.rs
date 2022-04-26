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
}

// 열거형 타입을 받는 함수를 정의할 수 있다.
fn route(ip_kind: IpAddrKind) {}
