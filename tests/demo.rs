extern crate duct;
extern crate roff;
#[macro_use]
extern crate pretty_assertions;

fn roff_to_ascii(input: &str) -> String {
    duct::cmd("troff", &["-a", "-mman"])
        .stdin_bytes(input)
        .stdout_capture()
        .read()
        .unwrap()
}

#[test]
fn demo() {
    use roff::*;

    let page = Roff::new("corrupt", 1)
        .section(
            "name",
            &["corrupt - modify files by randomly changing bits"],
        )
        .section(
            "SYNOPSIS",
            &[
                bold("corrupt"),
                " ".into(),
                "[".into(),
                bold("-n"),
                " ".into(),
                italic("BITS"),
                "]".into(),
                " ".into(),
                "[".into(),
                bold("--bits"),
                " ".into(),
                italic("BITS"),
                "]".into(),
                " ".into(),
                italic("file"),
                "...".into(),
            ],
        )
        .section(
            "description",
            &[
                bold("corrupt"),
                " modifies files by toggling a randomly chosen bit.".into(),
            ],
        )
        .section(
            "options",
            &[list(
                &[
                    bold("-n"),
                    ", ".into(),
                    bold("--bits"),
                    "=".into(),
                    italic("BITS"),
                ],
                &["Set the number of bits to modify. ", "Default is one bit."],
            )],
        );

    // use std::io::Write;
    // let mut f = ::std::fs::File::create("./tests/demo.generated.troff").unwrap();
    // f.write_all(&page.render().as_bytes());

    assert_eq!(
        roff_to_ascii(include_str!("./demo.troff")),
        roff_to_ascii(&page.render())
    );
}
