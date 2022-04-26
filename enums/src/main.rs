// 사용자 정의 데이터 타입을 정의하는 구조체 말고 또다른 방법 열거형
enum IpAddrKind {
    V4, // 열거형의 variants
    V6,
}

fn main() {
    // 열거형의 variants는 식별자 IpAddrKind에 의한 네임 스페이스가 생성된다.
    let four = IpAddrKind::V4; // 열거형의 인스턴스를 생성
    let six = IpAddrKind::V6;

    route(four);
    route(six);
}

// 열거형 타입을 받는 함수를 정의할 수 있다.
fn route(ip_kind: IpAddrKind) {}
