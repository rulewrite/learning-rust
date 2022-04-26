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
}

// 열거형 타입을 받는 함수를 정의할 수 있다.
fn route(ip_kind: IpAddrKind) {}
