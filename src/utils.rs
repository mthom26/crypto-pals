pub fn from_hex(hex_string: &str) -> Vec<u8> {
    let mut bytes = Vec::new();

    for index in 0..(hex_string.len() / 2) {
        let i = 2 * index;
        let byte = u8::from_str_radix(&hex_string[i..i + 2], 16).unwrap();
        bytes.push(byte);
    }

    bytes
}

pub fn into_hex(bytes: &[u8]) -> String {
    let hexes: Vec<String> = bytes.iter().map(|byte| format!("{:02x}", byte)).collect();
    hexes.join("")
}

pub fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();

    for (a_byte, b_byte) in a.iter().zip(b.iter()) {
        output.push(a_byte ^ b_byte);
    }

    output
}

pub fn xor_single_byte(bytes: &[u8], key: u8) -> Vec<u8> {
    let mut output = Vec::new();

    for byte in bytes.iter() {
        output.push(byte ^ key);
    }

    output
}

pub fn xor_repeating(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let length = key.len();

    for (i, byte) in bytes.iter().enumerate() {
        let key_byte = i % length;
        output.push(byte ^ key[key_byte]);
    }

    output
}

pub fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    let diff = a.iter().zip(b.iter()).fold(0, |acc, (elem_a, elem_b)| {
        acc + (elem_a ^ elem_b).count_ones()
    });

    diff
}

#[cfg(test)]
mod tests {
    use crate::utils::hamming_distance;

    #[test]
    fn hamming() {
        let a = "this is a test".as_bytes();
        let b = "wokka wokka!!!".as_bytes();

        assert_eq!(hamming_distance(a, b), 37);
    }
}
