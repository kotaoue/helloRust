pub fn pretty_print_html(input: &str) -> String {
    const INDENT: &str = "  ";
    const VOID_TAGS: [&str; 14] = [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta",
        "param", "source", "track", "wbr",
    ];

    let mut result = String::new();
    let mut indent = 0usize;
    let mut i = 0usize;
    let bytes = input.as_bytes();

    while i < bytes.len() {
        if bytes[i] == b'<' {
            let Some(rel_end) = input[i..].find('>') else {
                break;
            };
            let end = i + rel_end;
            let token = &input[i..=end];
            let content = token.trim_start_matches('<').trim_end_matches('>').trim();

            let is_closing = content.starts_with('/');
            let tag_name = content
                .trim_start_matches('/')
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .trim_end_matches('/');

            let is_self_closing = content.ends_with('/')
                || VOID_TAGS
                    .iter()
                    .any(|tag| tag.eq_ignore_ascii_case(tag_name));

            if is_closing {
                indent = indent.saturating_sub(1);
            }

            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(&INDENT.repeat(indent));
            result.push_str(token);

            if !is_closing && !is_self_closing {
                indent += 1;
            }

            i = end + 1;
        } else {
            let next_tag = input[i..].find('<').map_or(bytes.len(), |pos| i + pos);
            let text = input[i..next_tag].trim();
            if !text.is_empty() {
                result.push('\n');
                result.push_str(&INDENT.repeat(indent));
                result.push_str(text);
            }
            i = next_tag;
        }
    }

    result
}
