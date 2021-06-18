import requests
import pyjisho
import lxml.html as lh
from typing import List

SERVER = "http://localhost:8765"
DECK = "日本語語彙"
MODEL = "japanese"


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
    hinshi = word.hinshi
    if len(hinshi) == 0:
        # hinshi = input("品詞はなんですか？（名・動・形・形動・副・連語）").split(" ")
        hinshi = ["名"]
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
                        "jp-cn": word.jp_cn.definition if word.jp_cn else "",
                        "jp-en": word.jp_en.definition if word.jp_en else "",
                    },
                    "tags": hinshi,
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

    os.chdir(os.path.dirname(__file__))
    if len(sys.argv) == 1:
        raise Exception(f"Please provide a query, e.g. `{sys.argv[0]} sonaeru`")
    query = sys.argv[1]
    examples = "\n".join(sys.argv[2:]) if len(sys.argv) > 2 else ""
    word = pyjisho.search_exact_interactive(query)
    if not word:
        raise Exception("Aborted")
    add_note(word, examples)