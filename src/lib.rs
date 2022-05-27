pub trait AsciiCaseExt {
    fn contains_ignore_ascii_case(&self, sub: &Self) -> bool;
}

impl AsciiCaseExt for str {
    fn contains_ignore_ascii_case(&self, sub: &Self) -> bool {
        byte_slice_contains_pred(self.as_bytes(), sub.as_bytes(), |l, r| {
            l.to_ascii_lowercase() == r.to_ascii_lowercase()
        })
    }
}

fn byte_slice_contains_pred<P: FnMut(u8, u8) -> bool>(
    main: &[u8],
    sub: &[u8],
    mut predicate: P,
) -> bool {
    for main_cursor in 0..main.len() {
        if main[main_cursor..].len() < sub.len() {
            break;
        }
        if main[main_cursor..]
            .iter()
            .zip(sub.iter())
            .all(|(&l, &r)| predicate(l, r))
        {
            return true;
        }
    }
    false
}

#[test]
fn test_contains() {
    // Basic tests
    assert!("Hello World!".contains_ignore_ascii_case("woRlD"));
    assert!(!"Hello World!".contains_ignore_ascii_case("Dummy"));
    // The whole of `sub` needs to be matched, not just the beginning (o)
    assert!(!"Hello".contains_ignore_ascii_case("ohcool"));
    // (o should still match though)
    assert!("Hello".contains_ignore_ascii_case("o"));
}
