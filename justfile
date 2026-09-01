binary_dir := "/usr/local/bin/"
data_dir := "/usr/local/share/layshift"

build:
    DATA_DIR="." cargo build
run *args: build
    ./target/debug/layshift {{args}}
test:
    DATA_DIR="." cargo test
build-rpm:
    tar -czf layshift.tar.gz --transform 's,^,layshift/,' Cargo.toml Cargo.lock src/ layouts/
    mkdir -p ~/rpmbuild/SOURCES/
    mv layshift.tar.gz ~/rpmbuild/SOURCES/
    rpmbuild -ba packaging/rpm/layshift.spec
build-release:
    DATA_DIR={{data_dir}} cargo build --release
install: build-release
    sudo install -Dm755 target/release/layshift {{binary_dir}}/layshift
    sudo mkdir -p {{data_dir}}/layouts
    sudo cp -r layouts/. {{data_dir}}/layouts/
uninstall:
    sudo rm -f {{binary_dir}}/layshift
    sudo rm -rf {{data_dir}}
