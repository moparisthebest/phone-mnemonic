#!/bin/bash
set -uo pipefail

# run this script to download all available jmp.chat numbers

# get some jmp.chat numbers to feed phone-mnemonic one of these two ways manually:

# curl 'https://jmp.chat/sp1a/register1/rc.php?state=ky&city=louisville' | grep -Eo '\([0-9]{3}\) [0-9]{3}-[0-9]{4}' | tr -dc '[0-9\n]' >> nums.txt

# curl 'https://jmp.chat/sp1a/register1/?areacode=289' | grep -Eo '[0-9]{11}' | sed 's/^1//' >> nums.txt

# rm -rf us rate-centers
mkdir -p us rate-centers

# get US area codes once:
[ ! -f areacodes-us.txt ] && curl -s https://www.worldatlas.com/na/us/area-codes.html | grep -Eo 'area-code-[0-9]{3}\.html' | grep -Eo '[0-9]{3}' | sort -un > areacodes-us.txt

# first get all by area code, but this API is limited to 5000 in a single response
while IFS="" read -r area_code || [ -n "$area_code" ]
do
  # this API has the leading 1, strip it off
  echo $area_code
  curl -s "https://jmp.chat/sp1a/register1/?areacode=$area_code" | grep -Eo '[0-9]{11}' | sed 's/^1//' | sort -un > "us/$area_code.txt"
  sleep 1
done < areacodes-us.txt

# get US states once:
[ ! -f states.txt ] && curl -s 'https://worldpopulationreview.com/static/states/abbr-list.csv' | tail -n+2 | tr -d '"' > states.txt

# now get all by rate center, grouped by state
while IFS="" read -r state || [ -n "$state" ]
do
    # get rate centers, but only once
    [ ! -f "rate-centers/$state.txt" ] && curl -s "https://jmp.chat/sp1a/register1/all_rc.php?state=$state" | grep -Eo '<Name>[^<]+</Name>' | sed -e 's@^<Name>@@' -e 's@</Name>$@@' | sort -u > "rate-centers/$state.txt" && sleep 1

    rm -f "us/$state.txt"

    while IFS="" read -r city || [ -n "$city" ]
    do
        echo "$state / $city"
        # this API does not have the trailing 1, but uses a different format, only keep numbers
        curl -s "https://jmp.chat/sp1a/register1/rc.php?state=$state&city=$city" | grep -Eo '\([0-9]{3}\) [0-9]{3}-[0-9]{4}' | tr -dc '[0-9\n]' >> "us/$state.txt"
        sleep 1
    done < "rate-centers/$state.txt"

    # de-duplicate
    [ -f "us/$state.txt" ] && sort -un "us/$state.txt" | sponge "us/$state.txt"

done < states.txt