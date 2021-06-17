import requests
import pyjisho
import lxml.html as lh
from typing import List

SERVER = "http://localhost:8765"
DECK = "日本語語彙"
MODEL = "japanese"


def hinshi(definition: str) -> List[str]:
    definition = lh.fromstring(definition)
    hinshi = [h.text_content() for h in definition.xpath('//span[@class="pos"]')]
    hinshi = [h if h[0] != "動" else "動" for h in hinshi]
    if hinshi == ["名", "形動"]:
        hinshi = ["形動"]
    return hinshi


def exists(word):
    r = requests.post(
        SERVER,
        json={
            "action": "findCards",
            "version": 6,
            "params": {"query": "Front:" + word},
        },
    ).json()
    return r["error"] != None


def add_note(word: pyjisho.SearchResultSingle, examples: str):
    if len(word.jp.kanjis) > 0:
        front = "・".join(word.jp.kanjis)
        if word.jp.hiragana:
            front += f"[{word.jp.hiragana}]"
    else:
        front = word.jp.hiragana
    if exists(front):
        print("Already exists.")
        return
    r = requests.post(
        SERVER,
        json={
            "action": "addNote",
            "version": 6,
            "params": {
                "note": {
                    "deckName": DECK,
                    "modelName": MODEL,
                    "fields": {
                        "Front": front,
                        "examples": examples,
                        "jp": word.jp.definition,
                        "jp-cn": word.jp_cn.definition,
                        "jp-en": word.jp_en.definition,
                    },
                    "tags": hinshi(word.jp.definition),
                }
            },
        },
    ).json()

    if not r["error"]:
        print(f"Successfully added {front}.")
    else:
        raise Exception(f"Failed to add {front}.")


if __name__ == "__main__":
    import os
    import sys
    from romkan import to_hiragana
    from re import match

    os.chdir(os.path.dirname(__file__))
    if len(sys.argv) == 1:
        raise Exception(f"Please provide a query, e.g. `{sys.argv[0]} sonaeru`")
    query = sys.argv[1]
    if match("^[a-zA-Z]+$", query):
        query = to_hiragana(query)
    examples = "\n".join(sys.argv[2:]) if len(sys.argv) > 2 else ""
    word = pyjisho.search_exact_interactive(query)
    add_note(word, examples)