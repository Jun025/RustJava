# 2026-09-21 — 추천 후속작업 **2건 기각** (rustjava-prune-declined-followup-proposals-2026-09-21)

운영자 지시(2026-09-21): 「주요도가 낮은 작업들은 미진행 … 불필요하거나 우선순위가 낮은 작업들은
추천 작업 목록에서 삭제해 정리해 달라」. ★**닫을 목록은 총괄이 골랐고 이 회차는 고르지 않았다.**

**제품 코드 0줄.** 판단을 기록하는 회차이고, 제안 본문을 다시 쓰는 회차가 아니다.

---

## 1. 무엇을 닫았고 무엇을 남겼나

| ref | 처분 | 무엇을 요구하는 제안인가 |
|---|---|---|
| `2026-09-19-nonliteral-blind-spot-is-reported-not-gated#p0` | ★**기각** | 예외 이름 보고에 «제품/테스트» 구분을 더하라 — 검사기 다듬기 |
| `2026-09-20-lock-script-output-order#p0` | ★**기각** | 출력순서 잠금의 «튜플 언패킹» 구멍을 메워라 — 검사기 다듬기 |
| `2026-09-20-string-array-hiding-overflows-stack#p0` | **남김** | `Jvm::exception` 에 바닥을 — 런타임 재귀 |
| `2026-09-20-name-the-missing-bootstrap-class#p0` | **남김** | `Jvm::new` 가 패닉해도 되는가 — 런타임 결정 |

★**「틀렸다」고 닫는 것이 아니다.** 둘 다 실측 위에 서 있고 둘 다 오늘 참이다.
닫는 이유는 하나다 — ★**둘 다 파이프라인이 자기 검사기를 다듬는 일이고, 사용자가 닿는 결함이 아니다.**

## 2. ★«선언»이지 «삭제»가 아니다

`proposals[]` 의 원소는 **하나도 건드리지 않았다.** ref 는 `<basename>#p<0-기반 인덱스>` 라
원소를 지우면 ★**뒤 원소의 ref 가 조용히 다른 제안을 가리킨다.**
⇒ 기각은 별 파일의 `declinedProposals[]` 에 **ref 를 적는 것**으로 한다.

★**이 서식은 이 repo 에 이미 있다** — `2026-09-17-ldc-asm-regeneration-declined.json` 이
2026-09-16 의 제안을 같은 방식으로 기각했다. 새 서식을 만들지 않았다.

## 3. ★되살리는 법 (한 줄)

> `docs/worklog/2026-09-21-prune-low-priority-followups.json` 의 `declinedProposals[]` 에서
> 그 ref 줄을 **빼면** 다음 스캔에서 다시 열린다. 제안 본문은 손대지 않았으므로 원상 그대로다.

## 4. 검증 — ★주장하지 않고 **셌다**

소비자의 파생식은 `open = 전체 − adopted − declined − injected − dismissed` 이고
(`~/tower/bin/cockpitd.js` 의 `collectProposals`/`scanRepoSimple`),
`injected`·`dismissed` 는 **tower 의 원장 두 파일**에 있다.
★**그래서 이 repo 만 세면 32 가 나오고 4 가 나오지 않는다** — 재현하려면 그 둘을 함께 읽어야 한다.

```sh
python3 - <<'PY'
import json,glob,os
inj=json.load(open(os.path.expanduser('~/tower/data/injected.json')))
dis=json.load(open(os.path.expanduser('~/tower/data/dismissed.json')))
refs=[];disp=set()
for f in sorted(glob.glob('docs/worklog/*.json')):
    d=json.load(open(f));b=os.path.basename(f)[:-5]
    for i,_ in enumerate(d.get('proposals',[])): refs.append(f"{b}#p{i}")
    disp.update(d.get('adoptedProposals',[]));disp.update(d.get('declinedProposals',[]))
op=[r for r in refs if r not in disp and r not in inj and r not in dis]
print("refs",len(refs),"open",len(op)); [print(" ",r) for r in op]
PY
```

| | refs(전체) | open | 열린 ref |
|---|---|---|---|
| 착수 전 | **99** | ★**4** | nonliteral#p0 · lock-script-output-order#p0 · string-array#p0 · name-the-missing#p0 |
| 착수 후 | ★**99**(불변) | ★**2** | ★**string-array#p0 · name-the-missing#p0** — 남기기로 한 바로 그 둘 |

★**닫는 쪽만 세지 않았다** — 남기는 둘이 **그대로 열려 있음**을 같은 명령으로 확인했다(과잉 차단 0).
★**refs 총수 99 불변**이 「인덱스가 밀리지 않았다」의 직접 증거다.

★**스키마 검사기는 이 repo 에 «있다»**: `scripts/check-worklog-json.py`(CI 잡 `worklog_json` + DoD 4번째 줄).
`declinedProposals` 의 원소가 `#p` 를 포함하는 ref 인지까지 본다 — 돌렸고 **rc 0**.

## 5. ★남는 빚 — 숨기지 않는다

- 기각을 **제안이 든 파일이 아니라 제3 파일**에 적는다(repo 관용구 그대로). ⇒ 제안의 워크로그만 연
  사람은 그것이 닫혔다는 표시를 **못 본다**. 둘을 잇는 것은 소비자뿐이다.
- 두 제안이 가리킨 사실은 **그대로 남는다** — 출력순서 잠금의 구멍은 그 docstring 에,
  제품/테스트 미구분은 그 보고에. 바뀐 것은 **보드에 올라와 있느냐**뿐이다.
- open 수가 **repo 밖 원장 2개에 의존한다**. 나중에 여기서만 다시 재면 **32** 가 나온다 —
  그래서 위에 파생식과 명령을 통째로 적었다.
