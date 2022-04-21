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
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        // dbg! 매크로는 호출이 발생한 파일 및 행 번호를 출력한다.
        // 표현식 값의 소유권을 반환한다.
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {:#?}", rect1);

    // 소유권이 필요치 않으므로 rect1의 참조를 전달한다.
    dbg!(&rect1);

    println!("The rectangle has a nonzero width: it is {}", rect1.width());
}
