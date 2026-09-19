//! Regression tests for consecutive empty CRLF lines shifting later
//! annotations by one byte.
//!
//! All sources are built from explicit `\r\n` escapes so that neither the
//! editor nor Git line-ending conversion can rewrite them.

use annotate_snippets::{AnnotationKind, Level, Patch, Renderer, Snippet};

#[test]
fn single_line_label_after_leading_crlf_empty_lines() {
    // Two empty CRLF lines, then content: `foo` starts at byte 4, line 3.
    let source = "\r\n\r\nfoo bar\r\n";
    let input = &[Level::ERROR.primary_title("mismatched types").element(
        Snippet::source(source)
            .path("src/main.rs")
            .line_start(1)
            .annotation(
                AnnotationKind::Primary
                    .span(4..7)
                    .label("expected `bar`, found `foo`"),
            ),
    )];

    let expected_plain = "error: mismatched types
 --> src/main.rs:3:1
  |
3 | foo bar
  | ^^^ expected `bar`, found `foo`";
    assert_eq!(Renderer::plain().render(input), expected_plain);

    let expected_styled = "\u{1b}[1m\u{1b}[91merror\u{1b}[0m\u{1b}[1m: mismatched types\u{1b}[0m
 \u{1b}[1m\u{1b}[94m--> \u{1b}[0msrc/main.rs:3:1
  \u{1b}[1m\u{1b}[94m|\u{1b}[0m
\u{1b}[1m\u{1b}[94m3\u{1b}[0m \u{1b}[1m\u{1b}[94m|\u{1b}[0m foo bar
  \u{1b}[1m\u{1b}[94m|\u{1b}[0m \u{1b}[1m\u{1b}[91m^^^\u{1b}[0m \u{1b}[1m\u{1b}[91mexpected `bar`, found `foo`\u{1b}[0m";
    assert_eq!(Renderer::styled().render(input), expected_styled);
}

#[test]
fn multiline_annotation_after_leading_crlf_empty_lines() {
    // `foo` is bytes 4..7, `bar` is bytes 9..12; the span covers lines 3-4.
    let source = "\r\n\r\nfoo\r\nbar\r\n";
    let input = &[Level::ERROR.primary_title("mismatched types").element(
        Snippet::source(source)
            .path("src/main.rs")
            .line_start(1)
            .annotation(AnnotationKind::Primary.span(4..12).label("wrong block")),
    )];

    let expected_plain = "error: mismatched types
 --> src/main.rs:3:1
  |
3 | / foo
4 | | bar
  | |___^ wrong block";
    assert_eq!(Renderer::plain().render(input), expected_plain);

    let expected_styled = "\u{1b}[1m\u{1b}[91merror\u{1b}[0m\u{1b}[1m: mismatched types\u{1b}[0m
 \u{1b}[1m\u{1b}[94m--> \u{1b}[0msrc/main.rs:3:1
  \u{1b}[1m\u{1b}[94m|\u{1b}[0m
\u{1b}[1m\u{1b}[94m3\u{1b}[0m \u{1b}[1m\u{1b}[94m|\u{1b}[0m \u{1b}[1m\u{1b}[91m/\u{1b}[0m foo
\u{1b}[1m\u{1b}[94m4\u{1b}[0m \u{1b}[1m\u{1b}[94m|\u{1b}[0m \u{1b}[1m\u{1b}[91m|\u{1b}[0m bar
  \u{1b}[1m\u{1b}[94m|\u{1b}[0m \u{1b}[1m\u{1b}[91m|___^\u{1b}[0m \u{1b}[1m\u{1b}[91mwrong block\u{1b}[0m";
    assert_eq!(Renderer::styled().render(input), expected_styled);
}

#[test]
fn suggestion_with_crlf_empty_lines_in_middle() {
    // `two` is bytes 9..12 on line 4, after two consecutive empty CRLF lines.
    let source = "one\r\n\r\n\r\ntwo\r\n";
    let input = &[Level::HELP.secondary_title("rename the variable").element(
        Snippet::source(source)
            .path("src/main.rs")
            .line_start(1)
            .patch(Patch::new(9..12, "three")),
    )];

    let expected_plain = "help: rename the variable
 --> src/main.rs:4:1
  |
4 - two
4 + three
  |";
    assert_eq!(Renderer::plain().render(input), expected_plain);

    let expected_styled = "\u{1b}[1m\u{1b}[96mhelp\u{1b}[0m: rename the variable
 \u{1b}[1m\u{1b}[94m--> \u{1b}[0msrc/main.rs:4:1
  \u{1b}[1m\u{1b}[94m|\u{1b}[0m
\u{1b}[1m\u{1b}[94m4\u{1b}[0m \u{1b}[91m- two\u{1b}[0m
\u{1b}[1m\u{1b}[94m4\u{1b}[0m \u{1b}[92m+ three\u{1b}[0m
  \u{1b}[1m\u{1b}[94m|\u{1b}[0m";
    assert_eq!(Renderer::styled().render(input), expected_styled);
}

#[test]
fn label_on_last_line_with_trailing_crlf_empty_lines() {
    // Two consecutive empty CRLF lines at the end of the source; byte 12 is
    // the first column of the last (empty) line.
    let source = "one\r\ntwo\r\n\r\n\r\n";
    let input = &[Level::ERROR.primary_title("unexpected empty line").element(
        Snippet::source(source)
            .path("src/main.rs")
            .line_start(1)
            .annotation(AnnotationKind::Primary.span(12..12).label("here")),
    )];

    let expected_plain = "error: unexpected empty line
 --> src/main.rs:4:1
  |
4 |
  | ^ here";
    assert_eq!(Renderer::plain().render(input), expected_plain);

    let expected_styled =
        "\u{1b}[1m\u{1b}[91merror\u{1b}[0m\u{1b}[1m: unexpected empty line\u{1b}[0m
 \u{1b}[1m\u{1b}[94m--> \u{1b}[0msrc/main.rs:4:1
  \u{1b}[1m\u{1b}[94m|\u{1b}[0m
\u{1b}[1m\u{1b}[94m4\u{1b}[0m \u{1b}[1m\u{1b}[94m|\u{1b}[0m
  \u{1b}[1m\u{1b}[94m|\u{1b}[0m \u{1b}[1m\u{1b}[91m^\u{1b}[0m \u{1b}[1m\u{1b}[91mhere\u{1b}[0m";
    assert_eq!(Renderer::styled().render(input), expected_styled);
}

#[test]
fn non_ascii_line_after_leading_crlf_empty_lines() {
    // `αβγ` starts at byte 4; `α` and `β` are bytes 4..8. A one-byte shift
    // here would split a UTF-8 char boundary.
    let source = "\r\n\r\nαβγ\r\n";
    let input = &[Level::ERROR.primary_title("unknown identifier").element(
        Snippet::source(source)
            .path("src/main.rs")
            .line_start(1)
            .annotation(
                AnnotationKind::Primary
                    .span(4..8)
                    .label("not found in this scope"),
            ),
    )];

    let expected_plain = "error: unknown identifier
 --> src/main.rs:3:1
  |
3 | αβγ
  | ^^ not found in this scope";
    assert_eq!(Renderer::plain().render(input), expected_plain);

    let expected_styled = "\u{1b}[1m\u{1b}[91merror\u{1b}[0m\u{1b}[1m: unknown identifier\u{1b}[0m
 \u{1b}[1m\u{1b}[94m--> \u{1b}[0msrc/main.rs:3:1
  \u{1b}[1m\u{1b}[94m|\u{1b}[0m
\u{1b}[1m\u{1b}[94m3\u{1b}[0m \u{1b}[1m\u{1b}[94m|\u{1b}[0m αβγ
  \u{1b}[1m\u{1b}[94m|\u{1b}[0m \u{1b}[1m\u{1b}[91m^^\u{1b}[0m \u{1b}[1m\u{1b}[91mnot found in this scope\u{1b}[0m";
    assert_eq!(Renderer::styled().render(input), expected_styled);
}

#[test]
fn crlf_snippet_does_not_pollute_following_lf_snippet() {
    // The first snippet uses CRLF with leading empty lines; the second uses
    // plain LF. The second snippet's byte ranges must be computed from its
    // own source only.
    let crlf_source = "\r\n\r\nfoo\r\n";
    let lf_source = "\n\nbar\n";
    let input = &[Level::ERROR
        .primary_title("two files")
        .element(
            Snippet::source(crlf_source)
                .path("a.rs")
                .line_start(1)
                .annotation(AnnotationKind::Primary.span(4..7).label("first")),
        )
        .element(
            Snippet::source(lf_source)
                .path("b.rs")
                .line_start(1)
                .annotation(AnnotationKind::Primary.span(2..5).label("second")),
        )];

    let expected_plain = "error: two files
 --> a.rs:3:1
  |
3 | foo
  | ^^^ first
  |
 ::: b.rs:3:1
  |
3 | bar
  | ^^^ second";
    assert_eq!(Renderer::plain().render(input), expected_plain);

    let expected_styled = "\u{1b}[1m\u{1b}[91merror\u{1b}[0m\u{1b}[1m: two files\u{1b}[0m
 \u{1b}[1m\u{1b}[94m--> \u{1b}[0ma.rs:3:1
  \u{1b}[1m\u{1b}[94m|\u{1b}[0m
\u{1b}[1m\u{1b}[94m3\u{1b}[0m \u{1b}[1m\u{1b}[94m|\u{1b}[0m foo
  \u{1b}[1m\u{1b}[94m|\u{1b}[0m \u{1b}[1m\u{1b}[91m^^^\u{1b}[0m \u{1b}[1m\u{1b}[91mfirst\u{1b}[0m
  \u{1b}[1m\u{1b}[94m|\u{1b}[0m
 \u{1b}[1m\u{1b}[94m::: \u{1b}[0mb.rs:3:1
  \u{1b}[1m\u{1b}[94m|\u{1b}[0m
\u{1b}[1m\u{1b}[94m3\u{1b}[0m \u{1b}[1m\u{1b}[94m|\u{1b}[0m bar
  \u{1b}[1m\u{1b}[94m|\u{1b}[0m \u{1b}[1m\u{1b}[91m^^^\u{1b}[0m \u{1b}[1m\u{1b}[91msecond\u{1b}[0m";
    assert_eq!(Renderer::styled().render(input), expected_styled);
}
