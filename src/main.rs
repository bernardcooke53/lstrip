use clap::Parser;

const APPLICATION_NAME: &'static str = "uncomment";
const DEFAULT_LEADING_CHARS: &'static str = "#";

#[derive(Parser, Debug)]
#[command(author = "Bernard Cooke")]
#[command(version)]
#[command(about = r#"
uncomment

Strip leading characters from multiline text in the terminal."#)]
#[command(
    override_usage = format!(
    r#"
    {:} [TEXT] [OPTIONS]
    {:} [LEADING_CHARS] [TEXT] [OPTIONS]

In 1-argument form, print [TEXT] to stdout, trimming any leading hash (#) characters.
In 2-argument form, [TEXT] will be printed to stdout, but with the characters specified
in [LEADING_CHARS] trimmed from the output. Whitespace before the leading # or
[LEADING_CHARS] is always trimmed on lines which begin with those characters."#,
    APPLICATION_NAME,
    APPLICATION_NAME
    )
)]
struct CLI {
    #[arg(value_name = "TEXT OR LEADING CHARS", required = true)]
    text_or_leading_chars: String,

    #[arg(value_name = "TEXT")]
    maybe_text: Option<String>,

    #[arg(
        short = 'w',
        long = "whitespace",
        help = "Strip all leading whitespace after [LEADING_CHARS] (default false)"
    )]
    preserve_whitespace: bool,
}

fn main() {
    let args = CLI::parse();

    let (leading_chars, text) = match args.maybe_text {
        Some(text) => (args.text_or_leading_chars, text),
        None => (
            String::from(DEFAULT_LEADING_CHARS),
            args.text_or_leading_chars,
        ),
    };

    let lstrip_whitespace = args.preserve_whitespace;
    let output = lstrip_lines(&leading_chars, &text, lstrip_whitespace);
    println!("{}", output);
}

fn lstrip_lines(leading_chars: &String, text: &String, lstrip_whitespace: bool) -> String {
    text.lines()
        .map(|l| {
            l.trim_start_matches(|c: char| l.contains(leading_chars) && c.is_whitespace())
                .trim_start_matches(leading_chars)
                .trim_start_matches(|c: char| lstrip_whitespace && c.is_whitespace())
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_lstrip_lines_no_lstrip_ws() {
        let leading_chars = "#";
        let text = r#"
        # foo
        # bar
        # echo hello world"#;

        let expected = r#"
 foo
 bar
 echo hello world"#;

        let output = lstrip_lines(&leading_chars.to_string(), &text.to_string(), false);

        assert_eq!(output, expected);
    }

    #[test]
    fn test_lstrip_lines_lstrip_ws() {
        let leading_chars = "#";
        let text = r#"
        # foo
        # bar
        # echo hello world"#;

        let expected = r#"
foo
bar
echo hello world"#;

        let output = lstrip_lines(&leading_chars.to_string(), &text.to_string(), true);

        assert_eq!(output, expected);
    }

    #[test]
    fn test_lstrip_lines_mixed_lines_no_strip_ws() {
        let leading_chars = "#";
        let text = r#"
        # foo
        bar
        # echo hello world"#;

        let expected = r#"
 foo
        bar
 echo hello world"#;

        let output = lstrip_lines(&leading_chars.to_string(), &text.to_string(), false);

        assert_eq!(output, expected);
    }

    #[test]
    fn test_lstrip_lines_mixed_lines_strip_ws() {
        let leading_chars = "#";
        let text = r#"
        # foo
        bar
        # echo hello world"#;

        let expected = r#"
foo
bar
echo hello world"#;

        let output = lstrip_lines(&leading_chars.to_string(), &text.to_string(), true);

        assert_eq!(output, expected);
    }

    #[test]
    fn test_lstrip_lines_other_prefix_chars() {
        let leading_chars = "//";
        let text = r#"
        // foo
        bar
        # echo hello world"#;

        let expected = r#"
 foo
        bar
        # echo hello world"#;

        let output = lstrip_lines(&leading_chars.to_string(), &text.to_string(), false);

        assert_eq!(output, expected);
    }

    #[test]
    fn test_lstrip_lines_other_prefix_chars_strip_ws() {
        let leading_chars = "//";
        let text = r#"
        // foo
        bar
        # echo hello world"#;

        let expected = r#"
foo
bar
# echo hello world"#;

        let output = lstrip_lines(&leading_chars.to_string(), &text.to_string(), true);

        assert_eq!(output, expected);
    }
}
