#!/usr/bin/env python
import argparse


def main():
    # what year is it!?
    parser = argparse.ArgumentParser()
    parser.add_argument("year")
    args = parser.parse_args()

    # setup values
    year = args.year
    days = list(range(1, 26))

    # print defaults
    print("default = [...")
    for i, day in enumerate(days):
        if i % 5 == 0:
            print("    ", end="")

        print("\"y{}d{}\",".format(str(year), str(day).rjust(2, "0")), end="")

        if i % 5 == 4:
            print()

    print("...]")
    print()

    # print individual features
    for day in days:
        print("y{}d{} = []".format(str(year), str(day).rjust(2, "0")))


if __name__ == "__main__":
    main()
