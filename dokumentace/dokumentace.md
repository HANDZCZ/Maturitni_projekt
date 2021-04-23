---
#title: Piškvorky
#numbertitlepage: false
#subtitle: | 
#    MATURITNÍ PRÁCE – PROGRAMOVÁNÍ
#    
#    Střední průmyslová škola elektrotechnická a Vyšší odborná škola Pardubice
#date: Březen 2021
author:
- Jan Najman
#thanks:

#bookmarksopen: true
#bookmarkopenlevel: 2
pdfcreator: LaTeX via pandoc, made with care and extra sugar on top
# Overides author (on meta)
#author-meta:
title-meta: Piškvorky
keywords:
- Rust
- API
- RESTful
- Actix
- Actix Web
- Web
- Server
- Application
- Front-end
- programming
- framework
- Yew
subject: Maturitní práce na téma Piškvorky
colorlinks: true
# Maroon
linkcolor: Maroon
# Maroon
filecolor: Maroon
# Blue
citecolor: Blue
# Blue
urlcolor: Blue
# Defaults to linkcolor
toccolor: Black

header-includes: |
    \usepackage{float}
    \let\origfigure\figure
    \let\endorigfigure\endfigure
    \renewenvironment{figure}[1][2] {
        \expandafter\origfigure\expandafter[H]
    } {
        \endorigfigure
    }

    \usepackage{pdfpages}
    \usepackage{setspace}
    \usepackage{indentfirst}
    \setmainfont[
        BoldFont={* Bold},
        ItalicFont={* Italic},
        BoldItalicFont={* BoldItalic}
    ]{DejaVu Serif Condensed}
    
    \usepackage{fvextra}
    \DefineVerbatimEnvironment{Highlighting}{Verbatim}{breaklines,commandchars=\\\{\}}
    
    \usepackage[nosingleletter]{impnattypo}

    \newfontfamily\emojifont[Renderer=Harfbuzz]{Noto Color Emoji}
    \DeclareTextFontCommand{\emoji}{\emojifont}

lang: cs-CZ
pagestyle: empty

#fontfamily: helvet
fontsize: 12pt
geometry:
- top=25mm
- right=20mm
- left=30mm
- bottom=25mm
toc-depth: 6
nocite: '@*'
link-citations: true

numbersections: true
autoEqnLabels: true
codeBlockCaptions: true

lolTitle: Seznam výpisů
listingTitle: Výpis
# Defaults to linkcolor
lotcolor: Black
# Defaults to linkcolor
lofcolor: Black
---
\setstretch{1.25}
\newpage
\hyphenpenalty=10000

\includepdf[pages=-]{MP.pdf}

\newpage

## Anotace {.unlisted .unnumbered}

Práce se zabývá rychle rostoucím a čím dál více populárním systémovým jazykem Rust.
Ukazuje, jak je tento systémový programovací jazyk vyspělý a
jaké má výhody oproti ostatním systémovým jazykům.

Tato práce konkrétně se zabývá programováním RESTful API serveru
v programovacím jazyku Rust pomocí frameworku Actix Web
a programováním front-endové webové aplikace pomocí frameworku Yew.
\

Klíčová slova: Rust, API, RESTful, Actix, Actix Web, Web, Server, Aplikace, Front-end,
Front-end aplikace, programování, framework, Yew


\newpage

## Annotation {.unlisted .unnumbered}

This work deals with the rapidly growing and increasingly popular
system programming language Rust. 
It shows how advanced this system programming language is
and what its advantages are over other system languages.

This work specifically deals with programming RESTful API server in Rust programming language
using the Actix Web framework and programming a front-end web application using
framework Yew.
\

Keywords: Rust, API, RESTful, Actix, Actix Web, Web, Server, Application, Front-end,
Front-end applications, programming, framework, Yew

\newpage
\toc
\newpage
\pagestyle{plain}

\parindent 1,25cm
\parskip 12pt



\newpage

# Úvod {.unnumbered}

Poslední dobou se všichni pokouší optimalizovat své web servery již při jejich programování,
kvůli náporu, který by nemusely stíhat.
Kdo tak neprovede, může toho litovat a snaží se tento problém obejít jinak.
Tím že změní jazyk nebo vytvoří repliky své aplikace a dají na ně load-balancer,
tuto možnost nakonec musí využít všichni, při velmi vysokém náporu.

Bohužel nejpoužívanější jazyky k naprogramování web serveru jsou PHP, JS (Node.js) nebo Python (Flask, Django).
Dá se v nich rychle udělat co potřebujete, ale mají spoustu nevýhod, a hlavně všechny tyto jazyky jsou tzv.\ interpretované jazyky.
To znamená, že na pozadí běží nějaký engine (interpreter), který musí zpracovat daný kód za běhu.
Jejich největším problémem oproti kompilovaným programovacím jazykům jako jsou Rust, C++, C#
je jejich rychlost a možnost zjisti chybu při kompilaci.


| Jazyk | Pomalejší než C++ (gcc -O2) |
|-:|:-|
Rust | 7 %
C# | 78 %
Node.js | 93 %
PHP | 596 %
Python 3.5 | 1800 %
Python 2.7 | 2562 %

: Porovnání rychlosti jazyků [@performance_benchmark] {#tbl:porovnani_rychlosti_jazyku}

Rozhodl jsem se ukázat, že systémový programovací jazyk Rust
je natolik vyspělý, že nejen zvládne vše co jiné jazyky, ale i to, že je rychlejší
než konkurence.

Vybral jsem si programování RESTful API serveru a front-endové webové aplikace
z důvodu, že jsou to dnes nejpoužívanější technologie a mají budoucnost.

Téma Piškvorky jsem si vybral z více důvodů.
Mělo by na nich jít perfektně předvést alespoň základy obou z frameworků.
Tato hra by měla být každému povědomá a nemusím se zabývat vysvětlováním pravidel.


\newpage

# Analýza obdobných aplikací

Analýzu obdobných aplikací je dobré provádět, abyste získali představu, jak má vaše aplikace vypadat.
Co chcete, aby uměla a v čem byla lepší něž ostatní aplikace.

## turtlediary

Adresa: <https://www.turtlediary.com/game/tic-tac-toe-multiplayer.html>

Web nabízí možnost hrát piškvorky 3x3 s náhodnými lidmi, nebo s kamarádem.

### Kladné stránky

- ke hraní není potřeba registrace
- vizuální rozhraní hry

### Záporné stránky

- web není responzivní
- hrací plocha je moc malá oproti zbylému volnému místu
- web nemá statistiky nebo výpis nejlepších hráčů
- na mobilním telefonu není hrací plocha vidět celá

![turtlediary - https://www.turtlediary.com/game/tic-tac-toe-multiplayer.html](analýza1.png){#fig:analýza1_turtlediary}


\newpage

## Ultimate Tic Tac Toe

Adresa: <https://ultimate-t3.herokuapp.com>

Web nabízí možnost hrát piškvorky 3x3 na více polích s kamarádem přes internet nebo lokálně.

### Kladné stránky

- ke hraní není potřeba registrace
- čistý interface

### Záporné stránky

- vice hracích ploch
- web nemá statistiky nebo výpis nejlepších hráčů

![Ultimate Tic Tac Toe - https://ultimate-t3.herokuapp.com](analýza2.png){#fig:analýza2_utimate_ttt}


\newpage

## gametable

Adresa: <https://gametable.org/games/tic-tac-toe>

Web nabízí možnost hrát piškvorky 3x3 s kamarádem lokálně nebo proti AI.

### Kladné stránky

- ke hraní není potřeba registrace
- čistý interface
- hra má vysvětleny pravidla pod hrací plochou

### Záporné stránky

- není možnost hrát s někým přes internet
- web nemá statistiky nebo výpis nejlepších hráčů

![gametable - https://gametable.org/games/tic-tac-toe](analýza3.png){#fig:analýza3_gametable}


\newpage

# Návrh projektu

## Obecná struktura

Veškeré požadavky, které nebudou odkazovat na front-end zodpovídá API.
API je poté napojené na databázi a redis.

Front-end nemá sám o sobě žádný přístup k databázi nebo redisu.

Celá aplikace je napsaná tak, aby bylo možné ji dát do kontejnerů.
S pomocí kontejnerů je možné aplikaci libovolně a jednoduše škálovat.

## API

API spojuje vše dohromady. Poskytuje veškeré informace front-endu
a zpracovává veškeré příchozí informace.

Ukládá a kontroluje uživatelské relace v redisu. Ukládá, upravuje a maže
údaje v databázi.

## Redis

V redisu jsou ukládány uživatelské relace, aby k nim byl rychlý přístup a jejich ověření.
Každá relace má určitou životnost, kterou je možno změnit. V relaci je uložené id uživatele.

\newpage

## Databáze

Databáze ukládá informace o uživatelích a hrách pro dlouhodobé uložení dat.

Tabulka *users* ukládá informace o uživatelích. S vlastnostmi: 

\parskip 0pt

- *id* - unikátní id uživatele,
- *nick* - přezdívka uživatele,
- *gender* - pohlaví uživatele,
- *hash* - hashované heslo uživatele,
- *salt* - sůl pro hashování hesla,
- *email* - unikátní email uživatele,
- *created_at* - čas vytvoření účtu uživatele,
- *description* - popis uživatele.

\parskip 12pt



Tabulka *games* ukládá informace o hrách. S vlastnostmi:

\parskip 0pt

- *id* - unikátní id hry,
- *name* - název hry,
- *ended* - udává, jestli hra skončila,
- *last_played* - udává kdo naposled hrál,
- *data* - data hry,
- *created_at* - čas vytvoření hry,
- *moves_needed* - počet tahů potřebných pro výhru,
- *winner* - výherce.

\parskip 12pt



Tabulka *roles* ukládá role. S vlastnostmi:

\parskip 0pt

- *id* - unikátní id role,
- *name* - unikátní jméno role.

\parskip 12pt




Tabulka *game_requests* ukládá informace o pozvánkách na hru. S vlastnostmi

\parskip 0pt

- *id* - unikátní id pozvánky,
- *name* - název pozvánky,
- *last_played* - ukládá kdy naposled hrál (určuje kdo "začal" - při stejném nastavení hry může pokaždé začínat někdo jiný)
- *created_at* - čas vytvoření pozvánky,
- *moves_needed* - počet tahů potřebných pro výhru.

\parskip 12pt



Tabulka *roles_to_users* spojuje tabulky *roles* a *users*. Přiřazuje uživatelům role. S vlastnostmi:

\parskip 0pt

- *user_id* - id uživatele,
- *role_id* - id role.

\parskip 12pt



Tabulka *games_to_users* spojuje tabulky *games* a *users*. Přiřazuje hry k uživatelům. S vlastnostmi:

\parskip 0pt

- *game_id* - id hry,
- *user_id* - id uživatele.

\parskip 12pt



Tabulka *users_to_game_requests* spojuje tabulky *game_requests* a *users*. Přiřazuje pozvánky k uživatelům. S vlastnostmi:

\parskip 0pt

- *user_id* - id uživatele,
- *game_request_id* - id pozvánky,
- *accepted* - udává, zda uživatel pozvánku přijal.

\parskip 12pt


Diagram [@fig:er_diagram]

## Front-end

Slouží jako grafické zobrazení dat a druhořadá kontrola dat.
Zobrazuje informace o uživatelích a hrách.


## Administrace

Každý uživatel, který má roli *Admin* má zvýšená práva. Může upravovat ostatní uživatele,
vytvářet pozvánky, ve kterých nemusí být, nebo může obsahovat uživatele, 
kteří nemohou být normálně přidáni do pozvánky. Může také vypnout skoro všechny kontroly
jako je formát jména, hesla, popisu uživatele atd.

## Design a responzivita

Design a responzivita je řešená pomocí css knihovny UIkit [@uikit_docs].
Několik věcí je přepsáno pro vzhled aplikace. Tyto změny se nachází v souboru *uikit_addition.css*.

uikit_addition.css [@lst:uikit_addition_css]

\newpage

# Zpracování praktické části

## Použité technologie

Skoro celá aplikace je naprogramovaná v jazyce Rust,
jen databáze se píše v jazyce sql a má několik procedur.

### Rust

Rust je víceúčelový, kompilovaný programovací jazyk, vyvinutý organizací Mozilla Research.
Je navržen jako bezpečný a paralelní programovací jazyk.
Podporuje funkcionální, imperativně-procedurální, strukturované
a objektově orientované programování.

Vývoj je sponzorován Mozillou, ale jde o open source projekt.
Velké množství příspěvků pochází od členů komunity.


### SQL

SQL je jazyk specificky určený ke správě dat uchovávaných v systému správy relačních databází.
Je obzvláště užitečný při zpracování strukturovaných dat.
Dat zahrnujících vztahy mezi subjekty a proměnnými.

Umožňuje přístup k mnoha záznamům pomocí jediného příkazu.
Vylučuje potřebu specifikovat, jak dosáhnout záznamu, např. s indexem nebo bez něj.

### Knihovna roles

Mnou vytvořená knihovna, která při kompilaci načte role z databáze a převede je do datového typu ``enum``.

- *quote* - poskytuje makro pro převod datových struktur jazyka Rust na tokeny zdrojového kódu
- *syn* - je knihovna načítaní tokenů zdrojového kódu jazyka Rust do datového typy zdrojového kódu jazyka Rust
- *proc-macro2* - umožňuje definovat makra
- *dotenv* - načte ``.env`` soubor do proměnných prostředí
- *postgres* - synchronní klient pro databázi PostgreSQL

Zdrojový kód knihovny ``roles`` [@lst:roles_src]

### Back-end

Back-end je naprogramován celý v jazyce Rust.

- *structopt* - získává parametry pro program z příkazového řádku nebo z proměnných prostředí (Environment variables)
- *dotenv* - načte ``.env`` soubor do proměnných prostředí
- *thiserror* - slouží pro zacházení s chybami
- *actix-web* - hlavní knihovna pro web server
- *actix-redis* - používá se jako back-end pro knihovnu actix-session
- *actix-session* - používá se pro zacházení s uživatelskými relacemi
- *env_logger* - slouží jako back-end k výpisu akcí serveru (logging)
- *time* - používá se pro práci s časem
- *sqlx* - slouží ke komunikaci s databází
- *lazy_static* - je použit k vyhodnocování proměnných jen jednou a jen při použití
- *fancy-regex* - slouží pro kontrolu dat pomocí regexu
- *serde* - knihovna k serializaci datových struktur
- *serde_json* - slouží k serializaci do formátu JSON
- *futures-util* - nástroje pro práci s ``futures`` (asynchronní procesy)
- *futures* - implementace ``futures`` knihovny a ``std::future``
- *rand* - používá se pro generování náhodných věcí (čísel, vybírání položky z pole atd.)
- *argon2rs* - hashuje hesla pomocí algoritmu Argon2
- *log* - používá se k výpisu akcí serveru (logging)
- *serde_repr* - slouží k serializaci ``enum`` data typu
- *bincode* - serializuje datové struktury do formátu bincode
- *uuid* - používá se pro práci s univerzálními unikátními identifikátory (UUID)
- *tokio* - slouží jako back-end pro asynchronní procesy
- *roles* - mnou vytvořená knihovna, která při kompilaci načte role z databáze a převede je do datového typu ``enum``
- *actix-cors* - implementace CORS (Cross-origin resource sharing) pravidel pro Actix Web

\newpage

### Front-end

Front-end je naprogramován hlavně v jazyce Rust,
ale využívají se tam i jiné jazyky, jako je HTML, CSS a JS.

Rust knihovny:

- *yew* - knihovna pro vytváření více vláknových front-endových webových aplikací s WebAssembly
- *wasm-bindgen* - knihovna usnadňující interakci na vysoké úrovni mezi moduly wasm (WebAssembly) a JavaScriptem
- *yew-router* - směrovací knihovna pro knihovnu yew
- *wee_alloc* - alokátor pro WebAssembly
- *wasm-logger* - slouží jako back-end pro výpis akcí do konzole prohlížeče
- *log* - používá se k výpisu akcí serveru (logging)
- *roles* - mnou vytvořená knihovna, která při kompilaci načte role z databáze a převede je do datového typu ``enum``
- *lazy_static* - je použit k vyhodnocování proměnných jen jednou a jen při použití
- *fancy-regex* - slouží pro kontrolu dat pomocí regexu
- *serde* - knihovna k serializaci datových struktur
- *serde_json* - slouží k serializaci do formátu JSON
- *serde_repr* - slouží k serializaci ``enum`` data typu
- *time* - používá se pro práci s časem
- *bincode* - serializuje datové struktury do formátu bincode
- *strum* - poskytuje sadu maker pro snadnější práci s datovými typy ``enum`` a ``String``

CSS a JS knihovny:

- *UIkit* - modulární front-end knihovna pro vývoj rychlých a výkonných webových rozhraní

#### Server

Speciálně vytvořený server pro správnou funkci front-endu.

- *structopt* - získává parametry pro program z příkazového řádku nebo z proměnných prostředí (Environment variables)
- *dotenv* - načte ``.env`` soubor do proměnných prostředí
- *actix-web* - hlavní knihovna pro web server
- *env_logger* - slouží jako back-end k výpisu akcí serveru (logging)
- *actix-files* - slouží k práci se statickými soubory
- *thiserror* - slouží pro zacházení s chybami

## Databáze

Spousta akcí, které back-end podniká, jsou řešené skrz procedury.
Tímto způsobem dojde k zjednodušení kódu na back-endu
a k provedení akce není potřeba dělat několik dotazů na databázi.

Procedura pro vytvoření pozvánky [@lst:new_game_request_sql_procedure]

Procedura pro úpravu uživatele [@lst:update_user_sql_procedure]

Procedura pro úpravu pozvánky [@lst:update_invite_sql_procedure]

\newpage

## Datové struktury

### Enum

Datový typ ``enum`` v Rustu je podobný jako v jiných kompilovaných jazycích, jako je C,
ale má důležité rozdíly, díky nimž je podstatně výkonnější.
To, co Rust nazývá enumy, je běžněji známé jako algebraické datové typy,
pokud přicházíte z pozadí funkčního programování.
Důležitým detailem je, že každá varianta enumu může mít k sobě další data.

```{.rust .numberLines}
enum JednoduchyEnum {
    PrvniVarianta,
    DruhaVarianta,
    TretiVarianta,
}

enum Lokace {
    Neznama,
    Anonymni,
    Znama(Mesto),
}

enum KomplexniEnum {
    Nic,
    Neco(u32),
    HodneVeci {
        publikovat: bool,
        text: String,
    }
}
```

: Rust - datový typ ``enum`` {#lst:rust_data_type_enum}

První enum je obvyklý druh enumu, který najdete například v jazyce C.

Druhý ukazuje hypotetický příklad něčeho, co ukládá údaje o poloze,
přičemž ``Mesto`` je jakýkoli jiný typ, který je potřeba, například ``struct``.

Třetí příklad ukazuje druh dat, které může varianta ukládat, od ničeho, přes tuple, až po anonymní strukturu.

\newpage

### Struct

Datový typ ``struct`` v Rustu můžeme nalézt ve třech druzích. Regulární s jmennými vlastnostmi, takzvané tuple struktury a unit struktury.

```{.rust .numberLines}
struct Regulární {
    vlastnost1: f32,
    vlastnost2: String,
    pub vlastnost3: bool
}

struct Tuple(u32, String);

struct Unit;
```

: Rust - datový typ ``struct`` {#lst:rust_data_type_struct}

Regulární struktury jsou nejvíce používané.
Každá vlastnost je definována jménem a typem,
poté se k daným vlastnostem dá dostat pomocí
``struktura.vlastnost``.

Všechny vlastnosti struktury sdílejí stejnou proměnlivost jako samotná struktura,
takže ``struktura.vlastnost = 2;`` by bylo validní jen, kdyby celá struktura byla proměnlivá.
Přidáním ``pub`` před jméno vlastnosti způsobí,
že daná vlastnost je viditelná i v jiných modulech a je možno jí přímo číst nebo měnit.

Tuple struktury jsou podobné regulárním strukturám, ale jejich vlastnosti nemají žádná jména.
Pro přístup k vlastnostem se použije pozice vlastnosti, začínající od nuly ``struktura.0``, ``struktura.1`` atd.

Unit struktury jsou hlavně používány jako markery.
Nemají žádnou velikost, ale můžou být instancovány,
to z nich dělá typ stejný jako ``unit - ()``.
Jsou užitečné, když potřebujete implementovat ``trait`` na něčem,
ale nepotřebujete ukládat žádná data.

### Trait

Trait je jako interface, který datové typy mohou implementovat.

Traity můžou být složeny až ze tří druhů položek:

- funkce a metody
- typy
- konstanty

Traity také mohou sloužit jako markery nebo mohou nést jinou logiku, která není vyjádřená v jejich názvech.

```{.rust .numberLines}
struct Pes {
    vaha_kg: u8,
}
struct Kocka {
    jmeno: String,
    vaha_g: u16,
}
trait Vaha {
    fn vaha_v_kg(&self) -> f32;
}
impl Vaha for Pes {
    fn vaha_v_kg(&self) -> f32 {
        self.vaha_kg as f32
    }
}
impl Vaha for Kocka {
    fn vaha_v_kg(&self) -> f32 {
        self.vaha_g as f32 / 1000f32
    }
}
fn main() {
    let kocka = Kocka {
        jmeno: "Líza".into(),
        vaha_g: 2000,
    };
    let pes = Pes {
        vaha_kg: 2,
    };
    assert_eq!(kocka.vaha_v_kg(), pes.vaha_v_kg());
}
```

: Rust - typ ``trait`` {#lst:rust_type_trait}

\newpage

### Type

Definuje alias pro existující datový typ.

```{.rust .numberLines}
type Milimetr = u32;
type Kilogram = u32;

let m: Milimetr = 3;
let k: Kilogram = 3;

assert_eq!(m, k);
```

: Rust - ``type`` {#lst:rust_type_keyword}

Nevytváří nové datové typy, proto se ve výše uvedeném příkladu tři milimetry rovnají třem kilogramům.

## Back-end

### Správa uživatelů

Uživatelé jsou umístěni v tabulce *users*.
Role jsou uloženy v tabulce *roles* a jsou k uživatelům přiřazovány skrz tabulku *roles_to_users*.

Pokud má uživatel roli *Admin*, tak může upravovat kohokoli údaje bez omezení včetně rolí a hesla.

Pokud má uživatel roli *Banned*, tak nemůže vytvářet pozvánky,
ani nemůže být zahrnut do pozvánky jiným uživatelem.

Úprava uživatele po kontrole dat je poté řízena procedurou [@lst:update_user_sql_procedure].

### Vytváření žádostí o hru

Přihlášení uživatelé mají možnost vytvářet nové hry s různými parametry.

Nejprve se vytvoří žádost o hru, která se nachází v tabulce *game_requests*.
K dané žádosti na hru se přiřadí uživatelé skrz tabulku *users_to_game_requests*.

Pozvánky nemohou vytvářet uživatelé s rolí *Banned*.

Vytvoření pozvánky po kontrole dat je poté řízeno procedurou [@lst:new_game_request_sql_procedure]

### Vytvoření hratelné hry

Po vytvoření žádosti o hru ji musí všichni hráči potvrdit a hra bude vytvořena,
nebo někdo z pozvaných hráčů odmítne žádost a žádost o hru bude vymazána.

Jakmile je účast všech hráčů potvrzena, tak se vytvoří nová hra v tabulce *games*,
přiřadí se k ní uživatelé skrz tabulku *games_to_users* a žádost o hru je poté vymazána.

Úprava pozvánky po kontrole dat je poté řízena procedurou [@lst:update_invite_sql_procedure]

### Hraní hry {#sec:practical_part_playing_game}

Hrát můžete jen když jste na tahu, a pokud hrané políčko ještě nebylo použito.
Výhra hry se kontroluje na front-endu, pokud front-end usoudí,
že hráč vyhrál tak výhru oznámí back-endu a ten výhru zkontroluje.

Back-end kontroluje, jestli je hráč na tahu, jestli hra neskončila nebo jestli jeho tah je validní.
V případě, že hráč ohlásí výhru, ale server zjistí, že to tak není, tak daný tah zahodí a odpoví chybou.


## Front-end

### Přihlášení

Pole emailu je kontrolováno.
Pokud pole není validní, tak se nepošlou data na back-end.

Back-end data zkontroluje, a pokud zjistí, že nejsou validní,
tak žádost zahodí a vrátí chybu.

### Profil

Zobrazuje informace o uživateli a hry,
ve kterých se nachází.

Zobrazuje také počet výher, proher a remíz. 
Z těchto dat poté vypočítá winrate (výhry/prohry).

Pokud je uživatel na svém profilu nebo pokud má uživatel roli *Admin*,
tak se mu také zobrazí tlačítko na upravení profilu.

### Výpis uživatelů

Zobrazuje všechny registrované uživatele a pár informací o nich.

Uživatelům s rolí *Admin* se navíc zobrazuje tlačítko upravení profilu.

### Výpis her

Zobrazuje všechny rozehrané nebo dohrané hry s jejich hráči a stavem hry.

### Hraní hry

Hry jsou hrány na síti 30x30.

Uživatelé jsou zobrazováni s jejich symbolem před jménem a za jménem je napsáno, jestli jsou na tahu.

Hrát můžou jen uživatelé, kteří jsou v dané hře, ale dívat se může kdokoli.

Tahy uživatelů jsou kontrolovány, jestli jsou validní a jestli nastala výhra nebo remíza.

### Výpis pozvánek

Zobrazuje název pozvánky (později název hry), počet tahů k vítězství a id pozvánky.

Uživatel může pozvánku přijmout nebo odmítnout.

### Vytváření pozvánky

Při vytváření pozvánky jsou skoro všechna pole kontrolována.

Uživatelé s rolí *Admin* mají práva na vypnutí skoro všech kontrol.

### Úprava uživatele

Uživatel může upravovat vše kromě jeho rolí.

Uživatel s rolí *Admin* může upravovat vše a má možnost vypnout kontrolu,
která je vyžadována po ostatních uživatelích.

### Registrace

Všechna pole jsou kontrolována.
Pokud nějaké pole není validní, tak se nepošlou data na back-end.

Back-end data zkontroluje, a pokud zjistí, že nejsou validní,
tak žádost zahodí a vrátí chybu.

Pro hashování hesla se používá 128 znaková sůl a algoritmus Argon2.


\newpage

# Manuál pro spuštění aplikace

Aplikace se dá spustit více způsoby. Buď kompilací ze zdrojového kódu nebo pomocí kontejnerů.

## Instalace potřebných nástrojů

### Kompilace

Pro kompilaci potřebujeme nainstalovat kompilátor jazyka Rust.

Přejdeme na stránku stažení jazyka Rust <https://www.rust-lang.org/tools/install>
a stáhneme exe soubor.

Otevřeme a zadáme 1 pro instalaci a stiskneme enter.

Po dokončení instalace všech komponentů nainstalujeme ještě wasm-pack pro kompilaci do WebAssembly.

Přejdeme na stránku stažení nástroje wasm-pack <https://rustwasm.github.io/wasm-pack/installer/>
a stáhneme exe soubor.

Po stažení jej nainstalujeme.

### Použití kontejnerů

Pro použití kontejnerů potřebujeme nějaký software, který umožňuje spouštění kontejnerů a manipulaci s nimi.
Já jsem zvolil docker, jeden z nejznámějších nástrojů pro používání a výrobu kontejnerů.

Přejdeme na stránku stažení <https://hub.docker.com/editions/community/docker-ce-desktop-windows/>
a stáhneme exe soubor.

Nainstalujeme a vyzkoušíme funkčnost (Hello world! kontejner).

\newpage

## Příprava

### Kompilace

Kompilace není potřeba při použití kontejnerů.

Pro kompilaci aplikace je potřeba mít databázi připravenou dopředu.
Stačí se přihlásit do administrace databáze a spustit sql příkazy v přiloženém sql souboru.

Zkompilujeme back-end. Vstoupíme do složky ``backend`` a spustíme příkaz:

```bash
cargo build --release
```

: Příkaz pro kompilaci back-endu {#lst:back-end_compile_command}

Zkompilujeme front-end server. Vstoupíme do složky ``frontend/server`` a spustíme příkaz:

```bash
cargo build --release
```

: Příkaz pro kompilaci front-end serveru {#lst:front-end_server_compile_command}

Zkompilujeme front-end. Vstoupíme do složky ``frontend`` a spustíme příkaz:

```
build.bat
```

: Příkaz pro kompilaci front-endu {#lst:front-end_compile_command}

\newpage

### Použití kontejnerů

Pro jednoduché použití používám docker-compose.

Pro jednoduchou konfiguraci jsem vytvořil funkční příklad nastavení.

Tento příklad si zkopírujeme a přejmenujeme z ``docker-compose.yml.example``
na ``docker-compose.yml``.

Potom si ho upravíme podle vlastních preferencí.

Změníme tyto položky:

```.yaml
# Databáze
POSTGRES_USER: root             # Jméno uživatele databáze
POSTGRES_PASSWORD: password     # Heslo uživatele databáze

# Back-end
AUTH_TTL: 3600                  # Jak dlouho bude platit uživatelská relace v sekundách
AUTH_KEY: keykeykeykeykeykeykeykeykeykeykey   # Klíč pro uživatelské relace, musí mít minimálně 32 bajtů (32 znaků)
DATABASE_URL: postgres://root:password@postgres/maturitni_projekt   # Změníme jméno a heslo na to které jsme zvolili u databáze
ALLOWED_ORIGIN: http://mp.loc   # Adresa front-endu pro správné nastavení CORS
FRONTEND_DOMAIN: mp.loc         # Adresa front-endu bez protokolu pro přístup ke cookies na front-endu

# Front-end
API_DOMAIN: http://api.mp.loc   # Adresa back-endu pro komunikaci mezi front-endem a back-endem
```

: Nastavení proměnných prostředí pro kontejnery {#lst:container_env_config}

\newpage

Poté taky změníme překládání portů, abychom se na aplikaci vůbec dostali.

```.yaml
backend:
  ports:
    - "127.0.5.2:80:80" # Toto znamená překládej port 80 na adresu a port 127.0.5.2:80

frontend:
  ports:
    - "127.0.5.1:80:80" # Toto znamená překládej port 80 na adresu a port 127.0.5.1:80
```

: Nastavení překládání portů pro kontejnery {#lst:container_port_forwarding_config}

Samozřejmě tato aplikace má být postavená například za nginx server.
Ale pokud jen testujeme na svém počítači,
tak můžeme do souboru ``hosts`` přidat položky
pro přeložení adresy front-endu a back-endu:

```
127.0.5.1 mp.loc
127.0.5.2 api.mp.loc
```

: Úprava hosts souboru pro kontejnery {#lst:container_port_forwarding_config}

\newpage

## Spuštění

### Kompilace

Musíme mít spuštěnou databázi a redis.

1. Spustíme back-end. Stačí jen spustit exe soubor a popřípadě dodat další argumenty.

1. Spustíme front-end server. Stačí jen spustit exe soubor a popřípadě dodat další argumenty.

Pro jednoduché použití stačí vytvořit ``.env`` soubor,
ze kterého se načtou hodnoty do proměnných prostředí.
Program si je potom bere odtud.

### Použití kontejnerů

Spustíme příkaz, který nám stáhne, nastartuje a nakonfiguruje veškeré kontejnery:

```bash
docker-compose up -d
```

: Příkaz pro start kontejnerů {#lst:container_start_command}

\newpage

# Manuál pro používání aplikace

Po spuštění aplikace přejdeme na doménu front-endu.

![Hlavní stránka aplikace](usage_manual_home_page.png){#fig:usage_manual_home_page}

## Přihlášení

Vpravo nahoře klikneme na ``PŘIHLÁSIT SE`` a dostaneme se na stránku přihlášení.

![Stránka přihlášení](usage_manual_login_page.png){#fig:usage_manual_login_page}

Vyplníme údaje a klikneme na ``PŘIHLÁSIT``.

### První přihlášení

Pro první přihlášení se přihlásíme na administrátora s těmito údaji:

```yaml
EMAIL: r@r.r
HESLO: root
```

: Údaje administrátora při prvním přihlášení {#lst:admin_credentials_first_login}

![Stránka přihlášení - údaje administrátora](usage_manual_login_root_credentials.png){#fig:usage_manual_login_root_credentials}

Po přihlášení změňte heslo a ostatní údaje,
pokud tak neprovedete může dojít k odcizení administrátorského účtu!

## Registrace

Vpravo nahoře klikneme na ``PŘIHLÁSIT SE`` a dostaneme se na stránku přihlášení,
poté klikneme na ``Ještě nemám účet!``

Vyplníme veškeré údaje.

Údaje jsou kontrolovány a musí odpovídat těmto parametrům:

- *Nick* - musí obsahovat 3 až 12 znaků, povolené znaky jsou:
    - malá písmena od ``a`` až po ``z``
    - velká písmena od ``A`` až po ``Z``
    - číslice od ``0`` až do ``9``
- *Pohlaví* - musí obsahovat 1 až 50 znaků, povolené znaky jsou:
    - malá písmena od ``a`` až po ``z`` včetně diakritiky
    - velká písmena od ``A`` až po ``Z`` včetně diakritiky
    - číslice od ``0`` až do ``9``
    - speciální znaky ``&_-',.``, mezi tyto znaky patří také mezera
- *Email* - musí obsahovat 5 až 25 znaků, musí být podle standardu ``RFC2822``
- *Heslo* - musí obsahovat 8 až 25 znaků, musí obsahovat:
    - 2 malá písmena
    - 2 velká písmena
    - 2 speciální znaky
- *Heslo znovu* - musí mít stejnou hodnotu jako pole ``Heslo``
- *Něco o vás* - musí obsahovat 20 až 650 znaků, povolené znaky jsou:
    - malá písmena od ``a`` až po ``z`` včetně diakritiky
    - velká písmena od ``A`` až po ``Z`` včetně diakritiky
    - číslice od ``0`` až do ``9``
    - speciální znaky ``&_-',.``, mezi tyto znaky patří také mezera

![Stránka registrace](usage_manual_register_page.png){#fig:usage_manual_register_page}

## Profil uživatele {#sec:usage_manual_profile_page}

Na vlastní profil se můžeme dostat dvěma způsoby:

1. Vpravo nahoře najedeme myší na své jméno, rozbalí se nám nabídka a klikneme na ``Profil``.
2. Nahoře uprostřed klikneme na ``UŽIVATELÉ``, najdeme se v seznamu uživatelů a klikneme na své jméno.

![Stránka profilu](usage_manual_profile_page.png){#fig:usage_manual_profile_page}

## Seznam uživatelů

Na seznam uživatelů se dostaneme tak, že nahoře uprostřed klikneme na ``UŽIVATELÉ``.

![Stránka seznamu uživatelů](usage_manual_users_page.png){#fig:usage_manual_users_page}

### Administrátoři

Administrátoři mají navíc vespodu každé karty uživatele ještě tlačítko pro úpravu profilu daného uživatele.

![Stránka seznamu uživatelů - možnosti administrátora](usage_manual_users_admin_options.png){#fig:usage_manual_users_admin_options}

## Úprava profilu

Na stránce svého profilu ([@sec:usage_manual_profile_page]) klikneme na tlačítko ``UPRAVIT ÚČET``.

![Stránka úpravy profilu](usage_manual_edit_profile_page.png){#fig:usage_manual_edit_users_page}

### Administrátoři

Administrátoři mají na konci stránky více možností úpravy.

![Stránka úpravy profilu - možnosti administrátora](usage_manual_edit_profile_admin_options.png){#fig:usage_manual_edit_users_admin_options}

## Seznam her

Na seznam her se dostaneme tak, že nahoře uprostřed klikneme na ``HRY``.

![Stránka seznamu her](usage_manual_games_page.png){#fig:usage_manual_games_page}

## Seznam pozvánek {#sec:usage_manual_invites_page}

Na seznam pozvánek se dostaneme tak, že vpravo nahoře najedeme myší na své jméno, rozbalí se nám nabídka a klikneme na ``Pozvánky``.

![Stránka seznamu pozvánek](usage_manual_invites_page.png){#fig:usage_manual_invites_page}

Pro přijetí pozvánky klikněte na fajfku (\emoji{✔}) a pro zamítnutí na křížek (\emoji{❌}).

## Vytvoření pozvánky {#sec:usage_manual_new_invite_page}

Na seznamu pozvánek ([@sec:usage_manual_invites_page]) klikneme na tlačítko ``NOVÁ POZVÁNKA``.

Vyplníme veškeré údaje.

Údaje jsou kontrolovány a musí odpovídat těmto parametrům:

- *Jméno* - musí obsahovat 5 až 15 znaků, povolené znaky jsou:
    - malá písmena od ``a`` až po ``z``
    - velká písmena od ``A`` až po ``Z``
    - číslice od ``0`` až do ``9``
    - speciální znaky ``&_-',.``, mezi tyto znaky patří také mezera
- *Počet tahů k vítězství* - musí být číslice od ``3`` až do ``8``
- *UUID uživatelů* - tento údaj není kontrolovaný na front-endu, ale je kontrolovaný na back-endu,
toto pole musí obsahovat:
    - UUID uživatelů (lze získat ze seznamu uživatelů) oddělená novými řádky

![Stránka vytvoření pozvánky](usage_manual_new_invite_page.png){#fig:usage_manual_new_invite_page}

### Administrátoři

Administrátoři mají na konci stránky více možností úpravy.

![Stránka vytvoření pozvánky - možnosti administrátora](usage_manual_new_invite_admin_options.png){#fig:usage_manual_new_invite_admin_options}

## Hraní hry

Pro hraní hry musíte mít hru v probíhajícím stavu a být hráč dané hry.
Pokud žádnou z takových her nemáte, můžete ji vytvořit ([@sec:usage_manual_new_invite_page]).

Hru, kterou chcete hrát, můžete najít na svém profilu ([@sec:usage_manual_profile_page]).
Potom, co ji najdete, klikněte na tlačítko ``PŘEJÍT NA HRU``.

![Stránka hraní hry](usage_manual_game_page.png){#fig:usage_manual_game_page}

Před sebou bychom měli vidět mřížku a vpravo nebo pod ní seznam uživatelů.
Před uživatelským jménem je jeho symbol a za jménem je napsáno, jestli je na tahu.

Pod seznamem uživatelů můžeme vidět, kolik tahů potřebujeme k vítězství a jaký je status hry.

\newpage

# Závěr

Tento projekt vznikl proto,
aby ukázal, že frameworky v Rustu jsou dostatečně vyspělé
a výkonnější něž alternativy.
Tento fakt ukazuje i čím dál více rostoucí komunita
a stránka "Are we *web* yet?" [@arewewebyet],
která sleduje vývoj webových technologií v Rustu.

Jedním z mých úkolů také bylo držet se open source
a s radostí mohu prohlásit, že se mi to úspěšně povedlo.

Co se týče hlavních bodů maturitní práce,
tak mohu bez starostí říci,
že jsem je splnil do puntíku.

1. "Hry se budou ukládat do databáze" - hry jsou uloženy v databázi PostgreSQL
v tabulce *games*
2. "Hry budou moct být hrány na tahy nebo odehrány v celku" - hry můžete odehrát v jednom sezení nebo ve více,
nezáleží na tom, jak dlouho bude sezení trvat, zda 5 minut nebo 5 let
3. "Rozdělení uživatelů na přihlášené a nepřihlášené a administrátory" - uživatelé jsou rozděleni na nepřihlášené a přihlášené;
a pomocí role *Admin* na přihlášené a administrátory
4. "Tahy hráčů budou kontrolovány na serveru" - tahy jsou kontrolovány [@sec:practical_part_playing_game]
5. "Data odesílaná klientem nebo uživatelem budou mít co nejmenší velikost" - skoro všechna data jsou posílány ve formát JSON,
data herního pole jsou posílána v ještě menším formátu bincode
6. "Hry budou mít různé velikosti" - hry mohou být hrány na různý počet tahů
7. "Každý přihlášený uživatel bude mít statistiky" - každý uživatel má počet výher,
počet remíz a počet proher, z těchto dat je poté vypočítán winrate

\newpage

## Řešení problémů

Při řešení tohoto tématu jsem narazil na pár problémů,
které mě ale moc nevykolejily.
Dalo by se říct, že jsem s nimi počítal.

### Databáze

Jedním ze závažnějších problémů byla databáze.
Nejprve jsem pracoval s MySQL databází,
ale později se ukázalo, že má mnoho limitací a nevýhod.

Jednou z nich byla limitace datového typu string
neboli ``varchar`` (nebo ``char``), který neměl dostatečnou délku.
Později jsem taky zjistil, že datový typ ``varchar`` při různých operacích
je převáděn na datový typ ``char`` a zbylé bajty jsou vyplněny.
Takže místo porovnávání textu s různou délkou,
dochází k porovnávání textu se stejnou délkou (255 znaků).
MySQL má datový typ, který umožňuje ukládat delší text,
ale jak s ním pracuje mi vůbec nevyhovovalo.

Druhou z nevýhod byly procedury,
které se v tomto jazyce nedají psát!
Jakým způsobem se procedury píší, nedává žádný smysl.
Dokonce si dovoluji říct, že psát kód v assembly je jednodušší.

Z výše uvedených důvodů jsem přešel na open source databázi PostgreSQL.
Tato databáze vyřešila všechny mé problémy, a více.
Díky této databázi jsem byl schopný použít pro id tabulek datový typ UUID,
který nevyčerpám, ani když se budu snažit a také mohu používat data typu JSON nativně.
Mohu používat datový typ text bez jakéhokoli problému
či snížení výkonu.
Mohu vracet výsledky z insertů či updatů.
Ale jedna z nejdůležitějších vlastností byla pro mě možnost psát procedury,
které podporují základní věci jako je cyklus
(viz. [@lst:new_game_request_sql_procedure;@lst:update_user_sql_procedure;@lst:update_invite_sql_procedure]).

### Vyšší nápor na síť

Další problém, na který jsem narazil,
byla komunikace mezi databází a back-endem.
Díky tomu, že back-end potřeboval několik sql dotazů
pro správné vyhodnocení požadavku klienta,
tak jsem implementoval většinu do procedury.
Tímto způsobem se zmenšila zátěž jak na back-end,
tak na databázi a samozřejmě nápor na síť klesl.

### Cross-origin resource sharing

Třešničkou na dortu samozřejmě byly CORS.
Nejdříve jsem je neřešil a používal upravený prohlížeč,
ale dříve či později jsem je musel řešit.
Naštěstí na tento otravný problém autoři frameworku Actix Web mysleli
a vydali knihovnu ``actix-cors``.
V té stačí jen nakonfigurovat cors pravidla
buď pro celou aplikaci, nebo pro každý koncový bod,
nebo pro obojí.

## Vylepšení

Jedině co bych vylepšil je zobrazování her ([@fig:usage_manual_game_page])
a rovněž bych přidal více symbolů pro hráče,
tím bych navýšil maximální počet hráčů ve hře.
Zobrazování her bych vyřešil tak,
že bych přidal pořádný renderer,
například přes WebGL.
To by mi dovolilo využít plný prostor pro hraní
(256x256 - limitován datovým typem ``u8``),
který je momentálně limitován na 30x30,
kvůli současnému zobrazování her.

\newpage

# Seznam použité literatury a zdrojů informací

::: {#refs}
:::

\newpage

# Seznam použitých zkratek

| Zkratka | Význam |
|-:|:---------|
API | Application Programming Interface
CORS | Cross-origin resource sharing
EXE | Executable
JSON | JavaScript Object Notation
Regex | Regular expression
REST | Representational State Transfer
SPŠE | Střední průmyslová škola elektrotechnická
UUID | Universally unique identifier
VOŠ | Vysoká odborná škola
WebGL | Web Graphics Library

: Seznam použitých zkratek {#tbl:seznam_použitých_zkratek}


\newpage

# Seznam obrázků, tabulek, příloh

\lof
\lot
\lol

\newpage

# Přílohy

![ER Diagram](diagram.png){#fig:er_diagram}

```{.css .numberLines}
body {
    background-color: #545454;
}
.uk-navbar-container.uk-light:not(.uk-navbar-transparent) :not(.uk-navbar-primary) {
    background: #222;
}
.uk-dropdown.uk-light {
    background: #222;
}
.uk-dropdown li {
    padding-left: 5px;
    border-left: 2px solid transparent;
}
.uk-dropdown li.uk-active {
    border-color: #545454;
}
#mobile-navbar li {
    padding-left: 5px;
    border-left: 2px solid transparent;
}
#mobile-navbar li.uk-active {
    border-color: #545454;
}
.uk-navbar a {
	text-decoration: none
}
.uk-form-danger {
    color: #f0506e !important;
    border-color: #f0506e !important;
}
.uk-notification-message {
	background: #222;
}
body > div {
    padding-bottom: 1px;
}
```

: uikit_addition.css {#lst:uikit_addition_css}


```{.sqlpostgresql .numberLines}
create procedure new_game_request(_name text, _last_played uuid, _users_id uuid[], _moves_needed smallint)
    language plpgsql
as
$$
declare
    v_game_request_id uuid;
begin
    insert into game_requests (name, last_played, moves_needed)
    values (_name, _last_played, _moves_needed)
    returning game_requests.id into v_game_request_id;

    insert into users_to_game_requests (user_id, game_request_id)
    select user_id__, v_game_request_id
    from unnest(_users_id) user_id__;
    commit;
end;
$$;
```

: Procedura pro vytvoření pozvánky {#lst:new_game_request_sql_procedure}


```{.sqlpostgresql .numberLines}
create procedure update_user(_id uuid, _nick text DEFAULT NULL::text, _gender text DEFAULT NULL::text, _email text DEFAULT NULL::text, _hash bytea DEFAULT NULL::bytea, _salt character varying DEFAULT NULL::character varying, _roles integer[] DEFAULT NULL::integer[], _description text DEFAULT NULL::text)
    language plpgsql
as
$$
begin
    if _nick is not null then
        update users set nick = $2 where id = $1;
    end if;

    if _gender is not null then
        update users set gender = $3 where id = $1;
    end if;
    
    if _email is not null then
        update users set email = $4 where id = $1;
    end if;
    
    if _hash is not null then
        update users set hash = $5 where id = $1;
    end if;
    
    if _salt is not null then
        update users set salt = $6 where id = $1;
    end if;
    
    if _description is not null then
        update users set description = _description where id = _id;
    end if;

    if _roles is not null then
        delete from roles_to_users where user_id = _id;
        insert into roles_to_users (user_id, role_id) select _id, role_id__ FROM unnest(_roles) role_id__;
    end if;
    
    commit;
end;
$$;
```

: Procedura pro úpravu uživatele {#lst:update_user_sql_procedure}


```{.sqlpostgresql .numberLines}
create procedure update_invite (_user_id uuid, _game_request_id uuid, _accepted boolean, _data bytea)
    language plpgsql
as
$$
declare
    v_ready   bool;
    v_game_id uuid;
    v_exists  bool;
begin
    select exists(select * from users_to_game_requests where game_request_id = _game_request_id and user_id = _user_id)
    into v_exists;
    if not v_exists then
        raise exception 'User with id ''%'' is not part of game request with id ''%'' or game request with id ''%'' doesn''t exists', _user_id, _game_request_id, _game_request_id;
    end if;
    if _accepted then
        update users_to_game_requests set accepted = true where user_id = _user_id and game_request_id = _game_request_id;
        select not exists(
            select * from users_to_game_requests where game_request_id = _game_request_id and not accepted
        ) into v_ready;
        if v_ready then
            insert into games (name, data, last_played, moves_needed) select name, _data, last_played, moves_needed from game_requests where game_requests.id = _game_request_id returning games.id into v_game_id;
            insert into games_to_users (user_id, game_id) select users_to_game_requests.user_id, v_game_id from users_to_game_requests where game_request_id = _game_request_id;
            delete from users_to_game_requests where game_request_id = _game_request_id;
            delete from game_requests where id = _game_request_id;
        end if;
    else
        delete from users_to_game_requests where game_request_id = _game_request_id;
        delete from game_requests where id = _game_request_id;
    end if;
    commit;
end;
$$;
```

: Procedura pro úpravu pozvánky {#lst:update_invite_sql_procedure}


```{.rust .numberLines}
use postgres::{Client, NoTls};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, parse_quote, Ident, ItemEnum};

#[proc_macro_attribute]
pub fn get_roles_from_db(_attr: TokenStream, item: TokenStream) -> TokenStream {
    dotenv::dotenv().expect("Dotenv error");
    let mut parsed_enum = parse_macro_input!(item as ItemEnum);

    let database_url = std::env::var("DATABASE_URL").expect("Missing env variable DATABASE_URL");

    let mut client = Client::connect(&database_url, NoTls).expect("Couldn't create pool");

    let mut variants = Punctuated::new();
    for row in client
        .query("select name, id from roles", &[])
        .expect("Couldn't get roles from db")
    {
        let name = Ident::new(row.get(0), Span::call_site());
        let id: i16 = row.get(1);
        let variant: syn::Variant = parse_quote! {
            #name = #id as i32
        };
        variants.push(variant);
    }

    parsed_enum.variants = variants;

    (quote! {
        #parsed_enum
    })
    .into()
}
```

: Zdrojový kód knihovny ``roles`` {#lst:roles_src}


