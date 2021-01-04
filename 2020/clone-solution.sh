echo "Starting..."

cwd=$PWD

relative_source=$1
relative_target=$2

source="$cwd/$relative_source"
target="$cwd/$relative_target"

cp -R $source $target
find "$target/src" -type f -delete

echo "Copied $relative_source to $relative_target and emptied src/ directory."
echo "Done!"
