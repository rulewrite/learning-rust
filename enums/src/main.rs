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
}

// 열거형 타입을 받는 함수를 정의할 수 있다.
fn route(ip_kind: IpAddrKind) {}
