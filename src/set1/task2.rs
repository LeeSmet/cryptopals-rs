pub fn xor(input1: &[u8], input2: &[u8]) -> Vec<u8> {
    assert_eq!(input1.len(), input2.len());

    let mut result = Vec::with_capacity(input1.len());

    for (byte1, byte2) in input1.iter().zip(input2) {
        result.push(byte1 ^ byte2);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::set1::task1::hex_decode;

    #[test]
    fn test_xor() {
        let inp1_hex = "1c0111001f010100061a024b53535009181c";
        let inp2_hex = "686974207468652062756c6c277320657965";
        let expected_hex = "746865206b696420646f6e277420706c6179";

        let inp1 = hex_decode(inp1_hex);
        let inp2 = hex_decode(inp2_hex);
        let expected = hex_decode(expected_hex);

        let result = xor(&inp1, &inp2);

        assert_eq!(expected, result);
    }
}
