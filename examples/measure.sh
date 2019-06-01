#!/bin/bash

# bash implementation of what the rust program does

time=$(date +%s)
url="http://localhost:8086/write?db=timelog&precision=s"
payload="bash,pwd=$PWD n=1 $time"
curl -XPOST "$url" --data-binary "$payload" --silent

