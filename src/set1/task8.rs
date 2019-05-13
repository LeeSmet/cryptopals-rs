pub fn detect_aes_ecb(ciphertexts: &[&[u8]]) -> Option<usize> {
    for (idx, ciphertext) in ciphertexts.iter().enumerate() {
        if detect_dup_block(16, ciphertext) {
            return Some(idx);
        }
    }

    None
}

fn detect_dup_block(blocksize: usize, data: &[u8]) -> bool {
    assert!(data.len() % blocksize == 0);

    for block_idx in 0..data.len() / blocksize {
        let block = &data[block_idx * blocksize..(block_idx + 1) * blocksize];

        for future_block_idx in block_idx + 1..data.len() / blocksize {
            let future_block =
                &data[future_block_idx * blocksize..(future_block_idx + 1) * blocksize];

            if block == future_block {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::set1::task1::hex_decode;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_detect_aes_ecb() {
        let mut file = File::open("assets/task8.txt").expect("Failed to open file");

        let mut hex_data = String::new();

        file.read_to_string(&mut hex_data)
            .expect("Failed to hex decode file");

        // collect ciphertexts in vec, and decode
        let ciphertexts: Vec<Vec<u8>> =
            hex_data.split_whitespace().map(|s| hex_decode(s)).collect();

        // reference them
        // Can't seem to do this with collect
        // also seem to need the extra hint for the compiler
        let mut refs: Vec<&[u8]> = Vec::with_capacity(ciphertexts.len());
        for ciphertext in ciphertexts.iter() {
            refs.push(&ciphertext);
        }

        let cipher_idx = detect_aes_ecb(&refs);

        assert_eq!(cipher_idx, Some(132));
    }
}
