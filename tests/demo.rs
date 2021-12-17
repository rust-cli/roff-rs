use std::{
    io::Write,
    process::{Command, Stdio},
};

use roff::*;

fn roff_to_ascii(input: &str) -> String {
    let mut cmd = Command::new("troff")
        .args(["-a", "-mman"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(ref mut stdin) = cmd.stdin {
        stdin.write_all(input.as_bytes()).unwrap();
    }

    String::from_utf8(cmd.wait_with_output().unwrap().stdout).unwrap()
}

#[test]
fn demo() {
    let page = Roff::new("corrupt", ManSection::Executable)
        .date("2021-12-25")
        .manual("General Commands Manual")
        .source("corrupt v1")
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
    // f.write_all(&page.render().as_bytes()).unwrap();

    assert_eq!(
        roff_to_ascii(include_str!("./demo.troff")),
        roff_to_ascii(&page.render())
    );
}
