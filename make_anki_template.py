import convert_css

dict_template = ""

for dict_name in ["jp", "jp-cn", "jp-en"]:
    with open(f"css/{dict_name}.css") as f:
        css = f.read()
    dict_template += (
        '<iframe srcdoc="<html><body><head><style>'
        # + css
        + "</style></head><body>{{"
        + dict_name
        + '}}</body></html>"></iframe>\n\n<hr/>\n\n'
    )
template = (
    """\
<div class="front">
	{{furigana:Front}}
</div>

<hr id=answer>

{{Back}}

<hr id=examples>

<div class="examples">
	{{examples}}
</div>

<hr/>"""
    + dict_template
    + """\
<hr/>

<b>類語</b>

<div class="ruigo">
	{{furigana:類語}}
</div>

<hr/>
<b>反対語</b>

<div class="hantaigo">
	{{反対語}}
</div>"""
)

print(template)
