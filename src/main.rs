mod cowsay;
mod endate;
mod lolcat;

use std::io::{self};

fn main() {
    let date = endate::get_date();

    let cowsay_output = cowsay::cowsay_to_string(&date);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    lolcat::lolcat_write(&cowsay_output, &mut handle);
}
