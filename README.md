# Kanban Lite (Rust TUI)

Konzolos (TUI) Kanban alkalmazas Rust nyelven. A feladatkartyak a `Todo`, `Doing`, `Done` oszlopok kozott mozgathatok, szerkeszthetook, torolhetook, es JSON fajlba mentve megmaradnak ujrainditas utan.

## Projekt cel

A projekt celja egy egyszeru, stabil, billentyuzettel kezelheto Kanban alkalmazas keszitese, amely:
- 3 oszlopos tablaval dolgozik (`Todo`, `Doing`, `Done`),
- kezeli a kartya prioritasat (`Low`, `Medium`, `High`),
- tamogatja a kartya letrehozast, mozgatast, torlest es szerkesztest,
- hibaturoen kezeli a betoltesi/mentesi problemakat.

## Telepitesi lepesek (Windows)

1. Rust telepitese:

```powershell
winget install Rustlang.Rustup
rustup default stable
rustup component add rustfmt clippy
```

2. Projekt mappa megnyitasa:

```powershell
cd app-TUI
```

3. Fuggosegek letoltese es build:

```powershell
cargo build
```

## Futtatas

Fejlesztoi futtatas:

```powershell
cargo run
```

Tesztek futtatasa:

```powershell
cargo test
```

Kodminoseg ellenorzes:

```powershell
cargo fmt --check
cargo clippy -- -D warnings
```

## Billentyuk listaja

Normal mod:
- `Arrow Left/Right`: oszlop valtas
- `Arrow Up/Down`: kivalasztott kartya valtas az aktualis oszlopban
- `A`: uj kartya letrehozas
- `E`: kivalasztott kartya szerkesztese
- `M`: kivalasztott kartya mozgatasa a kovetkezo oszlopba (`Todo -> Doing -> Done -> Todo`)
- `D`: kivalasztott kartya torlese
- `P`: kivalasztott kartya prioritasanak valtoztatasa
- `H`: help panel ki/be
- `Q`: kilepes

Input mod (kartya hozzaadas/szerkesztes):
- `Enter`: kovetkezo mezo / mentes
- `Esc`: megszakitas
- `Backspace`: karakter torlese
- `P` vagy `Tab`: prioritas valtas
- gepeles: szovegbevitel

## Tarolas (persistencia)

- A tabla allapota a `data/board.json` fajlban tarolodik.
- Minden allapotvaltozas utan automatikusan mentes tortenik.
- Indulaskor, ha a fajl hianyzik vagy hibas JSON-t tartalmaz, az alkalmazas default boarddal indul es status uzenetet ad.

## Kotelezo technikak igazolasa

1. Higher Order Functions:
- Kartya szures oszlop szerint: `iter().filter(...).collect()` a `Board::cards_in_column` metodusban.
- Statisztika oszloponkent: a UI fejlecben oszloponkenti darabszamok szamolasa iterator alapon.

2. `while let` hasznalat:
- A fo esemenykezelo ciklus `while let Ok(event) = event::read()` mintat hasznal a billentyuesemenyek feldolgozasara.

3. Error handling:
- Sajat hibatipus (`AppError`) kezeli a `FileNotFound`, `PermissionDenied`, JSON parse es validacios hibakat.
- A storage reteg `Result<T, AppError>` alapu, felhasznalobarat status uzenettel.

## Projekt szerkezet (roviden)

- `src/main.rs`: alkalmazas inditas, event loop, terminal setup/cleanup
- `src/app.rs`: alkalmazas allapot, command kezeles
- `src/model.rs`: domain modellek (`Board`, `Card`, `Column`, `Priority`)
- `src/input.rs`: billentyu -> command mapping
- `src/ui.rs`: ratatui rendereles
- `src/storage.rs`: JSON beolvasas/mentes, validacio
- `src/error.rs`: sajat hibakezeles

## Megjegyzes

A projekt UTF-8 szoveges fajlokat hasznal, de a kod es dokumentacio celzottan egyszeru, konnyen reprodukalhato futtatast tamogat beadando kornyezetben.
