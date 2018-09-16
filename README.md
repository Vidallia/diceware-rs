## diceware-rs

#### Generate Diceware passphrases in the console quickly.

This program is functional but, not complete.
It does provide the correct output though.

#### Usage:

````
USAGE:
    fileio-prac [OPTIONS] [AMOUNT]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <FILE>     The generated file's name that will contain the generated passphrases.
        --len <LENGTH>    The amount of unique words in the passphrase [default: 3]

ARGS:
    <AMOUNT>    Sets the number of passphrases generated [default: 1]
````    


#### Example Output:

*Command:*
````
diceware-rs 3 --len 3
````

*Produces:*
````
Generated Phrase(s):
+------------------------------+
|  shiftless hypocrisy barber  |
+------------------------------+
|  frisk prepay nervy          |
+------------------------------+
|  wooing urology storable     |
+------------------------------+
````

The command to send the output to a file is just as easy, with ONLY the phrases being written, each on a new line

*Command:*
````
diceware-rs 3 --len -f out.txt
````

Check for the file in the same directory as the executable.
