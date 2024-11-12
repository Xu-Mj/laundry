pub(crate) mod chrono_serde;

use pinyin::ToPinyin;

pub fn gen_code(input: impl ToString) -> String {
    let input = input.to_string().trim().to_uppercase();
    if input.is_empty() {
        return "".to_string();
    }

    let mut result = String::new();

    for c in input.chars() {
        if is_chinese(c) {
            if let Some(py) = c.to_pinyin() {
                result.push_str(&py.plain().chars().next().unwrap_or(' ').to_string().to_uppercase());
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// gen by chatgpt
#[inline]
fn is_chinese(c: char) -> bool {
    (c >= '\u{4E00}' && c <= '\u{9FFF}')
        || (c >= '\u{3400}' && c <= '\u{4DBF}') // CJK Extension A
        || (c >= '\u{20000}' && c <= '\u{2A6DF}') // CJK Extension B
        || (c >= '\u{2A700}' && c <= '\u{2B73F}') // CJK Extension C
        || (c >= '\u{2B740}' && c <= '\u{2B81F}') // CJK Extension D
        || (c >= '\u{2B820}' && c <= '\u{2CEAF}') // CJK Extension E
        || (c >= '\u{2CEB0}' && c <= '\u{2EBEF}') // CJK Extension F
        || (c >= '\u{F900}' && c <= '\u{FAFF}') // CJK Compatibility Ideographs
        || (c >= '\u{2F800}' && c <= '\u{2FA1F}') // CJK Compatibility Ideographs Supplement
}

#[cfg(test)]
mod tests {
    use crate::utils::{gen_code, is_chinese};

    #[test]
    fn test_get_initial() {
        let input = "hello你好";
        let initials = gen_code(input);
        assert_eq!(initials, "HELLONH");
    }

    #[test]
    fn test_is_chinese() {
        let input = 'h';
        let initials = is_chinese(input);
        assert!(initials);
    }
}
