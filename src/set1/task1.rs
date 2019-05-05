const HEX_CHARSET: &'static str = "0123456789abcdef";

pub fn base64_decode(data: &str) -> Vec<u8> {
    assert!(data.len() % 4 == 0);

    base64::decode(data).unwrap()
}

pub fn base64_encode(data: &[u8]) -> String {
    base64::encode(data)
}

pub fn hex_decode(data: &str) -> Vec<u8> {
    assert!(data.len() % 2 == 0);

    let mut decoded = Vec::with_capacity(data.len() / 2);

    let mut cur_byte = 0;

    for (i, ch) in data.chars().enumerate() {
        if let Some((val, _)) = HEX_CHARSET.match_indices(ch).take(1).next() {
            if i % 2 == 0 {
                cur_byte = (val << 4) as u8;
            } else {
                cur_byte |= val as u8;
                decoded.push(cur_byte);
            }
        }
    }

    decoded
}

pub fn hex_encode(data: &[u8]) -> String {
    let mut hex = String::with_capacity(data.len() * 2);

    for byte in data {
        hex.push(HEX_CHARSET.chars().nth((byte >> 4) as usize).unwrap());
        hex.push(HEX_CHARSET.chars().nth((byte & 0xF) as usize).unwrap());
    }

    hex
}

#[cfg(test)]
mod tests {
    // import stufff from above
    use super::*;

    #[test]
    fn base64_to_hex() {
        let test_b64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let expected = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

        let bytes = base64_decode(test_b64);

        let hex = hex_encode(&bytes);

        assert_eq!(expected, hex);
    }

    #[test]
    fn hex_to_base64() {
        let test_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        let bytes = hex_decode(test_hex);

        let b64 = base64_encode(&bytes);

        assert_eq!(expected, b64);
    }
}
