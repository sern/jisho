from jisho import *

JP = 0b00000001
JP_CN = 0b00000010
JP_EN = 0b00000100

if __name__ == "__main__":
    import os
    import sys

    query = sys.argv[1]
    results = search_exact_interactive(query)
    # search_exact(query, JP | JP_CN, JP_EN)
    # for jp_result in results.jp:
    #     print(jp_result.hiragana, jp_result.kanjis)
    # flattened = [e.definition for e in results.jp + results.jp_cn + results.jp_en]
    print(results.hinshi)
    flattened = [e.definition for e in [results.jp, results.jp_cn, results.jp_en]]
    HEAD = """<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Words</title>
        <link rel="stylesheet" href="style.css">
    </head>
    """

    html = HEAD + "<body>"
    for definition in flattened:
        html += definition
    html += "</body>"

    if html:
        import webbrowser
        import os

        out = os.path.join(os.path.dirname(__file__), "out/result.html")
        with open(out, "w") as f:
            f.write(html)
        webbrowser.open("file://" + out)
    else:
        print("no results found")