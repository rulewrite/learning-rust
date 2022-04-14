// 튜플 처럼 구조체의 구성요소들은 각자 다른 타입을 가질 수 있음
// 각 구성요소들을 명명할 수 있어 값의 의미를 명확하게 할 수 있고 순서에 의존없이 접근할 수 있어 보다 유연함.
struct User {
    username: String, // field
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // 구조체는 정의된 양식이며 User 인스턴스 생성(실체화)
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("rulewrite"),
        active: true,
        sign_in_count: 1,
    };
}
