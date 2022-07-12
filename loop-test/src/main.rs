fn main() {
    let numbers = vec![1, 2, 3];

    let results: Vec<_> = numbers
        .iter()
        .map(|n| {
            let result = n * 100;
            println!("{} * 100 = {}", n, result);
            result
        })
        .map(|n| {
            let result = n + 10;
            println!("{} + 1 = {}", n, result);
            result
        })
        .map(|n| {
            let result = 0;
            println!("{}", 0);
            result
        })
        .collect();
}
