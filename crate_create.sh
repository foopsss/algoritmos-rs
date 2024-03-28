#!bin/bash

# Creo un proyecto de Cargo nuevo y realizo los ajustes necesarios.
echo -n "Introduzca un nombre para el proyecto de Cargo: "
read crate_name

cargo new $crate_name
echo ""

rm $crate_name/src/main.rs

echo "$crate_name/target" >> .gitignore
echo "$crate_name/Cargo.lock" >> .gitignore

echo "[package]" > $crate_name/Cargo.toml
echo "name = \"$crate_name\"" >> $crate_name/Cargo.toml
echo "version = \"0.1.0\"" >> $crate_name/Cargo.toml
echo "edition = \"2021\"" >> $crate_name/Cargo.toml

# Introduzco los nombres de los binarios que van a componer mi proyecto.
echo "Introduzca el prefijo de los ejercicios de la guía."
echo "No olvide utilizar guiones en vez de puntos."
echo ""
echo -n "Prefijo: "
read guide_number
echo ""

echo "¿Cuántos ejecutables va a tener el repositorio?"
echo "Entiéndase como un ejecutable por ejercicio de la guía."
echo ""
echo -n "Número de ejecutables: "
read binary_count

for ((i = 1; i <= $binary_count; i++));
do
	echo "" >> $crate_name/Cargo.toml
	echo "[[bin]]" >> $crate_name/Cargo.toml
	echo "name = \"$guide_number-$i\"" >> $crate_name/Cargo.toml
	echo "path = \"src/$guide_number-$i.rs\"" >> $crate_name/Cargo.toml
	echo "" >> $crate_name/src/$guide_number-$i.rs
done
