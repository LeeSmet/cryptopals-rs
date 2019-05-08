use crate::set1::task3::decode_fixed_xor;
use crate::set1::task5::repeating_key_xor;
//use itertools::Itertools;

pub fn hamming_distance(first: &[u8], second: &[u8]) -> usize {
    assert_eq!(first.len(), second.len());

    let mut score = 0;

    for (byte_first, byte_second) in first.iter().zip(second) {
        // get different bits
        let mut diff = byte_first ^ byte_second;

        // get score for this byte
        for _ in 0..8 {
            // increment score if lowest bit is different
            score += (diff & 1) as usize;
            // erase lowest bit
            diff = diff >> 1
        }
    }

    score
}

pub fn break_repeating_key_xor(
    data: &[u8],
    keysize_min: usize,
    keysize_max: usize,
) -> (Vec<u8>, Vec<u8>) {
    assert!(keysize_max >= keysize_min);

    let mut key_distance = std::collections::HashMap::with_capacity(keysize_max - keysize_min + 1);

    // figure out keysize
    for i in keysize_min..=keysize_max {
        let block1 = &data[0..i];
        let block2 = &data[i..i * 2];
        let block3 = &data[i * 2..i * 3];
        let block4 = &data[i * 3..i * 4];

        // remember normalized distance
        let d1 = hamming_distance(block1, block2);
        let d2 = hamming_distance(block2, block3);
        let d3 = hamming_distance(block3, block4);
        let d4 = hamming_distance(block1, block3);
        let d5 = hamming_distance(block1, block4);
        let d6 = hamming_distance(block2, block4);

        let d = (d1 + d2 + d3 + d4 + d5 + d6) as f64 / 6.;
        key_distance.insert(i, d / i as f64);
    }

    let mut lowest_distance = 0.;
    let mut keysize = 0;

    for (key, distance) in key_distance {
        if lowest_distance == 0. {
            lowest_distance = distance;
            keysize = key;
        }

        if distance < lowest_distance {
            lowest_distance = distance;
            keysize = key;
        }
    }

    // now that we have the keysize, cut up the ciphertext in blocks of that size,
    // and create new blocks with every ith element of the blocks, which are thus encrypted with
    // the same keybyte
    let mut transmuted_blocks: Vec<Vec<u8>> = Vec::with_capacity(keysize);
    for _ in 0..keysize {
        transmuted_blocks.push(Vec::with_capacity(data.len() / keysize));
    }

    for (i, byte) in data.iter().enumerate() {
        transmuted_blocks[i % keysize].push(*byte);
    }

    let mut key = Vec::with_capacity(keysize);
    for chunk in transmuted_blocks {
        let (key_byte, _, _) = decode_fixed_xor(&chunk);
        key.push(key_byte);
    }

    let plain = repeating_key_xor(data, &key);

    (key, plain)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::set1::task1::base64_decode;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_hamming_distance() {
        let first_string = "this is a test";
        let second_string = "wokka wokka!!!";

        let distance = hamming_distance(first_string.as_bytes(), second_string.as_bytes());

        assert_eq!(distance, 37);
    }

    #[test]
    fn test_break_xor() {
        let mut file = File::open("assets/task6.txt").expect("Failed to open file");

        let mut base64_data = String::new();

        file.read_to_string(&mut base64_data)
            .expect("Failed to base64 decode file");

        // Strip whitespace
        let assembled_base64_data: String = base64_data
            .split_whitespace()
            .map(|s| String::from(s))
            .collect();

        let data = base64_decode(&assembled_base64_data);

        let (key, _) = break_repeating_key_xor(&data, 2, 40);

        assert_eq!(key, "Terminator X: Bring the noise".as_bytes());
    }
}
