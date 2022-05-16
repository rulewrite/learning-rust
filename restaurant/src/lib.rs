// mod 모듈명
// {}을 사용하지 않고 세미콜론을 사용하면 러스트는 모듈과 동일한 이름을 가진 파일에서 모듈의 내용을 로드한다.
pub mod front_of_house;

fn serve_order() {}

mod back_of_house {
    // 모듈 내부에는 다른 모듈이 존재할 수 있다.
    // 아울러 구조체, 열거형, 상수, 트레잇, 함수 등을 보유 할 수 있다.
    fn fix_incorrect_order() {
        cook_order();
        // super::front_of_house::serving::serve_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // Breakfast에는 비공개 필드 seasonal_fruit가 존재하기 때문에 인스턴스 생성 함수를 제공해야 한다. 외부에선 seasonal_fruit의 값을 설정할 수 없기 떼문
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// use crate::front_of_house::hosting;
// 상대 경로 및 다시 내보내기
pub use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // 호밀 토스트로 여름의 아침 메뉴를 주문
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // toast는 public 필드로 값 액세스 가능
    meal.toast = String::from("Wheat");
    // seasonal_fruit 필드는 비공개로 액세스 불가
    // meal.seasonal_fruit = String::from("blueberries");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
