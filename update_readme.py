# Need to convert this to Rust soon :)
import os

PREFIX = "https://github.com/krshrimali/rust-leetcode/blob/main/"
README_PREFIX = """# rust-leetcode

Solving Leetcode problems in Rust

"""

README_SUFFIX = """
Full disclosure: I was inspired by [UncleScientist](https://www.youtube.com/c/UncleScientist) to do it this way, specially the testing.
"""

def make_href(title, link):
    return '[' + title + ']' + '(' + link + ')'

with open("README.md", 'w+') as readme_file:
    readme_file.write(README_PREFIX)
    count = 1
    with open("src/lib.rs") as lib_file:
        lines = lib_file.readlines()
        for line in lines:
            if line.startswith("pub mod"):
                line = line.strip("pub mod ").strip('\n').strip(';')
                filename = "src/" + line + ".rs"
                print(filename)
                assert os.path.isfile(filename)
                with open(filename) as code_file:
                    lines = code_file.readlines()[:2]
                    title = lines[0].strip('//').strip("\n").split(" Title: ")[1]
                    link = lines[1].strip('//').strip("\n").split(" Link: ")[1]
                first_part = make_href(title, link)
                second_part = make_href(f"`{filename}`", PREFIX + filename)
                readme_file.write(str(count) + ". " + first_part + ": " + second_part + '\n')
                count += 1
    readme_file.write(README_SUFFIX)
