#!/usr/local/bin/python3

import subprocess as sp
import sys

def main():
    print("Formatting code!")
    gitworkflow = [
        ["git", "add", "-u"],
        ["git", "commit", "-m", sys.argv[1]],
        ["git", "pull", "--rebase"],
        ["git", "push"]
    ]
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
        for cmd in gitworkflow:
            sp.call(cmd)
    

if __name__ == "__main__":
    main()