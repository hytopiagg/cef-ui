#!/usr/bin/env bash

# Constants.
ARTIFACTS="../artifacts"
URL="https://cef-builds.spotifycdn.com/cef_binary_121.3.15%2Bg4d3b0b4%2Bchromium-121.0.6167.184_macosarm64_minimal.tar.bz2"
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
    echo '#include "include/internal/cef_types_mac.h"'
    echo '#include "include/cef_sandbox_mac.h"'
} >> "$EVERYTHING_HEADER"

# Generate the Rust bindings.
bindgen "$EVERYTHING_HEADER" \
    --no-layout-tests \
    --no-doc-comments \
    --default-enum-style=rust \
    --constified-enum=cef_event_flags_t \
    --constified-enum=cef_touch_handle_state_flags_t \
    --constified-enum=cef_drag_operations_mask_t \
    --constified-enum=cef_cert_status_t \
    --constified-enum=cef_urlrequest_flags_t \
    --constified-enum=cef_context_menu_type_flags_t \
    --constified-enum=cef_context_menu_media_state_flags_t \
    --constified-enum=cef_context_menu_edit_state_flags_t \
    --constified-enum=cef_quick_menu_edit_state_flags_t \
    --output "$BINDINGS" -- \
    -I "$INCLUDE/capi" \
    -I "$INCLUDE" \
    -I .

# Pop back to the artifacts directory.
popd || exit

# Remove any previous binaries.
rm -rf cef cef-macos-arm64.tar.gz

# Prepare the binaries for distribution.
mkdir -p cef
cp -r "$EXTRACTED"/Release/* cef/
tar -czvf cef-macos-arm64.tar.gz cef/

# Move back to the original directory.
popd || exit

# Copy the final bindings to the correct location.
cp "$ARTIFACTS/$EXTRACTED/$BINDINGS" "../crates/cef-ui-bindings/src/macos_arm64"
