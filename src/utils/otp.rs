use rand::Rng;

pub fn generate_otp() -> String {
    // Creating an RNG (random number generator) instance.
    let mut rng = rand::thread_rng();

    // Generating a 6-character string by repeatedly picking random digits.
    let otp: String = (0..6)
        .map(|_| {
            // Generating a random digit (0-9).
            let digit = rng.gen_range(0..10);
            // Converting the digit to a character and returning it.
            std::char::from_digit(digit, 10).unwrap()
        })
        .collect();

    otp
}