use pretty_assertions::assert_eq;

#[test]
#[cfg(unix)]
fn demo() {
    use roff::*;

    if !has_command("troff") {
        return;
    }

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

pub(crate) fn has_command(command: &str) -> bool {
    let output = match std::process::Command::new(command)
        .arg("--version")
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            // CI is expected to support all of the commands
            if is_ci() && cfg!(target_os = "linux") {
                panic!(
                    "expected command `{}` to be somewhere in PATH: {}",
                    command, e
                );
            }
            return false;
        }
    };
    if !output.status.success() {
        panic!(
            "expected command `{}` to be runnable, got error {}:\n\
            stderr:{}\n\
            stdout:{}\n",
            command,
            output.status,
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!(
        "$ {command} --version
{}",
        stdout
    );
    if cfg!(target_os = "macos") && stdout.starts_with("GNU bash, version 3") {
        return false;
    }

    true
}

/// Whether or not this running in a Continuous Integration environment.
fn is_ci() -> bool {
    // Consider using `tracked_env` instead of option_env! when it is stabilized.
    // `tracked_env` will handle changes, but not require rebuilding the macro
    // itself like option_env does.
    option_env!("CI").is_some() || option_env!("TF_BUILD").is_some()
}
