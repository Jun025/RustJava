# 2026-09-12 — 남은 20곳 «규격 삼분» 후 ⒜ 갈래만 닫기

채택 제안 `2026-09-11-null-guard-audit-and-io-buffer-guards#p0`.

★**이 회차의 산출물은 «가드»가 아니라 «분류»다.** null 이 합법인 자리에 가드를 넣으면 규격 위반이고,
도달 불가에 넣으면 죽은 코드다.

## 0. 전수 — ★**내가 셌다**(그 수를 그대로 믿지 않았다)

`python3 scripts/audit-null-guards.py` @ `b50d7d0c`(= 이 PR 의 base) → **N 1,175 · M 34 · K 20**.
⇒ ★**20 이 맞다**(파일·심볼·인자 단위 전수는 아래 표).

## 1. ★분류 축을 «두 겹»으로 세웠다 — 그리고 한 겹이 이 저장소에서 무너졌다

⒜**규격이 NPE 를 요구** / ⒝**규격이 null 을 허용** / ⒞**도달 불가**.

★★**⒞ 판정 기준**(브리프 4: 「오늘의 호출부로만 판정하지 마라 · public API 면 도달 가능으로 보라」):
**«등재된 Java 클래스와 멤버가 public/protected 인가»**로 갈랐다 — 오늘 누가 부르는지는 보지 않았다.

★★**그런데 이 런타임은 접근 제어를 «강제하지 않는다»**(실측):
`IllegalAccessError`·접근 플래그 검사 **0건** · 게스트의 `java.*` 패키지 금지 **0건**
(이 repo 가 스스로 적어 둔 자인: 「access flags are not enforced, so bytecode can leave a negative here」).
⇒ ★**손으로 만든 바이트코드라면 package-private 멤버도 부를 수 있다.**
⇒ 그래서 ⒞ 를 **「도달 불가」가 아니라 ★«규격 적합 게스트 코드로는 도달 불가»**로 좁혀 읽어라 —
javac 가 그 호출을 **낼 수 없다**(클래스·멤버가 보이지 않는다). ★**그 한정을 숨기지 않는다.**

## 2. ★규격 근거 — «관측»으로 댔다 (OpenJDK 소스 열람 0)

`AGENTS.md` §Compatibility Sources 가 **OpenJDK 소스 참조를 금지**한다(`src.zip` 이 로컬에 있지만 **열지 않았다**).
허용된 축은 「public specifications, Javadocs, **observable behavior**」이고, 그중 **관측**을 썼다 —
**참조 JVM(OpenJDK 26.0.1)** 에 같은 호출을 넣어 무엇이 나오는지 쟀다.
★**대가**: 측정 판본이 **26** 이고 이 repo 의 타깃은 **8** 이다. null 인자 계약은 그 사이 바뀌지 않았다고 보지만,
★**그것은 가정이고 여기 적어 둔다**(반증되면 이 표의 ⒜/⒝ 가 흔들린다).

## 3. ★★전수표 — 20곳

### ⒜ 규격이 NPE 를 요구 (11)

| # | 자리(등재 서술자) | 참조 JVM 관측 | 우리 런타임 «착수 시» | 처분 |
|---|---|---|---|---|
| 1 | `DataInputStream.readUTF(Ljava/io/DataInput;)` `STATIC` | NPE | ★**호스트 abort** | **가드** |
| 2 | `FileInputStream.<init>(Ljava/io/File;)` `PUBLIC` | NPE | ★**abort** | **가드** |
| 3 | `FileOutputStream.<init>(Ljava/io/File;Z)` `PUBLIC` | NPE | ★**abort** | **가드** |
| 4 | `RandomAccessFile.<init>(Ljava/io/File;Ljava/lang/String;)` `PUBLIC` | NPE | ★**abort** | **가드** |
| 5 | `ZipEntry.<init>(Ljava/util/zip/ZipEntry;)` `PUBLIC` | NPE | ★**abort** | **가드** |
| 6 | `ZipFile.<init>(Ljava/io/File;)` `PUBLIC` | NPE | ★**abort** | **가드** |
| 7 | `ZipFile.getInputStream(Ljava/util/zip/ZipEntry;)` `PUBLIC` | NPE | 미관측(하단 ※) | **가드** |
| 8 | `JarURLConnection.<init>(Ljava/net/URL;)` `PROTECTED` | NPE | 미관측(하단 ※) | **가드** |
| 9 | `URLStreamHandler.setURL(…)` `PROTECTED` | NPE | 미관측(하단 ※) | **가드** |
| 10 | `Pattern.matches(Ljava/lang/String;Ljava/lang/CharSequence;)` `PUBLIC STATIC` | NPE | ★**이미 NPE** | ★**가드 안 넣음** |
| 11 | `Pattern.split(Ljava/lang/CharSequence;I)` `PUBLIC` | NPE | ★**이미 NPE** | ★**가드 안 넣음** |

★★**10·11 은 «감사의 오탐»이다** — `Pattern::matches` 는 `compile` 결과를 **같은 이름 `pattern` 으로 섀도잉**하고,
감사 도구는 **이름 기반 절차내 스캔**이라 그 섀도잉을 못 본다(도구 docstring 이 미리 선언한 한계 그대로).
★**규격 의무는 ⒜ 가 맞지만 런타임이 이미 충족**하므로 가드는 **죽은 코드**가 된다 ⇒ 넣지 않았다.
⇒ ★**「⒜ = 가드를 넣는다」가 아니다 — 「⒜ 이면서 «미충족»일 때만」 넣는다.**
★5·8·9 는 `PROTECTED` 지만 **public 클래스의 상속 가능 멤버**라 게스트가 서브클래스로 도달한다 ⇒ ⒜ 로 둔다.

### ⒝ 규격이 null 을 «허용» (1) — ★**가드를 넣으면 규격 위반이다**

| 자리 | 관측 | 근거 |
|---|---|---|
| `URL.<init>(Ljava/net/URL;Ljava/lang/String;Ljava/net/URLStreamHandler;)` 의 **`handler`** | ★**NO EXCEPTION** | 참조 JVM 에서 `new URL((URL)null,"http://a/b",(URLStreamHandler)null)` 이 **정상 반환**한다 — null handler 는 「기본 핸들러를 쓰라」는 뜻이다 |

★**손대지 않았다.** ※이 자리는 이 repo 구현이 그 파라미터를 **섀도잉해 무시**하므로 감사의 **오탐**이기도 하다.

### ⒞ 규격 적합 게스트 코드로는 도달 불가 (8) — ★**전부 non-public**

| 자리 | 클래스 플래그 | 멤버 플래그 | 왜 |
|---|---|---|---|
| `AbstractList$Itr`/`$ListItr` `<init>(Ljava/util/List;I)` | `empty()` | `empty()` | JDK 공개 규격에 **이 클래스·이 생성자가 없다** — javac 가 이 호출을 낼 수 없다 |
| `ArrayList$Itr.<init>(Ljava/util/List;I)` | `empty()` | `empty()` | 〃 |
| `LinkedList$ListItr.<init>(Ljava/util/LinkedList;I)` | `empty()` | `empty()` | 〃 |
| `Vector$Itr.<init>(Ljava/util/Vector;I)` | `empty()` | `empty()` | 〃 |
| `LinkedHashMap$Entry.onAccess(Ljava/util/HashMap;)` | `empty()` | `empty()` | 〃 |
| `LinkedHashMap$LinkedHashIterator.<init>(…)` | `ABSTRACT` | `empty()` | 〃 |
| `Matcher.<init>(Ljava/util/regex/Pattern;Ljava/lang/CharSequence;)` | `PUBLIC FINAL` | ★`empty()` | 클래스는 public 이나 **생성자가 package-private** — JDK 도 같다(`Pattern.matcher()` 로만 얻는다) |
| `org/rustjava/net/FileURLHandler.openConnection(Ljava/net/URL;)` | `empty()` | `PROTECTED` | ★**JDK 클래스가 아니다**(이 repo 고유) ⇒ **적용할 JDK 규격이 없다**. 호출자는 우리 URL 기구뿐 |

★**손대지 않았다** — 이유는 위 칸에 남겼다(다음 사람이 같은 20곳을 다시 훑지 않도록).

## 4. ⒜ 에 넣은 가드 9개 + 테스트

픽스처 `test-data/NullSpecGuards`(6케이스). ★**가드별 «없으면 실패하는» 시험을 «측정»했다** — 9곳을 하나씩 빼고 각각 빌드·실행:

| 가드 | 단일 제거 |
|---|---|
| `data_input_stream.read_utf_from_input` · `file_input_stream.init` · `file_output_stream.init_with_append` · `random_access_file.init_with_file` · `zip_entry.init_with_zip_entry` · `zip_file.init` | ★**red** (6) |
| ★`jar_url_connection.init` · `url_stream_handler.set_url` · `zip_file.get_input_stream` | **green** (3) |

★★**미커버 3 의 사유 — 「테스트를 안 썼다」가 아니라 「픽스처로 도달할 수 없다」다**:
⒜`JarURLConnection`·`URLStreamHandler` 는 **`ABSTRACT`** 라 게스트 서브클래스가 필요하고,
⒝`ZipFile.getInputStream` 은 **살아 있는 `ZipFile` 인스턴스**가 있어야 하는데
★**이 런타임에 `ZipOutputStream` 이 없어**(실측: `java/util/zip/` = `ZipEntry`·`ZipException`·`ZipFile`·`ZipFileEntries`) 픽스처가 zip 을 **만들 수 없다**.
⇒ ★**가드는 규격 근거로 들어갔고, 잠금은 후속이 진다**(`proposals`).

## 5. ★반증 가능성 (DoD ⒡)

`Matcher.<init>` 을 ⒞ 로 분류했다. ★**이 분류가 틀렸다면 무엇이 관측될까**:
**`javac` 로 컴파일한 게스트 코드가 `new Matcher(p, s)` 를 호출해 `NoSuchMethodError` 가 아니라 실행에 성공**하면 틀린 것이다
(그러면 그 자리는 도달 가능이고 ⒜/⒝ 로 재분류해야 한다). 오늘은 javac 가 **컴파일 자체를 거부**한다(생성자가 보이지 않는다).

## 6. 감사 수의 이동

`K` **20 → 11**(9 닫음). 남은 11 = ⒝ **1** + ⒞ **8** + ★**감사 오탐 2**(`Pattern` 2곳 — 런타임이 이미 규격을 충족한다).
★**즉 「K 11」을 «미해결 11」로 읽지 마라** — 규격상 닫을 것은 **0** 이다.

## 제안
- p0: `ZipOutputStream` 최소 구현 → `ZipFile.getInputStream` 가드 잠금(그리고 zip 왕복 테스트가 열린다).
- p1: 추상 클래스 서브클래싱 픽스처 → `JarURLConnection`·`URLStreamHandler` 가드 잠금.
