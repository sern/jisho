import zlib
import itertools
import pickle
from process_entry import *

# JP, JP_CN, JP_EN = [
#     "/System/Library/AssetsV2/com_apple_MobileAsset_DictionaryServices_dictionaryOSX/"
#     + p
#     + "/Contents/Resources/Body.data"
#     for p in [
#         "638773815497af8a6fcf3259489369d58f3f0e83.asset/AssetData/Sanseido Super Daijirin.dictionary",
#         "ada3657550da480622527a20c3c17343d1a9a420.asset/AssetData/Simplified Chinese - Japanese.dictionary",
#         "eda7c0d385d0ccb68e97b60251d63e9e2633b466.asset/AssetData/Sanseido The WISDOM English-Japanese Japanese-English Dictionary.dictionary",
#     ]
# ]
JP = "../raw/jp.data"
JP_CN = "../raw/jp-cn.data"
JP_EN = "../raw/jp-en.data"


def split_entries(input_bytes):
    # The first four bytes are always not UTF-8.
    input_bytes = input_bytes[4:]
    entries = []
    while True:
        # Find the next newline, which delimits the current entry.
        try:
            next_offset = input_bytes.index("\n".encode("utf-8"))
        except ValueError:  # No more new-lines -> no more entries!
            break
        entry_text = input_bytes[:next_offset].decode("utf-8")
        entries.append(entry_text)
        # There is always 4 bytes of chibberish between entries. Skip them
        # and the new lines (for a total of 5 bytes).
        input_bytes = input_bytes[next_offset + 5 :]
    return entries


def extract(path):
    entries = []
    with open(path, "rb") as f:
        content_bytes = f.read()[0x60:]
    while True:
        try:
            decompressor = zlib.decompressobj()
            chunk = decompressor.decompress(content_bytes[12:])
            content_bytes = decompressor.unused_data
            entries += split_entries(chunk)
            print(len(entries), len(content_bytes))
        except Exception as e:
            print(e)
            break
    return entries


if __name__ == "__main__":
    import os

    os.chdir("extract")
    if os.path.exists("jp_raw.pickle"):
        with open("jp_raw.pickle", "rb") as f:
            entries = pickle.load(f)
    else:
        entries = extract(JP)
        with open("jp_raw.pickle", "wb") as f:
            entries = pickle.dump(entries, f)
    process_jp(entries)

    if os.path.exists("jp-cn_raw.pickle"):
        with open("jp-cn_raw.pickle", "rb") as f:
            entries = pickle.load(f)
    else:
        entries = extract(JP_CN)
        with open("jp-cn_raw.pickle", "wb") as f:
            entries = pickle.dump(entries, f)
    process_jp_cn(entries)

    if os.path.exists("jp-en_raw.pickle"):
        with open("jp-en_raw.pickle", "rb") as f:
            entries = pickle.load(f)
    else:
        entries = extract(JP_EN)
        with open("jp-en_raw.pickle", "wb") as f:
            entries = pickle.dump(entries, f)
    process_jp_en(entries)


# with open("/Users/tianyishi/Projects/apple-dict/raw_extract/jp_raw.pickle", "wb") as f:
#     dic = pickle.dump(data, f)
