pub fn pad_pkcs7(msg: &mut Vec<u8>, blocksize: u8) {
    let needed_bytes = blocksize - (msg.len() % blocksize as usize) as u8;
    for _ in 0..needed_bytes {
        msg.push(needed_bytes);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pkcs7_padding() {
        let msg = "YELLOW SUBMARINE".as_bytes();
        let mut msg_vec = Vec::from(msg);

        pad_pkcs7(&mut msg_vec, 20);

        assert_eq!(
            msg_vec.as_ref(),
            [89, 69, 76, 76, 79, 87, 32, 83, 85, 66, 77, 65, 82, 73, 78, 69, 4, 4, 4, 4]
        );
    }
}
