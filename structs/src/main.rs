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
    // 러스트는 특정 필드만 변경할 수 있도록 허용하지 않기 때문에 인스턴스를 가변하기 위해 `mut`추가
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("rulewrite"),
    );
    user1.email = String::from("someone-email@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        // username: user1.username,
        // active: user1.active,
        // sign_in_count: user1.sign_in_count,
        // 아래와 같이 축약 가능. js에서는 상위에 구조해제 후 덮어쓰는 개념이지만 러스트는 아래에 둠.
        ..user1
    };

    // println!("{}", user1.username); // user1.username은 move 되어 사용 불가
    println!("{}, {}", user2.username, user2.email);
}

fn build_user(email: String, username: String) -> User {
    // 표현식으로 반환
    User {
        email, // 매개변수와 구조체 필드 이름이 동일하기 때문에 약식으로 사용 가능 (field init shorthand)
        username,
        active: true,
        sign_in_count: 1,
    }
}
