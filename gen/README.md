# Super(-ish)SAD

## How it works :canada:
The project in and of itself is dead simple:
1. [The generator](/tree/master/gen) generates csv files based on [the input provided](/tree/master/gen/data). They amount to about 1.8Gbs.
2. These files then serve as an input (in the form of a linux `find` output) for the [typer](https://github.com/DerivedMate/haskell-mono/blob/master/sql.hs), which generates an [sql script](https://github.com/DerivedMate/school-sql/blob/master/code.sql) for creating the database, tables, and importing the aforementioned data.

## Usage 
The `run.bash` script is (currently) suited only for my personal setup. Needless to say, feel free to alter it, but let me walk you through the general steps.
Firstly, ensure you've got both [Rust](https://www.rust-lang.org/tools/install), and [Haskell](https://www.haskell.org/platform/) installed on your machine. <br/>As to the actual commands:
1. `cd gen; ./gen.sh` to generate files
2. `cd ../; find gen/out*.csv > source.txt` to list the files in a source file
3. `<typer_dir>/compile.pl sql.hs; <typer_dir>/sql source.txt ';' proyecto > code.sql` to generate sql code
4. `mysql -p --local-infile=1 < code.sql` to execute the script (may take a while, because of the file sizes).

## Jak działa :poland:
Projekt w swej istocie jest dość prosty: 
1. [Generator](/tree/master/gen) generuje pliki csv z danymi na podstawie wprowadzonych danych. Wychodzi z tego ≈1.8Gb.
2. Są one później listowane do pliku, a ten wprowadzany do [typera](https://github.com/DerivedMate/haskell-mono/blob/master/sql.hs), który na ich podstawie generuje [script sql](https://github.com/DerivedMate/school-sql/blob/master/code.sql), który generuje bazę danych, tabele oraz wprowadza do nich dane z plików.

## Użycie
Szybka uwaga, jako że generowane są prawie 2Gb danych, ich wprowadzanie do bazy danych może zająć (bardzo)dużo czasu. Do sedna, żeby włączyć projekt, należy upewnić się, że zainstalowane są:
1. [Rust](https://www.rust-lang.org/tools/install) (preferowany w wersji nightly)
2. [Haskell](https://www.haskell.org/platform/)

`run.bash` script dostosowany jest (obecnie) tylko pod mój set-up, więc aby uruchomić projekt należy:
1. `cd gen; ./gen.sh` żeby wygenerować pliki
2. `cd ../; find gen/out*.csv > source.txt` żeby dodać pliki do listy źródeł
3. `<folder z typerem>/compile.pl sql.hs; <folder z typerem>/sql source.txt ';' proyecto > code.sql` aby wygenerować kod sql
4. `mysql -p --local-infile=1 < code.sql` aby wykonać powyższy script (może zająć trochę czasu).