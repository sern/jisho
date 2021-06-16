import daijirin

HEAD = """<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Words</title>
    <link rel="stylesheet" href="style.css">
</head>
"""


def make_results(query: str):
    results = daijirin.search_all_starts_with(query)
    if len(results) == 0:
        return None
    html = HEAD + "<body>"
    for result in results:
        html += result.definition
    html += "</body>"
    return html


if __name__ == "__main__":
    import sys

    html = make_results(sys.argv[1])
    if html:
        import webbrowser
        import os

        out = os.path.join(os.path.dirname(__file__), "out/result.html")
        with open(out, "w") as f:
            f.write(html)
        webbrowser.open("file://" + out)
    else:
        print("no results found")
