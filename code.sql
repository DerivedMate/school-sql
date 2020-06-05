create database if not exists proyecto;
use proyecto;
set global local_infile = True;

create table if not exists course_req (
	id int primary key,
	course_id int not null,
	required_id int not null
);
load data local infile 'data/course_req.tsv' into table course_req fields terminated by '	' ignore 1 lines;

create table if not exists course (
	id int primary key,
	subject_id int not null,
	nr int,
	spec int,
	hours int
);
load data local infile 'data/course.tsv' into table course fields terminated by '	' ignore 1 lines;

create table if not exists major_arche (
	id int primary key,
	name char(14),
	subject_id int not null,
	courses_req int
);
load data local infile 'data/major_arche.tsv' into table major_arche fields terminated by '	' ignore 1 lines;

create table if not exists major_req (
	id int primary key,
	major_arche_id int not null,
	course_id int not null
);
load data local infile 'data/major_req.tsv' into table major_req fields terminated by '	' ignore 1 lines;

create table if not exists major (
	id int primary key,
	student_id int not null,
	major_arche_id int not null
);
load data local infile 'data/major.tsv' into table major fields terminated by '	' ignore 1 lines;

create table if not exists parent (
	id int primary key,
	user_id int not null
);
load data local infile 'data/parent.tsv' into table parent fields terminated by '	' ignore 1 lines;

create table if not exists student (
	id int primary key,
	user_id int not null,
	group_id int not null
);
load data local infile 'data/student.tsv' into table student fields terminated by '	' ignore 1 lines;

create table if not exists subject (
	id int primary key,
	name char(11),
	name_short char(3)
);
load data local infile 'data/subject.csv' into table subject fields terminated by '	' ignore 1 lines;

create table if not exists teacher (
	id int primary key,
	user_id int not null,
	title varchar(7),
	head int
);
load data local infile 'data/teacher.tsv' into table teacher fields terminated by '	' ignore 1 lines;

create table if not exists user (
	id int primary key,
	name varchar(10),
	lastname varchar(13),
	email varchar(19),
	phone_number int,
	login varchar(8),
	password varchar(10)
);
load data local infile 'data/user.tsv' into table user fields terminated by '	' ignore 1 lines;

