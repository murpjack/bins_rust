//use serde_json;
//use std::fs;
use std::time::SystemTime;

fn main() {
    match get_data() {
        Ok(bin_day) => {
            // TODO: Remove this '!'
            if !is_bin_day_near(bin_day.date) {
                let bins_str = bin_day
                    .bins
                    .iter()
                    .map(|s| String::from(what_bin_day(s)))
                    .reduce(|cur: String, nxt| cur + &nxt)
                    .unwrap();
                println!("Bin day! {}", bins_str);
            }
        }
        Err(_) => panic!("Not today"),
    }
}

struct BinDay {
    date: u128,
    bins: [&'static str; 2],
}

fn get_data() -> Result<BinDay, &'static str> {
    // TODO: Find and get data from json
    //let input_path = "./bins.json";
    //let _bin_data = {
    //    let bin_data = std::fs::read_to_string(&input_path)?;
    //    println!("{}", bin_data);
    //    serde_json::from_str::<String>(&bin_data).unwrap();
    //};

    return Result::Ok({
        BinDay {
            date: 1678147200000,
            bins: ["recycling", "food and garden waste"],
        }
    });
}

fn is_bin_day_near(bin_day: u128) -> bool {
    let one_day = 72000000;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            // TODO: today_start should == today at 12am
            // TODO: tomorrow_end should == tomorrow at 11.59pm
            let now = n.as_millis();
            let today_start = now - (one_day / 2);
            let tomorrow_end = now + one_day;
            return bin_day >= today_start && bin_day < tomorrow_end;
        }
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

enum BinType {
    Biodegradable,
    Landfill,
    Recyclable,
    NoBin,
}

fn what_bin_day(s: &str) -> &str {
    match s {
        "recycling" => to_icon(BinType::Recyclable),
        "food and garden waste" => to_icon(BinType::Biodegradable),
        "rubbish" => to_icon(BinType::Landfill),
        _ => to_icon(BinType::NoBin),
    }
}

fn to_icon(bin_type: BinType) -> &'static str {
    match bin_type {
        BinType::Recyclable => "♻️ ",
        BinType::Biodegradable => "💩",
        BinType::Landfill => "🗑",
        BinType::NoBin => "",
    }
}
