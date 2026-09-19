use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};

use snapbox::{assert_data_eq, file, str};

#[test]
fn case() {
    // Two blank CRLF lines in the middle of the file, built from explicit
    // bytes so editors/git cannot rewrite the EOLs.
    let source = "fn main() {\r\n\r\n\r\n    foo();\r\n}";
    assert_eq!(&source.as_bytes()[11..17], b"\r\n\r\n\r\n");

    let input = &[Level::ERROR.primary_title("oops").element(
        Snippet::source(source)
            .path("<current file>")
            .annotation(AnnotationKind::Primary.span(10..26).label("the body")),
    )];

    let expected_plain = str![[r#"
error: oops
 --> <current file>:1:11
  |
1 |   fn main() {
  |  ___________^
... |
4 | |     foo();
  | |_________^ the body
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected_plain);

    let expected_ascii = file!["crlf_blank_lines_middle.ascii.term.svg": TermSvg];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = file!["crlf_blank_lines_middle.unicode.term.svg": TermSvg];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);
}
