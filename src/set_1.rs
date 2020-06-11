#[cfg(test)]
mod tests {
    use crate::{from_hex, into_hex, xor, xor_single_byte};
    use base64::encode;
    use std::str;

    #[test]
    fn challenge_01() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        let bytes = from_hex(hex);
        let encoded = encode(&bytes);

        assert_eq!(encoded, expected);
    }

    #[test]
    fn challenge_02() {
        let a = "1c0111001f010100061a024b53535009181c";
        let b = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";

        let (a_bytes, b_bytes) = (from_hex(a), from_hex(b));
        let xor = xor(a_bytes, b_bytes);

        assert_eq!(into_hex(xor), expected);
    }

    #[test]
    fn challenge_03() {
        let hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        let bytes = from_hex(hex);

        // The key byte is `88` or `X`
        for (index, i) in (0..255).enumerate() {
            let xor = xor_single_byte(bytes.clone(), i);

            if let Ok(val) = str::from_utf8(&xor) {
                println!("{} - {}", index, val);
            } else {
                println!("{} - Utf8 Error", index);
            }
        }

        assert!(true);
    }
}
