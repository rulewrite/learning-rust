#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // & 불변으로 빌림
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 필드와 동일한 이름의 메소드도 사용할 수 있다.
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /**
     * self 파라미터를 갖지 않아도 선언할 수 있다.
     * 연관 함수(associated functions)
     * - 구조체와 연관되어 있기 때문에 "연관"
     * - 인스턴스가 아니기 때문에 여전히 "함수"
     * 생성자로써 많이 사용된다. ex. String::from()
     * 해당 함수는 구조체의 네임스페이스에 있으며 모듈에 의해 생성된 네임스페이스에서도 :: 연산자가 사용된다.
     */
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        // dbg! 매크로는 호출이 발생한 파일 및 행 번호를 출력한다.
        // 표현식 값의 소유권을 반환한다.
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 100,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {:#?}", rect1);

    // 소유권이 필요치 않으므로 rect1의 참조를 전달한다.
    dbg!(&rect1);

    println!("The rectangle has a nonzero width: it is {}", rect1.width());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sqare1 = Rectangle::square(3);
    println!("sqare1 is {:#?}", sqare1);
}
