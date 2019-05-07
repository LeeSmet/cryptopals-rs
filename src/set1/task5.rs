pub fn repeating_key_xor(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len());

    for (data_byte, key_byte) in data.iter().zip(key.iter().cycle()) {
        result.push(data_byte ^ key_byte);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::set1::task1::hex_encode;

    #[test]
    fn test_repeating_key_xor() {
        let plain = r#"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal"#;
        let key = "ICE";

        let expected_hex = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        let plain_bytes = plain.as_bytes();
        let key_bytes = key.as_bytes();

        let cipher = repeating_key_xor(plain_bytes, key_bytes);

        let cipher_hex = hex_encode(&cipher);

        assert_eq!(&cipher_hex, expected_hex);
    }
}
