# Need to convert this to Rust soon :)
import os, sys

PREFIX = "https://github.com/krshrimali/rust-leetcode/blob/main/"
README_PREFIX = """# rust-leetcode

Solving Leetcode problems in Rust

"""

README_SUFFIX = """
Full disclosure: I was inspired by [UncleScientist](https://www.youtube.com/c/UncleScientist) to do it this way, specially the testing.
"""


def make_href(title, link):
    return "[" + title + "]" + "(" + link + ")"


def update_readme(debug: bool):
    with open("README.md", "w+") as readme_file:
        readme_file.write(README_PREFIX)
        count = 1
        with open("src/lib.rs") as lib_file:
            lines = lib_file.readlines()
            for line in lines:
                if line.startswith("pub mod"):
                    line = line.strip("pub mod ").strip("\n").strip(";")
                    filename = "src/" + line + ".rs"
                    if debug:
                        print("File: ", filename)
                    assert os.path.isfile(filename), f"{filename} doesn't exist..."
                    with open(filename) as code_file:
                        lines = code_file.readlines()[:2]
                        try:
                            title = (
                                lines[0].strip("//").strip("\n").split(" Title: ")[1]
                            )
                            link = lines[1].strip("//").strip("\n").split(" Link: ")[1]
                        except Exception as e:
                            msg = "Please look at other files to see the format. // Title: and // Link: as first 2 lines."
                            print(msg)
                            raise e
                    first_part = make_href(title, link)
                    second_part = make_href(f"`{filename}`", PREFIX + filename)
                    row = str(count) + ". " + first_part + ": " + second_part + "\n"
                    if debug:
                        print(f"Parsed parts: {first_part}, {second_part}")
                        print(f"Row: {row}")
                    readme_file.write(row)
                    count += 1
        readme_file.write(README_SUFFIX)


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(
        prog="ReadmeUpdater",
        description="Generates the README from src/lib.rs and src/*.rs files",
    )
    parser.add_argument("-debug", "--debug", default=False)
    args = parser.parse_args()
    update_readme(args.debug in ["True", "true", "TRUE"])
