# Kanban Lite (Rust TUI) - Iteracios megvalositasi terv

Projekt cel: konzolos TUI alkalmazas, ahol a feladatkartyak a `Todo`, `Doing`, `Done` oszlopok kozott mozgathatok, prioritas cimkevel es fajlba mentett allapottal.

Kotelezo technikak lefedese:
- Higher Order Functions: szures, rendezes, statisztika (`iter`, `filter`, `map`, `collect`).
- while let: fo event loop billentyu esemenyekkel.
- Error handling: fajl I/O, JSON parse, invalid user input kezelese.

## 0. Iteracio - Kornyezet es telepites

Cel: stabil Rust fejlesztoi kornyezet, futtathato ures projekt.

### 0.1 Szukseges eszkozok telepitese (Windows)
1. Rustup + toolchain:

```powershell
winget install Rustlang.Rustup
rustup default stable
rustup component add rustfmt clippy
```

2. Build eszkoztar ellenorzese:

```powershell
rustc --version
cargo --version
```

3. Opcionlis, de eros ajanlas:

```powershell
cargo install cargo-watch
```

### 0.2 Projekt inicializalas
1. Letrehozas:

```powershell
cargo new app-tui --bin
cd app-tui
```

2. Fuggosegek felvetele:

```powershell
cargo add ratatui crossterm
cargo add serde --features derive
cargo add serde_json
cargo add anyhow thiserror
```

3. Elso futas:

```powershell
cargo run
```

Kimenet: lefordul es lefut a kezdo projekt.

Elfogadasi kriterium:
- `cargo run` hiba nelkul megy.
- `cargo clippy` es `cargo fmt --check` nem dob kritikus hibat.

---

## 1. Iteracio - Alap architektura es domain modellek

Cel: tiszta, bovitheto kodszerkezet.

### 1.1 Modulstruktura
Javasolt szerkezet:
- `src/main.rs` - app inditas, hibakezeles.
- `src/app.rs` - globalis allapot, parancsok.
- `src/model.rs` - Card, Column, Priority, Board.
- `src/ui.rs` - ratatui rajzolas.
- `src/input.rs` - billentyu esemenyek mapelese commandokra.
- `src/storage.rs` - allapot mentes/betoltes JSON-be.
- `src/error.rs` - sajat hibak (`thiserror`).

### 1.2 Domain modellek
Szukseges adattipusok:
- `Priority`: Low, Medium, High.
- `Column`: Todo, Doing, Done.
- `Card`: id, title, description, priority, column.
- `Board`: `Vec<Card>`, selected column/index.

Kimenet: forditheto, ures de stabil adatmodell.

Elfogadasi kriterium:
- Modellek `serde` derive-okkal szerializalhatok.
- Van legalabb 3 hardcoded minta kartya.

---

## 2. Iteracio - TUI alapkepernyo es navigacio

Cel: lathato 3 oszlopos Kanban UI, billentyus mozgas.

### 2.1 Kepernyo felepites (ratatui)
- 3 oszlop (`Layout`) vizszintesen: Todo / Doing / Done.
- Kijelolt oszlop es kartya vizualis kiemelese.
- Alul rovid help sor: pl. `Arrows: mozgas`, `A: uj kartya`, `M: move`, `Q: kilepes`.

### 2.2 Event loop (`while let`)
- `while let Ok(event) = read_event()` mintara epulo loop.
- Navigacio:
	- bal/jobb: oszlop valtas,
	- fel/le: kartya valtas oszlopon belul,
	- `q`: kilepes.

### 2.3 Minimalis command kezeles
- Bemenet -> command enum -> app state modositas.
- UI redraw minden allapotvaltozas utan.

Kimenet: jol kezelheto, interaktiv alap TUI.

Elfogadasi kriterium:
- Nem omlik ossze ures oszlopnal sem.
- Kijeloles mindig valid indexen marad.

---

## 3. Iteracio - Kartyamuveletek (MVP funkciok)

Cel: valoban hasznalhato Kanban alkalmazas.

### 3.1 Uj kartya letrehozas
- Egyszeru input mod (modal/prompt-szeru megoldas):
	- title kotelezo,
	- description opcionalis,
	- priority valaszthato.

### 3.2 Kartya mozgatasa oszlopok kozott
- `m` gomb: Todo -> Doing -> Done korforgas.
- Alternativ: dedikalt bal/jobb mozgatok.

### 3.3 Kartya torlese
- `d` gomb: kijelolt kartya torlese, visszajelzessel.

### 3.4 Higher Order Functions hasznalat
Peldak:
- Csak adott oszlop kartya listaja: `iter().filter(...)`.
- Priority szerinti cimkeformatalas: `map(...)`.
- Megjelenitheto listava alakitas: `collect::<Vec<_>>()`.

Kimenet: teljes minimum feature set.

Elfogadasi kriterium:
- Uj kartya hozzaadas, mozgatas, torles stabilan mukodik.
- Nincs panic normal hasznalat mellett.

---

## 4. Iteracio - Tarolas es hibakezeles

Cel: app allapot fennmaradjon ujrainditas utan.

### 4.1 JSON alapu persistencia
- Betoltes indulaskor pl. `data/board.json` fajlbol.
- Mentes kilepeskor vagy minden modositas utan.

### 4.2 Robusztus error handling
- `Result<T, AppError>` mintat hasznalni.
- Kulon hibakategoriak:
	- fajl nem talalhato,
	- jogosultsagi problema,
	- hibas JSON,
	- validacios hiba (pl. ures title).
- Felhasznalobarat hibauzenet a status sorban.

### 4.3 Default allapot fallback
- Ha nincs fajl vagy serult: induljon default boarddal.
- Logika: ne alljon le a program csak azert, mert nincs mentett allapot.

Kimenet: megbizhato mentes/betoltes.

Elfogadasi kriterium:
- Kilepes utan ujranyitaskor a board allapota visszajon.
- Serult JSON eseten sem crashel az app.

---

## 5. Iteracio - Minosegi fejlesztes es polish

Cel: beadando minosegu felhasznaloi elmeny.

### 5.1 UX javitasok
- Szinkodolt prioritas tagek.
- Header statisztika: hany kartya van oszloponkent.
- Status sor: aktualis mod, utolso muvelet eredmenye.

### 5.2 Billentyuk dokumentalasa
- Beepitett help panel (`h` gomb).
- Konzekvens keymap.

### 5.3 Kodminoseg
- `cargo fmt`
- `cargo clippy -- -D warnings`
- Felesleges klonok es allocok minimalizalasa.

Kimenet: latvanyos, demozhato vegleges UI.

Elfogadasi kriterium:
- Kezelo felulet egyertelmu, gyorsan tanulhato.
- Linter es formatter tiszta.

---

## 6. Iteracio - Teszteles, dokumentacio, beadando csomag

Cel: atadhato, ellenorizheto projekt.

### 6.1 Teszteles
Egyszeru unit tesztek javasoltak:
- Kartya mozgatasi szabalyok.
- Ures oszlop viselkedes.
- Szerializacio/deszerializacio (`serde_json`).

### 6.2 README tartalom
Legyen benne:
- projekt cel,
- telepitesi lepesek,
- futtatas,
- billentyuk listaja,
- melyik kotelezo technikakat hogyan hasznalod.

### 6.3 Demo script (2-3 perc)
Demozas forgatokonyv:
1. App inditas.
2. Uj kartya letrehozas.
3. Kartya mozgatasa Todo -> Doing -> Done.
4. Kilepes es ujrainditas (persistencia bemutatasa).
5. Hibas allapotfajl szimulacio + hibaturo indulas.

Kimenet: beadashoz kesz, reprodukalhato projekt.

Elfogadasi kriterium:
- Kulso gepen is fut a README alapjan.
- A tanari kovetelmenyek kozul legalabb 2 technika egyertelmuen bizonyithato.

---

## Ajanlott idobontas (realista)
- Iteracio 0: 0.5 nap
- Iteracio 1: 0.5 nap
- Iteracio 2: 1 nap
- Iteracio 3: 1 nap
- Iteracio 4: 1 nap
- Iteracio 5: 0.5 nap
- Iteracio 6: 0.5 nap

Osszesen: kb. 5 nap, kenyelmes tempoban 1 het.

## Kockazatok es kezelesuk
- Kockazat: TUI event loop osszetettnek tunik.
	Kezeles: eloszor csak navigacio + kilepes, utana funkciok egyenkent.
- Kockazat: allapot index elszallas torles utan.
	Kezeles: minden modositas utan index clamp.
- Kockazat: JSON inkompatibilitas model valtozasnal.
	Kezeles: `Default` fallback, verziozhato root struktura.

## Definition of Done (vegso)
- Mukodo 3 oszlopos Kanban TUI.
- Kartya hozzaadas, torles, mozgatas mukodik.
- Persistencia van es hibaturo.
- Hasznalva van legalabb 2 kotelezo technika, kodban jol lathatoan.
- Dokumentalt futtatasi es hasznalati lepesek.
