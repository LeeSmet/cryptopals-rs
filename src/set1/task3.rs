pub fn string_score(data: &[u8]) -> usize {
    let mut score = 0;

    for byte in data {
        match byte {
            48...57 => score += 1,  // digits
            65...90 => score += 2,  // Capitals
            97...122 => score += 3, // a-z
            _ => {}
        }
    }

    score
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::set1::*;

    #[test]
    fn test_string_score() {
        let test_strings = ["abc", "12-02", "To be or not to be"];
        let expected_scores = [9, 4, 38];

        for (test_str, expected_score) in test_strings.iter().zip(expected_scores.iter()) {
            assert_eq!(&string_score(test_str.as_bytes()), expected_score);
        }
    }

    #[test]
    fn decode_fixed_xor() {
        let test_hex = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let test_data = task1::hex_decode(test_hex);

        let mut candidate = Vec::new();
        let mut candidate_score = 0;
        let mut candidate_key = 0;

        for byte in 0..255u8 {
            let key = vec![byte; test_data.len()];
            let plain = task2::xor(&test_data, &key);

            let score = string_score(&plain);

            eprintln!("{} -> {}: {}", byte, score, String::from_utf8_lossy(&plain));

            if score > candidate_score {
                candidate_score = score;
                candidate_key = byte;
                candidate = plain;
            }
        }

        assert_eq!(candidate_key, 88);
        assert_eq!(candidate_score, 78);
        assert_eq!(
            &String::from_utf8(candidate).unwrap(),
            &"Cooking MC's like a pound of bacon"
        );
    }
}
