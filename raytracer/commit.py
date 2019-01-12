#!/usr/local/bin/python3

import subprocess as sp
import sys, os
from typing import Dict
from distutils.dir_util import copy_tree

def main(cwd: str, cargo: Dict[str, str]):
    gitworkflow = [
        ["git", "add", "-u"],
        ["git", "commit", "-m", sys.argv[1]],
        ["git", "pull", "--rebase"],
        ["git", "push"]
    ]
    package_name = cargo["package"]["name"]
    doc_dir = f"{cwd}/target/doc"
    doc_dest_dir = f"{cwd}/docs"
    print("Formatting code!")
    fmt = sp.check_output(["cargo", "fmt"])
    print("Generating docs!")
    docs = sp.check_output(["cargo", "doc", "--no-deps"])
    copy_tree(doc_dir, doc_dest_dir)
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
    cwd = os.getcwd()
    if not os.path.exists(f"{cwd}/Cargo.toml"):
        print("Not inside a crate!")
    else:
        with open(f"{cwd}/Cargo.toml", 'r') as f:
            cargo_lines = filter(lambda s: s != "", map(lambda x: x.strip("\n").strip(), f.readlines()))
        cargo = {}
        sublevel = None
        for l in cargo_lines:
            if l[0] == "[":
                sublevel = l[1:-1]
                cargo[sublevel] = {}
            elif sublevel is not None:
                split = list(map(lambda x: x.strip(), l.split("=")))
                cargo[sublevel][split[0]] = split[1].replace('"', '')
        main(cwd, cargo)

