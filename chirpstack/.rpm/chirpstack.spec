%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: chirpstack
Summary: ChirpStack is an open-source LoRaWAN(TM) Network Server
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: MIT
Group: Applications/System
Source0: %{name}-%{version}.tar.gz
URL: https://www.chirpstack.io/

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}
sed -i "s/\$MQTT_BROKER_HOST/localhost/" %{buildroot}/etc/chirpstack/*.toml
sed -i "s/\$POSTGRESQL_HOST/localhost/" %{buildroot}/etc/chirpstack/*.toml
sed -i "s/\$REDIS_HOST/localhost/" %{buildroot}/etc/chirpstack/*.toml

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
/lib/systemd/system/chirpstack.service
%config /etc/chirpstack/*
