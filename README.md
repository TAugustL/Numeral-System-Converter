<h1>Numeral System Converter</h1>
<p>I wrote this program as an exercise to get used to rust. It can convert decimal, hexadecimal, binary and octal numbers into one another.</p>

<h3>How to use:</h3>
<p>When starting the program you will be prompted to select the number type of your number (e.g. hex for hexadecimal):</p>

```
  ========================
  NUMERAL SYSTEM CONVERTER
  ========================
  Enter start number type:
  [dec] for decimal
  [hex] for hexadecimal
  [bin] for binary
  [oct] for octal

  hex
```

<p>Next you will be prompted to enter your number (e.g. f2).</p>

```
  Enter start value:

  f2
```
<p>Now the program will output the number converted to the other number systems.</p>

```
  —————————————————————————
  |Results for 0xf2:      |
  |                       |
  |Dec number: 242        |
  |Bin number: 0b11110010 |
  |Oct number: 0o362      |
  —————————————————————————
```

<hr>

<sub>Note: you must _not_ enter the prefixes for the number bases (0x for hexadecimal, 0b for binary, 0o for octal)</sub>
