echo "Starting..."

cwd=$PWD

relative_source=$1
relative_target=$2

source="$cwd/$relative_source"
target="$cwd/$relative_target"

cp -R $source $target
rm -rf "$target/src"
mkdir "$target/src"

echo "[v] -> Copied $relative_source to $relative_target and emptied src/ directory."

touch "$target/src/main.rs"
echo "[v] -> Created an empty main.rs in $target/src."

# Create symbolic link to data.txt, create an empty data.txt if it does not exist.
rm "$target/data.txt"
year="$(basename "$cwd")"
data_basepath="$(dirname "$cwd")"
data_dir="${data_basepath}-data/$year/$relative_target"
data_path="$data_dir/data.txt"
if [ ! -d $data_dir ]; then
    echo "[!] -> Could not find data directory for $data_dir"
    mkdir $data_dir
    echo "... data directory created: $data_dir"
fi
if [ ! -f $data_path ]; then
    echo "[!] -> Could not find data.txt for $relative_target."
    touch $data_path
    echo "... data.txt created: $data_path"
fi
ln -s $data_path $target
echo "[v] -> Symbolic link to data.txt created"

echo "Done!"
