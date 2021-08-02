import requests
import pyjisho
import lxml.html as lh
from typing import List
import re

SERVER = "http://localhost:8765"
DECK = "日本語語彙"
MODEL = "japanese"


def adverb_pattern(word: str) -> str:
    if len(word) == 4:
        if word[0] == word[2] and word[1] == word[3]:
            return "ABAB"
        if word[3] == "と":
            if word[2] == "っ":
                if word[2] in ["ゅ", "ゃ"]:
                    return "Aっと"
                else:
                    return "ABっと"
            else:
                return "ABCと"
    if len(word) == 3 and word[1] == "っ":
        return "Aっと"


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


def add_note(word: pyjisho.SearchResultSingle, examples: str = ""):
    if len(word.jp.kanjis) > 0:
        front = "・".join(word.jp.kanjis)
        if word.jp.hiragana:
            front += f"[{word.jp.hiragana}]"
    else:
        front = word.jp.hiragana
    tags = word.hinshi
    if len(tags) == 0:
        # hinshi = input("品詞はなんですか？（名・動・形・形動・副・連語）").split(" ")
        tags = ["名"]
    if "副" in tags:
        pat = adverb_pattern(word.jp.hiragana)
        if pat:
            tags.append(pat)
    print(f"Tags: {tags}")
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
                    "tags": tags,
                }
            },
        },
    ).json()

    if not r["error"]:
        print(f"Successfully added {front}.")
    else:
        raise Exception(f"Failed to add {front}.")


def search_and_add(query):
    word = pyjisho.search_exact_interactive(query)
    if word:
        add_note(word, examples)
    else:
        print("Not found.")


if __name__ == "__main__":
    import os
    import sys

    os.chdir(os.path.dirname(__file__))
    if len(sys.argv) == 1:
        query = input("Query: ")
    else:
        query = sys.argv[1]
    # examples = "\n".join(sys.argv[2:]) if len(sys.argv) > 2 else ""
    while True:
        search_and_add(query)
        query = input("Query: (skip to abort): ")