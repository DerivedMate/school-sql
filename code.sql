create database if not exists proyecto;
use proyecto;
set global local_infile = True;


create table if not exists `cell` (
	id int primary key,
	period_id int not null,
	lesson_id int not null,
	week_day int
);
load data local infile 'gen/out/cell.csv' into table `cell` fields terminated by ';' ignore 1 lines;


create table if not exists `class` (
	id int primary key,
	student_id int not null,
	lesson_id int not null,
	date date,
	attendance varchar(7)
);
load data local infile 'gen/out/class.csv' into table `class` fields terminated by ';' ignore 1 lines;


create table if not exists `course` (
	id int primary key,
	subject_id int not null,
	nr int,
	spec tinyint,
	hours int,
	slots int
);
load data local infile 'gen/out/course.csv' into table `course` fields terminated by ';' ignore 1 lines;


create table if not exists `course_req` (
	id int primary key,
	course_id int not null,
	required_id int not null
);
load data local infile 'gen/out/course_req.csv' into table `course_req` fields terminated by ';' ignore 1 lines;


create table if not exists `excuse` (
	id tinyint primary key,
	excuser_id int not null,
	class_id int not null
);
load data local infile 'gen/out/excuse.csv' into table `excuse` fields terminated by ';' ignore 1 lines;


create table if not exists `group` (
	id int primary key,
	teacher_id int not null,
	name varchar(2),
	start_year int
);
load data local infile 'gen/out/group.csv' into table `group` fields terminated by ';' ignore 1 lines;


create table if not exists `lesson` (
	id int primary key,
	course_id int not null,
	teacher_id int not null,
	trimester_id int not null,
	classroom int,
	color char(6)
);
load data local infile 'gen/out/lesson.csv' into table `lesson` fields terminated by ';' ignore 1 lines;


create table if not exists `major_arche` (
	id int primary key,
	name varchar(28),
	subject_id int not null,
	courses_req int
);
load data local infile 'gen/out/major_arche.csv' into table `major_arche` fields terminated by ';' ignore 1 lines;


create table if not exists `major_req` (
	id int primary key,
	arche_id int not null,
	course_id int not null
);
load data local infile 'gen/out/major_req.csv' into table `major_req` fields terminated by ';' ignore 1 lines;


create table if not exists `parent` (
	id int primary key,
	user_id int not null
);
load data local infile 'gen/out/parent.csv' into table `parent` fields terminated by ';' ignore 1 lines;


create table if not exists `parenthood` (
	id int primary key,
	parent_id int not null,
	student_id int not null
);
load data local infile 'gen/out/parenthood.csv' into table `parenthood` fields terminated by ';' ignore 1 lines;


create table if not exists `period` (
	id int primary key,
	from time,
	to time
);
load data local infile 'gen/out/period.csv' into table `period` fields terminated by ';' ignore 1 lines;


create table if not exists `registration` (
	id int primary key,
	lesson_id int not null,
	student_id int not null,
	time time
);
load data local infile 'gen/out/registration.csv' into table `registration` fields terminated by ';' ignore 1 lines;


create table if not exists `student` (
	id int primary key,
	user_id int not null,
	group_id int not null
);
load data local infile 'gen/out/student.csv' into table `student` fields terminated by ';' ignore 1 lines;


create table if not exists `subject` (
	id int primary key,
	name varchar(22),
	name_short varchar(7)
);
load data local infile 'gen/out/subject.csv' into table `subject` fields terminated by ';' ignore 1 lines;


create table if not exists `substitution` (
	id int primary key,
	teacher_id int not null,
	class_id int not null,
	classroom int
);
load data local infile 'gen/out/substitution.csv' into table `substitution` fields terminated by ';' ignore 1 lines;


create table if not exists `teacher` (
	id int primary key,
	user_id int not null,
	title varchar(7),
	is_head tinyint
);
load data local infile 'gen/out/teacher.csv' into table `teacher` fields terminated by ';' ignore 1 lines;


create table if not exists `trimester` (
	id int primary key,
	start_date date
);
load data local infile 'gen/out/trimester.csv' into table `trimester` fields terminated by ';' ignore 1 lines;


create table if not exists `user` (
	id int primary key,
	name varchar(10),
	last_name varchar(11),
	email varchar(30),
	phone_number int,
	login varchar(14),
	password varchar(21)
);
load data local infile 'gen/out/user.csv' into table `user` fields terminated by ';' ignore 1 lines;

