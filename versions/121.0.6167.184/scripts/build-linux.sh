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
cd "$EXTRACTED" || exit

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
bindgen "$EVERYTHING_HEADER" --no-layout-tests --no-doc-comments --output "$BINDINGS" -- \
    -I "$INCLUDE/capi" \
    -I "$INCLUDE" \
    -I .

# Ignore various warnings.
TEMP=$(mktemp)
{
    echo "#[allow(non_camel_case_types)]"
    echo "#[allow(non_upper_case_globals)]"
    echo "#[allow(non_snake_case)]"
    cat "$BINDINGS"
} > "$TEMP" && mv "$TEMP" "$BINDINGS"

# Strip all debug symbols.
strip Release/*.so
strip Release/chrome-sandbox

# Pop back to the original directory.
popd || exit
