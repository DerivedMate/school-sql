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
pub const c_max_behavior_id: i64 = 5;

use crate::utils;

pub fn read_subjects() -> Vec<Subject> {
    let file = File::open("./data/subjects.json").unwrap();
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap()
}

pub fn gen_subjects() {
    let fh = read_subjects();
    let mut o_subjects = utils::create_table("out/subject.csv", b"id;name;name_short\n");

    for sub in fh.iter() {
        utils::write_entry(
            format!("{};{};{}\n", sub.id, sub.name, sub.short),
            &mut o_subjects,
        );
    }
}

pub fn gen_courses() {
    let fh = read_subjects();
    let mut o_courses =
        utils::create_table("out/course.csv", b"id;subject_id;nr;spec;hours;slots\n");
    let mut o_course_req = utils::create_table("out/course_req.csv", b"id;course_id;required_id\n");
    let mut o_major_arches =
        utils::create_table("out/major_arche.csv", b"id;name;subject_id;courses_req\n");
    let mut o_major_req = utils::create_table("out/major_req.csv", b"id;arche_id;course_id\n");

    let mut last_req_id = 0;
    let mut last_major_req_id = 0;

    for sub in fh.into_iter() {
        // Insert major arches
        let id_major_arche = sub.id;
        let name_major_arche = format!("{} major", &sub.name);
        let nr_courses_req = match sub.short.as_str() {
            "eng" | "lat" | "spa" => 9,
            _ => 0,
        };

        utils::write_entry(
            format!(
                "{};{};{};{}\n",
                id_major_arche, name_major_arche, sub.id, nr_courses_req
            ),
            &mut o_major_arches,
        );

        // Insert courses
        for i in c_course_range[0]..c_course_range[1] {
            let iter_id = sub.id * c_course_range_size * 2;
            let id_base = iter_id + i * 2;
            let id_spec = id_base + 1;
            let sub_id = sub.id;
            let nr_spec = 200 + i;
            let nr_base = 100 + i;
            let slots = 50;
            let hours = match sub.short.as_str() {
                "reg.stu" => 24,
                "phy.edu" => 36,
                _ => 60,
            };

            // Add all requirements explicitly
            // Insert base
            utils::write_entry(
                format!(
                    "{};{};{};{};{};{}\n",
                    id_base, sub_id, nr_base, 0, hours, slots
                ),
                &mut o_courses,
            );
            // Insert spec
            utils::write_entry(
                format!(
                    "{};{};{};{};{};{}\n",
                    id_spec, sub_id, nr_spec, 1, hours, slots
                ),
                &mut o_courses,
            );
            if nr_courses_req == 0 {
                utils::write_entry(
                    format!("{};{};{}\n", last_major_req_id, id_major_arche, id_spec),
                    &mut o_major_req,
                );

                last_major_req_id += 1;
            }

            // Insert requirements
            for j in 0..=(i * 2) {
                let id_req = iter_id + j;
                if id_req >= id_spec {
                    break;
                }
                utils::write_entry(
                    format!("{};{};{}\n", last_req_id, id_spec, id_req),
                    &mut o_course_req,
                );
                last_req_id += 1;

                if j % 2 == 0 && id_req != id_base {
                    utils::write_entry(
                        format!("{};{};{}\n", last_req_id, id_base, iter_id + j),
                        &mut o_course_req,
                    );
                    last_req_id += 1;
                }
            }
        }
    }
}

pub fn gen_behavior_arches() {
    let mut o_behavior_arches = utils::create_table("out/behavior_arche.csv", b"id;name;points\n");

    for (i, (name, points)) in vec![
        ("are you having a laugh", 0),
        ("ya ain't good enough", 1),
        ("pandejo", 2),
        ("decent", 3),
        ("snazzy", 4),
        ("sublime", 5),
    ]
    .iter()
    .enumerate()
    {
        utils::write_entry(
            format!("{};{};{}\n", i, name, points),
            &mut o_behavior_arches,
        );
    }
}
