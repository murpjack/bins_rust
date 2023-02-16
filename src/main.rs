use serde_derive::{Deserialize, Serialize};
use std::time::SystemTime;

fn main() {
    show_message()
}

fn show_message() -> () {
    // TODO: Remove unwrap & handle errors
    let bin_str = std::fs::read_to_string("src/bins.json").unwrap();
    let bin_day = serde_json::from_str::<serde_json::Value>(&bin_str)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .find_map(|raw_day| match raw_day["date"].as_str() {
            Some(date_str) => {
                let date = date_str.parse::<u128>().unwrap();
                // TODO: Remove this '!' otherwise print first date in json file
                if !is_bin_day_near(date) {
                    return maybe_bin_day(raw_day);
                } else {
                    return None;
                }
            }
            _ => None,
        });

    match bin_day {
        Some(today) => {
            let icons = BinDay::bins_to_icons(&today);
            println!("Bin day! {}", icons)
        }
        _ => panic!("Not today"),
    }
}

fn maybe_bin_day(ab: &serde_json::Value) -> Option<BinDay> {
    match (ab["date"].as_str(), ab["bins"].as_array()) {
        (Some(date_str), Some(raw_bins)) => Some(BinDay {
            date: date_str.parse::<u128>().unwrap(),
            bins: raw_bins
                .iter()
                .map(|raw_bin| BinType::from_str(raw_bin.as_str().unwrap()))
                .collect::<Vec<BinType>>(),
        }),
        _ => None,
    }
}

fn is_bin_day_near(next_date: u128) -> bool {
    let one_day = 72000000;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => {
            // TODO: today_start should == today at 12am
            // TODO: tomorrow_end should == tomorrow at 11.59pm
            let now = n.as_millis();
            let today_start = now - (one_day / 2);
            let tomorrow_end = now + one_day;
            next_date >= today_start && next_date < tomorrow_end
        }
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BinDay {
    date: u128,
    bins: Vec<BinType>,
}

impl BinDay {
    fn bins_to_icons(&self) -> String {
        self.bins
            .iter()
            .map(|bin_type| String::from(BinType::to_icon(bin_type)))
            .reduce(|cur: String, nxt| cur + &nxt)
            .unwrap()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
enum BinType {
    Biodegradable,
    Landfill,
    Recyclable,
    NoBin,
}

impl BinType {
    fn to_icon(&self) -> &str {
        match self {
            BinType::Recyclable => "♻️ ",
            BinType::Biodegradable => "💩",
            BinType::Landfill => "🗑",
            BinType::NoBin => "",
        }
    }

    fn from_str(bin_str: &str) -> BinType {
        match bin_str {
            "recycling" => BinType::Recyclable,
            "food and garden waste" => BinType::Biodegradable,
            "rubbish" => BinType::Landfill,
            _ => BinType::NoBin,
        }
    }
}
