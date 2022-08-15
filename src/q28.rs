/*
    28 - Implement strStr
    Time and Space: Algorithm dependent
    There are 3 algorithm choices: KMP, Rabin Karp, Boyer Moore
*/
pub fn str_str(haystack: String, needle: String) -> i32 {
    rabin_karp(&haystack, &needle)
        .map(|x| x as i32)
        .unwrap_or(-1)
}

/*
    m = word.len(), n = pat.len()
    Time: O(m - n)
    Space: O(1)
*/
fn rabin_karp(word: &str, pat: &str) -> Option<usize> {
    const ALPHABET_SIZE: usize = 26;
    let word = word.as_bytes();
    let pat = pat.as_bytes();
    let m = word.len();
    let n = pat.len();

    if m < n {
        return None;
    }

    let pat_hash = pat.iter().enumerate().fold(0, |prev, (i, ch)| {
        prev + (ch - b'a') as usize * ALPHABET_SIZE.pow(i as u32)
    });
    let mut word_hash = word[..n].iter().enumerate().fold(0, |prev, (i, ch)| {
        prev + (ch - b'a') as usize * ALPHABET_SIZE.pow(i as u32)
    });

    for i in 0..=m - n {
        if word_hash == pat_hash {
            return Some(i);
        }
        if i == m - n {
            break;
        }
        word_hash -= (word[i] - b'a') as usize;
        word_hash /= ALPHABET_SIZE;
        word_hash += (word[i + n] - b'a') as usize * ALPHABET_SIZE.pow(n as u32 - 1);
    }
    None
}

#[test]
fn t1() {
    let tcases = [("hello", "ll", 2), ("abc", "c", 2), ("c", "c", 0)];
    for (haystack, needle, index) in tcases {
        assert_eq!(str_str(haystack.to_owned(), needle.to_owned()), index);
    }
}
