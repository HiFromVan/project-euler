fn main() {
    let m = 999;

    // 这里的匿名函数就是你公式里的每一项
    let f = |k: u64| {
        let n = m / k;
        k * n * (n + 1) / 2
    };

    let result = f(3) + f(5) - f(15);
    println!("The answer is {}", result);
}