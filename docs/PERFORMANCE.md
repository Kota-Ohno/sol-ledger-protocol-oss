# Performance notes

## Rust verifier cache (2026-07-14)

400-event JSONL chainをrelease buildのCLIで検証し、同一process起動条件・同一fixtureで
3回ずつ測定しました。

| implementation | run 1 | run 2 | run 3 |
| --- | ---: | ---: | ---: |
| validator compiled per event | 0.12 s | 0.12 s | 0.12 s |
| process-wide compiled validator cache | 0.02 s | 0.02 s | 0.02 s |

最初のcold build/runは比較から除外しています。cacheはschema kindごとに一度だけ構築し、
検証結果や入力は保持しません。同時に、入力全体16 MiB・1行1 MiB・100,000 eventsの
上限を追加し、巨大入力を読む前にfail closedします。
