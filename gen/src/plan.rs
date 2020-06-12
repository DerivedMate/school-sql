use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct Period {
    pub from: String,
    pub to: String,
}

const c_last_year: i64 = 2419;
const c_first_year: i64 = 2020;

fn read_periods() -> Vec<Period> {
    let file = File::open("./data/periods.json").unwrap();
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap()
}

pub fn gen_trimesters() {
    let mut o_trimesters = File::create("out/trimester.csv").unwrap();

    o_trimesters.write(b"id;start_date\n").unwrap();
    for year in c_first_year..=c_last_year {
        let s_days = vec!["09-02", "11-24", "02-16"];
        let n = s_days.len() as i64;
        for (i, s_day) in s_days.iter().enumerate() {
            let start_date = format!("{}-{}", year, s_day);
            o_trimesters
                .write(
                    format!("{};{}\n", (year - c_first_year) * n + i as i64, start_date).as_bytes(),
                )
                .unwrap();
        }
    }
}

pub fn gen_periods() {
    let mut o_periods = File::create("out/period.csv").unwrap();

    let fh = read_periods();

    o_periods.write(b"id;from;to\n").unwrap();
    for (i, p) in fh.iter().enumerate() {
        o_periods
            .write(format!("{};{};{}\n", i, p.from, p.to).as_bytes())
            .unwrap();
    }
}
