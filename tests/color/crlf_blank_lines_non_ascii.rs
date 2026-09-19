use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};

use snapbox::{assert_data_eq, file, str};

#[test]
fn case() {
    // Non-ASCII third line after two blank CRLF lines, built from explicit
    // bytes so editors/git cannot rewrite the EOLs.
    let source = "\r\n\r\nこんにちは、世界";
    assert_eq!(&source.as_bytes()[..4], b"\r\n\r\n");

    let input = &[Level::ERROR.primary_title("oops").element(
        Snippet::source(source)
            .path("<current file>")
            .annotation(AnnotationKind::Primary.span(22..28).label("world")),
    )];

    let expected_plain = str![[r#"
error: oops
 --> <current file>:3:7
  |
3 | こんにちは、世界
  |             ^^^^ world
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected_plain);

    let expected_ascii = file!["crlf_blank_lines_non_ascii.ascii.term.svg": TermSvg];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = file!["crlf_blank_lines_non_ascii.unicode.term.svg": TermSvg];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);
}
