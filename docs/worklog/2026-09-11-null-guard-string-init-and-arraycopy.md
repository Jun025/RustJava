# 2026-09-11 — null 인자 9경로에 가드 (호스트 abort → NullPointerException)

채택 제안 `2026-09-11-s5-duplicate-issuance-stale-next-close#p0`.

`ClassInstanceRef::deref` 는 `self.instance.as_ref().unwrap()` 이라 null 이 닿으면 **Rust 패닉 = 호스트
프로세스 abort** 다. `String.<init>` 오버로드 10건 중 `([CII)`·`(II[C)` 만 가드가 있었고 `[B` 계열은 전부
뚫려 있었다 — ★**가드가 «짝이 안 맞는» 것**이 이 결함의 형태였다.

## 한 일
- 진입부 `is_null()` 가드 **9곳**(upstream 관용구 그대로):
  `string.rs` **7** — `([B)` · `([C)` · `([BII)` · `([BLjava/lang/String;)` · `([BIILjava/lang/String;)` ·
  `(Ljava/lang/String;)` · `(Ljava/lang/StringBuffer;)`
  `system.rs` **2** — `arraycopy` 의 `src` · `dest`
- 픽스처 `test-data/NullArgGuards.{java,class,txt}` — 9케이스, 전부 `caught NPE` 기대.

## 제안과 갈린 점
제안·STATE ③-2 는 **「8경로」**라 했는데 실측은 **9곳**이다. `System.arraycopy` 는 `src` 와 `dest` 가
**각각** 뚫려 있었고 그 표가 그것을 1건으로 셌다. ⇒ 인자 단위로 세면 9다.

## 검증 — 양방향
- **정상 → green**: `cargo test --test test_class` **ok** · DoD 7종 rc=0 · `cargo test --all` **554/0/1**.
  ★계수가 안 늘어난 것이 맞다 — `test_class` 는 `test-data/` 를 **한 테스트 함수**가 순회한다.
- **개악 → red**: 가드 전건 되돌리면 `test_class` **FAILED** —
  `called Option::unwrap() on a None value` at `jvm/src/class_instance.rs:108` ⇒ ★**픽스처가 공허하지 않다.**

## 픽스처 컴파일 메모 (다음 사람용)
- `javac --release 8` 로 친다 → major **52**, 기존 픽스처와 동일. ★`invokedynamic`·CP 태그 15~18 **0건**
  (javac 9+ 는 문자열 `+` 만으로도 invokedynamic 을 내는데 이 파서는 아직 못 읽는다 — STATE ④).
- ★**`test-data/` 안에서 컴파일하지 마라** — 그 디렉터리의 `Exception.class` 가 `java.lang.Exception` 을
  **가려** `incompatible types: Exception cannot be converted to Throwable` 로 죽는다(실측). 스크래치에서 쳐서 복사한다.
- 이 맥에는 시스템 JDK 가 없다. `/opt/homebrew/opt/openjdk/bin/javac`(26.0.1)를 썼다.

## 제안
- p0: 런타임 전체의 남은 `ClassInstanceRef` 직접 deref 진입점 감사 — 이 회차는 STATE ② 가 **열거한 범위만** 닫았다.
