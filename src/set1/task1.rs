const HEX_CHARSET: &'static str = "0123456789abcdef";
const BASE64_CHARSET: &'static str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn base64_decode(data: &str) -> Vec<u8> {
    assert!(data.len() % 4 == 0);

    let mut decoded = Vec::with_capacity(data.len() / 4 * 3);

    let mut cur_byte = 0;
    for (i, ch) in data.chars().enumerate() {
        if let Some((val, _)) = BASE64_CHARSET.match_indices(ch).take(1).next() {
            match i % 4 {
                0 => cur_byte = (val as u8) << 2,
                1 => {
                    cur_byte |= (val as u8) >> 4;
                    decoded.push(cur_byte);
                    cur_byte = (val as u8) << 4
                }
                2 => {
                    cur_byte |= (val as u8) >> 2;
                    decoded.push(cur_byte);
                    cur_byte = (val as u8) << 6;
                }
                3 => {
                    cur_byte |= val as u8;
                    decoded.push(cur_byte);
                }
                _ => unreachable!(), // make compiler happy
            }
        }
    }

    decoded
}

pub fn base64_encode(data: &[u8]) -> String {
    let mut b64 = String::with_capacity(data.len() * 4 / 3); //TODO: not perfect

    let mut overflow = 0;

    for (i, byte) in data.iter().enumerate() {
        match i % 3 {
            0 => {
                b64.push(BASE64_CHARSET.chars().nth((byte >> 2) as usize).unwrap());
                overflow = (byte & 0x03) << 4;
            }
            1 => {
                b64.push(
                    BASE64_CHARSET
                        .chars()
                        .nth((overflow | (byte >> 4)) as usize)
                        .unwrap(),
                );
                overflow = (byte & 0x0F) << 2;
            }
            2 => {
                b64.push(
                    BASE64_CHARSET
                        .chars()
                        .nth((overflow | (byte >> 6)) as usize)
                        .unwrap(),
                );
                b64.push(BASE64_CHARSET.chars().nth((byte & 0x3F) as usize).unwrap());
            }
            _ => unreachable!(),
        }
    }

    b64
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
