#!/usr/bin/env bash

function line() {
  echo "---------------------------------------------------------"
}


line
echo "Starting entrypoint.sh $(date)"
line

# Check if file exists if not throw exit code 1
if [ ! -f "/bot/name-bot" ]; then
  echo "File not found!"
  exit 1
else
  /bot/name-bot
fi



