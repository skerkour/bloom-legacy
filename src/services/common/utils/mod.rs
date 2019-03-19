use rand::Rng;

pub fn random_digit_string(n: usize) -> String {
    let mut rng = rand::thread_rng();

    return (0..n).map(|_| rng.gen_range(0, 10))
        .map(|x| x.to_string())
        .collect();
}

pub fn random_bytes(n: usize) -> Vec<u8> {
    return (0..n).map(|_| rand::random::<u8>()).collect();
}

pub fn random_hex_string(n: usize) -> String {
    let bytes = random_bytes(n);
    return hex::encode(bytes);
}


// func RandomHex(n uint) (string, error) {
// 	b, err := RandomBytes(n)
// 	if err != nil {
// 		log.Error("generating random hex", rz.Err(err), rz.Uint("n", n))
// 	}
// 	return hex.EncodeToString(b), err
// }
