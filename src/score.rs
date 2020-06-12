pub fn get_score(bytes: Vec<u8>) -> f32 {
    let mut score = 0.0;

    for byte in bytes {
        score += get_char_score(byte);
    }

    score
}

pub fn get_char_score(input: u8) -> f32 {
    match input {
        b'a' | b'A' => 8.497,
        b'b' | b'B' => 1.492,
        b'c' | b'C' => 2.202,
        b'd' | b'D' => 4.253,
        b'e' | b'E' => 11.162,
        b'f' | b'F' => 2.228,
        b'g' | b'G' => 2.015,
        b'h' | b'H' => 6.094,
        b'i' | b'I' => 7.546,
        b'j' | b'J' => 0.153,
        b'k' | b'K' => 1.292,
        b'l' | b'L' => 4.025,
        b'm' | b'M' => 2.406,
        b'n' | b'N' => 6.749,
        b'o' | b'O' => 7.507,
        b'p' | b'P' => 1.929,
        b'q' | b'Q' => 0.095,
        b'r' | b'R' => 7.587,
        b's' | b'S' => 6.327,
        b't' | b'T' => 9.356,
        b'u' | b'U' => 2.758,
        b'v' | b'V' => 0.978,
        b'w' | b'W' => 2.560,
        b'x' | b'X' => 0.150,
        b'y' | b'Y' => 1.994,
        b'z' | b'Z' => 0.077,
        b',' | b'.' | b'"' | b' ' | b':' | b';' | b'\'' | b'\n' => 0.0,
        _ => -10.0,
    }
}
