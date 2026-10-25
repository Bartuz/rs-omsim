"""Charts for BENCHMARKS.md from omsim-bench output.

    uv run --with matplotlib python bench/plot.py OUTDIR LABEL=FILE.tsv [LABEL=FILE.tsv ...]

Each FILE.tsv is one `omsim-bench --out` run with an engine named `c` (the oracle) and
one named `rust`; the first file may hold more engines (compiler controls). Writes
light and dark SVGs of each chart to OUTDIR.
"""

import csv
import sys
from collections import defaultdict
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

THEMES = {
    "light": dict(
        surface="#fcfcfb", ink="#0b0b0b", muted="#52514e", grid="#e4e3df", neutral="#a3a29b",
        series=["#2a78d6", "#eb6834", "#1baf7a"],
    ),
    "dark": dict(
        surface="#1a1a19", ink="#f0efec", muted="#c3c2b7", grid="#383835", neutral="#6b6a64",
        series=["#3987e5", "#d95926", "#199e70"],
    ),
}


def load(path):
    """{engine: {solution: min_ns}}"""
    times = defaultdict(dict)
    with open(path, newline="") as f:
        for row in csv.DictReader(f, delimiter="\t"):
            times[row["engine"]][row["solution"]] = int(row["min_ns"])
    return times


def style(ax, t):
    ax.set_facecolor(t["surface"])
    for side in ("top", "right"):
        ax.spines[side].set_visible(False)
    for side in ("left", "bottom"):
        ax.spines[side].set_color(t["grid"])
    ax.tick_params(colors=t["muted"], labelsize=9)
    ax.grid(True, color=t["grid"], linewidth=0.6)
    ax.set_axisbelow(True)


def figure(t, width, height):
    fig, ax = plt.subplots(figsize=(width, height), dpi=100)
    fig.patch.set_facecolor(t["surface"])
    style(ax, t)
    return fig, ax


def title(fig, t, text, sub):
    fig.text(0.012, 0.97, text, color=t["ink"], fontsize=12, fontweight="bold", va="top")
    fig.text(0.012, 0.97 - 0.32 / fig.get_figheight(), sub, color=t["muted"], fontsize=9, va="top")


def totals_chart(corpora, t):
    """Total time of each engine relative to C as upstream builds it."""
    rows = [(f"{label}  ·  {len(c['c'])} solutions", c) for label, c in corpora]
    fig, ax = figure(t, 8, 1.5 + 0.5 * len(rows))
    ys = range(len(rows))[::-1]
    for y, (name, c) in zip(ys, rows):
        common = c["c"].keys() & c["rust"].keys()
        tc = sum(c["c"][s] for s in common) / 1e9
        tr = sum(c["rust"][s] for s in common) / 1e9
        speedup = tc / tr
        ax.barh(y, speedup, height=0.55, color=t["series"][0], edgecolor=t["surface"], linewidth=2)
        ax.text(speedup + 0.04, y, f"{speedup:.2f}×   C {tc:.1f} s → Rust {tr:.1f} s",
                va="center", color=t["ink"], fontsize=9)
    ax.axvline(1, color=t["muted"], linewidth=1)
    ax.set_yticks(list(ys), [name for name, _ in rows], color=t["ink"], fontsize=9)
    ax.set_xlim(0, max(3.2, ax.get_xlim()[1] * 1.45))
    ax.set_xlabel("speed-up over C (total time, higher is faster)", color=t["muted"], fontsize=9)
    ax.grid(axis="y", visible=False)
    fig.subplots_adjust(left=0.30, right=0.98, top=1 - 0.8 / fig.get_figheight(), bottom=0.6 / fig.get_figheight())
    title(fig, t, "Rust engine vs C, whole corpus", "both engines in one process, one harness; a solution's time is the best of 3–5 interleaved runs")
    return fig


def controls_chart(times, t):
    """C compiled other ways, to separate the engine from the compiler."""
    order = [e for e in times if e != "rust"] + ["rust"]
    common = set.intersection(*(set(times[e]) for e in order))
    base = sum(times["c"][s] for s in common)
    fig, ax = figure(t, 8, 1.5 + 0.45 * len(order))
    ys = range(len(order))[::-1]
    names = {"c": "C, gcc -O2 (upstream Makefile)", "rust": "Rust, cargo --release"}
    for y, e in zip(ys, order):
        total = sum(times[e][s] for s in common)
        color = t["series"][0] if e == "rust" else t["neutral"]
        ax.barh(y, base / total, height=0.55, color=color, edgecolor=t["surface"], linewidth=2)
        ax.text(base / total + 0.04, y, f"{base / total:.2f}×   {total / 1e9:.1f} s",
                va="center", color=t["ink"], fontsize=9)
    labels = [names.get(e, "C, " + e.removeprefix("c-").replace("-", " -")) for e in order]
    ax.set_yticks(list(ys), labels, color=t["ink"], fontsize=9)
    ax.axvline(1, color=t["muted"], linewidth=1)
    ax.set_xlim(0, max(3.2, ax.get_xlim()[1] * 1.45))
    ax.set_xlabel("speed-up over upstream C build (total time)", color=t["muted"], fontsize=9)
    ax.grid(axis="y", visible=False)
    fig.subplots_adjust(left=0.30, right=0.98, top=1 - 0.8 / fig.get_figheight(), bottom=0.6 / fig.get_figheight())
    title(fig, t, "Compiler controls on the omsim test corpus", "C built two more ways: the gap is the engine, not the compiler")
    return fig


def per_solution_chart(corpora, t):
    """Speed-up of each solution against its C time: one panel per corpus."""
    fig, axes = plt.subplots(1, len(corpora), figsize=(3.2 * len(corpora) + 0.6, 4.2), dpi=100, sharey=True)
    axes = [axes] if len(corpora) == 1 else list(axes)
    fig.patch.set_facecolor(t["surface"])
    lo, hi = 1, 1
    points = []
    for label, c in corpora:
        common = sorted(c["c"].keys() & c["rust"].keys())
        xs = [c["c"][s] / 1e6 for s in common]
        ys = [c["c"][s] / c["rust"][s] for s in common]
        hi = max(hi, max(ys))
        lo = min(lo, min(ys))
        points.append((label, xs, ys))
    for ax, (label, xs, ys) in zip(axes, points):
        style(ax, t)
        ax.set_xscale("log")
        ax.set_yscale("log")
        ax.scatter(xs, ys, s=4, color=t["series"][0], alpha=0.35, linewidths=0, rasterized=True)
        ax.axhline(1, color=t["muted"], linewidth=1)
        median = sorted(ys)[len(ys) // 2]
        slower = sum(y < 1 for y in ys)
        ax.set_title(f"{label}\nmedian {median:.2f}×, slower than C: {slower} of {len(ys):,}",
                     color=t["ink"], fontsize=9, loc="left")
        ax.set_xlabel("C time per solution (ms)", color=t["muted"], fontsize=9)
        ax.set_ylim(lo * 0.8, hi * 1.25)
    axes[0].set_ylabel("speed-up over C (log)", color=t["muted"], fontsize=9)
    for ax in axes:
        ax.yaxis.set_major_formatter(matplotlib.ticker.FuncFormatter(lambda v, _: f"{v:g}×"))
        ax.xaxis.set_major_formatter(matplotlib.ticker.FuncFormatter(lambda v, _: f"{v:g}"))
    fig.subplots_adjust(left=0.09, right=0.98, top=0.75, bottom=0.13, wspace=0.08)
    title(fig, t, "Per solution: where the speed-up comes from", "each dot is one solution; above the line Rust is faster")
    return fig


def distribution_chart(corpora, t):
    """Share of solutions at or above each speed-up, one line per corpus."""
    fig, ax = figure(t, 8, 3.6)
    ax.set_xscale("log")
    at_least_2 = []
    for i, (label, c) in enumerate(corpora):
        common = c["c"].keys() & c["rust"].keys()
        ys = sorted(c["c"][s] / c["rust"][s] for s in common)
        n = len(ys)
        share = [1 - k / n for k in range(n)]
        color = t["series"][i]
        ax.plot(ys, share, color=color, linewidth=2, label=label)
        k = next((k for k, y in enumerate(ys) if y >= 2), n)
        at_least_2.append(f"{label}: {1 - k / n:.0%}")
    ax.axvline(1, color=t["muted"], linewidth=1)
    ax.axvline(2, color=t["grid"], linewidth=1)
    ax.set_ylim(0, 1.02)
    ax.set_xlim(0.7, 20)
    ax.text(0.98, 0.55, "at least 2× faster:\n" + "\n".join(at_least_2), transform=ax.transAxes,
            ha="right", va="top", color=t["ink"], fontsize=9, linespacing=1.5)
    ax.yaxis.set_major_formatter(matplotlib.ticker.PercentFormatter(1))
    ax.xaxis.set_major_formatter(matplotlib.ticker.FuncFormatter(lambda v, _: f"{v:g}×"))
    ax.set_xlabel("speed-up over C", color=t["muted"], fontsize=9)
    ax.set_ylabel("solutions at least this fast", color=t["muted"], fontsize=9)
    legend = ax.legend(frameon=False, fontsize=9, loc="upper right")
    for text in legend.get_texts():
        text.set_color(t["ink"])
    fig.subplots_adjust(left=0.1, right=0.98, top=0.8, bottom=0.14)
    title(fig, t, "Distribution of per-solution speed-ups", "share of solutions whose Rust run is at least x times faster than C (axis cut at 20×)")
    return fig


def main():
    if len(sys.argv) < 3:
        sys.exit(__doc__)
    out = Path(sys.argv[1])
    out.mkdir(parents=True, exist_ok=True)
    corpora = []
    for spec in sys.argv[2:]:
        label, path = spec.split("=", 1)
        corpora.append((label, load(path)))
    plt.rcParams["svg.fonttype"] = "path"
    plt.rcParams["svg.hashsalt"] = "omsim"  # stable ids, so unchanged charts diff clean
    for theme, t in THEMES.items():
        suffix = "" if theme == "light" else "-dark"
        charts = {
            "speedup": totals_chart(corpora, t),
            "per-solution": per_solution_chart(corpora, t),
            "distribution": distribution_chart(corpora, t),
        }
        if len(corpora[0][1]) > 2:
            charts["compilers"] = controls_chart(corpora[0][1], t)
        for name, fig in charts.items():
            fig.savefig(out / f"{name}{suffix}.svg", facecolor=t["surface"], metadata={"Date": None})
            plt.close(fig)
    print(f"wrote {len(charts) * 2} charts to {out}")


if __name__ == "__main__":
    main()
