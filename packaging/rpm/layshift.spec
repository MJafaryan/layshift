Name: layshift
Version: 2.1.0
Release: 1%{?dist}
Summary: A clipboard tool for converting text between keyboard layouts

License: GPL-3.0
URL: https://github.com/MJafaryan/layshift
Source0: %{name}.tar.gz

Requires: (wl-clipboard if libwayland-client)
Requires: (xclip if xorg-x11-server-Xorg)

%description
A small Linux clipboard tool for converting text between keyboard layouts.

%prep
%setup -q -n layshift

%build
DATA_DIR=%{_datadir}/layshift cargo build --release

%install
install -Dm755 target/release/layshift %{buildroot}%{_bindir}/layshift
mkdir -p %{buildroot}%{_datadir}/layshift
cp -r layouts %{buildroot}%{_datadir}/layshift/

%files
%{_bindir}/layshift
%{_datadir}/layshift/

%changelog
* Tue Sep 01 2026 M.Jafaryan
- Initial RPM package
