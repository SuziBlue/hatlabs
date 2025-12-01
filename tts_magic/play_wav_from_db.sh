#!/bin/bash

# Usage: ./play_wav_from_db.sh <db_path> <row_id>
DB_PATH="$1"
ROW_ID="$2"
TMP_FILE="/tmp/output_$ROW_ID.wav"

if [ -z "$DB_PATH" ] || [ -z "$ROW_ID" ]; then
  echo "Usage: $0 <db_path> <row_id>"
  exit 1
fi

# Extract BLOB as hex
HEX_BLOB=$(sqlite3 "$DB_PATH" "SELECT hex(audio) FROM tts_entries WHERE id = $ROW_ID;")

if [ -z "$HEX_BLOB" ]; then
  echo "Error: No audio blob found for id $ROW_ID."
  exit 1
fi

# Convert hex to binary
echo "$HEX_BLOB" | xxd -r -p > "$TMP_FILE"

# Check file
file "$TMP_FILE"

# Play it
if command -v aplay &> /dev/null; then
  aplay "$TMP_FILE"
elif command -v afplay &> /dev/null; then
  afplay "$TMP_FILE"
elif command -v paplay &> /dev/null; then
  paplay "$TMP_FILE"
else
  echo "No supported audio player (aplay, afplay, paplay) found."
  exit 1
fi
