# Contributing to Kasane Logic

[🇯🇵 日本語版](#日本語版)

## Welcome!

We welcome all contributions including bug reports, feature suggestions, documentation fixes, and test additions.

## Development Procedure

1. **Fork this repository.**
2. **Clone the forked repository.**
3. **Create a working branch locally.**  
   - Branch name format: `<type>/<short-description>(-<issue-number>)` (include the issue number if one exists)  
   - Types:  
     - `feat` → New feature  
     - `fix` → Bug fix  
     - `docs` → Documentation update  
     - `refactor` → Code refactoring  
     - `chore` → Miscellaneous tasks (e.g., CI configuration, dependency updates)  
   - Example: `feat/add-user-login-123`
4. **Develop and commit.**  
   - Commit message: a short, descriptive summary
5. **Push to GitHub.**
6. **Create a Pull Request.**  
   - Title format: `[type] #<issue-number> short description` (include the issue number if one exists)  

## Testing Philosophy

This library prioritizes accuracy as its primary goal. Therefore, we maintain a comprehensive testing strategy:

- Tests are extensively developed to ensure maximum coverage
- When code improvements result in behavior changes, we discuss whether it's a bug or an intended modification
- Comprehensive testing enables detection of behavioral changes
- Tests can be executed with `cargo test`

## Performance Testing Philosophy

Performance benchmarks are designed to optimize function performance:

- Benchmarks can be executed with `cargo bench`
- Performance improvements are continuously pursued for all functions
- Testing is conducted using the Criterion framework
- Coverage for all functions is still in development

---

# 日本語版

## 歓迎！

バグ報告・機能提案・ドキュメント修正・テスト追加など、あらゆる貢献を歓迎します。

## 開発手順

1. **本リポジトリをforkする**
2. **forkしたリポジトリをcloneする**
3. **ローカルで作業用branchを作成**
   - ブランチ名は`<種別>/<短い説明>(-<issue番号>)`issue番号はissueが存在する場合
   - 種別
     - `feat` → 新機能
     - `fix` → バグ修正
     - `docs` → ドキュメント修正
     - `refactor` → リファクタリング
     - `chore` → 雑務（CI設定、依存パッケージ更新など）
   - 例:`feat/add-user-login-123`
4. **開発・commit**
   - commitメッセージは任意の短い説明
5. **GitHubにpush**
6. **Pull Request**
   - タイトルはブランチ名を参照し、`[種別] #<issue番号> 短い説明`issue番号はissueが存在する場合

## テストに関する考え方

このライブラリは正確性を第一として作成します。よって、テストは最大限充実する方針で整備します：

- コードを改善したときに以前のテストと異なる挙動になった場合にバグの発生なのか修正なのかを議論します
- とにかくたくさんのテストを充実させることで挙動が変わった場合に検知できるようになります
- `cargo test`で実行できます

## パフォーマンステストに関する考え方

関数のパフォーマンスが高くなるように改良を行います：

- `cargo bench`で実行できます
- criterion を用いてテストを行います
- まだ全ての関数を網羅できていません