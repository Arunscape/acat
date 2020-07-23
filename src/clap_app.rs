use clap::{crate_authors, crate_name, crate_version, App as ClapApp, Arg};
use std::env;

pub fn clap_app() -> ClapApp<'static, 'static> {
    ClapApp::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("acat - concatenate files and print on the standard output, except with chaotic colours")
        .arg(
            Arg::with_name("FILE")
                .help("File(s) to print / concatenate. Use '-' for standard input.")
                .multiple(true)
                .empty_values(false),

        )
        .arg(
            Arg::with_name("show-all")
                .long("show-all")
                .short("A")
                .help("equivalent to -vET")
                .display_order(0)
        )
        .arg(
            Arg::with_name("number-nonblank")
                .long("number-nonblank")
                .overrides_with("number")
                .short("b")
                .help("number nonempty output lines, overrides -n")
                .display_order(1)
        )
        .arg(
            Arg::with_name("e")
                .short("e")
                .help("equivalent to -vE")
                .display_order(2)
        )
        .arg(
            Arg::with_name("show-ends")
                .long("show-ends")
                .short("E")
                .help("display $ at end of each line")
                .display_order(3)
        )
        .arg(
            Arg::with_name("number")
                .long("number")
                .short("n")
                .help("number all output lines")
                .display_order(4)
        )
        .arg(
            Arg::with_name("squeeze-blank")
                .long("squeeze-blank")
                .short("s")
                .help("suppress repeated empty output lines")
                .display_order(5)
        )
        .arg(
            Arg::with_name("t")
                .short("t")
                .help("equivalent to -vT")
                .display_order(6)
        )
        .arg(
            Arg::with_name("show-tabs")
                .long("show-tabs")
                .short("T")
                .help("display TAB characters as ^I")
                .display_order(7)
        )
        .arg(
            Arg::with_name("u")
                .short("u")
                .help("(ignored)")
                .display_order(8)
        )
        .arg(
            Arg::with_name("show-nonprinting")
                .long("show-nonprinting")
                .short("v")
                .help("use ^ and M- notation, except for LFD and TAB")
                .display_order(9)
        )
        .arg(
            Arg::with_name("hue")
                .long("hue")
                .takes_value(true)
                .possible_values(&["mono", "red", "orange", "yellow", "green", "blue", "purple", "pink"])
        )
        .arg(
            Arg::with_name("luminosity")
                .long("luminosity")
                .takes_value(true)
                .possible_values(&["bright", "light", "dark", "random"])
        )
}
