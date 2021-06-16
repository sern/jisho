FALLBACK_FONT = "Noto Serif CJK JP"

MAP = [
    ("-apple-system-text-background", "white"),
    ("-apple-system-secondary-label", "rgba(0, 0, 0, 0.498)"),
    ("-apple-system-tertiary-label", "rgba(0, 0, 0, 0.26)"),
    ("font-family: -apple-system", f"font-family: 'Hiragino Sans', '{FALLBACK_FONT}'"),
]


def convert_css(input):
    with open(input) as f:
        content = f.read()
    for (s, d) in MAP:
        content = content.replace(s, d)
    content = content.replace('"', "'")
    return content


if __name__ == "__main__":
    import os

    os.chdir("css")
    for p in [p for p in os.listdir() if "_raw" in p]:
        out = convert_css(p)
        with open(p.replace("_raw", ""), "w") as f:
            f.write(out)
