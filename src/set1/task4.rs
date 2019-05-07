use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn load_strings<P>(path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path).unwrap(); // if there is an error, too bad
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let mut result = Vec::new();

    for line in contents.lines() {
        result.push(line.to_string());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::set1::{task1, task3};

    #[test]
    fn find_encrypted_string() {
        let lines = load_strings("assets/task4.txt");

        let mut candidate_score = 0;
        let mut candidate_key = 0;
        let mut candidate_plaintext = Vec::new();
        let mut line_index = 0;

        for (idx, line_bytes) in lines.iter().map(|line| task1::hex_decode(line)).enumerate() {
            let (key, score, plain) = task3::decode_fixed_xor(&line_bytes);

            if score > candidate_score {
                candidate_score = score;
                candidate_key = key;
                candidate_plaintext = plain;
                line_index = idx;
            }
        }

        eprintln!(
            "{}: {} -> {}: {}",
            line_index,
            candidate_key,
            candidate_score,
            String::from_utf8_lossy(&candidate_plaintext)
        );

        assert_eq!(line_index, 170);
        assert_eq!(candidate_key, 53);
    }
}
