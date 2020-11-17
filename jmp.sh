#!/bin/bash

# get some jmp.chat numbers to feed phone-mnemonic one of these two ways:

curl 'https://jmp.chat/sp1a/register1/rc.php?state=ky&city=louisville' | grep -Eo '\([0-9]{3}\) [0-9]{3}-[0-9]{4}' | tr -dc '[0-9\n]' >> nums.txt

curl 'https://jmp.chat/sp1a/register1/?areacode=289' | grep -Eo '[0-9]{11}' >> nums.txt
