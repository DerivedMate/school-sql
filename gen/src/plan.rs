use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use crate::utils;

#[derive(Debug, Deserialize)]
pub struct Period {
    pub from: String,
    pub to: String,
}

pub const c_last_year: i64 = 2419;
pub const c_first_year: i64 = 2020;
pub const c_max_period_id: i64 = 9;
pub const c_max_trimester_id: i64 = 1199;

fn read_periods() -> Vec<Period> {
    let file = File::open("./data/periods.json").unwrap();
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap()
}

pub fn gen_trimesters() {
    let mut o_trimesters = utils::create_table("out/trimester.csv", b"id;start_date\n");

    for year in c_first_year..=c_last_year {
        let s_days = vec!["09-02", "11-24", "02-16"];
        let n = s_days.len() as i64;
        for (i, s_day) in s_days.iter().enumerate() {
            let start_date = format!("{}-{}", year, s_day);
            utils::write_entry(format!("{};{}\n", (year - c_first_year) * n + i as i64, start_date), &mut o_trimesters);
        }
    }
}

pub fn gen_periods() {
    let mut o_periods = utils::create_table("out/period.csv", b"id;from;to\n");

    let fh = read_periods();

    for (i, p) in fh.iter().enumerate() {
        utils::write_entry(format!("{};{};{}\n", i, p.from, p.to), &mut o_periods);
    }
}
