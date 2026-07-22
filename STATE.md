# STATE

## 진행중
- (없음)

## 완료
- [rustjava-runtime-time-todo-impl] RuntimeImpl 시간 API `todo!()` 3건 제거(now/sleep/yield) +
  test_utils `r#yield` 구현 + tokio `time` 피처 추가 + 회귀 잠금 픽스처(`test_data/TimeApi`).
  브랜치 `runtime-time-impl`, PR #2 게이트② 대기.
- [rustjava-classfile-parse-error-propagation] 클래스파일 파싱 실패를 패닉 대신
  `java.lang.ClassFormatError` 로 전파(절단/매직 불일치/미지원 상수풀 태그 구분).
  브랜치 `classfile-parse-error-propagation`, PR 게이트② 대기.

## 다음
- PR approve 후 머지, 브랜치 정리(`gh pr merge --delete-branch` → `git branch -D` → `git fetch --prune`)
- ★두 PR 모두 STATE.md/REPORT.md 를 추가하므로 나중에 머지되는 쪽에서 add/add 충돌 예상 —
  선행 PR 머지 후 후행 브랜치에 `git merge main` 하고 후행(superset) 내용 채택으로 해소.
- (범위 밖 잔여) `jvm_rust/src/interpreter.rs:629` `todo!()` — 별건 티켓 필요
