#!/bin/bash

cd $(dirname "$0")

mkdir -p logs
mkdir -p state

DATE=$(date "+%Y_%m_%d")

date | tee -a logs/logs_$DATE.txt
/usr/local/bin/node ./src/index.js 2>&1 | tee -a logs/logs_$DATE.txt
