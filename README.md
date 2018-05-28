# roff-rs

[![Build Status](https://travis-ci.org/killercup/roff-rs.svg)][Travis]
[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]
![License](https://img.shields.io/crates/l/roff.svg)
[![crates.io](https://img.shields.io/crates/v/roff.svg)][Crates.io]

[Travis]: https://travis-ci.org/killercup/roff-rs
[Crates.io]: https://crates.io/crates/roff
[Documentation]: https://docs.rs/roff/

[Roff](http://man7.org/linux/man-pages/man7/roff.7.html) generation library.

## Examples

```rust
extern crate roff;

use roff::*;

let page = Roff::new("corrupt", 1)
    .section("name", &["corrupt - modify files by randomly changing bits"])
    .section("SYNOPSIS", &[
        bold("corrupt"), " ".into(),
        "[".into(), bold("-n"), " ".into(), italic("BITS"), "]".into(),
        " ".into(),
        "[".into(), bold("--bits"), " ".into(), italic("BITS"), "]".into(),
        " ".into(),
        italic("file"), "...".into(),
    ])
    .section("description", &[
        bold("corrupt"),
        " modifies files by toggling a randomly chosen bit.".into(),
    ])
    .section("options", &[
        list(
            &[bold("-n"), ", ".into(), bold("--bits"), "=".into(), italic("BITS")],
            &[
                "Set the number of bits to modify. ",
                "Default is one bit.",
            ]
        ),
    ]);
```

Which outputs:
```troff
.TH CORRUPT 1
.SH NAME
corrupt \- modify files by randomly changing bits
.SH SYNOPSIS
.B corrupt
[\fB\-n\fR \fIBITS\fR]
[\fB\-\-bits\fR \fIBITS\fR]
.IR file ...
.SH DESCRIPTION
.B corrupt
modifies files by toggling a randomly chosen bit.
.SH OPTIONS
.TP
.BR \-n ", " \-\-bits =\fIBITS\fR
Set the number of bits to modify.
Default is one bit.
```

Which will be shown by the `man(1)` command as:

```txt
CORRUPT(1)  General Commands Manual  CORRUPT(1)

NAME
       corrupt   -  modify  files  by  randomly
       changing bits

SYNOPSIS
       corrupt [-n BITS] [--bits BITS] file...

DESCRIPTION
       corrupt modifies  files  by  toggling  a
       randomly chosen bit.

OPTIONS
       -n, --bits=BITS
              Set the number of bits to modify.
              Default is one bit.

                                     CORRUPT(1)
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
