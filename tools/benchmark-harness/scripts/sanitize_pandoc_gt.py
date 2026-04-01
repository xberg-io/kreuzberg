#!/usr/bin/env python3
"""Sanitize pandoc-generated markdown ground truth files.

Removes common pandoc artifacts that don't represent the actual document
structure. Run after `pandoc -f {format} -t gfm --wrap=none`:

    python sanitize_pandoc_gt.py input.md > output.md

Or pipe:
    pandoc -f docbook -t gfm --wrap=none input.xml | python sanitize_pandoc_gt.py > output.md
"""
import re
import sys


def sanitize(text: str) -> str:
    lines = text.split("\n")
    result = []
    i = 0

    while i < len(lines):
        line = lines[i]

        # Remove pandoc div wrappers (::: {.class})
        if re.match(r"^:::\s*(\{.*\})?\s*$", line):
            i += 1
            continue

        # Remove {.class} and {#id} attributes from headings
        line = re.sub(r"\s*\{[.#][^}]*\}\s*$", "", line)

        # Remove {.class} attributes from fenced code blocks
        line = re.sub(r"^(```)\s*\{[.][^}]*\}\s*$", r"\1", line)
        # Convert ``` {.python} to ```python
        m = re.match(r"^```\s*\{\s*\.(\w+)\s*\}\s*$", line)
        if m:
            line = f"```{m.group(1)}"

        # Fix escaped characters that pandoc over-escapes
        # Only at line start for list markers and auto-numbering
        if re.match(r"^\s*\\[*+-]\s", line):
            line = line.replace("\\*", "*", 1).replace("\\+", "+", 1).replace("\\-", "-", 1)
        if re.match(r"^\s*\\#\.", line):
            line = line.replace("\\#", "#", 1)

        # Remove <!-- end list --> pandoc markers
        if line.strip() == "<!-- end list -->":
            i += 1
            continue

        # Collapse 3+ consecutive blank lines to 2
        if line.strip() == "" and result and result[-1].strip() == "":
            blank_count = 1
            while result and result[-1].strip() == "":
                blank_count += 1
                if blank_count > 2:
                    result.pop()
            i += 1
            continue

        result.append(line)
        i += 1

    # Trim trailing blank lines, ensure single trailing newline
    while result and result[-1].strip() == "":
        result.pop()

    return "\n".join(result) + "\n" if result else ""


def main():
    if len(sys.argv) > 1:
        with open(sys.argv[1]) as f:
            text = f.read()
    else:
        text = sys.stdin.read()

    sys.stdout.write(sanitize(text))


if __name__ == "__main__":
    main()
