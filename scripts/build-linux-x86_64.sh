#!/usr/bin/env bash

# Constants.
ARTIFACTS="../artifacts"
URL="https://cef-builds.spotifycdn.com/cef_binary_121.3.15%2Bg4d3b0b4%2Bchromium-121.0.6167.184_linux64_minimal.tar.bz2"
DECODED_URL=$(printf "$(echo -e "${URL//%/\\x}")")
FILENAME=$(basename $DECODED_URL)
EXTRACTED="${FILENAME%.tar.bz2}"
INCLUDE="include"
EVERYTHING_HEADER="everything.h"
BINDINGS="bindings.rs"

# Ensure artifacts directory exists.
mkdir -p "$ARTIFACTS"

# Move to the artifacts folder.
pushd "$ARTIFACTS" || exit

# Download if not already downloaded.
if [ ! -f "$FILENAME" ]; then
    curl -o "$FILENAME" "$URL"
fi

# Check if the directory has been extracted.
if [ ! -d "$EXTRACTED" ]; then
    tar -xvf "$FILENAME"
fi

# Move to the extracted directory.
pushd "$EXTRACTED" || exit

# Generate the everything header.
find "$INCLUDE/capi" -type f -name '*.h' ! -path '*/test/*' -print0 | \
    xargs -0 -I {} echo '#include "{}"' > "$EVERYTHING_HEADER"

# Add additional headers.
{
    echo '#include "include/cef_version.h"'
    echo '#include "include/internal/cef_logging_internal.h"'
    echo '#include "include/internal/cef_trace_event_internal.h"'
} >> "$EVERYTHING_HEADER"

# Generate the Rust bindings.
bindgen "$EVERYTHING_HEADER" \
    --no-layout-tests \
    --no-doc-comments \
    --default-enum-style=rust \
    --output "$BINDINGS" -- \
    -I "$INCLUDE/capi" \
    -I "$INCLUDE" \
    -I .

# Strip all debug symbols.
strip Release/*.so
strip Release/chrome-sandbox

# Pop back to the artifacts directory.
popd || exit

# Prepare the binaries for distribution.
mkdir -p cef
cp -r "$EXTRACTED"/Release cef/
cp -r "$EXTRACTED"/Resources cef/
tar -czvf cef-linux-x86_64.tar.gz cef/

# Move back to the original directory.
popd || exit

# Copy the final bindings to the correct location.
cp "$ARTIFACTS/$EXTRACTED/$BINDINGS" "../crates/bindings-linux-x86_64/src"
