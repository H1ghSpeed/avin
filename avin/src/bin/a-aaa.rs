#![allow(dead_code)]
#![allow(unused)]

use avin_analyse::*;
use avin_core::*;
use avin_simulator::*;
use avin_strategy::*;
use avin_utils::*;
use avin_data::*;
use chrono::prelude::*;
// use chrono::{NaiveDate, NaiveDateTime};



#[tokio::main]
async fn main() {

    avin_utils::init_logger();
    let m = SourceMoex::new();
    let iid = Manager::find_iid("MOEX_SHARE_GAZP").unwrap();
    // let begin = Utc.with_ymd_and_hms(2025, 8, 4, 19, 20, 0).unwrap();
    // let till = Utc.with_ymd_and_hms(2025, 8, 5, 19, 20, 0).unwrap();
    let begin = NaiveDateTime::parse_from_str("2025-08-28 11:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let till = NaiveDateTime::parse_from_str("2025-08-28 23:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let bars = m.get_bars(&iid, MarketData::BAR_1H, begin, till).await;
    dbg!(bars);
}
