Name:           rustyuki
Version:        0.2.0
Release:        1%{?dist}
Summary:        Rust CLI to generate and install Fedora Unified Kernel Images

License:        MIT
URL:            https://github.com/GlitchSlayed/RustyUKI
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rust

%description
RustyUKI is a Rust-native CLI for building and installing Unified Kernel Images
on Fedora-based systems via dracut and ukify.

%prep
%autosetup -n %{name}-%{version}

%build
cargo build --release --locked

%install
install -Dpm0755 target/release/rustyuki %{buildroot}%{_bindir}/rustyuki

%files
%license LICENSE
%doc README.md
%{_bindir}/rustyuki

%changelog
* Fri Mar 13 2026 RustyUKI CI <ci@example.com> - 0.2.0-1
- Initial Fedora RPM spec for automated CI builds
