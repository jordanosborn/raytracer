#!/usr/local/bin/python3

import subprocess as sp
import sys, os
from typing import Dict
from distutils.dir_util import copy_tree

def commit(cwd: str, cargo: Dict[str, str]):
    gitworkflow = [
        ["git", "add", "-u"],
        ["git", "commit", "-m", sys.argv[2]],
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
    copy_tree(doc_dir, doc_dest_dir, update=1)
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
    
def debug(cwd: str, cargo: Dict[str, str]):
    sp.call(["cargo", "run"] + sys.argv[2:])

def run(cwd: str, cargo: Dict[str, str]):
    sp.call(["cargo", "run", "--release"] + sys.argv[2:])

def test(cwd: str, cargo: Dict[str, str]):
    sp.call(["cargo", "test"] + sys.argv[2:])

def fmt(cwd: str, cargo: Dict[str, str]):
    sp.call(["cargo", "fmt"] + sys.argv[2:])

def lint(cwd: str, cargo: Dict[str, str]):
    sp.call(["cargo", "clippy"] + sys.argv[2:])

def doc(cwd: str, cargo: Dict[str, str]):
    sp.call(["cargo", "doc"] + sys.argv[2:])

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
        if "commit" == sys.argv[1]:
            commit(cwd, cargo)
        elif "debug" == sys.argv[1]:
            debug(cwd, cargo)
        elif "run" == sys.argv[1]:
            run(cwd, cargo)
        elif "test" == sys.argv[1]:
            test(cwd, cargo)
        elif "fmt" == sys.argv[1]:
            fmt(cwd, cargo)
        elif "lint" == sys.argv[1]:
            lint(cwd, cargo)
        elif "doc" == sys.argv[1]:
            doc(cwd, cargo)

