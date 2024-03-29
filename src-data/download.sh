#! /bin/bash

# Check the current version
VERSION_FILE=version.txt
VERSION=$(cat $VERSION_FILE 2>/dev/null || echo 0)
echo "Current version: $VERSION"

# Check the latest version
DATA_URL="https://github.com/bot4dofus/Datafus/releases"
LATEST_DATA_URL="$DATA_URL/latest"
LATEST_VERSION=$(curl -sL -I -o /dev/null -w %{url_effective} $LATEST_DATA_URL | rev | cut -d'/' -f1 | rev)
echo "Latest version: $LATEST_VERSION"

# Download the latest version
if [ $VERSION == $LATEST_VERSION ]; then
	echo "Already up to date!"
else
	echo "Downloading the latest version..."
	DOWNLOAD_URL="$DATA_URL/download/$LATEST_VERSION/data.zip"
	TMP_DIR=$(mktemp -d)
	DATA_DIR=$(dirname $0)/data
	curl -sL $DOWNLOAD_URL -o $TMP_DIR/data.zip
	unzip -q -o $TMP_DIR/data.zip -d $TMP_DIR
	echo "Downloaded to $TMP_DIR"

	# Copy useful files
	cp -r "$TMP_DIR/data/C/" "$DATA_DIR"
	cp -r "$TMP_DIR/data/B/" "$DATA_DIR"

	# Clean up
	rm -rf $TMP_DIR

	# Update the version file
	echo $LATEST_VERSION > $VERSION_FILE
	echo "Updated to version $LATEST_VERSION"
	echo "Data downloaded to $DATA_DIR"

	echo "Done!"
fi
