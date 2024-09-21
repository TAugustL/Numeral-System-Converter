<h1>Numeral System Converter</h1>
<p>I wrote this program as an exercise to get used to rust. It can convert numbers from base 2 up to base 36 into one another.</p>

<h3>How to use:</h3>
<p>When starting the program you will be prompted to enter 3 numbers: your start value, its current base and the target base:</p>

```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ NUMERAL SYSTEM CONVERTER ┃
┣━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃ Enter start number,      ┃
┃ start base and           ┃
┃ target base(s)           ┃
┃ (e.g. '4 6 16 7')        ┃
┣━━━━━━━━━━━━━━━━━━━━━━━━━━┛
┃ 100 14 4 3 2

```

<p>In this case: convert 100 from base 14 into the bases 4, 3 and 2.</p>
<p>Hint: the digits in the number bases go from 0-9, then a-z and then A-Z.</p>
<p>The program will now output the number converted to the other bases.</p>

```
┣━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ 100 from base 14 to:     ┃
┣╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍┫
┃ base 4: 3010             ┃
┠──────────────────────────┨
┃ base 3: 21021            ┃
┠──────────────────────────┨
┃ base 2: 11000100         ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

<hr>

<sub>Note: you must _not_ enter the prefixes for the number bases (0x for hexadecimal, 0b for binary, 0o for octal)</sub>
