use serde_derive::{Deserialize, Serialize};
use std::convert::identity;
use std::time::{Duration, SystemTime};

pub struct BinDay {
    date: u128,
    bins: Vec<BinType>,
}

// TODO: Can this be removed?
#[derive(Deserialize, Serialize)]
struct RawBinDay {
    date: String,
    bins: Vec<String>,
}

impl BinDay {
    fn bins_to_icons(&self) -> String {
        self.bins
            .iter()
            .map(|bin_type| String::from(BinType::to_icon(bin_type)))
            .reduce(|cur: String, nxt| cur + &nxt)
            .unwrap()
    }

    fn find_nearest(&self) -> Option<&Self> {
        let day_secs = Duration::new(((1000 * 60) * 60) * 24, 0); // 86400000;

        let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(t) => t,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };

        let next = self
            .date
            .try_into()
            .map_or_else(|err| panic!("{}", err.to_string()), Duration::from_millis);

        // TODO: today_start should == today at 12am
        // TODO: tomorrow_end should == tomorrow at 11.59pm
        now.checked_sub(day_secs)
            .zip(now.checked_add(day_secs * 2))
            .and_then(|(today_start, tomorrow_end)| {
                (today_start..tomorrow_end).contains(&next).then_some(self)
            })
    }

    fn read_list_of_dates() -> Vec<BinDay> {
        let json_string_ref = match std::fs::read_to_string("./bins.json") {
            Result::Ok(json_string) => json_string,
            Result::Err(e) => panic!("{}", e.to_string()),
        };

        match serde_json::from_str::<Vec<RawBinDay>>(&json_string_ref) {
            serde_json::Result::Ok(json_list) => json_list
                .iter()
                .map(|day| BinDay {
                    date: day
                        .date
                        .parse::<u128>()
                        .map_or_else(|err| panic!("{}", err.to_string()), identity),
                    bins: day.bins.iter().map(BinType::from_string).collect(),
                })
                .collect(),

            serde_json::Result::Err(err) => panic!("{}", err.to_string()),
        }
    }

    pub fn print() -> String {
        BinDay::read_list_of_dates()
            .iter()
            .find_map(BinDay::find_nearest)
            .map(BinDay::bins_to_icons)
            .unwrap_or_else(|| format!("It's not bin day."))
    }
}

enum BinType {
    Biodegradable,
    Landfill,
    Recyclable,
    NoBin,
}

impl BinType {
    fn from_string(bin_string: &String) -> Self {
        match bin_string.as_str() {
            "recycling" => BinType::Recyclable,
            "food and garden waste" => BinType::Biodegradable,
            "rubbish" => BinType::Landfill,
            _ => BinType::NoBin,
        }
    }

    fn to_icon(&self) -> &str {
        match self {
            BinType::Recyclable => "♻️ ",
            BinType::Biodegradable => "💩",
            BinType::Landfill => "🗑",
            BinType::NoBin => "",
        }
    }
}
