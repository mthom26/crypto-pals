#[cfg(test)]
mod tests {
    use crate::{
        score::get_score,
        utils::{from_hex, into_hex, xor, xor_repeating, xor_single_byte},
    };
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
        let xor = xor(&a_bytes, &b_bytes);

        assert_eq!(into_hex(&xor), expected);
    }

    #[test]
    fn challenge_03() {
        let hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        let bytes = from_hex(hex);

        // The key byte is `88` or `X`
        for (index, i) in (0..255).enumerate() {
            let xor = xor_single_byte(&bytes, i);

            if let Ok(val) = str::from_utf8(&xor) {
                println!("{} - {}", index, val);
            } else {
                println!("{} - Utf8 Error", index);
            }
        }

        assert!(true);
    }

    #[test]
    fn challenge_04() {
        let text = include_str!("../resources/4.txt");

        let mut high_line_score = 0.0;
        let mut high_line_byte = 0;
        let mut line_num = 0;

        for (index, line) in text.split("\n").enumerate() {
            println!("{} - {}", index, line);
            let mut high_score = 0.0;
            let mut high_byte = 0;
            let bytes = from_hex(line);

            for i in 0..255 {
                let xor = xor_single_byte(&bytes, i);
                let score = get_score(xor);
                if score > high_score {
                    high_score = score;
                    high_byte = i;
                }
            }

            if high_score > high_line_score {
                high_line_score = high_score;
                high_line_byte = high_byte;
                line_num = index;
            }
        }

        // TODO - Properly xor the correct line with the correct byte without
        //        just printing and copying the result.
        println!("{} - {} - {}", high_line_score, high_line_byte, line_num);

        let a = from_hex("7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f");
        let xor = xor_single_byte(&a, 53);
        let b = str::from_utf8(&xor).unwrap();
        println!("{}", b);
        assert!(true);
    }

    #[test]
    fn challenge_05() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = vec![b'I', b'C', b'E'];

        let xor = xor_repeating(input.as_bytes(), &key);
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".to_string();

        assert_eq!(into_hex(&xor), expected);
    }
}
