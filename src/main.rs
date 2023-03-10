extern crate neovim_lib;
use neovim_lib::{Neovim, NeovimApi, Session};
use serde_derive::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
fn main() {
    let mut event_handler = EventHandler::new();
    event_handler.recv();
}

enum Messages {
    Show,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "show" => Messages::Show,
            _ => Messages::Unknown(event),
        }
    }
}

struct EventHandler {
    nvim: Neovim,
    bin_day: String,
}

impl EventHandler {
    fn new() -> EventHandler {
        let session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);
        let bin_day = BinDay::print();

        EventHandler { nvim, bin_day }
    }

    fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, _values) in receiver {
            match Messages::from(event) {
                Messages::Show => {
                    let print = &self.bin_day;
                    self.nvim.command(&format!("echo \"{}\"", print)).unwrap();
                }

                // Handle anything else
                Messages::Unknown(event) => {
                    self.nvim
                        .command(&format!("echo \"Unknown command: {}\"", event))
                        .unwrap();
                }
            }
        }
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

    fn is_near(next_date: u128) -> bool {
        let day_secs = Duration::new(((1000 * 60) * 60) * 24, 0); // 86400000;
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(now) => {
                // TODO: today_start should == today at 12am
                // TODO: tomorrow_end should == tomorrow at 11.59pm
                match (now.checked_sub(day_secs), now.checked_add(day_secs * 2)) {
                    (Some(today_start), Some(tomorrow_end)) => {
                        let nexty = Duration::from_millis(next_date.try_into().unwrap());
                        (today_start..tomorrow_end).contains(&nexty)
                    }
                    _ => false,
                }
            }
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    }

    fn from_value(ab: &serde_json::Value) -> Option<BinDay> {
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

    fn print() -> String {
        // TODO: Remove unwrap & handle errors -- Implement and_then for serde_json::Results
        let bin_str = std::fs::read_to_string("./bins.json").unwrap();
        let bin_day = serde_json::from_str::<serde_json::Value>(&bin_str)
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .find_map(|raw_day| match raw_day["date"].as_str() {
                Some(date_str) => {
                    let date = date_str.parse::<u128>().unwrap();
                    if BinDay::is_near(date) {
                        return BinDay::from_value(raw_day);
                    } else {
                        return None;
                    }
                }
                _ => None,
            });

        match bin_day {
            Some(today) => {
                let icons = BinDay::bins_to_icons(&today);
                return format!("{} {}", String::from("Bin day!"), icons);
            }
            _ => format!("Keep calm. It's not bin day today or tomorrow"),
        }
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
            BinType::Recyclable => "?????? ",
            BinType::Biodegradable => "????",
            BinType::Landfill => "????",
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
