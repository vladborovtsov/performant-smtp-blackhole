#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
DEST="$SCRIPT_DIR/../data"

# Define file names
KEY_FILE="$DEST/dummy.key"
CERT_FILE="$DEST/dummy.crt"
PFX_FILE="$DEST/dummy.pfx"
PASSWORD="your_password"

mkdir -p $DEST

# Generate a private key
openssl genrsa -out "$KEY_FILE" 2048

# Generate a self-signed certificate
openssl req -new -x509 -key "$KEY_FILE" -out "$CERT_FILE" -days 365 -subj "/C=US/ST=Dummy/L=Dummy/O=Dummy/CN=localhost"

# Generate a PKCS#12 file
openssl pkcs12 -export -out "$PFX_FILE" -inkey "$KEY_FILE" -in "$CERT_FILE" -passout pass:$PASSWORD

# Clean up - Remove key and cert files if you don't need them
rm "$KEY_FILE" "$CERT_FILE"

echo "Dummy SSL certificate generated in $PFX_FILE"
