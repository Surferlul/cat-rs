#!/usr/bin/env python3
import argparse
import subprocess
from pathlib import Path


def test_params(cat: Path, binary: Path, params: list[str], stdin: bytes = b"") -> None:
    outputs = []
    for executable in (cat, binary):
        outputs.append(b"")
        p = subprocess.Popen([executable, *params], stdin=subprocess.PIPE, stdout=subprocess.PIPE)  # noqa: S603
        outputs[-1] += p.communicate(input=stdin)[0]

    assert(outputs[0] == outputs[1])  # noqa: S101

def test_basic_usage(cat: Path, binary: Path) -> None:
    test_params(cat, binary, ["-", __file__, __file__], b"test")
    test_params(cat, binary, [__file__, "-", __file__], b"test")
    test_params(cat, binary, ["-", __file__, "-", __file__], b"test")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("-c", "--cat", help="gnu cat binary", type=Path)
    parser.add_argument("-b", "--binary", help="custom echo binary", type=Path)
    args = parser.parse_args()
    cat = args.cat
    binary = args.binary

    test_basic_usage(cat, binary)

    print("tests completed successfully")


if __name__ == "__main__":
    main()
