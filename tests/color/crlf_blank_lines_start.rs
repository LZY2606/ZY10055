use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};

use snapbox::{assert_data_eq, file, str};

#[test]
fn case() {
    // Two blank CRLF lines at the start of the file, built from explicit
    // bytes so editors/git cannot rewrite the EOLs.
    let source = "\r\n\r\nhello world";
    assert_eq!(&source.as_bytes()[..5], b"\r\n\r\nh");

    let input = &[Level::ERROR.primary_title("oops").element(
        Snippet::source(source)
            .path("<current file>")
            .annotation(AnnotationKind::Primary.span(10..15).label("label")),
    )];

    let expected_plain = str![[r#"
error: oops
 --> <current file>:3:7
  |
3 | hello world
  |       ^^^^^ label
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected_plain);

    let expected_ascii = file!["crlf_blank_lines_start.ascii.term.svg": TermSvg];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = file!["crlf_blank_lines_start.unicode.term.svg": TermSvg];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);
}
