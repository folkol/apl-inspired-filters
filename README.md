# APL inspired Unix filters

Some APL functions reinterpeted as unix filters.

- monadic functions work on stdin
- dyadic functions work on stdin as RHS and `arguments` as LFS(!)
- singleton argument are extended to match the length of stdin
- no nesting
- only support vectors, for now
- in general, call them by the function name if only one Valence -- glyph name otherwise

## TODO

- support tabular / rank 2 arrays?
- extend multiple arguments to match ranks of stdin?
- read arrays from file with @argument?
- decide what multiple @arguments mean
- empty stdin but arguments present?
- consider naming (maximum / ceiling, etc)
- is empty stdin ok for dyadic functions? I think so?


- ~~+: Plus Sign, Conjugate, Plus~~
- ~~^: ,, And Lowest Common Multiple~~
- ~~!: Exclamation Mark, Factorial, Binomial~~
- ~~⌈: Upstile, Ceiling, Maximum~~
- ~~○: Circle, Pi Times, Circular~~
- ~~?: Question Mark, Roll, Deal~~
- ~~⊥: Up Tack Symbol,, Decode~~
- L: ,, Mix
- ~~×: Times Sign, Direction Signum, Times~~
- ~~÷: Divide Sign, Reciprocal, Divide~~
- ↓: Down Arrow, ,Drop
- ⊂: Left Shoe,Enclose,
- ~~⊤: Down Tack Symbol, ,Encode~~
- =: Equal Sign, ,Equal
- ~: Tilde,Not,Without
- ⍀: Slope Bar, ,Expand First
- \*: Star,Exponential,Power
- ⍷: Epsilon Underbar, ,Find
- ⌊: Downstile,Floor,Minimum
- ⍕: Thorn Symbol,Format Monadic,Format Dyadic
- ⍒: Grade Down,Grade Down Monadic,Grade Down Dyadic
- ⍋: Grade Up,Grade Up Monadic,Grade Up Dyadic
- ≥: Greater Than Or Equal To Sign, ,Greater Or Equal
- ⊢: Right Tack,Identity,Right
- ⍳: Iota,Index Generator,Index Of
- ⌷: Squad Symbol,Materialise,Index
- ∩: Up Shoe, ,Intersection
- ⍸: Iota Underbar,Where,Interval Index
- ⊣: Left Tack,Same,Left
- ≤: Less Than Or Equal To Sign, ,Less Or Equal
- ⍟: Log,Natural Logarithm,Logarithm
- |: Stile,Magnitude,Residue
- ≡: Equal Underbar Sign, ,Match
- ⌹: Domino,Matrix Inverse,Matrix Divide
- ∊: Epsilon, ,Membership
- -: Minus Sign,Negative,Subtract
- ⍲: Logical NAND Symbol, ,Nand
- ⊆: Left Shoe Underbar,Nest,
- ⍱: Logical NOR Symbol, ,Nor
- ≠: Not Equal To,Unique Mask,Not Equal
- ≢: Equal Underbar Slash,Tally,Not Match
- ∨: Logical Or, ,Or Greatest Common Divisor
- ⊃: Right Shoe, ,Pick
- ,: Comma,Ravel,
- ⍴: Rho,Shape,Reshape
- ⍪: Comma Bar,Table,
- ↑: Up Arrow, ,Take
- ⍉: Circle Backslash,Transpose Monadic,Transpose Dyadic
- ∪: Set Union,Unique,Union

## References

- tabular text as arrays
- https://aplwiki.com/wiki/Mnemonics
- https://aplwiki.com/wiki/Overview
- https://en.wikipedia.org/wiki/APL_syntax_and_symbols
- https://tryapl.org
- https://help.dyalog.com/18.2/index.htm
- https://www.jsoftware.com/papers/APLDictionary.htm

## primitive functions


    Name(s)	Notation	Meaning	Unicode code point
    Roll	?B	One integer selected randomly from the first B integers	U+003F ?
    Ceiling	⌈B	Least integer greater than or equal to B	U+2308 ⌈
    Floor	⌊B	Greatest integer less than or equal to B	U+230A ⌊
    Shape, Rho	⍴B	Number of components in each dimension of B	U+2374 ⍴
    Not, Tilde	∼B	Logical: ∼1 is 0, ∼0 is 1	U+223C ∼
    Absolute value	∣B	Magnitude of B	U+2223 ∣
    Index generator, Iota	⍳B	Vector of the first B integers	U+2373 ⍳
    Exponential	⋆B	e to the B power	U+22C6 ⋆
    Negation	−B	Changes sign of B	U+2212 −
    Conjugate	+B	The complex conjugate of B (real numbers are returned unchanged)	U+002B +
    Signum	×B	¯1 if B<0; 0 if B=0; 1 if B>0	U+00D7 ×
    Reciprocal	÷B	1 divided by B	U+00F7 ÷
    Ravel, Catenate, Laminate	,B	Reshapes B into a vector	U+002C ,
    Matrix inverse, Monadic Quad Divide	⌹B	Inverse of matrix B	U+2339 ⌹
    Pi times	○B	Multiply by π	U+25CB ○
    Logarithm	⍟B	Natural logarithm of B	U+235F ⍟
    Reversal	⌽B	Reverse elements of B along last axis	U+233D ⌽
    Reversal	⊖B	Reverse elements of B along first axis	U+2296 ⊖
    Grade up	⍋B	Indices of B which will arrange B in ascending order	U+234B ⍋
    Grade down	⍒B	Indices of B which will arrange B in descending order	U+2352 ⍒
    Execute	⍎B	Execute an APL expression	U+234E ⍎
    Monadic format	⍕B	A character representation of B	U+2355 ⍕
    Monadic transpose	⍉B	Reverse the axes of B	U+2349 ⍉
    Factorial	!B	Product of integers 1 to B	U+0021 !
    Depth	≡B	Nesting depth: 1 for un-nested array	U+2261 ≡
    Table	⍪B	Makes B into a table, a 2-dimensional array.	U+236A ⍪
