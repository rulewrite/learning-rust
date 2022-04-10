fn main() {
  // 1. 변수
  // 변수는 기본적으로 불변. `mut`을 선언하여 가변성 변수로 설정 가능
  let mut x = 5;
  println!("The value of x is : {}", x);
  x = 6;
  println!("The value of x is: {}", x);

  // 상수는 표현식으로만 선언될 수 있다. (런타임 중에 계산되는 함수 결과 등에 사용 불가)
  // 타입 선언을 필히 해야 함. 추론이 안되는 듯?
  // 컴파일 중에 결과를 평가할 수 있으므로 10,800처럼 난해하지 않고, 이해하기 쉽도록 선언.
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  println!("The value of x is: {}", THREE_HOURS_IN_SECONDS);

  // Shadowing
  // 같은 이름의 새 변수를 선언할 수 있다. 이전 변수를 가리게(Shadowing) 됨.
  let y = 5;
  let y = y + 1;
  {
    let y = y * 2;
    println!("내부 스코프에서 y 변수의 값은: {}", y); // 12
  }
  println!("y 변수의 값은: {}", y); // 6

  // Shadowing은 형변환하여 재선언이 가능하지만 `mut`는 불가하다.
  let spaces = "   "; // 문자열

  // spaces_len과 같은 대체 이름이 아닌 그대로 재선언이 가능하다.
  let spaces = spaces.len(); // 숫자형
  println!("공백 갯수는: {}", spaces);

  // 즉, 다음은 불가함.
  // let mut spaces2 = "   ";
  // spaces2 = spaces2.len();

  // 2. 데이터 타입
  // 러스트에 사용되는 모든 값은 특정 데이터 타입을 가지며 크게 스칼라, 컴파운드 두가지로 나눠진다.
  // 러스트는 정적 언어로, 컴파일 시에 반드시 모든 변수의 타입이 정해져 있어야 하며, 타입 추론을 지원한다.

  // 아래와 같이 여러 데이터 유형으로 변경이 될 수 있을 경우 명확히 타입을 명시해주어야 한다.
  let guess: u32 = "42".parse().expect("Not a number!");
  println!("{}", guess);

  // 2.1. 스칼라타입
  // 정수, 부동소수점 숫자, boolean, 문자 4가지

  // 2.1.1. 정수형
  // 소수점이 없는 수자로 2장 추리게임에서 `u32` 타입을 사용하며, 부호 없는 32비트 변수임을 나타냈다.
  // 부호가 있을 경우(음수가 가능할 경우) `u` 대신 `i`를 사용하며 2의 보수를 사용하여 저장한다.
  // i8, u8
  // i16, u16
  // i32, u32: 기본 값은 i32이며 일반적으로 64-bit 시스템에서도 가장 빠르다.
  // i64, u64
  // isize, usize: 프로그램 동작 환경에 따라 변화, 64-bit 아키텍쳐면 64bit, 주로 컬렉션 타입의 인덱싱으로 사용
  // 기타 숫자 리터럴은 다음이 있다.
  let a = 57u8; // u8 타입으로 지정
  let b = 98_222; // 숫자를 더 읽기 쉽게. 98,222임
  let c = 0xff; // 16진수 (Hexadecimal)
  let d = 0o77; // 8진수 (Octal)
  let e = 0b1111_0000; // 2진수 (Binary)
  let f = b'A'; // u8, Byte (u8만)
  println!("Number literals: {}, {}, {}, {}, {}, {}", a, b, c, d, e, f);
}
