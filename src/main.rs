use std::env;

fn main() {
    /*
       Outline:
       - api - to_Name | to_icon | struct Options (long or short)
       - It should - given --short - display correct bin icon
       - It should - given --long - display "Don't forget, tomorrow is bin day!!"
       - It should appear in the vim statusbar
       - It should find a list of which bin, on which dates

    */

    let args: Vec<String> = env::args().collect();
    let bin_name = &args[1];
    println!(
        "Don't forget, tomorrow is bin day -> {}!!",
        find_bin_icon(bin_name)
    );
}

fn find_bin_icon(bin_name: &str) -> &str {
    match bin_name {
        "blue" => "♻️ ",
        "brown" => "💩",
        "black" => "🗑",
        _ => "Not a bin",
    }
}
