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
    doc_dir = f"{cwd}/target/doc"
    doc_dest_dir = f"{cwd}/docs"
    print("Formatting code!")
    sp.check_output(["cargo", "fmt"])
    print("Generating docs!")
    sp.check_output(["cargo", "doc", "--no-deps", "--document-private-items"])
    copy_tree(doc_dir, doc_dest_dir, update=1)
    sp.check_output(["cargo", "bench"])
    try:
        print("Linting code!")
        sp.check_output(["cargo", "clippy", "--all-features", "--", "-D", "warnings"])
        print("Testing code!")
        sp.check_output(["cargo", "test"])
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

def bench(cwd: str, cargo:Dict[str, str]):
    sp.call(["cargo", "bench"] + sys.argv[2:])

if __name__ == "__main__":
    dispatch = {
        "commit": commit,
        "debug": debug,
        "run": run,
        "test": test,
        "fmt": fmt,
        "lint": lint,
        "doc": doc,
        "bench": bench
    }
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
        if sys.argv[1] in dispatch.keys():
            dispatch[sys.argv[1]](cwd, cargo)
        else:
            print("No valid arguments supplied!")
