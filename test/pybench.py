from __future__ import print_function

import time
import timeit

from pygments.lexers.html import HtmlLexer

one_text = open('input/test.html').read()

for n in [1, 2, 3, 5, 10, 100]:
    text = one_text * n
    times = timeit.repeat(lambda: list(HtmlLexer().get_tokens(text)), repeat=200//n, number=1)
    print(n, 'times:', format(int(round(min(times)*1000000000)), ','), 'ns')

