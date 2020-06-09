use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct Subject {
    pub id: i64,
    pub name: String,
    pub short: String,
}

pub const c_max_sub_index: i64 = 13;
pub const c_course_range: Vec<i64> = vec![1, 8];
pub const c_course_range_size: i64 = c_course_range[1] - c_course_range[0];

pub fn read_subjects() -> Vec<Subject> {
    let file = File::open("./data/subjects.json").unwrap();
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap()
}

pub fn gen_courses() {
    let fh = read_subjects();
    let mut o_courses = File::create("out/course.tsv").unwrap();
    let mut o_course_req = File::create("out/course_req.tsv").unwrap();

    println!("id	subject_id	nr	spec	hours");
    for sub in fh.iter() {
        for i in c_course_range[0]..=c_course_range[1] {
            let id = sub.id * c_course_range_size * 2 + i * 2;
            let sub_id = sub.id;
            let nr_spec = 200 + i;
            let nr_base = 100 + i;
            let hours = match sub.short.as_str() {
                "reg.stu" => 24,
                "phy.edu" => 36,
                _ => 60,
            };

            // Add all requirements explicitly
            // Insert base
            o_courses.write(format!("{}	{}	{}	{}	{}\n", id, sub_id, nr_base, 0, hours));
            // Insert spec
            o_courses.write(format!("{}	{}	{}	{}	{}\n", id + 1, sub_id, nr_spec, 1, hours));

            // Insert requirements
            for j in 0..i {}
        }
    }
}
