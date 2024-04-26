use pretty_assertions::assert_eq;

#[test]
#[cfg(unix)]
fn demo() {
    use roff::*;

    let page = Roff::new()
        .control("TH", ["CORRUPT", "1"])
        .control("SH", ["NAME"])
        .text(vec![roman(
            "corrupt - modify files by randomly changing bits",
        )])
        .control("SH", ["SYNOPSIS"])
        .text(vec![
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
        ])
        .control("SH", ["DESCRIPTION"])
        .text(vec![
            bold("corrupt"),
            " modifies files by toggling a randomly chosen bit.".into(),
        ])
        .control("SH", ["OPTIONS"])
        .control("TP", [])
        .text(vec![
            bold("-n"),
            ", ".into(),
            bold("--bits"),
            "=".into(),
            italic("BITS"),
        ])
        .text(vec![
            "Set the number of bits to modify. ".into(),
            "Default is one bit.".into(),
        ])
        .to_roff();

    assert_eq!(
        roff_to_ascii(include_str!("./demo.troff")),
        roff_to_ascii(&page)
    );
}

fn roff_to_ascii(input: &str) -> String {
    duct::cmd("troff", &["-a", "-mman"])
        .stdin_bytes(input)
        .stdout_capture()
        .read()
        .unwrap()
}
