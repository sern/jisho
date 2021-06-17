import pickle
import lxml.etree as le
import lxml.html as lh
import re
import json
from lxml.etree import HTMLParser

BRACKETS_RE = re.compile("[《》〈〉]")

DIC_FILE = "/Users/tianyishi/Projects/apple-dict/スーパー大辞林_raw.pickle"


def json_dump_pretty(data, f):
    json.dump(data, f, ensure_ascii=False, indent=2)


def process_common(entry: str):
    return entry.replace("d:entry", "div")


def process_hiragana_lxml_result(hiragana: str):
    return (
        hiragana[0].text_content().replace("・", "").strip() if len(hiragana) > 0 else ""
    )


def process_jp(entries):
    print("Processing Japanese Dictionary...")
    entries = [process_jp_entry(e) for e in entries]
    with open("jp.json", "w") as f:
        json_dump_pretty(entries, f)


def process_jp_entry(entry: str):
    entry = process_common(entry)
    entry = entry.replace('<span class="gp">\u2008</span>', "")
    entry_x = lh.fromstring(entry)
    hiragana = process_hiragana_lxml_result(entry_x.xpath('//span[@class="hw"]'))
    kanjis = [
        BRACKETS_RE.sub("", kj.text_content().strip())
        for kj in entry_x.xpath('//span[@class="hg x_xh0"]//span[@class="f"]')
    ]
    # title = kanjis[0] if len(kanjis) > 0 else hiragana
    return (hiragana, kanjis, entry)


def process_jp_cn(entries):
    print("Processing Japanese-Chinese Dictionary...")
    MID = 68782
    cn_jp_entries = entries[:MID]
    jp_cn_entries = entries[MID:]
    cn_jp_entries = [process_cn_jp_entry(e) for e in cn_jp_entries]
    jp_cn_entries = [process_jp_cn_entry(e) for e in jp_cn_entries]
    with open("jp-cn.json", "w") as f:
        json_dump_pretty(jp_cn_entries, f)
    with open("cn-jp.json", "w") as f:
        json_dump_pretty(cn_jp_entries, f)


def process_jp_cn_entry(entry: str):
    entry = process_common(entry)
    entry_x = lh.fromstring(entry)
    hiragana = process_hiragana_lxml_result(entry_x.xpath('//span[@class="hw"]'))
    kanjis = [
        BRACKETS_RE.sub("", kj.text_content().strip())
        for kj in entry_x.xpath('//span[@class="hwg x_xh0"]//span[@class="hv t_kanji"]')
    ]
    return (hiragana, kanjis, entry)


def process_cn_jp_entry(entry: str):
    """'<div xmlns:d="http://www.apple.com/DTDs/DictionaryService-1.0.rng" id="z_crcjNEW006810-0000-J017" d:title="成功" class="entry"><span class="hwg x_xh0"><span d:dhw="1" role="text" class="hw">成功 </span><span class="pr"><span d:prn="1" d:pr="US solitary" soundFile="05000" class="ph ty_pinyin">chénggōng <d:prn></d:prn></span></span></span><span class="gramb x_xd0"><span d:pos="1" class="gr x_xdh">動詞 <d:pos></d:pos></span><span class="semb x_xd1"><span d:def="1" class="trg x_xd2"><span class="trans">成功する． </span><d:def></d:def></span><span class="exg x_xd2 hasSn"><span class="x_xdh x_xdh"><span class="sn">▸ </span><span class="ex">试验<span class="rf">成功</span>了 </span></span><span class="trg x_xd3"><span class="trans ty_通常">実験は成功した</span><span class="gp">. </span></span></span><span class="exg x_xd2 hasSn"><span class="x_xdh x_xdh"><span class="sn">▸ </span><span class="ex">得到<span class="rf">成功 </span></span></span><span class="trg x_xd3"><span class="trans ty_通常">成功を収める</span><span class="gp">. </span></span></span><span class="exg x_xd2 hasSn"><span class="x_xdh x_xdh"><span class="sn">▸ </span><span class="ex">大会开得很<span class="rf">成功 </span></span></span><span class="trg x_xd3"><span class="trans ty_通常">大会はとてもうまくいった</span><span class="gp">. </span></span></span><span class="exg x_xd2 hasSn"><span class="x_xdh x_xdh"><span class="sn">▸ </span><span class="ex"><span class="rf">成功</span>地解决 </span></span><span class="trg x_xd3"><span class="trans ty_通常">みごとに解決する</span><span class="gp">. </span></span></span><span class="synList x_xd2"><span class="lbl x_rr">同義語</span><span class="synGroup"><span class="syn"> 胜利 <span class="pr"><span d:pr="US solitary" class="ph ty_pinyin">shènglì </span></span></span></span></span><span class="antList x_xd2"><span class="lbl x_rr">反義語</span><span class="antGroup"><span class="ant"> 失败 <span class="pr"><span d:pr="US solitary" class="ph ty_pinyin">shībài </span></span></span></span></span></span></span></div>'"""
    entry = process_common(entry)
    cn = re.search(r'd:title="(.+?)"', entry)[1]
    return (cn, entry)


def process_jp_en(entries):
    print("Processing Japanese-English Dictionary...")
    MID = 47371
    en_jp_entries = entries[:MID]
    jp_en_entries = entries[MID:]
    en_jp_entries = [process_en_jp_entry(e) for e in en_jp_entries]
    jp_en_entries = [process_jp_en_entry(e) for e in jp_en_entries]
    with open("jp-en.json", "w") as f:
        json_dump_pretty(jp_en_entries, f)
    with open("en-jp.json", "w") as f:
        json_dump_pretty(en_jp_entries, f)


def process_jp_en_entry(entry: str):
    entry = process_common(entry)
    entry_x = lh.fromstring(entry)
    hiragana = process_hiragana_lxml_result(entry_x.xpath('//span[@class="hw"]'))
    kanjis = [
        BRACKETS_RE.sub("", kj.text_content().strip())
        for kj in entry_x.xpath('//span[@class="hwg x_xh0"]//span[@class="hv"]')
    ]
    return (hiragana, kanjis, entry)


def process_en_jp_entry(entry: str):
    entry = process_common(entry)
    en = cn = re.search(r'd:title="(.+?)"', entry)[1]
    return (en, entry)
