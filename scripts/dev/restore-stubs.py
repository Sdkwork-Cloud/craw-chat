#!/usr/bin/env python3
"""Restore enterprise/recruitment/report stub pages from the original zip snapshot."""
import io, os, sys

orig = sys.argv[1] + r'\packages'
base = r'E:\sdkwork-space\sdkwork-im\apps\sdkwork-im-h5\packages'
RENAMES = [
    ('@sdkwork/clawchat-mobile-commons', '@sdkwork/im-h5-commons'),
    ('@sdkwork/clawchat-mobile-core', '@sdkwork/im-h5-core'),
    ('@sdkwork/clawchat-mobile-chat', '@sdkwork/im-h5-chat'),
    ('@sdkwork/clawchat-mobile-user', '@sdkwork/im-h5-user'),
    ('@sdkwork/clawchat-mobile-contacts', '@sdkwork/im-h5-contacts'),
    ('@sdkwork/clawchat-mobile-enterprise', '@sdkwork/im-h5-enterprise'),
    ('@sdkwork/clawchat-mobile-recruitment', '@sdkwork/im-h5-recruitment'),
    ('@sdkwork/clawchat-mobile-report', '@sdkwork/im-h5-report'),
    ('@sdkwork/clawchat-mobile-approval', '@sdkwork/im-h5-approval'),
    ('@sdkwork/clawchat-mobile-attendance', '@sdkwork/im-h5-attendance'),
    ('@sdkwork/clawchat-mobile-calendar', '@sdkwork/im-h5-calendar'),
    ('@sdkwork/clawchat-mobile-channels', '@sdkwork/im-h5-channels'),
    ('@sdkwork/clawchat-mobile-ai-writing', '@sdkwork/im-h5-ai-writing'),
    ('@sdkwork/clawchat-mobile-types', '@sdkwork/im-h5-types'),
]
JOBS = [
    ('sdkwork-clawchat-mobile-enterprise', 'sdkwork-im-h5-enterprise'),
    ('sdkwork-clawchat-mobile-recruitment', 'sdkwork-im-h5-recruitment'),
    ('sdkwork-clawchat-mobile-report', 'sdkwork-im-h5-report'),
]


def walk(d, out):
    for f in os.listdir(d):
        p = os.path.join(d, f)
        if os.path.isdir(p):
            if f not in ('node_modules', 'dist'):
                walk(p, out)
        elif f.endswith(('.ts', '.tsx')):
            out.append(p)


restored = 0
for orig_pkg, cur_pkg in JOBS:
    oroot = os.path.join(orig, orig_pkg, 'src')
    croot = os.path.join(base, cur_pkg, 'src')
    files = []
    walk(oroot, files)
    for op in files:
        rel = os.path.relpath(op, oroot)
        cp = os.path.join(croot, rel)
        if not os.path.exists(cp):
            continue
        cur = io.open(cp, encoding='utf-8').read()
        if ('CapabilityUnavailablePage' in cur or 'CapabilityUnavailable' in cur
                or 'UnavailableError' in cur or len(cur.splitlines()) < 25):
            content = io.open(op, encoding='utf-8').read()
            for a, b in RENAMES:
                content = content.replace(a, b)
            io.open(cp, 'w', encoding='utf-8', newline='').write(content)
            restored += 1
            print('restored', cur_pkg + '/' + rel.replace(os.sep, '/'))
print('total restored:', restored)
