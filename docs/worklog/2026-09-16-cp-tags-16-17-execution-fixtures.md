# 2026-09-16 — javac does emit condy; the fixture that proves the parser reads it

Ticket: `rustjava-cp-tags-16-17-execution-fixtures`.
Adopted proposal: `2026-09-16-cp-tags-15-18-parse#p2`.

## What the proposal actually asked for

Read it rather than the ticket title, because they are not the same question. The proposal's
`why`:

> 16·17 의 파싱 분기는 단위 테스트가 덮지만, 그 분기가 «실제 javac 산출물»에서 도는 것은 본 적이
> 없다. 오프셋 실수가 단위 테스트를 통과하고 실물에서만 드러나는 형태가 이 파서에서 가능하다.

So the deliverable is **real javac output whose constant pool carries tags 16 and 17**, not
hand-assembled bytes. Its `tradeoff` pre-registered the failure mode: "javac 가 16·17 을 내는
코드를 찾는 것이 일의 전부일 수 있고, 찾지 못하면 회차가 «못 찾았다» 로 끝난다."

**It did not end that way.** Both were found.

### Relationship to the sibling round (contract 1)

`rustjava-ldc-tags-15-16-17-still-malformed` ran first and its verdict is on a **different axis**:
it measured `ldc` *operands* and found javac emits none for tags 15/16/17 over 1,252,714 `ldc`
sites. That verdict does not remove this ticket's premise, because this one is about tags
**present in the constant pool** — which that round's own report says happens ("태그 15·16 은 오직
부트스트랩 메서드 «인자»로만 등장한다"). Tag 17's *pool presence* was never measured there.
Tag 15 is that round's subject and is deliberately not asserted here.

## The finding: what makes javac emit a CONSTANT_Dynamic

Measured over OpenJDK 26's own `.jmod` files with an exact constant pool walk (no instruction
decoding, so no drift — 27,902 classes, 0 unparseable):

| tag | classes carrying it | entries |
|---|---|---|
| 16 MethodType | **1,747** | 8,434 |
| 17 Dynamic | ★ **1** | 3 |

The single class is `jdk/jpackage/internal/PackageBuilder`, and its Dynamic entries are
`Ljava/lang/Enum$EnumDesc;` and `Ljava/lang/constant/ClassDesc;` bootstrapped by
`ConstantBootstraps.invoke`. Working back from that to a source construct, and checking four
candidate shapes one at a time:

| shape | Dynamic entries |
|---|---|
| ★ `switch (Object o) { case Suit.HEARTS -> …; case Suit.SPADES -> …; default -> …; }` | **3** |
| `switch (Object o) { case Suit.HEARTS -> …; case String s -> …; default -> …; }` | 2 |
| plain enum switch `switch (Suit s) { case HEARTS -> …; }` | **0** |
| sealed-interface pattern switch | **0** |

**The trigger is a qualified enum constant as a case label on a non-enum selector** (JEP 441).
javac cannot encode the constant as an ordinary reference there, so it describes it symbolically
as an `Enum$EnumDesc` through a dynamically-computed constant.

The two shapes that produce **zero** matter as much as the one that produces three: a round that
tried "enum switch" and "pattern switch" — both of which sound like they should be it, and both
of which the previous round did try — concludes javac never emits condy. That is how the previous
round honestly reached "찾지 못했다".

## The fixture

`test-data/indy/ConstantKinds.java` → `ConstantKinds.class` (+ two nested classes), compiled with
`javac --release 21`. One file carries **all four** tags, because a class can obviously have both
a lambda and a qualified-enum switch:

| tag | entries | from |
|---|---|---|
| 15 MethodHandle | 7 | bootstrap method refs |
| 16 MethodType | 1 | `LambdaMetafactory.metafactory` bootstrap argument |
| 17 Dynamic | 3 | the qualified enum case labels |
| 18 InvokeDynamic | 3 | the lambda and the two switch call sites |

Ground truth: **OpenJDK 26.0.1 runs it to completion**, printing `h3`, rc=0. It is a working
program, not a shape contrived to hit a parser arm.

## Why the proposal's predicted failure mode is real — measured

The existing unit test `parses_method_handle_family_tags` feeds each tag to `parse_tagged`
**one at a time**, which fixes every operand width. It never touches `parse_all`'s slot
accounting, where the classic mistake is treating an entry as the two-slot kind that only `Long`
and `Double` are. So:

```
is_double_entry also matches Dynamic (or MethodType)
  parses_method_handle_family_tags ............................. ok       ← isolated: blind to it
  real_javac_output_carries_...through_a_whole_pool ............ FAILED
  test_class_carrying_every_method_handle_family_tag_... ....... FAILED
      java.lang.ClassFormatError: Invalid class file
```

Every later pool index shifts by one, the class stops parsing, and the user is told the file is
corrupt. This is exactly "오프셋 실수가 단위 테스트를 통과하고 실물에서만 드러나는 형태" — it was a
prediction, and it is now a measurement.

## Bidirectional, in output

| direction | state | whole suite |
|---|---|---|
| ⒜ | fixture tests **removed** + the `Dynamic` slot bug applied | **558 passed / 0 failed — green** |
| ⒝ | fixture tests **restored** + the same bug | **2 red**, at two layers |
| — | restored | 560 / 0 / 1 |

⒜ is the part worth reading twice: with these two tests gone, **nothing else in the repository
notices** that bug. The axis was not merely thinly covered, it was uncovered.

## What each assertion is bitten by

| claim | what bites it |
|---|---|
| a class with all four tags reports *unsupported*, not *corrupt* | the end-to-end test (`run_class`) |
| **the fixture actually contains tags 16 and 17** | the `constant_pool.rs` unit test's per-tag counts |
| a shifted pool is visible at all | the same test also pins that the last pool index is still a `Utf8` |
| the tags survive as a *sequence*, not just individually | `parse_all` end to end, which the isolated unit tests never reach |

The second row is the one that is easy to skip and should not be. Without it, a future JDK that
stopped emitting condy would leave the end-to-end test **green while asserting nothing** — the
fixture would quietly become a class with no tag 17 in it, and the round's whole point would
evaporate silently. That is the failure this ledger keeps naming.

## Stale claims — counted, not listed

```
ledger-grep -rn "실행 픽스처|단위 테스트만|태그 16·17|16·17" --include='*.md' .   →  10 lines
```

Of those, **2** mention this round's subject, and **neither is false**:

- `REPORT.md:21` and `docs/worklog/2026-09-16-cp-tags-15-18-parse.md:72` — the previous round's
  follow-up and prose, which say "javac 가 그 둘을 내는 평범한 코드를 **찾지 못했다**". That is a
  statement about what that round found, and it is accurate about that round.

So nothing was rewritten. The disposition is recorded where the convention puts it:
`adoptedProposals: ["2026-09-16-cp-tags-15-18-parse#p2"]` in this round's `.json`, which removes
the card. The remaining 8 hits are unrelated (`upstream-sync-approach.md`'s conflict counts
contain the digits `16·17`).

## Scope

- No runtime change: `verifier.rs` and `interpreter.rs` untouched, `BootstrapMethods` parsing
  untouched (that is the sibling L ticket's work), tag 15 not asserted (sibling round's subject).
- The only production-code change is a test module addition in `classfile/src/constant_pool.rs`.

## Numbers

- `cargo test --all`: **558 / 0 / 1 → 560 / 0 / 1** (+2, zero regressions).
- DoD 7 commands, rc=0 each.
