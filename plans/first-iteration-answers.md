# Rust alapok - valaszok az 1. iteracio kodjahoz

Ez a dokumentum a kerdeslistad minden pontjara valaszol, kifejezetten a projektedben szereplo kod alapjan.

## 1) pub = public?
Rovid valasz: igen, a pub annyit jelent, hogy publikus (lathato mas modulok szamara is).

Reszletesebben:
- Rustban alapbol majdnem minden privat.
- Ha egy struct, enum, fuggveny vagy mezo ele pub kerul, az kivulrol is elerheto (a modulhatarokon at).
- Gyakori valtozatok:
  - pub: mindenhol lathato
  - pub(crate): csak az aktualis crate-en belul lathato
  - pub(super): csak a szulo modul szamara lathato

## 2) _frame jeloles mit takar?
A valtozonev ele tett alahuzas (_) azt jelzi, hogy szandekosan lehet, hogy nem lesz hasznalva.

Pelda:
- fn render(_frame: &mut Frame<'_>, _app: &App)

Mi ertelme:
- A fordito nem ad warningot arra, hogy nincs hasznalva az adott parameter.
- Gyakori iteracios fejlesztesnel: mar most kialakitod a fuggveny alairasat, de a belso logika kesobb jon.

## 3) Mi a &mut jelentese?
Az & egy referencia (kolcsonzes), a mut pedig mutalhato (modosithato).

Tehat:
- &T: csak olvasasi referencia (immutable borrow)
- &mut T: modosithato referencia (mutable borrow)

Fontos szabaly:
- Egyszerre vagy tobb olvasasi referencia lehet, vagy egy darab modosithato referencia.
- Ez segit elkerulni adatversenyeket es bizonyos memoriahibakat.

## 4) Frame<'_> ez mi pontosan? Hogyan mukodik?
A Frame a ratatui konyvtar rajzolasi kontextusa (abba rajzolsz widgeteket az adott render korben).

A <'_> resz egy lifetime jeloles:
- A lifetime azt mondja meg, meddig ervenyes egy referencia.
- Az '_ jelenti: "ezt a lifetime-ot a fordito kovetkeztesse ki".

Tehat a Frame<'_> annyi, mint: egy olyan Frame, ami egy adott, implicit elettartamra kolcsonzott adatot tartalmaz.

## 5) _app: &App - Ez hogyan van?
Ez egy fuggveny parameter:
- _app: a parameter neve
- : &App: tipusa egy immutable referencia App-ra

Az alahuzas itt is azt jelzi, hogy lehet, hogy a parameter jelenleg nincs hasznalva.

## 6) crate az dependency? Mi a crate pontosan?
A crate a Rust alap egysege (forditasi/csomagolasi egyseg).

Ket tipusa lehet:
- bin crate: futtathato program (nalad ez az app-tui)
- lib crate: konyvtar

Dependency-k is crate-ek:
- pl. anyhow, serde_json, ratatui mind kulso crate-ek, amiket Cargo.toml-ban adtal hozza.

Tehat a crate NEM csak dependency. A sajat projekted maga is crate.

## 7) -> Result<Board, AppError> mit jelol? Promise?
Ez a fuggveny visszateresi tipusa.

Jelentese:
- Siker eseten Board
- Hiba eseten AppError

A Result egy enum:
- Ok(T)
- Err(E)

Ez nem Promise.
- Promise JavaScript async modell.
- Rustban async esetben Future van.
- A Result hibakezelesre szol (sync vagy async kodban is).

## 8) fs:: mit jelent?
A fs itt a std::fs modult jelenti (fajlrendszer muveletek), ami a standard library resze.

Peldaid:
- fs::read_to_string(...)
- fs::write(...)

A :: jel operatort namespace/path feloldasra hasznaljuk.

## 9) serde_json:: ez honnan jon?
A serde_json egy kulso crate (Cargo.toml-ban benne van dependencykent).

Ezt hasznalod JSON szerializacio/deszerializaciora:
- serde_json::from_str(...)
- serde_json::to_string_pretty(...)

A serde crate + derive feature adja a Serialize/Deserialize derive-okat, a serde_json pedig a konkret JSON formatot.

## 10) Ok(board) honnan jon? Mit csinal?
Az Ok a std::result::Result enum egyik variansa.

- Ok(board): sikeres eredmeny
- Err(valami_hiba): hibas eredmeny

A legtobb esetben a Result tipus prelude miatt automatikusan elerheto, ezert nem kell kulon importalni.

## 11) #[derive(...)] hogyan mukodik? Mik ezek?
Ez egy attributum, ami derive macrokat futtat.

Pelda:
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]

Mit general:
- Debug: debug kiiras lehetosege (pl. println! debug mod)
- Clone: explicit masolas (deep-copy jellegu a mezoktol fuggoen)
- Copy: implicit, olcso bit-szintu masolas (csak bizonyos tipuskombinacioknal lehet)
- Serialize: adat JSON-ba irhato (vagy mas formatba)
- Deserialize: adat JSON-bol visszaolvashato
- PartialEq: == es != osszehasonlitas lehetosege
- Eq: teljes ekvivalencia marker trait (PartialEq kiegeszitese)

Lenyeg:
- Nem "varazslat", hanem kodgenerator macro, ami trait implementaciokat hoz letre neked.

## 12) u64 mit jelent? Milyen hasonlo tipusok vannak?
u64 jelentese:
- u = unsigned (nem negativ)
- 64 = 64 bit

Leggyakoribb bepitett numerikus tipusok:
- Elojeles egeszek: i8, i16, i32, i64, i128, isize
- Elojel nelkuli egeszek: u8, u16, u32, u64, u128, usize
- Lebegopontos: f32, f64

Egyeb alapok:
- bool
- char
- str (szelet), String (dinamikus szoveg)
- tuple (pl. (i32, bool))
- array (pl. [i32; 3])
- slice (pl. &[i32])
- referencia (&T, &mut T)

## 13) struct mit takar?
A struct egy sajat, osszetett adattipus (rekord jellegu), mezonevekkel.

Pelda:
- Card struct: id, title, description, priority, column

Haszna:
- Osszetarto adatokat egy tipusba szervezel.
- Kesobb impl blokkban metodusokat adsz hozza.

## 14) usize mi pontosan?
A usize egy platformfuggo, elojel nelkuli egesz tipus, merete megegyezik a pointer meretevel.

- 64 bites rendszeren altalaban 64 bites
- 32 bites rendszeren altalaban 32 bites

Tipikusan indexelesre hasznaljuk (pl. vector index).

## 15) Vec<Card> mi ez pontosan?
A Vec<T> a Rust dinamikusan novekvo tombje (heapen tarolt lista).

- Vec<Card> = Card elemek listaja
- Olyan, mint mas nyelvekben a dynamic array / ArrayList

Muveletek:
- push, pop, iter, filter, map, collect, stb.

## 16) impl mit jelent? Hasonlit class-hoz?
Az impl blokkban adsz metodusokat egy tipushoz, vagy trait-et valositasz meg.

Ket forma:
- impl Type { ... }  -> inherens metodusok/asszocialt fuggvenyek
- impl Trait for Type { ... } -> trait implementacio

Class-hoz hasonlit annyiban, hogy tipushoz kapcsolsz metodusokat.
Kulonbseg:
- Rustban nincs klasszikus OOP class + inheritance modell.
- Trait-ekkel oldjuk meg a kozos viselkedest.

## 17) pub fn with_sample_cards() -> Self { ... }
Reszek:
- pub fn with_sample_cards(): publikus asszocialt fuggveny
- -> Self: a visszateresi tipus az aktualis tipus (itt Board)

Mi a Self:
- Self (nagy S): tipus alias az aktualis impl tipusra.
- self (kis s): a metodus aktualis peldanya (instance), ha parameterkent szerepel.

Miert jo a Self:
- Ha atnevezed a tipust, kevesebb helyen kell javitani.
- Olvashatobb trait implementaciokban.

## 18) vec![...] miert van felkialtojel?
A vec! egy macro, nem fuggveny.

Rustban a macrohivast ! jeloli:
- println!(...)
- vec![...]

A vec![a, b, c] letrehoz egy Vec-et a megadott elemekkel.

## 19) fn default() -> Self { Self::with_sample_cards() } hogy mukodik?
Ez a Default trait kotelezo metodusa.

- fn default() -> Self
  - azt mondja: adj vissza egy alapertelmezett peldanyt az adott tipusbol
- Self::with_sample_cards()
  - meghivja az asszocialt fuggvenyt ugyanazon tipuson

Itt tehat a Board default allapota a 3 minta kartyas board.

## 20) impl Default for Board { ... } - mit jelent a for Board?
Ez trait implementacio.

- Default: egy szabvanyos trait a rustban
- for Board: ezt a trait-et a Board tipusra valositod meg

Ennek eredmenye:
- hasznalhatod a Board::default() hivast
- generic kodban is mukodik, ahol T: Default kovetelmeny van

## 21) use anyhow::Result; es a use szo jelentese
A use behoz egy nevet az aktualis scope-ba, hogy rovidebben hasznalhasd.

- use anyhow::Result;
  - az anyhow crate Result tipusat hozza be
  - ez jellemzoen alias: anyhow::Result<T> == Result<T, anyhow::Error>

Miert jo:
- rovidebb alairasok (pl. fn main() -> Result<()> )
- egyseges hibakezeles alkalmazasszinten

## 22) app.rs teljes bemutatasa
Az app.rs most egy minimalis allapotkezelo mag.

1. use crate::model::Board;
- Behozza a Board tipust a model modulbol.

2. Command enum
- A felhasznaloi szandek absztrakcioja:
  - Quit
  - MoveLeft, MoveRight, MoveUp, MoveDown
  - NoOp
- Ez kesobb osszekoti az inputot az alkalmazas logikaval.

3. App struct
- board: Board
  - A teljes kanban allapot
- should_quit: bool
  - Jelzi, hogy ki kell-e lepni a fo loopbol

4. impl App
- App::new()
  - Letrehoz egy kezdo allapotot
  - board a Board::default()-bol jon (tehat most a 3 minta kartya)
  - should_quit kezdetben false

- apply_command(&mut self, command: Command)
  - &mut self miatt modosithatja az app allapotat
  - Jelenleg csak a Quit parancsot kezeli
  - Ha Quit erkezik, should_quit true lesz

Mi varhato a kovetkezo iteracioban:
- MoveLeft/MoveRight stb. tenylegesen modositani fogja a kijelolt oszlopot/kartyat.
- A NoOp tipikusan azt jelenti majd: semmi allapotvaltozas.

## 23) Mini osszefoglalo
A jelenlegi kodod nagyon jo alap:
- Megvan a modulhatar
- Megvannak a domain modellek
- Megvan a tipikus Rust-os hibakezelesi minta (Result)
- Megvan a trait alapu alapertelmezett allapot (Default)

Ezzel keszen allsz a 2. iteraciora (event loop + rajzolas + navigacio).
