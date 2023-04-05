mod error;
use error::AppError;
use rand::Rng;
use serde_derive::{Deserialize, Serialize};

fn to_result<T>(val: serde_json::Result<T>) -> Result<T, AppError> {
    match val {
        serde_json::Result::Ok(t) => Result::Ok(t),
        serde_json::Result::Err(_err) => Result::Err(AppError::InvalidJson),
    }
}

#[derive(Debug)]
pub struct BinDay {
    date: i64,
    bins: Vec<BinType>,
}

#[derive(Deserialize, Serialize)]
struct RawBinDay {
    date: String,
    bins: Vec<String>,
}

impl BinDay {
    fn bins_to_icons(self) -> String {
        self.bins
            .iter()
            .map(BinType::to_icon)
            .collect::<Vec<String>>()
            .join("")
    }

    fn find_near(self) -> Option<Self> {
        let bin_date = chrono::Duration::milliseconds(self.date);

        let now = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .map_err(|_err| AppError::InvalidSystemTime)
            .and_then(|t| chrono::Duration::from_std(t).map_err(|_err| AppError::InvalidDuration))
            .unwrap();

        let start = now.checked_sub(&chrono::Duration::days(1));
        let end = now.checked_add(&chrono::Duration::days(2));

        start
            .zip(end)
            .and_then(|(mon, tue)| (mon..tue).contains(&bin_date).then_some(self))
    }

    fn find_nearest() -> Result<Self, AppError> {
        BinDay::decode().map(|list| match list.into_iter().find_map(BinDay::find_near) {
            Some(b) => b,
            None => BinDay::dud(),
        })
    }

    fn decode() -> Result<Vec<BinDay>, AppError> {
        let config = std::fs::read_to_string("./bins.json")?;

        let json_list = to_result(serde_json::from_str::<Vec<RawBinDay>>(&config))?;

        Result::Ok(
            json_list
                .iter()
                .map(|day| BinDay {
                    date: day
                        .date
                        .parse::<i64>()
                        .map_err(|_| AppError::InvalidDateString)
                        .unwrap(),
                    bins: day.bins.iter().map(BinType::decode).collect(),
                })
                .collect::<Vec<BinDay>>(),
        )
    }

    fn dud() -> Self {
        let mut rng = rand::thread_rng();
        BinDay {
            date: rng.gen::<i64>(),
            bins: vec![BinType::NoBin],
        }
    }

    pub fn print() -> String {
        match BinDay::find_nearest() {
            Result::Ok(v) => BinDay::bins_to_icons(v),
            Result::Err(err) => err.to_string(),
        }
    }
}

#[derive(Debug)]
enum BinType {
    Biodegradable,
    Landfill,
    Recyclable,
    NoBin,
}

impl BinType {
    fn decode(bin_string: &String) -> Self {
        match bin_string.as_str() {
            "recycling" => BinType::Recyclable,
            "food and garden waste" => BinType::Biodegradable,
            "rubbish" => BinType::Landfill,
            _ => BinType::NoBin,
        }
    }

    fn to_icon(&self) -> String {
        let icon = match self {
            BinType::Recyclable => "♻️ ",
            BinType::Biodegradable => "💩",
            BinType::Landfill => "🗑 ",
            BinType::NoBin => "🦄",
        };
        String::from(icon)
    }
}
