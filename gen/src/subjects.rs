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
pub const c_course_range: [i64; 2] = [0, 8];
pub const c_course_range_size: i64 = c_course_range[1] - c_course_range[0] + 1;

pub fn read_subjects() -> Vec<Subject> {
    let file = File::open("./data/subjects.json").unwrap();
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap()
}

pub fn gen_courses() {
    let fh = read_subjects();
    let mut o_courses = File::create("out/course.tsv").unwrap();
    let mut o_course_req = File::create("out/course_req.tsv").unwrap();

    let mut last_req_id = 0;

    o_courses.write(b"id	subject_id	nr	spec	hours\n").unwrap();
    o_course_req.write(b"id	course_id	required_id\n").unwrap();
    for sub in fh.iter() {
        for i in c_course_range[0]..c_course_range[1] {
            let iter_id = sub.id * c_course_range_size * 2;
            let id_base = iter_id + i * 2;
            let id_spec = id_base + 1;
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
            o_courses
                .write(format!("{}	{}	{}	{}	{}\n", id_base, sub_id, nr_base, 0, hours).as_bytes())
                .unwrap();
            // Insert spec
            o_courses
                .write(format!("{}	{}	{}	{}	{}\n", id_spec, sub_id, nr_spec, 1, hours).as_bytes())
                .unwrap();

            // Insert requirements
            for j in 0..=(i * 2) {
                let id_req = iter_id + j;
                if id_req >= id_spec {
                    break;
                }
                o_course_req
                    .write(format!("{}	{}	{}\n", last_req_id, id_spec, id_req).as_bytes())
                    .unwrap();
                last_req_id += 1;

                if j % 2 == 0 && id_req != id_base {
                    o_course_req
                        .write(format!("{}	{}	{}\n", last_req_id, id_base, iter_id + j).as_bytes())
                        .unwrap();
                    last_req_id += 1;
                }
            }
        }
    }
}
