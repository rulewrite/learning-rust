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

  // 특정 타입이 담을 수 있는 숫자 범위를 넘어설 경우 정수 오버플로가 발생함.
  // - 디버그 모드에서 컴파일 될 땐 패닉이 발생하며 검사된다.
  // - 릴리즈 모드로 컴파일 할 땐 패닉을 유발하는 정수 오버플로 검사가 이루어지지 않으며, 2의 보수 래핑을 사용한다.
  // 최대값이 최소값으로 넘어간다. 예시는 아래와 같다.
  // 0~255까지 표현 가능
  let g: u8 = 255;
  // println!("number: {}, {}, {}", g, g + 1, g + 2); // 디버그 모드 컴파일에서 패닉
  println!(
    "number: {}, {}, {}",
    g,
    g.wrapping_add(1),
    g.wrapping_add(2)
  ); // 255, 0, 1

  // 2.1.2. 부동소수점 타입
  // f32
  // f64: 기본 값이며 최신 CPU에서 속도가 f32와 동일하며 더 정밀한 표현이 가능하기 때문
  let x = 2.0; // f64
  let y: f32 = 3.0; // f32
  println!("Floating-point: {}, {}", x, y);

  // 수학적 연산
  // addition
  let sum = 5 + 10;

  // subtraction
  let difference = 95.5 - 4.3;

  // multiplication
  let product = 4 * 30;

  // division
  let quotient = 56.7 / 32.2;
  let floored = 2 / 3; // Results in 0

  // remainder
  let remainder = 43 % 5;
  println!(
    "Operations result: {}, {}, {}, {}, {}, {}",
    sum, difference, product, quotient, floored, remainder
  );

  // 2.1.3. boolean
  // true, false 두 값을 가지며 크기는 1byte(8bit)이다. -> CPU가 다룰 수 있는 최소 사이즈
  let t = true;
  let f = false;
  println!("Boolean: {}, {}", t, f); // Boolean: true, false

  // 2.1.4. 문자 타입
  // 문자열과 달리 작은 따옴표를 사용한다.
  // 크기는 4Byte로 유니코드 스칼라 값을 나타내기 때문에 (즉, ASCII 보다 넓음)
  // 억양 표시가 있는 문자나 한국어,중국어,일본어 등 표의 문자, 이모티콘 등이 가능함.
  let c = 'z';
  let z = 'ℤ';
  let heart_eyed_cat = '😻';
  println!("Char: {}, {}, {}", c, z, heart_eyed_cat);
}
