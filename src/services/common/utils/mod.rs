use rand::Rng;

pub fn random_digit_str(n: usize) -> String {
    let mut rng = rand::thread_rng();

    return (0..n).map(|_| rng.gen_range(0, 10))
        .map(|x| x.to_string())
        .collect();
}
