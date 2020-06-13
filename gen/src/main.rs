extern crate rand;
extern crate serde_json;
#[macro_use]
extern crate serde;
extern crate csv;
extern crate hex;
extern crate hsl_ish;

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

const c_teachers: i64 = 100;
const c_students: i64 = 600 + c_teachers;
const c_parents: i64 = 300 + c_students;
const c_iter_size: i64 = 1000;
const c_group_period: i64 = 50;
const c_max_course_id: i64 = 285;
const c_max_classroom: i64 = 52;

mod plan;
mod subjects;

#[derive(Debug, Deserialize)]
struct User {
    pub id: i64,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: i64,
    pub login: String,
    pub password: String,
}

fn read_file(path: String) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).unwrap()
}

fn gen_phone_number(rng: &mut ThreadRng) -> i64 {
    let min = (10 as i64).pow(8);
    let max = (10 as i64).pow(9) - 1;

    (rng.gen::<f64>() * ((max - min) as f64)).floor() as i64 + min
}

fn gen_email(name: &String, last_name: &String, rng: &mut ThreadRng) -> String {
    let suffixes = vec!["gmail.com", "o2.pl", "yahoo.com", "wp.pl", "protonmail.com"];
    let suffix_i = (rng.gen::<f64>() * (suffixes.len() as f64)).floor() as usize;

    format!(
        "{}.{}@{}",
        name.get(..3).unwrap(),
        last_name,
        suffixes[suffix_i]
    )
}

fn gen_login(name: &String, last_name: &String) -> String {
    format!("{}{}", name.get(..3).unwrap(), last_name)
}

fn gen_password(name: &String, last_name: &String) -> String {
    format!("{}{}", name, last_name)
}

fn gen_users() {
    let mut rng = rand::thread_rng();
    let names: Vec<String> = read_file("./data/name.json".to_owned())
        .iter()
        .map(|a| a.to_lowercase())
        .collect();
    let last_names: Vec<String> = read_file("./data/last_name.json".to_owned())
        .iter()
        .map(|a| a.to_lowercase())
        .collect();

    let m = last_names.len();
    println!("id;name;last_name;email;phone_number;login;password");

    for (i, n) in names.iter().enumerate() {
        for (j, l) in last_names.iter().enumerate() {
            let id = i * m + j;
            let email = gen_email(&n, &l, &mut rng);
            let phone_number = gen_phone_number(&mut rng);
            let login = gen_login(&n, &l);
            let password = gen_password(&n, &l);

            println!(
                "{};{};{};{};{};{};{}",
                id, n, l, email, phone_number, login, password,
            );
        }
    }
}

fn gen_teacher(user: &User, j: i64, iter: i64, head: bool) -> String {
    let titles = vec!["prof.", "dr.hab.", "dr.", "mgr.", "doc."];
    let title = titles[(j % titles.len() as i64) as usize];

    format!(
        "{};{};{};{}\n",
        iter * c_teachers + j,
        user.id,
        title,
        head as i8
    )
}
static ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
fn make_group_name(id: i16) -> String {
    let n = ASCII_UPPER.len();
    let mut out = vec![];
    let mut j = (id % c_group_period as i16) as usize;

    loop {
        out.push(ASCII_UPPER[j % n]);
        j /= n;
        if j <= 0 {
            break;
        }
    }

    out.iter()
        .fold(String::new(), |acc, c| c.to_string() + &acc)
}

fn group_id_of_index(j: i64, iter: i64) -> i64 {
    iter * (c_iter_size / c_group_period) + j / c_group_period
}
fn gen_group(j: i64, iter: i64) -> String {
    let year_0 = 2020;
    let id = group_id_of_index(j, iter) as i16;
    let teacher_id = iter * (c_iter_size / c_group_period) + j / c_group_period; // iter * c_teachers + j / (iter+1);
    let name = make_group_name(id); // roman::Roman::from(id % c_group_period as i16 + 1);
    let start_year = year_0 + id / c_group_period as i16;
    format!("{};{};{};{}\n", id, teacher_id, name, start_year)
}

fn student_id(j: i64, iter: i64) -> i64 {
    iter * (c_students - c_teachers) + j - c_teachers
}
fn gen_student(j: i64, iter: i64) -> String {
    let id = student_id(j, iter);
    let user_id = iter * (c_students - c_teachers) + j;
    let group_id = iter * c_group_period + j / c_group_period - 2; // group_id_of_index(j - c_parents, iter);

    format!("{};{};{}\n", id, user_id, group_id)
}

fn parent_id_of_index(j: i64, iter: i64) -> i64 {
    iter * (c_parents - c_students) + j - c_students
}
fn gen_parent(j: i64, iter: i64) -> String {
    let id = parent_id_of_index(j, iter);
    let user_id = j;

    format!("{};{}\n", id, user_id)
}

fn gen_parenthood(j: i64, iter: i64) -> String {
    let parent_id = parent_id_of_index(j, iter);
    let id_ = iter * (c_parents - c_students) + j - c_students;
    let k = id_ * (c_students - c_teachers) / (c_parents - c_students);
    let id = k;
    format!(
        "{};{};{}\n{};{};{}\n",
        id,
        parent_id,
        k,
        id + 1,
        parent_id,
        k + 1
    )
}

fn gen_user_groups() {
    let mut rng = rand::thread_rng();
    // output files
    let mut o_students = File::create("out/student.csv").unwrap();
    let mut o_teachers = File::create("out/teacher.csv").unwrap();
    let mut o_parent = File::create("out/parent.csv").unwrap();
    let mut o_parenthood = File::create("out/parenthood.csv").unwrap();
    let mut o_group = File::create("out/group.csv").unwrap();
    let mut o_lessons = File::create("out/lesson.csv").unwrap();
    let mut o_cells = File::create("out/cell.csv").unwrap();
    let mut o_registrations = File::create("out/registration.csv").unwrap();
    let mut o_classes = File::create("out/class.csv").unwrap();
    let mut o_excuses = File::create("out/excuse.csv").unwrap();
    let mut o_substitutions = File::create("out/substitution.csv").unwrap();

    // Last ids
    let mut last_lesson_id = 0;
    let mut last_cell_id = 0;
    let mut last_reg_id = 0;
    let mut last_excuse_id = 0;
    let mut last_substitution_id = 0;

    // input
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path("out/user.csv")
        .unwrap();

    // print headers
    o_students.write(b"id;user_id;group_id\n").unwrap();
    o_teachers.write(b"id;user_id;title;is_head\n").unwrap();
    o_parent.write(b"id;user_id\n").unwrap();
    o_parenthood.write(b"id;parent_id;student_id\n").unwrap();
    o_group.write(b"id;teacher_id;name;start_year\n").unwrap();
    o_lessons
        .write(b"id;course_id;teacher_id;trimester_id;classroom;color\n")
        .unwrap();
    o_cells.write(b"id;period_id;lesson_id;week_day\n").unwrap();
    o_registrations
        .write(b"id;lesson_id;student_id;time\n")
        .unwrap();
    o_classes
        .write(b"id;student_id;lesson_id;date;attendance\n")
        .unwrap();
    o_excuses.write(b"id;excuser_id;class_id\n").unwrap();
    o_substitutions.write(b"id;teacher_id;class_id;classroom\n").unwrap();

    for (i, r_u) in rdr.deserialize().enumerate() {
        let i = i as i64;
        let j = i % c_iter_size;
        let iter = i / c_iter_size;
        let u: User = r_u.unwrap();

        if j < c_teachers {
            let is_head = j % 373 == 0 && iter % 1000 == 0;
            o_teachers
                .write(gen_teacher(&u, j, iter, is_head).as_bytes())
                .unwrap();
            // Generate the teacher's lessons
            let teacher_id = iter * c_teachers + j;
            for trimester_id in (plan::c_max_trimester_id - 5)..=plan::c_max_trimester_id {
                let course_id = teacher_id % c_max_course_id;
                let color = hsl_ish::Hsl::new(rng.gen::<f64>() * 360., 0.7, 0.5);
                let color = hsl_ish::Rgb::from(color);
                let color = hex::encode(vec![color.r, color.g, color.b]);
                let classroom = (rng.gen::<f64>() * c_max_classroom as f64) as i64 + 1;
                o_lessons
                    .write(
                        format!(
                            "{};{};{};{};{};{}\n",
                            last_lesson_id, course_id, teacher_id, trimester_id, classroom, color
                        )
                        .as_bytes(),
                    )
                    .unwrap();

                let p = ((plan::c_max_period_id - 1) as f64 * rng.gen::<f64>()) as i64;
                for week_day in 0..=4 {
                    let period_id = if week_day == 1 { p + 1 } else { p };
                    o_cells
                        .write(
                            format!(
                                "{};{};{};{}\n",
                                last_cell_id, period_id, last_lesson_id, week_day
                            )
                            .as_bytes(),
                        )
                        .unwrap();
                    last_cell_id += 1;
                }
                last_lesson_id += 1;
            }
        } else if j < c_students {
            o_students.write(gen_student(j, iter).as_bytes()).unwrap();
            for d_lesson_id in 0..=7 {
                let lesson_id = last_lesson_id - d_lesson_id;
                let student_id = student_id(j, iter);
                let time = "12:00:00";
                let attendance_date = "2020-09-03";
                let attendances = vec!["none", "present", "absent", "excused"];
                let attendance = attendances[lesson_id % attendances.len()];
                let id = last_reg_id;
                let class_id = last_reg_id;
                o_registrations
                    .write(format!("{};{};{};{}\n", id, lesson_id, student_id, time).as_bytes())
                    .unwrap();

                // Mark the student's attendance on the lesson
                o_classes
                    .write(
                        format!(
                            "{};{};{};{};{}\n",
                            class_id, student_id, lesson_id, attendance_date, attendance
                        )
                        .as_bytes(),
                    )
                    .unwrap();
                // Excuse if necessary
                if attendance == "excused" {
                    o_excuses
                        .write(
                            format!(
                                "{};{};{}\n",
                                last_excuse_id,
                                j - (c_students - c_teachers),
                                class_id
                            )
                            .as_bytes(),
                        )
                        .unwrap();
                }

                if rng.gen::<f64>() > 0.9 {
                    let teacher_id = j - (c_students - c_teachers);
                    let new_classroom = (rng.gen::<f64>() * c_max_classroom as f64) as i64 + 1;

                    o_substitutions
                        .write(
                            format!("{};{};{};{}\n", last_substitution_id, teacher_id, class_id, new_classroom)
                                .as_bytes(),
                        )
                        .unwrap();
                    last_substitution_id += 1;
                }

                last_reg_id += 1;
            }
        } else if j < c_parents {
            o_parent.write(gen_parent(j, iter).as_bytes()).unwrap();
            o_parenthood
                .write(gen_parenthood(j, iter).as_bytes())
                .unwrap();
        } else {
            eprintln!("Unmatched range: {}", j);
        }

        /*
            Create a new group.
            Triggers at the beginning of a new iteration
        */
        if j % c_group_period == 0 {
            o_group.write(gen_group(j, iter).as_bytes()).unwrap();
        }
    }
}

fn main() {
    subjects::gen_subjects();
    subjects::gen_courses();

    gen_users();
    gen_user_groups();

    plan::gen_trimesters();
    plan::gen_periods();
}
