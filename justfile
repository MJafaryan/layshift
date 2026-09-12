binary_dir := "/usr/local/bin"
data_dir := "/usr/local/share/layshift"

# See the list of recipes
default:
    @just --list

# Check the project errors
[group: 'Development']
check:
    DATA_DIR="." cargo check

# Build the project for testing and development
[group: 'Development']
build:
    DATA_DIR="." cargo build

# Run the project
[group: 'Development']
run *args: build
    ./target/debug/layshift {{args}}

# Test the project
[group: 'Development']
test:
    DATA_DIR="." cargo test

# Build project for rpm-based distros
[group: 'Build']
build-rpm:
    source_dir="$HOME/rpmbuild/SOURCES"
    source_archive="layshift.tar.gz"

    tar -czf "$source_archive" --transform 's,^,layshift/,' Cargo.toml Cargo.lock src/ layouts/

    mkdir -p "%source_dir"
    mv "%source_archive" "$source_dir"

    rpmbuild -ba packaging/rpm/layshift.spec

# Build project for debian-based distros
[group: 'Build']
build-deb:
    data_dir="/usr/share/layshift"

    package_dir="target/deb"
    package_usr_dir="$package_dir/usr"
    package_control_dir="$package_dir/DEBIAN"
    package_data_dir="$package_usr_dir/share/layshift"
    package_binary_dir="$package_usr_dir/bin"

    mkdir -p "$package_control_dir"
    mkdir -p "$package_binary_dir"
    mkdir -p "$package_data_dir"

    DATA_DIR="$data_dir" cargo build --release

    cp packaging/deb/control "$package_control_dir/"
    install -Dm755 target/release/layshift "$package_binary_dir/layshift"
    cp -r layouts/ "$package_data_dir/"

    dpkg-deb --build --root-owner-group "$package_dir"

# Build project from source code
[group: 'Build']
build-release:
    DATA_DIR={{data_dir}} cargo build --release

# Build and install project from source code
[group: 'Build']
install: build-release
    sudo install -Dm755 target/release/layshift {{binary_dir}}/layshift
    sudo mkdir -p {{data_dir}}/layouts
    sudo cp -r layouts/. {{data_dir}}/layouts/

# Uninstall manual installed project
[group: 'Build']
uninstall:
    sudo rm -f {{binary_dir}}/layshift
    sudo rm -rf {{data_dir}}
