use std::char;

const RADIX: u32 = 36;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let should_space = |i| i % 5 == 0  && i != 0;

    return invert_characters(plain).chars()
        .enumerate()
        .flat_map(|(i, c)| match i {
            _ if should_space(i) => vec![" ".to_string(), c.to_string()],
            _ => vec![c.to_string()]
        })
        .collect::<String>();
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    return invert_characters(cipher);
}

fn invert_characters(input: &str) -> String {
    return input.to_ascii_lowercase().chars()
        .flat_map(|c| match c.to_digit(RADIX) {
            Some(digit) if c.is_numeric() => vec![digit],
            Some(digit) if c.is_alphabetic() => vec![RADIX - 1 - (digit - 10)],
            _ => vec![],
        })
        .map(|d| char::from_digit(d, RADIX).unwrap())
        .collect::<String>();
}
