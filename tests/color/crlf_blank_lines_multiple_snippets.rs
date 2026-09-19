use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};

use snapbox::{assert_data_eq, file, str};

#[test]
fn case() {
    // A CRLF-heavy first snippet must not pollute the source ranges of the
    // LF-only snippet that follows it. Built from explicit bytes so
    // editors/git cannot rewrite the EOLs.
    let first = "a\r\n\r\n\r\nb";
    assert_eq!(&first.as_bytes()[1..7], b"\r\n\r\n\r\n");
    let second = "alpha\nbeta\ngamma";

    let input = &[
        Level::ERROR.primary_title("oops").element(
            Snippet::source(first)
                .path("first.rs")
                .annotation(AnnotationKind::Primary.span(7..8).label("first")),
        ),
        Level::ERROR.primary_title("oops again").element(
            Snippet::source(second)
                .path("second.rs")
                .annotation(AnnotationKind::Primary.span(11..16).label("second")),
        ),
    ];

    let expected_plain = str![[r#"
error: oops
 --> first.rs:4:1
  |
4 | b
  | ^ first
  |
error: oops again
 --> second.rs:3:1
  |
3 | gamma
  | ^^^^^ second
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected_plain);

    let expected_ascii = file!["crlf_blank_lines_multiple_snippets.ascii.term.svg": TermSvg];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = file!["crlf_blank_lines_multiple_snippets.unicode.term.svg": TermSvg];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);
}
