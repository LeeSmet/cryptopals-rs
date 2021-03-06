pub fn string_score(data: &[u8]) -> usize {
    let mut score = 0;

    for byte in data {
        match byte {
            48...57 => score += 1,      // digits
            65...90 => score += 3,      // Capitals
            97...122 => score += 10,    // a-z
            32 => score += 5,           // space is reasonably common, more so than capitals
            33 | 44 | 46 => score += 1, // award a point for  exclamation mark | period | dot
            _ => {}
        }
    }

    score
}

pub fn decode_fixed_xor(data: &[u8]) -> (u8, usize, Vec<u8>) {
    let mut candidate_score = 0;
    let mut candidate_key = 0;
    let mut candidate_plaintext = Vec::new();

    for byte in 0..255u8 {
        let key = vec![byte; data.len()];
        let plain = super::task2::xor(data, &key);

        let score = string_score(&plain);

        if score > candidate_score {
            candidate_score = score;
            candidate_key = byte;
            candidate_plaintext = plain;
        }
    }

    (candidate_key, candidate_score, candidate_plaintext)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::set1::*;

    #[test]
    fn test_string_score() {
        let test_strings = ["abc", "12-02", "To be or not to be"];
        let expected_scores = [30, 4, 148];

        for (test_str, expected_score) in test_strings.iter().zip(expected_scores.iter()) {
            assert_eq!(&string_score(test_str.as_bytes()), expected_score);
        }
    }

    #[test]
    fn test_decode_fixed_xor() {
        let test_hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let test_data = task1::hex_decode(test_hex);

        let (supposed_key, _, _) = decode_fixed_xor(&test_data);

        assert_eq!(supposed_key, 88);
    }
}
