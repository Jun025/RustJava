# STATE

## 진행중
- (없음)

## 완료
- [rustjava-runtime-time-todo-impl] RuntimeImpl 시간 API `todo!()` 3건 제거(now/sleep/yield) +
  test_utils `r#yield` 구현 + tokio `time` 피처 추가 + 회귀 잠금 픽스처(`test_data/TimeApi`).
  브랜치 `runtime-time-impl`, PR 게이트② 대기.

## 다음
- PR approve 후 머지, 브랜치 정리(`gh pr merge --delete-branch` → `git branch -D` → `git fetch --prune`)
- (범위 밖 잔여) `jvm_rust/src/interpreter.rs:629` `todo!()` — 별건 티켓 필요
