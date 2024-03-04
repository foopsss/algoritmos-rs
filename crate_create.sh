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
echo "Introduzca los nombres de los binarios que va a tener el proyecto."
echo "No se olvide de separarlos con espacios y de escribir los nombres" 
echo "con guiones. Tampoco utilice puntos."
echo ""
echo "Ejemplo: 1-13 1-14 1-15 1-16 1-17 1-18 1-19"
echo ""

echo -n "Binarios: "
read binary_names

binaries=( $binary_names )

for arg in ${binaries[@]}
do
	echo "" >> $crate_name/Cargo.toml
	echo "[[bin]]" >> $crate_name/Cargo.toml
	echo "name = \"$arg\"" >> $crate_name/Cargo.toml
	echo "path = \"src/$arg.rs\"" >> $crate_name/Cargo.toml
	echo "" >> $crate_name/src/$arg.rs
done
