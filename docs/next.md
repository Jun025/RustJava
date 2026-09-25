# 다음

`STATE.md` `## 다음` 을 옮긴 것이다(2026-09-25 · `rustjava-report-state-md-per-round-files-port-from-wie`).
그 뒤로 `STATE.md` 는 동결됐다. 규칙은 `AGENTS.md` §Round Worklog 에 있다.

★**이 파일에는 카드가 표현하지 못하는 것만 적는다**: 선행 사슬과 카드 밖 항목(PR 번호). 열린 카드 목록은 적지 마라 — 아래 명령으로 세라.
옮기기 전 `STATE.md` 의 「순서 없음」 4줄 중 2줄은 이미 닫혀 있었다(`…bootstrap-argument-index-and-tag#p0`·`…unraisable-error-variant#p0`).
직접 적어 둔 목록은 이렇게 낡는다.

```sh
# 열린 카드 = proposals − adopted − declined (tower 의 «− injected» 술어가 아니다)
python3 -c "
import json,glob,os
refs=[];done=set()
for f in sorted(glob.glob('docs/worklog/*.json')):
    d=json.load(open(f));b=os.path.basename(f)[:-5]
    refs+=[f'{b}#p{i}' for i in range(len(d.get('proposals',[])))]
    done|=set(d.get('adoptedProposals',[]))|set(d.get('declinedProposals',[]))
print(*[r for r in refs if r not in done],sep='\n')"
```

1. **선행 사슬**: `2026-09-17-link-lambdametafactory#p1`(결정) → `#p0`(어댑터) → `java.lang.invoke` 패키지(카드 없음 · 근거 = `rustjava-runtime/src/classes/java/lang/invoke` **부재**) → `2026-09-17-string-concat-recipe-arity#p1`.
2. **카드 밖**: 없음. PR #81 은 2026-09-26 에 닫혔다 — #83(`750d30d4`)과 같은 변경이었다(`docs/worklog/2026-09-26-pr81-closed-duplicate-of-pr83.md`).

지난 판(⓪-사료·①~⑤)은 동결된 `STATE.md` `## 다음` 아래에 그대로 있다.
