#!/bin/bash
set -e

APP_VERSION="$(/usr/bin/python3 getversion.py)"

VERSION="$APP_VERSION"
DEB_VERSION="9999"
PACKAGE="nexus-rest-bmc"
PACKAGE_NAME="$PACKAGE"_"$VERSION"
YEAR=$(date +"%Y")
export DEBFULLNAME="Unofficial"
export DEBEMAIL="nobody@example.com"

# Clean and create build dir
rm -Rf builddeb/ || true
mkdir builddeb/
cd builddeb/


mkdir -p "$PACKAGE_NAME/lib/systemd/system/"
mkdir -p "$PACKAGE_NAME/usr/bin/"
mkdir -p "$PACKAGE_NAME/etc/nginx/sites-available/nexus"

cp ../systemd/nexus-rest-bmc.service "$PACKAGE_NAME/lib/systemd/system/"
cp ../target/release/nexus-rest-bmc  "$PACKAGE_NAME/usr/bin/" || true  # Support native builds
cp ../target/armv7-unknown-linux-gnueabihf/release/nexus-rest-bmc  "$PACKAGE_NAME/usr/bin/" || true  # Support cross builds
find "$PACKAGE_NAME/usr/bin/nexus-rest-bmc"  # Check if a binary exists
cp ../nginx/nexus-rest-bmc.conf  "$PACKAGE_NAME/etc/nginx/sites-available/nexus"

cd "$PACKAGE_NAME"

# Create manifest
mkdir debian
dch --create -v "$VERSION"-"$DEB_VERSION" --package "$PACKAGE" --distribution stable "Please visit https://github.com/nexus-unity/rest-bmc for details!"
echo "10" > ./debian/compat

echo "Source: $PACKAGE" > ./debian/control
echo "Maintainer: $DEBFULLNAME <$DEBEMAIL>" >> ./debian/control
echo "Section: misc" >> ./debian/control
echo "Priority: optional" >> ./debian/control
echo "Standards-Version: 3.9.2" >> ./debian/control
echo "Build-Depends: debhelper (>= 10)" >> ./debian/control
echo "" >> ./debian/control
echo "Package: $PACKAGE" >> ./debian/control
echo "Architecture: any" >> ./debian/control
echo "Depends: \${misc:Depends}, \${shlibs:Depends}, bash (>= 2.1.0), systemd, nginx, nexus-nginx-config, nexus-drv-bmc" >> ./debian/control
echo "Description: BMC REST API" >> ./debian/control
echo "	Webserver providing a REST API" >> ./debian/control

# Copyright
echo "Copyright $YEAR $DEBFULLNAME $DEBEMAIL" > ./debian/copyright
echo "Licenced under /usr/share/common-licenses/GPL-3" >> ./debian/copyright

# Rules
echo "#!/usr/bin/make -f" >>  ./debian/rules
echo "" >> ./debian/rules
echo "export DH_VERBOSE=1" >> ./debian/rules
echo "" >> ./debian/rules
echo "%:" >>  ./debian/rules
echo -e "\tdh \$@ --with-systemd --no-automatic-dbgsym" >>  ./debian/rules

# Deb format
mkdir ./debian/source
echo "3.0 (quilt)" > ./debian/source/format

# Install script
echo "lib/* lib/" >> ./debian/install
echo "usr/* usr/" >> ./debian/install
echo "etc/* etc/" >> ./debian/install


# Post install script
echo "#!/bin/bash -e" >> ./debian/postinst
echo "#DEBHELPER#" >> ./debian/postinst
echo "deb-systemd-invoke reload nginx" >> ./debian/postinst

# Pre removal script
echo "#!/bin/bash -e" >> ./debian/prerm
echo "#DEBHELPER#" >> ./debian/prerm
echo "deb-systemd-invoke reload nginx" >> ./debian/prerm

#cd "$PACKAGE_NAME"
debuild -b -uc -us --no-sign
