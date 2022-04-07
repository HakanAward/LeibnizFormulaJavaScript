fn main() {
    let mut pi: f64 = 0.0;

    for i in 0..100_000_000 {
        let numerator: f64 = if i % 2 == 0 { 4.0 } else { -4.0 };
        let denominator: f64 = (2.0 * i as f64) + 1.0;

        pi += numerator / denominator;
    }

    println!("Pi: {}", pi);
}
