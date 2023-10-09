use clap::Arg;
use clap::Command;
use ansi_term::Colour;
use ansi_term::Style;

pub fn param_cmd() {
    ansi()
}

fn _create_args() {
    let _cmd = Command::new("Milk Program")
        .version("v1.1.1")
        .author("Min.Co")
        .about("Long time no see")
        .arg(Arg::new("file")
            .short('f').long("file").help("parsing a file")
        )
        .arg(Arg::new("num")
            .short('n').long("num").help("parsing a num")
        )
        .arg(Arg::new("count")
            .short('c').long("count").help("parsing a total")
        ).get_matches();

    //let x = cmd.get_one("").unwrap();
}

fn ansi() {
    // print color text
    println!("This is {} in color, {} in color and {} in color",
    Colour::Red.paint("red"), Colour::Blue.paint("blue"), Colour::Green.paint("green"));

    println!("This {} in color", Colour::Purple.bold().paint("bold"));

    println!("This {} in style",Style::new().bold().paint("bold"))
}
