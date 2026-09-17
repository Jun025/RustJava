# 2026-09-17 — Is each fixture broken in exactly one way?

> ★★**[교정 2026-09-17 · 게이트② request-changes · 승계 회차 `…-p0-fix`]**
> 이 문서가 적은 **「검사 18건 전건 single-defect」는 «측정»이 아니었다.**
> 판정식이 18건 중 **14건**에서 `repaired` 를 **정본 인자로 다시 지어** 둘째 결함을 버렸다
> ⇒ 그 14건은 **구조적으로 MULTI-DEFECT 를 낼 수 없었다**. ★**실측된 것은 4/18**(`indy` 근접실패)이다.
> ★승계 회차가 `repaired` 를 `given` 에서 파생시켜 **4족 전건**의 순환을 끊었고,
> 커밋된 18건은 **여전히 전건 통과**하지만 그것은 이제 «항진»이 아니라 «결과»다.
> 근거·개악 5종의 전/후 표는 `REPORT.md` 의 `…-p0-fix` 항목에 있다.

`taskId: rustjava-adopt-class-format-mutation-audit-p0` ·
adopts `2026-09-16-class-format-mutation-audit#p0`

The mutation audits so far asked one half of the question: *does each test fail when the thing it
names is broken?* This is the other half — **is each fixture broken in exactly one way?**

It matters because the failure is silent. A fixture that is wrong twice keeps its test green after
one of the two is repaired, and nothing anywhere says that the other stopped being covered.

## The test, and why byte equality rather than loading

The proposal suggested "remove the named defect and check the file now loads". Every fixture here
is produced by a generator that takes the defect **as a parameter**, which makes a stronger test
available at lower cost:

```
generator(name, …defect…)    == the committed fixture      (the generator still describes it)
generator(name, …repaired…)  == generator(name, …canonical…)   (and nothing else differs)
```

Stronger, because a second defect our loader happens not to care about would pass a loading test
and fail this one. Cheaper, because it needs no JVM — it is bytes against bytes.

## Result

```
검사 18건 · 결함 없음(유효 파일) 5건 · 합계 23건 · rc=0
```

**All 18 defective fixtures are single-defect.** ★**[교정] 이 문장은 «쓰인 시점에는 4/18 만 측정한 것»이었다** — 위 머리의 교정 블록 참조. 승계 회차가 판정식을 고친 뒤 같은 수가 다시 나왔고, 이제 그 수는 문장 그대로를 뜻한다. The five others carry no defect at all and are
listed rather than skipped, so the census stays complete:

| family | checked | axis examples |
|---|---|---|
| `indy` near misses | 6 | owning class · method name · descriptor · reference kind · static argument |
| `ldc` | 9 | `ldc2_w` width · illegal tags 13/14/19 · class file version · missing BootstrapMethods · static argument index · bootstrap index · duplicate table |
| `cp` | 3 | tag 13 / 14 / 19 |
| no defect | 5 | `MakeConcat`, `LdcMethodHandle`, `LdcMethodType`, `LdcDynamic`, `Ldc2WDynamic` — valid files whose feature is unimplemented |

This is the "row of already fine" the proposal predicted. It is still worth having: the discipline
had been applied per round and **never measured across the corpus**, so until now "all our fixtures
carry one defect" was a habit, not a fact.

## What it does not cover — said in the script, not only here

* **javac output** (`Lambda`, `StringConcat`, `ConstantKinds`, `LambdaKinds`) — not defective;
  "exactly one" does not apply to zero.
* **Legal-but-unimplemented** files — same; the file is valid and the feature is missing.
* **Fixtures byte-patched inside a test** — they never exist on disk, so this script cannot read
  them. Their single-defectness rests on the comment beside each, which is weaker, and that is the
  honest limit of this round.

## Mutations

| | mutation | result |
|---|---|---|
| **M1** | plant a second defect (`NotFactoryDescriptor` also gets reference kind 7) | **rc=1** · `MULTI-DEFECT — 수리 후에도 1 바이트 남는다` |
| **M2** | flip one byte of a committed fixture | **rc=2** · `GENERATOR DRIFT — 커밋본을 재현하지 못한다` |

M2 is the half that keeps the first equation honest: if the generator stops describing the
committed bytes, every verdict above is vacuous, and the script says so instead of passing.
