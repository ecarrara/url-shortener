const BASE: u64 = 62;

static ALPHABET: [char; BASE as usize] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z',
];

pub fn encode(value: u64) -> String {
    let mut digits = Vec::with_capacity(7);
    let mut value = value;

    while value > 0 {
        let rem = value % BASE;
        digits.push(ALPHABET[rem as usize]);
        value /= BASE;
    }

    digits.iter().rev().collect()
}
