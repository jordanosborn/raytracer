#!/usr/local/bin/python3

import subprocess as sp
import sys

def main():
    print("Formatting code!")
    fmt = sp.check_output(["cargo", "fmt"])
    try:
        print("Linting code!")
        lint = sp.check_output(["cargo", "clippy", "--all-features", "--", "-D", "warnings"])
        print("Testing code!")
        test = sp.check_output(["cargo", "test"])
    except sp.CalledProcessError:
        print("Failed!")
    else:
        print("Commiting changes!")
        sp.call(["git", "add", "-u"])
        sp.call(["git", "commit", "-m", sys.argv[1]])
    

if __name__ == "__main__":
    main()