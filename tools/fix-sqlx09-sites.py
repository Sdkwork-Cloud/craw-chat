"""Fix sqlx 0.9 dynamic-SQL call sites driven by cargo error locations.

Usage: python fix-sqlx09-sites.py <cargo-error-log> <repo-root>
"""
import io
import os
import re
import sys


def extract_sites(log_path, cwd):
    sites = []
    for line in io.open(log_path, encoding="utf-8", errors="replace"):
        m = re.match(r'\s*-->\s*([A-Za-z]:\\[^:]+\.rs|[^\\][^:]+\.rs):(\d+):(\d+)', line)
        if m:
            p = m.group(1)
            if not os.path.isabs(p):
                p = os.path.join(cwd, p)
            sites.append((p, int(m.group(2))))
    return sites


def fix_line(line):
    # sqlx::query(&ident) -> sqlx::query(sqlx::AssertSqlSafe(ident.as_str()))  (ident: String local)
    m = re.search(r'(sqlx::(query|query_as|query_scalar|raw_sql)\(&)([a-z_][a-z0-9_]*)(\))', line)
    if m:
        return line[:m.start(1)] + "sqlx::{}(sqlx::AssertSqlSafe({}.as_str()))".format(m.group(2), m.group(3)) + line[m.end(4):]
    # sqlx::query(ident) -> sqlx::query(sqlx::AssertSqlSafe(ident))  (ident: &str binding)
    m = re.search(r'(sqlx::(query|query_as|query_scalar|raw_sql)\(&?)([a-z_][a-z0-9_]*)(\))', line)
    if m:
        return line[:m.start(1)] + "sqlx::{}(sqlx::AssertSqlSafe({}))".format(m.group(2), m.group(3)) + line[m.end(4):]
    return line


def process(log_path, cwd):
    sites = extract_sites(log_path, cwd)
    print("sites: {}".format(len(sites)))
    # Cache file contents so multiple sites in one file are all applied.
    cache = {}
    for path, lineno in sites:
        if path not in cache:
            try:
                cache[path] = io.open(path, encoding="utf-8").read().splitlines(keepends=True)
            except OSError:
                continue
        lines = cache[path]
        if lineno - 1 >= len(lines):
            continue
        idx = lineno - 1
        new = fix_line(lines[idx])
        if new != lines[idx]:
            lines[idx] = new
    n = 0
    for path, lines in cache.items():
        content = "".join(lines)
        orig = io.open(path, encoding="utf-8").read()
        if content != orig:
            io.open(path, "w", encoding="utf-8", newline="\n").write(content)
            n += 1
            print("fixed file:", path)
    print("files: {}".format(n))


if __name__ == "__main__":
    process(sys.argv[1], sys.argv[2])
