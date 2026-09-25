## [2026-09-26] 람다 클래스의 리플렉션 노출 — 현 상태 유지로 결정 (rustjava-2026-09-17-link-lambdametafactory-reflection-visibility-decision)
- 무엇을: 제안 `2026-09-17-link-lambdametafactory#p1` 을 결정했다 — 람다 클래스는 지금처럼 일반 레지스트리에 등록되고 `getName`·(실행 후) `forName` 에 보인다. 선택지 3개와 대가·재개 조건은 `docs/lambda-class-reflection-visibility.md`. 제품 코드 변경 0.
- 왜: 차이에 닿는 wie 게스트 경로가 없다. `invokedynamic` 은 major ≥ 51 에서만 합법이고 `validation.rs` 가 그 미만을 거부하는데, wie 게스트 jar 는 major 47 · `LambdaMetafactory` 참조 0(참조 365건은 전부 데스크톱용 벤더 에뮬레이터 도구 jar).
- 사용자 영향: 없음. 사슬 다음 칸 `#p0`(어댑터)이 선행 조건 없이 열렸다(`docs/next.md`). 후속 제안 없음.
