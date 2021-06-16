import lxml.html as lh


div = lh.fromstring(
    '<div xmlns:d="http://www.apple.com/DTDs/DictionaryService-1.0.rng" id="7073" d:title="𩺊" class="entry" lang="ja"><span class="hg x_xh0"><span d:prn="1" role="text" class="hw">あら <d:prn></d:prn></span><span class="pr t_アクセントG"><span class="xrg"><span class="xr"><a href="x-dictionary:r:fbm_AccentPatterns" title="アクセントの型"><span class="ph t_アクセント x_rr">2</span></a></span></span></span><span class="fg">【<span class="f"><span class="general-text">𩺊</span></span>】</span></span><span class="sg"><span class="se1 x_xd0"><span class="se2Group x_xd0"><span class="se2 x_xd0"><span class="se3 x_xd1"><span class="msDict t_first"><span d:def="1" role="text" class="df">スズキ目の海魚。全長1<span class="general-text">メートル</span>に達する。体形はスズキに似て，やや長く側扁し，口はとがって大きい。背は灰褐色で腹は白色。幼魚には口から尾に至る灰褐色の縦帯がある。冬が旬で美味。北海道以南からフィリピンまでのやや深海に分布。ホタ。スズキ。<d:def></d:def></span></span></span></span></span></span></span></div>'
)

import string
from lxml.etree import HTMLParser
from lxml.etree import XMLParser

# print(div.xpath('//span[@class="hw"]')[0].text)

div = lh.fromstring(
    '<div xmlns:d="http://www.apple.com/DTDs/DictionaryService-1.0.rng" id="1850" d:title="赤恥" class="entry" lang="ja"><span class="hg x_xh0"><span d:prn="1" role="text" class="hw">あかはじ <d:prn></d:prn></span><span class="vg x_weak"><span class="v"><span class="rf">―</span>はぢ </span></span><span class="pr t_アクセントG"><span class="xrg"><span class="xr"><a href="x-dictionary:r:fbm_AccentPatterns" title="アクセントの型"><span class="ph t_アクセント x_rr">0</span></a></span></span></span><span class="fg">【<span class="f">赤恥</span>】</span></span><span class="sg"><span class="se1 x_xd0"><span class="se2Group x_xd0"><span class="se2 x_xd0"><span class="se3 x_xd1"><span class="msDict t_first"><span d:def="1" role="text" class="df">ひどい恥。あかっぱじ。<d:def></d:def></span><span role="text" class="eg">「<span class="ex x_ni"><span class="rf">―</span>をかく</span>」</span></span></span></span></span></span></span></div>'
)

# print(div.xpath('//span[@class="hw"]'))
