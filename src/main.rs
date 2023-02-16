use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::time::SystemTime;

fn main() {
    match get_data() {
        Ok(bin_day) => {
            // TODO: Remove this '!'
            if !is_bin_day_near(bin_day.date) {
                let bins_str = bin_day
                    .bins
                    .iter()
                    .map(BinType::to_icon)
                    .map(String::from)
                    .reduce(|cur: String, nxt| cur + &nxt)
                    .unwrap();
                println!("Bin day! {}", bins_str);
            }
        }
        Err(err) => panic!("Not today because {}", err),
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

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BinDay {
    date: u128,
    bins: Vec<BinType>,
}

fn get_data() -> Result<BinDay, String> {
    // TODO: Parse data here
    // TODO: Remove unwrap & handle errors
    let bin_str = std::fs::read_to_string("src/bins.json").unwrap();
    let raw_bin_days = serde_json::from_str::<serde_json::Value>(&bin_str).unwrap();

    let bin_days = raw_bin_days
        .as_array()
        .unwrap()
        .iter()
        .map(|raw_bin_day: &serde_json::Value| {
            let raw_day: serde_json::Value = raw_bin_day.to_owned();
            let not_raw = raw_day.as_object().unwrap().to_owned();
            let dt: u128 = (not_raw["date"].as_str().unwrap()).parse().unwrap();
            let bins: Vec<serde_json::Value> = raw_day["bins"].as_array().unwrap().to_owned();

            return BinDay {
                date: dt,
                bins: bins
                    .iter()
                    .map(|x| return x.as_str().unwrap())
                    .map(|bin_str| BinType::from_str(bin_str))
                    .collect::<Vec<BinType>>(),
            };
        })
        .collect::<Vec<BinDay>>();

    // TODO: This is always the first value in the list
    let firsty: BinDay = bin_days.first().cloned().unwrap();

    return Result::Ok(firsty);
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
