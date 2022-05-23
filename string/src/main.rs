fn main() {
    // 데이터를 담을 수 있는 빈 문자열
    let mut s = String::new();

    // 초기 값이 있는 문자열
    let data = "initial contents";
    let s = data.to_string();
    // 이 메소드는 리터럴에서도 동일하게 작동한다.
    let s = "initial contents".to_string();

    // 문자열 리터럴로 String 타입 생성하기
    let s = String::from("initial contetns");

    // 문자열은 UTF-8로 인코딩 되므로 인코됭된 어떤 데이터라도 담을 수 있다.
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // 문자열 확장하기
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s); // foobar

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2); // push_str이 소유권을 갖는 매개변수가 아니기 때문에 여전히 s2는 액세스할 수 있다.

    let mut s = String::from("lo");
    s.push('l'); // 단일 문자 추가시에는 push를 사용한다.

    // + 연산으로 문자열 결합
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1은 여기로 옮겨져 더이상 사용할 수 없다.
                       // println!("s1 is {}", s1);
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);

    // 여러개의 문자열 결합 format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3); // tic-tac-toe

    let s1 = String::from("hello");
    // let h = s1[0]; // 문자열 인덱싱은 지원하지 않는다.

    // 문자열 슬라이싱 하기
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // Зд

    // 문자열 순회하기
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
