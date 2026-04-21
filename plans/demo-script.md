# Demo Script (2-3 perc)

## Celozott bemutatas

1. App inditas
- Parancs: `cargo run`
- Mutasd meg a 3 oszlopot (`Todo`, `Doing`, `Done`) es az alap mintakartyakat.

2. Uj kartya letrehozas
- Nyomd meg: `A`
- Add meg:
  - Title: `Beadando veglegesites`
  - Description: `README + tesztek + demo script`
  - Priority: `High` (ha kell, `P` vagy `Tab`)
- Enterekkel lepj tovabb es ments.
- Mutasd meg, hogy a kartya megjelent a kijelolt oszlopban.

3. Kartya mozgatasa Todo -> Doing -> Done
- Jelold ki az uj kartyat.
- Nyomd meg ketszer az `M` billentyut.
- Mutasd meg, hogy az oszlop valtozik: `Todo -> Doing -> Done`.

4. Kilepes es ujrainditas (persistencia)
- Kilepes: `Q`
- Ujrainditas: `cargo run`
- Ellenorizd, hogy a kartya es allapota ugyanugy latszik (fajlbol betoltve).

5. Hibas allapotfajl szimulacio + hibaturo indulas
- Kilepes utan nyisd meg a `data/board.json` fajlt, es irj bele ervenytelen JSON-t (pl. `{ invalid }`).
- Inditsd ujra: `cargo run`
- Mutasd meg a status sort: hibauzenet utan default boarddal indul az app, nem crashel.

## Zaras (20-30 mp)

- Emeld ki a kotelezo technikakat:
  - Higher Order Functions (`iter/filter/collect`)
  - `while let` event loop
  - Error handling (`Result`, sajat `AppError`, fallback indulasi logika)
- Mondd ki, hogy a projekt README alapjan kulso gepen reprodukalhato.
