"""Wrap sqlx 0.9 dynamic SQL calls with AssertSqlSafe (paren-aware, string-aware)."""
import io
import os
import sys

MARKERS = [
    "sqlx::query(&format!(",
    "sqlx::query_as(&format!(",
    "sqlx::query_scalar(&format!(",
    "sqlx::raw_sql(&format!(",
    "sqlx::query(&catalog_sql(",
    "sqlx::query_as(&catalog_sql(",
]


def wrap_and_fix(content):
    out = []
    i = 0
    n = len(content)
    while i < n:
        matched = None
        for m in MARKERS:
            if content.startswith(m, i):
                matched = m
                break
        if matched is None:
            out.append(content[i])
            i += 1
            continue
        # matched ends with "&ident(" -> emit "sqlx::<fn>(sqlx::AssertSqlSafe(ident("
        prefix = matched[:-1].replace("&", "sqlx::AssertSqlSafe(", 1) + "("
        out.append(prefix)
        i += len(matched)
        depth = 1
        in_str = False
        j = i
        while j < n:
            c = content[j]
            if in_str:
                if c == "\\":
                    j += 2
                    continue
                if c == '"':
                    in_str = False
                j += 1
                continue
            if c == '"':
                in_str = True
            elif c == "(":
                depth += 1
            elif c == ")":
                depth -= 1
                if depth == 0:
                    out.append(content[i:j + 1])
                    out.append(")")
                    i = j + 1
                    break
            j += 1
        else:
            out.append(content[i:])
            i = n
    return "".join(out)


def process(path):
    content = io.open(path, encoding="utf-8").read()
    fixed = wrap_and_fix(content)
    if fixed != content:
        io.open(path, "w", encoding="utf-8", newline="\n").write(fixed)
        return True
    return False


def walk_rs(roots):
    for root in roots:
        if os.path.isfile(root):
            yield root
            continue
        for dirpath, dirnames, filenames in os.walk(root):
            dirnames[:] = [d for d in dirnames if d not in ("node_modules", "target", ".git", "generated")]
            for fn in filenames:
                if fn.endswith(".rs"):
                    yield os.path.join(dirpath, fn)

if __name__ == "__main__":
    total = 0
    for p in walk_rs(sys.argv[1:]):
        if process(p):
            total += 1
            print("fixed:", p)
    print(f"TOTAL: {total}")
