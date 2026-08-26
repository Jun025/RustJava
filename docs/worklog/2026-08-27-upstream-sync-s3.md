# S3 — upstream 컷 `822504b` 머지 (오류 분류 축)

티켓 `rustjava-upstream-sync-s3`. 정본 = `docs/upstream-sync-approach.md` §5(7회차 분할 · 한 티켓 = 한 컷).

## 무엇을 했나
upstream `822504b`(#180 *Harden JVM runtime correctness*) **1커밋**을 머지하고 **충돌 11건**을 해소했다.
축은 계획서가 이름 붙인 「오류 분류 / PR #3」이다.

| 파일 | 처분 | 이유 |
|---|---|---|
| `classfile/src/{class,constant_pool,error,lib}.rs` | upstream 채택 | 우리 `ParseError`(Rust 변형 5) → upstream `ClassFileError`. **Java 예외가 1종 → 4종**으로 세분화된다 |
| `jvm_rust/src/class_definition.rs` · `src/runtime.rs` · `test_utils/src/lib.rs` | upstream 채택 | 위의 종속. `verifier::verify(&class)` 가 클래스 정의 시점에 들어온다 |
| `java_runtime/src/classes/java/lang/string.rs` | ★양쪽 병합 | upstream 골격 + 우리 `charset::Charset` 라우팅. upstream 의 `decode_str`/`encode_str` **중복 표 삭제** |
| `java_runtime/tests/classes/java/lang/test_string.rs` | 합집합 | 양쪽이 같은 자리에 다른 테스트를 넣었다 |
| `java_runtime/src/classes/java/lang/thread.rs` | ★양쪽 병합 | upstream 본문 + PR #4 의 수동 span. `#[tracing::instrument]` 재유입 0 |
| `AGENTS.md` | 합집합 | 우리 「Round Worklog」 절 ↔ upstream 「Testing Boundaries」 절 |

부수 1건 — `tests/test_class_format.rs`: **문구 단정 3건 삭제**(`"Truncated"`·`"tag 18"`·`"magic"`),
`ClassFormatError` **종류 단정은 유지**. 계획서 §4-A 가 예고한 「충돌 0으로 조용히 깨지는」 자리다.

## 실측
- 충돌 재측정 **11** — 계획서 예측 **+9** 에 2건 추가. ⑴`AGENTS.md`(계획서 이후 우리가 만든 절)
  ⑵★`thread.rs` — **S1 이 이미 닫은 파일이 다시 충돌했다**(`822504b` 가 같은 함수를 재작성).
- 조상 무손상: `git merge-base HEAD upstream/main` = `af4f6f8` ⇒ S2 가 했던 `-s ours` 복원 **불필요**.
  대신 **S2 브랜치 위에 쌓았다**(PR #13 미착지 · `main` 기준이면 S2 충돌 5건 재현).
- CI `rust.yml` 4종 **전건 rc=0** · `cargo test --all` **216 passed / 0 failed / 1 ignored**(S2 191 → +25).
- `tests/test_class_format.rs` **4/4** · `git grep 'tracing::instrument\|tracing-attributes'` **0건**.
- 우리 고유 줄 생존 대조: base 이후 우리가 추가한 `.rs` **321줄** 중 부재 **81줄**, 전건 의도한 해소
  (`ParseError` 기구 · `thread.rs` 구본문 · 완화한 문구 단정 3줄).

## 부수 — `STATE.md` 「다음」 절 갱신 (티켓 요구 0)
③-0 「`rustjava-pr8-claude-md-prune-disposition`」이 **이미 해소된 일**을 최우선으로 가리키고 있었다.
근거였던 두 사실이 모두 뒤집혔다: PR #8 은 `MERGED`(2026-08-18T19:26:08Z → `00bddf3`)이고
`reports/rustjava-claude-md-prune.review.md` 는 **실재**한다(승계 `-fix` 리니지 포함). ⇒ 항목을 닫고
순서를 재부여했으며, ⑤ 운영 메모의 「열린 PR 2건 · #8 좌초」 표도 현실(열린 PR = **#13 하나**)로 갈았다.
