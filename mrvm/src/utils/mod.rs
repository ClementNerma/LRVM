
/// Convert a list of bytes to a list of words
pub fn bytes_to_words(bytes: impl AsRef<[u8]>) -> Vec<u32> {
    let bytes = bytes.as_ref();

    let rem = bytes.len() % 4;
    let mut words = Vec::with_capacity(bytes.len() / 4 + if rem == 0 { 1 } else { 0 });
    let mut word = 0;

    for (i, byte) in bytes.iter().enumerate() {
        word += u32::from(*byte) << ((3 - (i % 4)) * 8);

        if i % 4 == 3 {
            words.push(word);
            word = 0;
        }
    }

    if rem != 0 {
        words.push(word << ((4 - rem) * 8));
    }

    words
}

/// Convert a list of words to a list of bytes
pub fn words_to_bytes(bytes: impl AsRef<[u32]>) -> Vec<u8> {
    bytes.as_ref().iter().map(|word| word.to_be_bytes().to_vec()).flatten().collect()
}
