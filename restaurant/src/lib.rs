// mod 모듈명
mod front_of_house {
    // 모듈 내부에는 다른 모듈이 존재할 수 있다.
    // 아울러 구조체, 열거형, 상수, 트레잇, 함수 등을 보유 할 수 있다.
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
