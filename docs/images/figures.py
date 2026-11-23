# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (C) 2026 Agent-IX
"""filament-integration's documentation figures, rendered by the docs-figures skill.

Only the version-lock table is defined here. Its rows come from Cargo.lock, which
docs/images/figures.json cannot yet express (data-driven rows). The other figures are
defined in docs/images/figures.json and rendered by `make figures`.

Regenerate from the repository root with the agent-ix/dev-tools skill:

    python3 -m pip install fonttools
    python3 <dev-tools>/skills/docs-figures/scripts/render.py docs/images/figures.py

Add --check to confirm the committed SVGs are current. The version-lock table is
read from Cargo.lock, so the pinned revisions are never typed by hand.
"""

import re
from collections import OrderedDict
from pathlib import Path

from docs_figures import FONTS, SMALL, TITLE, Figure  # noqa: F401

STYLE = "ix-docs"

ROOT = Path(__file__).resolve().parents[2]

# ---- repository data ---------------------------------------------------------


def locked_repositories():
    """Every git repository Cargo.lock pins: (repo, pinned by, commit, crate count)."""
    lock = (ROOT / "Cargo.lock").read_text(encoding="utf-8")
    repos = OrderedDict()
    pattern = r'name = "([^"]+)"\nversion = "[^"]+"\nsource = "git\+https://github.com/agent-ix/([^?"]+)\?(branch|rev)=([^#"]+)#([0-9a-f]+)"'
    for crate, repo, kind, ref, commit in re.findall(pattern, lock):
        entry = repos.setdefault(repo, {"kind": kind, "commit": commit, "crates": []})
        entry["crates"].append(crate)
    rows = [(r, e["kind"], e["commit"][:10], len(e["crates"])) for r, e in repos.items()]
    return sorted(rows, key=lambda row: (-row[3], row[0]))


# ---- figures -----------------------------------------------------------------


def lock():
    """The version lock: every git repository pinned in Cargo.lock."""
    rows = locked_repositories()
    row_h = 34
    head = 34
    f = Figure(
        "lock",
        980,
        head + row_h * len(rows) + 8,
        "The version lock is Cargo.lock. It pins "
        + ", ".join(r for r, _, _, _ in rows)
        + " each to one exact commit, so every crate of a repository resolves to a single copy.",
    )
    cols = [0, 330, 520, 760]
    for x, s in zip(cols, ["REPOSITORY", "TRACKS", "PINNED COMMIT", "CRATES LOCKED"]):
        f.text(x + 12, 16, s, "muted", "mono", SMALL)
    for i, (repo, kind, commit, n) in enumerate(rows):
        y = head + i * row_h
        role = "ink"
        f.rect(0, y, 980, row_h - 6, role, fill=role != "ink")
        f.text(12, y + 21, repo, role, "title", TITLE)
        f.text(cols[1] + 12, y + 21, "branch main" if kind == "branch" else "fixed revision", "muted", "mono", SMALL + 1)
        f.text(cols[2] + 12, y + 21, commit, "model", "mono", SMALL + 1)
        f.text(cols[3] + 12, y + 21, str(n), "ink", "mono", SMALL + 1)
    return f


FIGURES = [lock]
