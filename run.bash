echo "Comliping haskell"
cd ~/Proyectos/haskell/ && ./compile.pl sql.hs;
echo "Generating data"
cd ~/Proyectos/sql/proyecto-escuela/gen;
rm out/*; cargo run -q --release > out/user.tsv;
find out/* > injection-source.txt;
echo "Running typer"
~/Proyectos/haskell/sql injection-source.txt '	' proyecto > ../code.sql;