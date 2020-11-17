use std::io::{self, BufRead, BufWriter, Write};

const PHONE: &'static [&'static [u8]] = &[
    &[b'0'],
    &[b'1'],
    &[b'2', b'A', b'B', b'C'],
    &[b'3', b'D', b'E', b'F'],
    &[b'4', b'G', b'H', b'I'],
    &[b'5', b'J', b'K', b'L'],
    &[b'6', b'M', b'N', b'O'],
    &[b'7', b'P', b'Q', b'R', b'S'],
    &[b'8', b'T', b'U', b'V'],
    &[b'9', b'W', b'X', b'Y', b'Z'],
];

fn mnemonic_lookup() -> [u8; 91] {
    // would be neat if we could do this at compile-time instead, anyhow, it's fast
    // const fn is a good fit, but currently doesn't support for loops or mutable references
    // so I can't figure out how to do it
    let mut mnemonic = [0u8; b'Z' as usize + 1]; // 91 long
    for button in PHONE {
        for letter in button.iter() {
            mnemonic[*letter as usize] = button[0];
        }
    }
    // keep these the same
    mnemonic[b'\n' as usize] = b'\n';
    mnemonic[b'\r' as usize] = b'\r';
    mnemonic
}

fn recurse_mnemonic(line: &[u8], stdout: &mut dyn Write, orig: bool, name: &mut Vec<u8>, i: usize) -> io::Result<()> {
    let byte = line[i];
    if byte == b'\n' {

        if orig {
            name.push(b' ');
            stdout.write_all(&name)?;
            stdout.write_all(&line)?;
        } else {
            name.push(b'\n');
            stdout.write_all(&name)?;
        }
        name.pop();

        return Ok(());
    }

    /*
    let b: u8 = line[i];
    let c: char = b as char;

    let digit = c.to_digit(10);
    if digit.is_none() {
        return Err(io::Error::new(io::ErrorKind::Other, "digit is none"));
    }

    let digit = digit.unwrap();
    */
    // shortcut for above
    let digit = byte - 48;

    // make sure we are in range
    if digit > 9 {
        return Err(io::Error::new(io::ErrorKind::Other, "digit is > 9"));
    }

    let chars = PHONE[digit as usize];

    for char in chars {
        name.push(*char);
        recurse_mnemonic(line, stdout, orig, name, i + 1)?;
        name.pop();
    }
    Ok(())
}

fn print_phone(line: &mut [u8], stdout: &mut dyn Write, orig: bool, mnemonic: &[u8]) -> io::Result<()> {

    if orig {
        let i = line.len() - 1;
        line[i] = b' ';
        stdout.write_all(&line)?;
        line[i] = b'\n';
    }

    for char in line.iter_mut() {
        let index = *char as usize;
        // make sure we are in range
        if index > 90 { // b'Z' is 90
            return Err(io::Error::new(io::ErrorKind::Other, "index is > 90"));
        }
        *char = mnemonic[index];
        if *char == 0u8 {
            // default value, character not mapped
            return Err(io::Error::new(io::ErrorKind::Other, "invalid character"));
        }
    }

    stdout.write_all(&line)?;

    Ok(())
}

fn main() -> io::Result<()> {
    let mut reverse = false;
    let mut orig = false;
    for a in std::env::args().skip(1) {
        reverse |= &a == "-r" || &a == "--reverse";
        orig |= &a == "-o" || &a == "--orig";
        if &a == "-h" || &a == "--help" {
            eprintln!(r#"usage: phone-mnemonic [options...]
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
        "#);
            return Ok(());
        }
    }
    //eprintln!("reverse: {} orig: {}", reverse, orig);

    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut stdout = BufWriter::new(stdout);

    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    //let mut stdin = BufReader::new(stdin); // seems slower

    let mut buf = Vec::with_capacity(12);

    if !reverse {
        let mut name = buf.clone();

        loop {
            let num_bytes = stdin.read_until(b'\n', &mut buf)?;
            if num_bytes == 0 {
                break;
            }
            // .ok() ignores this result, which we want to do and just continue processing next line
            recurse_mnemonic(&buf[0..num_bytes], &mut stdout, orig, &mut name, 0).ok();
            buf.clear();
        }
    } else {
        let mnemonic = mnemonic_lookup();

        loop {
            let num_bytes = stdin.read_until(b'\n', &mut buf)?;
            if num_bytes == 0 {
                break;
            }
            // .ok() ignores this result, which we want to do and just continue processing next line
            print_phone(&mut buf[0..num_bytes], &mut stdout, orig, &mnemonic).ok();
            buf.clear();
        }
    }

    Ok(())
}
