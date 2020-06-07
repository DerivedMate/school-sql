create database if not exists proyecto;
use proyecto;
set global local_infile = True;

create table if not exists group (
	id int primary key,
	teacher_id int not null,
	name varchar(5),
	start_year int
);
load data local infile 'out/group.tsv' into table group fields terminated by '	' ignore 1 lines;


create table if not exists parenthood (
	id int primary key,
	parent_id int not null,
	student_id int not null
);
load data local infile 'out/parenthood.tsv' into table parenthood fields terminated by '	' ignore 1 lines;


create table if not exists parent (
	id int primary key,
	user_id int not null
);
load data local infile 'out/parent.tsv' into table parent fields terminated by '	' ignore 1 lines;


create table if not exists student (
	id int primary key,
	user_id int not null,
	group_id int not null
);
load data local infile 'out/student.tsv' into table student fields terminated by '	' ignore 1 lines;


create table if not exists teacher (
	id int primary key,
	user_id int not null,
	title varchar(7),
	is_head tinyint
);
load data local infile 'out/teacher.tsv' into table teacher fields terminated by '	' ignore 1 lines;


create table if not exists user (
	id int primary key,
	name varchar(9),
	last_name varchar(11),
	email varchar(30),
	phone_number int,
	login varchar(14),
	password varchar(20)
);
load data local infile 'out/user.tsv' into table user fields terminated by '	' ignore 1 lines;


