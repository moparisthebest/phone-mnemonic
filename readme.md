phone-mnemonic
--------------

```
$ phone-mnemonic -h
usage: phone-mnemonic [options...]
Read phone numbers on stdin and write all possible mnemonics to stdout

 -h, --help                      print this usage text
 -r, --reverse                   convert mnemonic to phone number instead
 -o, --orig                      print mnemonic followed by space then phone number on
                                 each output line

Examples:

  phone-mnemonic <nums.txt | grep COOLNUM > coolnums.txt; convert nums.txt to mnemonics, look
                                                          for one containing the string COOLNUM
  phone-mnemonic -r -o < coolnums.txt > nums_to_get.txt;  reverse but keep original into a file
                                                          for easy lookup/number finding
```