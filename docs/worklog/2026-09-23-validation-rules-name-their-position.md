# 2026-09-23 — validation rules name the position they stopped at

Ticket: `rustjava-2026-09-18-bootstrap-argument-index-and-tag-adopt-p0` · adopts `2026-09-18-bootstrap-argument-index-and-tag#p0`.

## Count (@origin/main `c654ae2e`)
The premise "15 rules, 14 bool" was off by one: `validate_class` has **14** rules, **13** returning a fixed sentence.

| # | rule | position in hand | asserted before |
|---|---|---|---|
| 1 | this_class does not name a class | N (one name) | 1 |
| 2 | super_class does not name a class | N (one name) | 0 |
| 3 | an interface entry does not name a class | Y interface index | 0 |
| 4 | a constant pool entry names a missing or wrong-kind entry | Y pool index | 1 |
| 5 | class file version does not support a constant tag | Y pool index | 1 |
| 6 | a dynamic constant names no bootstrap method | Y pool index | 0 |
| 7 | a single-valued class attribute appears more than once | Y attribute position (already `enumerate`d) | 1 |
| 8 | a field descriptor is malformed | Y field index | 0 |
| 9 | multiple ConstantValue attributes on a field | Y field index | 0 |
| 10 | a ConstantValue does not match its field descriptor | Y field index | 0 |
| 11 | a method descriptor is malformed | Y method index | 1 |
| 12 | an abstract or native method carries a Code attribute | Y method index | 0 |
| 13 | a method does not have exactly one Code attribute | Y method index | 1 |

(bootstrap-argument rule = the 14th, already structured by #73.)

## Where to stop
Stop criterion: **the enum grows by table kind, never by rule.** 11 rules index into only 5 tables,
so one `InvalidFormatAt { cause, location: Location }` carries all of them and the rule stays the
sentence it already was. A rule qualifies if it walks a table and the failing element's index is a
number a reader can find in the file. Rules 1–2 check one name and have nothing to point at — they
stay `InvalidFormat`. `ClassFileError` stays `Copy` and the same size (the existing size test passes unchanged).

Converted: **11**. Accept/reject boundary: predicates unchanged, only `all`/`any` → "first offender".

## Message
`<old sentence> (<table> #<n>)` — old sentence first so substring asserts and readers still match.
Pool index is the 1-based `#N` `javap -v` prints; other tables are zero-based like `InvalidBootstrapArgument`.
No file bytes (names, descriptors) are carried.

## Follow-up
- p0: the three parse-level refusals in `class.rs` were out of scope and are not counted.
