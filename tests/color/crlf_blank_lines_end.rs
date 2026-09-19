use annotate_snippets::{Level, Patch, Renderer, Snippet, renderer::DecorStyle};

use snapbox::{assert_data_eq, file, str};

#[test]
fn case() {
    // Two blank CRLF lines at the end of the file, built from explicit
    // bytes so editors/git cannot rewrite the EOLs.
    let source = "foo();\r\n\r\n\r\n";
    assert_eq!(&source.as_bytes()[6..12], b"\r\n\r\n\r\n");

    let input = &[Level::HELP.primary_title("try this").element(
        Snippet::source(source)
            .path("<current file>")
            .patch(Patch::new(10..10, "bar();")),
    )];

    let expected_plain = str![[r#"
help: try this
 --> <current file>:3:1
  |
3 | bar();
  |
"#]];
    let renderer = Renderer::plain();
    assert_data_eq!(renderer.render(input), expected_plain);

    let expected_ascii = file!["crlf_blank_lines_end.ascii.term.svg": TermSvg];
    let renderer = Renderer::styled();
    assert_data_eq!(renderer.render(input), expected_ascii);

    let expected_unicode = file!["crlf_blank_lines_end.unicode.term.svg": TermSvg];
    let renderer = renderer.decor_style(DecorStyle::Unicode);
    assert_data_eq!(renderer.render(input), expected_unicode);
}
