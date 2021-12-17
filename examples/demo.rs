extern crate roff;

// View this example by running `cargo run --example demo | man -l -`.

fn main() {
    use roff::*;

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

    println!("{}", page.render());
}
