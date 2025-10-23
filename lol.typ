// This template is directly based on the 'celluloid' typst template
// by Casey Dahlin. See https://typst.app/universe/package/celluloid for
// the original template. This template has been heavily modified and is
// not fit for use outside of Rustwell, we instead refer to using the
// fantastic celluloid template for writing screenplays in typst. Below
// is the original copyright notice from celluloid.

// Copyright (C) 2025 Casey Dahlin <sadmac@google.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//         http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#let line_spacing(blanks) = 0.65em + blanks * 1.23em
#let dialogue_counter = counter("dialogue")

#let screenplay(
  titlepage: false,
  title: none,
  credit: none,
  authors: none,
  source: none,
  draft_date: none,
  contact: none,
  doc,
) = {
  set page(
    margin: ( top: 1in, left: 1.5in, right: 1in, bottom: 0.5in),
    header-ascent: 0.5in,
  )
  set text(
    font: "Courier Prime",
    size: 12pt,
  )
  set par(spacing: line_spacing(1))

  show heading: h => {
    set text(size: 12pt, weight: "regular")
    set block(spacing: line_spacing(1))
    h
  }

  if titlepage {
    page({
    align(center, {
        upper(title)

        if credit != none {
          block(credit, above: 1in)
        }
        if authors != none {
          block(authors, spacing: 0.5in)
        }
        if source != none {
          block(source, spacing: 0.6in)
        }
        if draft_date != none {
          block(draft_date, spacing: 0.9in)
        }
      })

      if contact != none {
        align(bottom, box(align(left, contact)))
      }
      counter(page).update(0)
    }, margin: ( top: 2in, bottom: 1in ))
  }

  set page(
    header: context {
      if counter(page).get().at(0) > 0 {
        align(right, counter(page).display("1."))
      }
    },
  )

  doc
}

#let dialogue_raw(character, dialogue, paren: none, left_inset, left_inset_name, right_inset_name) = {
  context {
    set par(spacing: line_spacing(0))
    let char_text = if type(character) == str {
      character
    } else {
      character.text
    }
    dialogue_counter.step()
    let dialogue_count = dialogue_counter.get().at(0)
    let dialogue_header_counter = counter("dialogue_header" + str(dialogue_count))
    let dialogue_footer_counter = counter("dialogue_footer" + str(dialogue_count))
    grid(
      grid.header(block(par([#upper(context {
        dialogue_header_counter.step()
        let paren = if dialogue_header_counter.get() != (0,) {
          "CONT’D"
        } else {
          paren
        }
        character
        if paren != none {
          [ (#paren)]
        }
      })]), inset: (left: left_inset))),
      block(dialogue, spacing: line_spacing(0)),
      grid.footer(block({
        dialogue_footer_counter.step()
        context {
          if dialogue_footer_counter.get() != dialogue_footer_counter.final() {
            [(MORE)]
          }
        }
      }, inset: (left: left_inset), spacing: line_spacing(0))),
      inset: (left: left_inset_name, right: right_inset_name),
      gutter: line_spacing(0),
    )
  }
}

#let dialogue(character, dialogue, paren: none) = dialogue_raw(character, dialogue, paren: paren, 1.5in, 1in, 1.5in)

#let dual_dialogue(character1, dialogue1, paren1: none, character2, dialogue2, paren2: none) = grid(
  columns: 2,
  dialogue_raw(character1, dialogue1, paren: paren1, 1in, 0.5in, 1in),
  dialogue_raw(character2, dialogue2, paren: paren2, 1in, 0.5in, 1in),
)

#let lyrics(cont) = {
  block(
    upper(cont),
    above: line_spacing(0),
    inset: (left: 1in, right: 1.5in),
  )
}

#let parenthetical(content) = {
  context{
    block(
      par([#content], hanging-indent: measure("(").width),
      inset: (left: 0.5in, right: 1in), spacing: line_spacing(0)
    )
  }
}

#let scene(cont) = {
  heading(block(upper(cont), above: line_spacing(2), below: line_spacing(1)))
}

#let centered(cont) = {
  block({
    align(center, block(cont))
  }, breakable: false, width: 100%, below: line_spacing(2))
}

#let transition(name) = {
  align(right, block([#upper(name)], spacing: line_spacing(1), inset: (right: 2em)))
}

#show: screenplay.with(
  titlepage: true,
  title: [#text(" METAspexet 2021/2022")],
  credit: [#text(" written by")],
  authors: [#text(" METAspexets manusgrupp")],
  source: none,
  draft_date: none,
  contact: none,
)
#pagebreak()
#text(".")#text(weight: "bold","AKT 1")
#text("  ")
#text(weight: "bold","Musiknummer 1")
#text("[[Karaktärerna turas om att sjunga om kriget och platsen de befinner sig på. Välkomnar de nya rekryterna (och publiken) till settingen. Hemskt och jobbigt, men man måste hålla god min.]]")
#pagebreak()
#scene[#text("SCEN 1")]
#text("[[")#text(style: "italic","Karaktärer: Klimt, Schumacher, Müller, Flammenwerfer")#text("]]")
#text("[[")#text(style: "italic","Plats: Centralmaktsläger")#text("]]")
#text("[[Vi introduceras till centralmaktssoldaterna Klimt och Schumacher samt att det snart är jul. Flammenwerfer introduceras. Müller introduceras och etableras som person med makt men som inte är så närvarande hela tiden. Det framgår att det är ganska lugnt på fronten och centralmaktssoldaterna inte vill kriga.]]")
#text("EXT. CENTRALSMAKTSLÄGRET")
#text("I centralmaktslägret sitter Klimt och läser ”Västnytt”. Schumacher spatserar in. [[En generell kommentar för hela scenen: Det skulle vara roligt om man kunde få in ett \"Varför krigar vi, nu igen?\" skämt i denna scen så att skämtet är fastställt tidigt. ]] [[Bra idé]]")
#dialogue[#text("SCHUMACHER")][#underline[#text("Du! Klimt")]#text("! Du, såg du de där duvorna flyga förbi? Det kanske är franska brevduvor? [[https://en.wikipedia.org/wiki/Ehrhardt")#underline[#text("E-V/4")] #text("https://en.wikipedia.org/wiki/List")#underline[#text("of")]#text("combat")#underline[#text("vehicles")]#text("of")#underline[#text("World")]#text("War")#underline[#text("I")] #text("idk...]] [[Högst skeptisk mot detta, tbh]]")]
#text("Klimt lyssnar inte så noga utan mumlar mest något till svar.")
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(lite exalterad)")] #text("Jag har hört att i Frankrike så kallar de dem för brieduvor. För att fransmän alltid skickar med brieost med sina brev. [[Fämst på grund av detta]]")]
#dialogue[#text("KLIMT")][#underline[#text("Mmm, absolut, Schumacher")]#text(". Fåglar, Frankrike, prästost. Jättefint.")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(Fundersamt)")] #text("Hmm, tror du att duva skulle smaka gott med brieost? Kanske svartpeppar? Men jag har ingen svartpeppar…") #text("Svartkrut! Ja, det kan jag använda! Det smakar säkert kryddigt! Jag ska gå och se om jag kan få tag på en duva!!")]
#text("Schumacher börjar springa iväg för att krydda en duva med krut men Klimt tar snabbt henne i armen.")
#dialogue[#text("KLIMT")][#text("Du borde nog inte gå iväg just nu, så nära inpå vårt skift. Och du kanske kan hitta på någon annan krydda än krut?")]
#dialogue[#text("SCHUMACHER")][#text("Hmm, du har kanske rätt.")]
#text("Klimt ser på Schumacher som att hon alltid har rätt och fortsätter läsa tidningen.")
#dialogue[#text("SCHUMACHER")][#text("Du, Klimt? Vad är det du läser för något?")]
#dialogue[#text("KLIMT")][#text("Det här? Det är tidningen, Västnytt.")]
#dialogue[#text("SCHUMACHER")][#text("Jaha, vad händer på fronten idag då? [[Vad tror vi om att tweaka detta till \"Vad händer idag då?\" varpå svaret blir \"inget nytt, iallafall\"?]] [[Jag lämnar det for now men ja, det är nog mer nice]]")]
#dialogue[#text("KLIMT")][#text("Inget nytt, i alla fall.")]
#text("Vänta en stund medans publiken gapskrattar. Klimt stänger sedan tidningen och vänder sig mot Schumacher. [[Hybrisen är på topp, ser jag ;)]]")
#text("KLIMT [[Likaså känns det weird att Klimt använder Schumachers förnamn hen borde använda \"menige Schumacher\", Schumacher eller Soldat. Men eftersom att det är lite casual så kanske Schumacher räcker
Första gången de säger varandras namn skulle man kunna inkluderar rang så att det untroduceras till publiken]] [[Håller med. Vi borde introducera rank lite snabbt tidigt och sedan göra oss av med det och bara använda någon form av namn utom när generalerna adresserar eller adresseras!]]
Men du, Schumacher, jag börjar bli lite hungrig. Vad bjuder du på idag?")
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(med en suck)")] #text("Korv.")]
#dialogue[#text("KLIMT")][#text("Åh, korv som är så gott! Och kanske lite sauerkraut till det?")]
#dialogue[#text("SCHUMACHER")][#text("Japp. Precis som igår. Och i förrgår. Och dagen innan det. Tänk om vi bara kunde inta Frankrike någon gång så hade vi kunnat äta något nytt! Jag ska säga dig, Klimt, att när jag och mamma reste till Kina, då blev vi bjudna på alla möjliga godsaker. Och samma sak var det i Amerika, i Egypten och till och med i Grekland. Där var det annat än korv, det är ett som är säkert.")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(lagom nyfiket)")] #text("Jaha? Och vad är det med Frankrike som skulle vara så speciellt om du nu har varit på alla de där ställena?")]
#dialogue[#text("SCHUMACHER")][#text("Ja men det är ju det jag inte vet! Det är det okända som lockar mig. Jag har endast hört rykten om vad som väntar mig på andra sidan -") #parenthetical[#text("(pekar ut mot fronten)")] #text("- det där… Men jag säger dig, Klimt, en dag ska jag dit.")]
#dialogue[#text("KLIMT")][#text("Jotack, men inte idag. Vi sitter fast där vi är, så du får nog nöja dig med korv och sauerkraut.")]
#dialogue[#text("SCHUMACHER")][#text("Men, Klimt, på tal om maten? Det var något jag tänkte fråga dig om. [[Känns konstigt att ha säger herr/fru tror att nog borde vara hela underofficer eller möjligen vilken specifik typ av underofficer t.ex andre sergant") #text("Om det blir för mycket att säga \"underofficer Klimt\" varje gång så kan man välja att bara säga Underofficer eller Klimt]] [[Enligt moderna standards ska det väl egentligen vara antingen \"Herr/Fru Underofficer\" eller \"Underofficer Klimt\". Men för att dialogen ska flyta bättre och med tanke på att de är kompisar skulle nog bara Klimt låta bäst]] [[Är det Schumacher som lagar maten? Vet ej varför jag fick för mig det, har vi sagt det någon gång?]] [[Det stämmer, Schumacher är ju den tyska kocken!]]")]
#dialogue[#text("KLIMT")][#text("Jaha?")]
#dialogue[#text("SCHUMACHER")][#text("Det råkar inte… vara någon ny leverans på väg? Någon mat som jag kan laga inför i övermorgon?")]
#dialogue[#text("KLIMT")][#text("I övermorgon? Nej, vadå, har du slut på sauerkraut eller?")]
#dialogue[#text("SCHUMACHER")][#text("Inte än, men det är ju jul! Det är klart man ska ha julmat till jul.")]
#text("Flammenwerfer sticker in huvudet på scenen. [[Precis som Kramer <3 Exakt som Kramer. Behåll att Flammenwerfer ska vara lite Kramer till manusgenomläsningen]]")
#dialogue[#text("FLAMMENWERFER")][#underline[#text("Flammenwerfer")]#text(" kan laga mat. [[Jag tror nästan att det funkar att Flammenwerfer tilltalar sig själv i tredje person med tanke på vem som spelar denne och hur denne har tolkat karaktären]]")]
#dual_dialogue[#text("KLIMT")][#text("NEJ!")][#text("SCHUMACHER")][]
#text("JA!")
#dialogue[#text("FLAMMENWERFER")][#text("Jag bara tänder en eld och slänger på lite korv. Inte så svårt.")]
#dialogue[#text("KLIMT")][#text("Flammenwerfer, sist du grillade korv brände du nästan ned lägret i processen!")]
#dialogue[#text("SCHUMACHER")][#text("Men det var en väldigt god korv.")]
#text("Klimt suckar åt Schumacher.")
#dialogue[#text("KLIMT")][#underline[#text("Hans Flammenwerfer. Du är sprängämnesspecialist.")]#text(" Låt Schumacher sköta matlagningen och så håller du dig så långt ifrån något brännbart som möjligt. [[Här hade jag hela namnet+roll i första versionen. Vet ej om det behövs längre.]]")]
#dialogue[#text("FLAMMENWERFER")][#parenthetical[#text("(drömmande)")] #text("Men det är så roligt med mat! Att se hur den går från en färg till en annan ju mer kärlek man ger den.")]
#dialogue[#text("SCHUMACHER")][#text("Du, Flammenwerfer… Du råkar inte veta något som kan tänkas ge lite extra smak till dagens middag? Något som har lite kick, du vet?")]
#dialogue[#text("FLAMMENWERFER")][#parenthetical[#text("(lyser upp)")] #text("Åh, Schumacher! Jag vet exakt vad du behöver!")]
#text("Flammenwerfer börjar springa därifrån. Klimt lyfter armen lite slött som om att hon tänkte på att springa efter men gör inte det. Müller kommer in och Flammenwerfer kolliderar nästan med honom, men stoppar sig själv och faller på marken direkt framför Müller. [[Något bättre anledning för Flammenwerfer att göra sorti skulle nog behövas]]")
#dialogue[#text("MÜLLER")][#text("Vad gör du på marken, soldat? Uppställning, genast!")]
#text("Flammenwerfer ställer sig snabbt upp. Klimt gömmer undan sin tidning.")
#dialogue[#text("KLIMT, SCHUMACHER, FLAMMENWERFER")][#parenthetical[#text("(honnör) [[De borde verkligen göra honnör när de hälsar på generalen, men jag vet inte riktigt hur de står på scenen. De ställer dig ju bredvid varandra senare.]] [[Jag tror att Joaquin och Erik löser det!]]")] #underline[#text("General von Müller!")]]
#dialogue[#text("MÜLLER")][#text("Soldater! Uppställning! Avlägg rapport!")]
#text("Klimt och Schumacher ställer sig hastigt bredvid Flammenwerfer i givakt.")
#dialogue[#text("MÜLLER")][#underline[#text("Fanjunkare Franziska Klimt!")]]
#dialogue[#text("KLIMT")][#text("General! Materialförrådet är städat, lådorna är staplade och inventarierna är inventerade.")]
#dialogue[#text("MÜLLER")][#text("Mycket bra. ")#underline[#text("Korporal Liesel Schumacher!")]]
#dialogue[#text("SCHUMACHER")][#text("Maten är snart klar! Det blir korv… som vanligt.")]
#dialogue[#text("MÜLLER")][#underline[#text("Specialist Hans Flammenwerfer.")]]
#dialogue[#text("FLAMMENWERFER")][#text("Jag är här!")]
#text("Müller stirrar strängt på soldaterna en kort stund men sedan spricker ett leende fram och han slår ut med armarna.")
#dialogue[#text("MÜLLER")][#text("Strålande! Lediga! [[Omformulera, få detta att låta mer naturligt]] [[Och mindre expositionit]] [[Bättre?]]")]
#text("Müller ändrar tydligt sitt sätt att föra sig på till ett mer informellt sådant.")
#dialogue[#text("MÜLLER")][#text("Jaha, ni har nästa vakt, eller hur? Allt redo?")]
#dialogue[#text("SCHUMACHER")][#text("Självklart, general von Müller.")]
#dialogue[#text("MÜLLER")][#text("Snälla, vi är alla vänner här. ")#underline[#text("Kalla mig general Müller.")]]
#text("Schumacher kastar en förvirrad blick mot Klimt.")
#dialogue[#text("MÜLLER")][#text("Så? Allt går bra alltså? Skönt att höra, fienden skakar säkert av rädsla på andra sidan. I den här takten har vi dem säkert besegrade ")#underline[#text("innan nyår")]#text(", eller vad säger ni, fanjunkare Klimt?")]
#dialogue[#text("KLIMT")][#text("Absolut, general Müller! Det är precis som du säger.")]
#dialogue[#text("MÜLLER")][#text("Mycket bra! Mycket bra! Nej, jag ville mest komma förbi och se hur mina soldater har det. Det gäller att ni är på er vakt. En bra soldat är alltid på alerten! Nåja, återgå till era poster. Jag har fler stationer att inspektera.")]
#text("Müller börjar vända sig för att gå av scenen men Schumacher sträcker upp handen.")
#dialogue[#text("SCHUMACHER")][#text("General Müller, får vi någon ")#underline[#text("julmat i övermorgon")]#text("?")]
#dialogue[#text("MÜLLER")][#text("Julmat? Vi har ett krig att vinna först. Vi måste lägga press på fienden och backa dem ända ner i havet! När vi väl intagit Paris kan vi festa på så mycket julbord vi vill. Seså, utgå! [[Müller säger mer \"absolut, men sen. När vi intagit Paris kan vi äta så mycket julbord vi vill!!\"]]")]
#text("Müller går av scenen. De andra slappnar av och vänder sig mot varandra. Schumacher suckar.")
#dialogue[#text("SCHUMACHER")][#text("Jag som hade hoppats att generalen skulle vara lite mer medgörlig.")]
#dialogue[#text("FLAMMENWERFER")][#text("Men se det från den ljusa sidan! Flammenwerfer kan förbereda julkorv! [[Ska Flammenwerfer verkligen prata om sig själv i tredje person? Det låter...lite dumt imo]] [[Jag tycker att vi behåller det och låter Axel avgöra själv hur dumt det blir. Kan vara en grej som funkar med rösten har ger karaktären]]")]
#dialogue[#text("KLIMT")][#text("Vill jag veta vad \"julkorv\" är?")]
#dialogue[#text("FLAMMENWERFER")][#text("Det är korv… med sådan där \"kick\" som du ville ha.")]
#dialogue[#text("SCHUMACHER")][#text("Jaaaa!! Kom igen, vi går till köket! Och du, du ska få smaka på en ny grej jag provat, jag hittade en rödvit svamp på baksidan av en hink förut som såg väldigt god ut.")]
#text("De alla lämnar scenen åt samma håll. Klimt ser lite orolig ut över att låta Flammenwerfer komma nära köket.")
#pagebreak()
#scene[#text("SCEN 2")]
#text("[[")#text(style: "italic","Karaktärer: Tolkien, Nightingale, Senap, General Bollbirth, Cartier")#text("]]")
#text("[[")#text(style: "italic","Plats: Allierat läger")#text("]]")
#text("[[Vi introduceras till de allierade soldaterna genom att Nightingale introduceras till dem. Cartier introducerar sig själv iochmed att ingen annan gör det. Bollbirth och Senap lämnar för att diskutera i generalens tält. Scenen slutar med att resterande tar skydd från ett kraschande plan.]]")
#text("EXT. DET ALLIERADE LÄGRET")
#text("Löjtnant Senap vankar fram och tillbaka på scenen medan Tolkien sitter på andra sidan scen och skriver. Cartier står och röker i ett hörn. ")
#dialogue[#text("SENAP")][#parenthetical[#text("(Muttrar argt för sig själv)")] #text("Det här är fasansfullt! Det duger inte, det är fullständig skandal! Vad har hänt med soldatandan sedan jag var en rekryt?! Det är skräp och smuts i hela lägret och det är bara några minuter innan-")]
#text("Ett trumpetfanfar ljuder genom luften och alla reser sig upp och kollar omkring. Senap ställer sig i en riktigt ordentlig givakt, och kollar lite surt på Tolkien och Cartier när de inte gör det.")
#dialogue[#text("SENAP")][#text("Uppställniiiiing! Skärpning i ledet! Nya rekryter på väg. Visa dem ett varmt välkomnande, soldater!")]
#text("Tolkien gör ett ärligt försök till givakt, men med väldigt dålig hållning. Cartier bara himlar med ögonen och muttrar något om britter. Senap hinner dock inte göra något åt honom innan Nightingale och en grupp soldater kommer in. Nightingale stannar och vinkar medan de andra går vidare. [[Tänker att det kan bli lite kul om Senap övertänker och försöker göra allting så seriöst för Nightingales anländande, för att denne sedan ska komma in supercasual och väldigt oformellt]]")
#dialogue[#text("NIGHTINGALE")][#text("Det är hit jag ska! Lycka till på era positioner också! Ni vet var jag finns om ni behöver få något amputerat!")]
#text("Resten av de nya soldaterna fortsätter bort, medan Nightingale vänder sig Senap och kompani.")
#dialogue[#text("SENAP")][#text("Välkommen! ")#underline[#text("Löjtnant Janice Senap")]#text(", till er tjänst.")]
#text("Senap sträcker fram handen mot Nightingale. ")
#dialogue[#text("NIGHTINGALE")][#underline[#text("Philip Nightingale, läkare och blivande fältsjukvårdare.")]#text(" Trevligt att träffas.")]
#dialogue[#text("SENAP")][#text("Ja, det är ju alltid välkommet med fler sjukvårdare. Du kommer att vara under mitt befäl, och jag kommer inte att acceptera någon slöhet.")]
#dialogue[#text("NIGHTINGALE")][#text("Utmärkt! Bara peka mig mot fältsjukhuset så kommer jag igång direkt. Kanske får jag en chans att testa min nya bensåg! Jag kan knappt bärga mig! [[Jag tycker att skottskada kommentaren kan utvecklas på. Jag tycker att den kan vara lite längre och mer morbid för att stärka Nightingales karaktärsdrag tidigt. ]] [[Håller med]] [[\"Jag kanske får se en riktig skottskada! Eller en amputation! Eller en...\"]] [[Om Nightingale har jobbat förut har han säkert redan sett \"en riktig skottskada\" så det går inte ihop. Annars kan man ändra till \"färsk skottskada\" ?]]")]
#text("Nightingales ansikte lyser upp. Tolkien skruvar på sig lite obekvämt. Senap blir lite tagen på sängen av kommentaren och stirrar en stund på Nightingale som inte inser hur morbid den var. [[Kommentaren om skottskadan är ju borta nu]]")
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(Mumlar lite nerstämd för sig själv)")] #text("Jag hade gärna bara läst om det…")]
#dialogue[#text("SENAP")][#text("Eh… ja, det kan mycket väl bli så att du får se alla möjliga skador som kan uppstå på ett slagfält som detta… [[Potentiellt omfrasera]]")]
#text("Senap gör en gest mot Tolkien och Cartier som står bakom henne.")
#dialogue[#text("SENAP")][#text("Men låt oss fokusera på att göra dig hemmastadd i lägret.")]
#dialogue[#text("SENAP")][#underline[#text("Menige Tolkien, Cartier")]#text("…")]
#dialogue[#text("CARTIER")][#parenthetical[#text("(Rättar Senap med väldigt irriterad röst. Obs, detta är på franska.)")] #underline[#text("Soldat Cartier")]#text(".")]
#dialogue[#text("SENAP")][#parenthetical[#text("(Ignorerar att Cartier avbröt henne)")] #text("…ta tillfället i akt och bekanta er med vår nya sjukvårdare. Han kan komma att rädda era liv.")]
#text("Cartier står kvar där han står men tar cigaretten ur mungipan.")
#dialogue[#text("CARTIER")][#underline[#text("Claude Cartier, ensam fransman")]#text(", fången med alla dessa okultiverade svin i denna lerpöl. [[Kan vi tweaka den här linen så att det framgår tydligare att Cartier inte vill ens yttra att han är budbärare?]]")]
#text("Cartier fortsätter att röka. Tolkien sträcker på sig och sträcker fram handen.")
#dialogue[#text("TOLKIEN")][#text("Ja, hej, trevligt trevligt, ")#underline[#text("John Tolkien")]#text(", till er tjänst.")]
#text("Nightingale tar Tolkiens hand.")
#dialogue[#text("NIGHTINGALE")][#underline[#text("Philip Nightingale")]#text(". Ett nöje.")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(munter)")] #text("Åååh, vad jag längtat till att få komma hit, till krigets front! Det finns verkligen ingen mer lärorik och utmanande plats för en läkare. Visste ni att en på tre soldater förväntas behöva en blodtransfusion innan sommaren? Och, ja, vi har verkligen inget överskott på blod just nu, haha. [[Har väldigt svårt för Nightingales personlighet...]] [[\"Och som ni kan tänka er är det inte så lätt att få tag på blod nuförtiden, så... ja haha\"]] [[Om man vill göra det lite värre eller tydligare typ]] [[Det är ju typ bara en vecka kvar på året. Det blir väldigt många på kort tid liksom.]] [[Ja, bra lösning]]")]
#text("Tolkien ser skärrad ut. Senap tittar på sig själv, Cartier, och Tolkien i tur och ordning.")
#dialogue[#text("CARTIER")][#text("Honhonhon, jag skulle hellre gå en månad utan en enda baguette än att låta en britt operera på mig! Och jag ska säga dig, plåsterpojke, att jag äter väldigt mycket baguetter. [[Vissa vill ha tillbaka plåsterpojke...]]")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(Stelt skratt)")] #text("Ahaha, ni fransmän är allt för roliga, ni…")]
#text("Cartier bara himlar med ögonen och röker vidare. Väldigt tydlig med att han inte skämtar.")
#dialogue[#text("NIGHTINGALE")][#text("Men, ni som varit här länge; berätta, berätta! Hur är livet på fronten? Hur ser det ut ute i ingenmansland?")]
#dialogue[#text("CARTIER")][#text("Hah! Ingenmansland? ")#underline[#text("Vi har knappt SETT en tysk på månader.")] #text("Om de bara hade vettet att skicka ut några med lite ryggrad, såsom jag själv: då hade kriget varit slut för länge sedan.")]
#dialogue[#text("TOLKIEN")][#text("Eh, ursäkta honom, han är bara lite upprörd för att ")#underline[#text("han blev brevbärare")]#text(" istället för att få slåss. Men han har en poäng - vi har sett väldigt lite av ingenmansland, är jag rädd. ")#underline[#text("Vi bombarderar mest fienden, och fienden bombarderar oss.")]]
#text("Senap tar sig in i diskussionen.")
#dialogue[#text("SENAP")][#text("Bombardemang och bombardemang… Bara bombardemang! På min fars tid var det allt annorlunda. Då stormade man fienden i ärofyllda slag, istället för att bara sitta i en lerpöl.")]
#text("Strålkastare på Tolkien.")
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(Pratar väldigt målande)")] #text("Men om du verkligen vill veta, livet på fronten är lite som livets mynt. På ena sidan, en katalysator för alla mänsklighetens ondskor som får oss att förtränga vår egen medkänsla i ett sista desperat försök att få se morgondagens ljus. Men å andra sidan, finns det en ovanlig skönhet när solen målar landskapet varmt i morgonljuset, och daggen blänker som diamanter, samtidigt som man är omgiven av vänner som i hemskheten delar en helig länk med varandra som inte kan fås förutan ett gemensamt hot av döden. Rädslan av vetskapen om att du kan sprängas bort vilken minut som helst - samtidigt, tristessen från vanan av att inte ha någon inverkan på sitt öde. Varje morgon singlas slanten, och varje kväll ser man vilken sida den landat på, om man får fortsätta till nästa dag… [[Superbra replik! Hoppas att Emil kan lära sig den och att den inte är för lång, den fångar verkligen Tolkiens sätt att se på världen]]")]
#text("Kort paus då Tolkiens ord får sjunka in.")
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(Lite tagen)")] #text("Oj… wow. Du har en talang för ord du. Aldrig funderat på att skriva en bok?")]
#text("Senap avbryter deras samtal.")
#dialogue[#text("SENAP")][#underline[#text("General Bollbirth")]#text(" är här! Giiiivakt!")]
#text("Bollbirth kommer in på scenen, Senap gör honnör och Cartier rätar motvilligt till sig. Tolkien gömmer sig lite instinktivt bakom Nightingale som blir lite förvånad men som snart vänder sin uppmärksamhet mot Bollbirth.")
#dialogue[#text("BOLLBIRTH")][#text("Lediga!")]
#text("Bollbirth vänder sig till Senap.")
#dialogue[#text("BOLLBIRTH")][#underline[#text("Löjtnant Senap")]#text("! Avlägg rapport!")]
#dialogue[#text("SENAP")][#text("Allt tyst på fronten, general. Tyskarna verkar gömma sig i sina skyttegravar. ")#underline[#text("Vår nya fältläkare har också precis anlänt")]#text(".")]
#text("Bollbirth lägger märke till Nightingale.")
#dialogue[#text("BOLLBIRTH")][#text("Ah, välkommen till fronten. Din ankomst är inte en minut för sen. Jag har en massa soldater som behöver lappas ihop innan söndag. De har en front att försvara!")]
#dialogue[#text("NIGHTINGALE")][#text("Det ska bli… Men det är inte långt till söndag, och är de skadade så måste de få vila lite…")]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(Verkar inte ens höra ett skit av Nightingales mesiga snack om medkänsla)")] #text("Bad jag om din åsikt? Skynda dig till sjukstugan så snart du packat upp. Jag har ett krig att vinna.")]
#text("Bollbirth vänder sig till Senap.")
#dialogue[#text("BOLLBIRTH")][#text("Löjtnant Senap. Följ med mig till mitt tält. ")#underline[#text("Jag har ett par viktiga nya planer att diskutera med dig")]#text(".")]
#dialogue[#text("SENAP")][#text("Självklart, general!")]
#text("Bollbirth och Senap marscherar av scenen. ")
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(inte imponerad)")] #text("Vilken solstråle.")]
#text("Tolkien stiger lite skamsen fram från bakom Nightingale.")
#dialogue[#text("TOLKIEN")][#text("Generalen kan vara väldigt barsk ibland. Men hon har rätt i att vi behöver en läkare. Den förra var blodrädd, så sist jag skar mig i fingret behövde vi kalla in grannbataljonens läkare efter att han svimmade och…")]
#text("De blir avbrutna att de hör ett avlägset surrande. Nightingale ställer sig och spanar mot något i fjärran.")
#dialogue[#text("NIGHTINGALE")][#text("Vad är det där? Är det en fågel? En flygande man i blå pyjamas och röd mantel? Eller är det…")]
#dialogue[#text("TOLKIEN")][#underline[#text("Ett flygplan!")]#text(" Ta skydd!")]
#text("Tolkien slänger sig ned och drar med sig Nightingale. Cartier står kvar nonchalant och tittar bort mot där planet till synes störtar bortom scenen.")
#text("Efter en komiskt lång tystnad tittar Tolkien upp.")
#dialogue[#text("TOLKIEN")][#text("Har det försvunnit? Är kusten…")]
#text("Tolkien hinner inte prata färdigt innan planet kraschar i en stor explosion.")
#pagebreak()
#scene[#text("SCEN 3")]
#text("[[")#text(style: "italic","Karaktärer: Senap, Bollbirth")#text("]]")
#text("[[")#text(style: "italic","Plats: Bollbirths tält")#text("]]")
#text("[[")#text(style: "italic","Vi får reda på att de allierade planerar Operation Senapsgas. Bollbirth berättar om att shit is about to go down. Senap är högst skeptisk.")#text("]]")
#text("INT. BOLLBIRTHS TÄLT")
#text("Senap och Bollbirth kliver in i tältet. ")
#dialogue[#text("SENAP")][#parenthetical[#text("(Andfådd)")] #text("Måste de verkligen lägga det så högt upp bara för att det är ett högkvarter…")]
#dialogue[#text("BOLLBIRTH")][#text("HA! Som att någon som jag skulle leva nere i leran med patrasket. Nej, här uppe kan jag övervaka min armé och styra det här kriget till en säker vinst.")]
#dialogue[#text("SENAP")][#text("Självklart, general Bollbirth! Jag menade inte att ifrågasätta dig. [[Vet inte riktigt om vi vill använda sir eller inte. Kände att det behövdes något ord för senap att visa att hon både följer hierarkin och verkligen ser upp till Bollbirth (i alla fall hennes position). Kan inte svenskt militärspråk]]")]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(Väldigt överlägset)")] #text("Gör det inte till en vana bara.")]
#text("Bollbirth går och lutar sig över ett stort bord med kartor på.")
#dialogue[#text("BOLLBIRTH")][#text("Nåväl, jag kallade inte hit dig för småprata. Jag har stora planer på gång som kommer revolutionera kriget. Aldrig mer kommer vi behöva förlita oss på odugliga fotsoldater. ")#underline[#text("Med vårt nya vapen")]#text(" blir vinsten vår i ett enda knapptryck. ")#underline[#text("En spion är på väg hit just nu med receptet på detta nya supervapen!")]]
#text("Bollbirth ställer sig med armarna rakt ut och talar väldigt mycket som en Disney-skurk. (kanske kan någon storslagen musik spelas här)")
#dialogue[#text("BOLLBIRTH")][#text("SEEENAAAP…")]
#dialogue[#text("SENAP")][#parenthetical[#text("(Ställer sig i honnör)")] #text("Ja, general! [[Vet inte riktigt om det här skämtet fungerar, men det är om man kan få det att verka som att hon trodde hon blev tilltalad]] [[Tycker att vi kan prova, iallafall]]")]
#text("Musiken stoppar tvärt och Bollbirth skakar på huvudet.")
#dialogue[#text("BOLLBIRTH")][#text("Nej! Inte du, Senap. Jag pratar om ")#underline[#text("vårt nya supervapen")]#text(":")]
#text("Bollbirth ställer sig likadant som hon stod innan hon blev avbruten. (musiken hoppar direkt upp till samma storslagna idé som innan)")
#dialogue[#text("BOLLBIRTH")][#underline[#text("SEEENAAAPSGAAAS!")]]
#text("Bollbirth står kvar så nästan komiskt länge. Senap klappar lite försiktigt händerna.")
#dialogue[#text("SENAP")][#parenthetical[#text("(Tveksam)")] #text("Låter fantastiskt, men ursäkta min okunnighet, men jag förstår inte riktigt hur lite gas kommer hjälpa oss att vinna kriget…")]
#dialogue[#text("BOLLBIRTH")][#text("Du förstår, löjtnant Senap, att det här är inte vilken gas som helst. Det här är ett vapen på en helt ny nivå! ")#underline[#text("Senapsgas")]#text(" kan slinka in genom de minsta springor och nå varenda soldat i deras läger. Det kommer sprida terror över hela fronten. Kan du tänka dig, Senap: en hel bataljon fullständigt eliminerade på en vindstilla eftermiddag, är det inte underbart!? [[Jag insåg nu att det i synopsisen står att Bollbirth ska säga det riktigt brutala efter att  Senap har gått, men får ändra det sen]]")]
#dialogue[#text("SENAP")][#text("Eeeeeh, är du helt säker på det här, general? Jag menar, kriget går ju ändå helt… okej. Slåss med gas… Låter inte det lite… ohederligt? [[Senap borde nog vara mer fokuserad på att behålla heder snarare än att ses som god. Hinner inte skriva om nu]]")]
#dialogue[#text("BOLLBIRTH")][#text("Ohederligt!? HAHA! ")#underline[#text("Det finns ingen heder i skyttegravar")]#text(". Senap, som befäl har vi en skyldighet att avsluta det här kriget snabbt, vilka medel de än må vara.")]
#dialogue[#text("SENAP")][#text("Om du säger det så…")]
#dialogue[#text("BOLLBIRTH")][#text("Så ska det låta. Jag vill att du går och ser till att ")#underline[#text("vi är redo för massproduktion så fort vi får receptet")]#text(". Seså.")]
#text("Bollbirth tar ett papper (antagligen med instruktioner) från sitt skrivbord och ger det till senap innan hon avfärdar henne med en vinkning. Senap gör honnör.")
#dialogue[#text("SENAP")][#text("Ja, general!")]
#text("Senap lämnar tältet med skeptisk uppsyn.")
#dialogue[#text("BOLLBIRTH")][#text("Pttf, ")#text(style: "italic","heder")#text("! Förstår ingen att jag har ett krig att vinna! Och när jag väl får tag i min senapsgas, då kan jag äntligen göra slut på det här ändlösa dödläget. När ")#underline[#text("General Müller")]#text(" är ur vägen så slår vi oss fram hela vägen till Berlin, oavsett hur många insekter jag måste offra på vägen. Om nu bara den där ")#underline[#text("spionen")]#text(" kunde komma hit någon gång så att jag kan få mitt recept. [[tänker att det kan vara snyggt att nämna müller här inför musiknummret. tankar?]] [[Jag gillar fortfarande inte att Generalerna pratar om varandra så jag röstar nej]]")]
#text("  ")
#text(weight: "bold","Musiknummer 2")
#text("[[Generalerna börjar sjunga en låt om att det nu är dags att ta kriget till nästa nivå. Soldaterna har sedan ångest och vill inte kriga. Senap sjunger om att leva upp till sin far och sina tvivel.]]")
#text("  ")
#pagebreak()
#scene[#text("SCEN 4")]
#text("[[")#text(style: "italic","Karaktärer: Röde Baronen, Schumacher, Klimt, Flammenwerfer")#text("]]")
#text("[[Plats: Centralmaktsläger med flygplanskuliss]]")
#text("[[Röde Baronen gör entré och får storyn i rullning. En väska med hemliga dokument har ramlat ur planet medan det kraschade. Röde Baronen kravlar sig ur planet och borstar av sig smutsen. Röde Baronen monologar hur denne är en mullvad som för de allierade och har förlorat en väska med viktiga dokument. Dokumenten är receptet på senapsgas. Rekryterar centralmaktssoldaterna för att ta sig ut i ingenmansland och hitta väskan.]]")
#text("EXT. CENTRALMAKTSLÄGRET [[Alternativt de är i en bit av ingenmansland?]]")
#text("Röde Baronen snubblar in på scen. Har en fallskärmsryggsäck på sig med klippta trådar släpandes efter sig.")
#dialogue[#text("RÖDE BARONEN")][#text("Nej, nej, NEJ! Mitt plan! Min plan! Hur ska den kunna utföras nu… SCHEISSE!! [[Osäker på om publiken kommer fatta denna. ]] [[Den otroliga poesin, menar du? I dunno, de som fattar får fatta. Inte som att man blir förvirrad om man inte förstår den]]")]
#text("Schumacher, Klimt och Flammenwerfer kommer inspringande från andra sidan. De ser mycket oroliga ut, men blir lättade när de ser Röde Baronen, som inte har sett dem än.")
#dialogue[#text("FLAMMENWERFER")][#parenthetical[#text("(mot planet)")] #text("Wow! Vilken jävla smäll!")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(mot Röde Baronen)")] #text("Hallå, mår du bra!? Är du skadad?? [[Potentiellt ge Klimt mer personlighet. Hon gör mer vad som behövs än vad hon VILL.]] [[Vad vill hon då? Vi sa förut att Klimt är en mycket rimlig figur, så hennes reaktion på hela situationen är mest baserat på (hur jag antog) en vuxen person som är rimlig skulle ha hanterat det. Det kan ju också visa på karaktär, om man gör det som behövs eller det som man själv tror behövs. Eller?]] [[I just den här linen är Klimt fine. Det här var mer en generell kommentar för Klimt. Men det är egentligen mest i scen 1 vi vill ändra henne, inte här.]]")]
#text("Baronen vänder sig om, mycket förvirrad.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(defensivt)")] #text("Woah, woah, woah. Stanna där ni är.")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(seriöst)")] #text("Oj oj, ta det lugnt, bara lugn. Vi såg planet som kraschade. ")#underline[#text("Du måste vara piloten")]#text(", inte sant?")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(med en gnutta osäkerhet)")] #text("Uh, ja. Det stämmer… Jag är en pilot, ja. Inget annat. Och, ni, ni är… soldater? Från Cen- [[Viktigt att detta inte framstår som att han är förvirrad från kraschen, utan att han försöker att inte avslöja för mycket om sig själv. Vet inte hur man gör det tydligare tho.]] [[Kanske typ \"jag är en hemlig spio... jag menar bara en vanlig pilot\" fast såklart lite snyggare skrivet. Skulle nog göra det tydligt för publiken iaf, men kanske blir svårt att inte få de andra att verka dumma för att de inte fattar]]")]
#dialogue[#text("SCHUMACHER")][#text("Centralmakterna, ja! Vi är tyskar, precis som du!!")]
#dialogue[#text("RÖDE BARONEN")][#underline[#text("Jag är faktiskt ungrare…")]]
#dialogue[#text("KLIMT")][#text("Du ser inte ut att må bra, vill du ta och komma med oss till lägret? Vi kan säkert hitta en dokt- [[Ju längre de pratar, desto onaturligare låter det att de niar baronen]] [[Låter skådis avgöra detta, tbh]]")]
#text("Baronen rätar snabbt på sig och försöker få kontroll över situationen.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(Samlat)")] #text("Aahah, nej, nej, det är väldigt snällt av er, men det behövs inte. Det är inte första gången jag har gjort en sådan här manöver.")]
#dialogue[#text("FLAMMENWERFER")][#text("Inte? Brukar du hoppa ut ur brinnande plan på din fritid??")]
#dialogue[#text("RÖDE BARONEN")][#text("Hah, inte på fritiden, men det är sånt som händer när man träffas mitt i luften! Även om det i detta fall bara var problem med motorn… Men nog om det. Ursäkta mig, jag tror inte ni nämnde era namn…? [[Blev han träffad? Var det inte bara motorn som gick sönder? Eller refererar han till andra gånger han har kraschat?]] [[han refererar till de gångerna han blivit träffad förut, men jag kan göra det tydligare]] [[Försökt fixa]]")]
#text("Röde Baronen skakar hand med alla tre medans de presenterar sig själva.")
#dialogue[#text("KLIMT")][#text("Franziska Klimt. Mycket trevligt att träffas. [[Klaura, never forget...]] [[Tror att du stavade Liesel fel ;) Det var Schumacher som ni ville kalla Klaura, så uppenbarligen har du glömt redan XD]] [[Va?! Det måste vara denna sjukdom som slagit ut mina synapser helt enkelt.]]")]
#dialogue[#text("RÖDE BARONEN")][#text("Mycket.")]
#text("Schumacher ger knappt Röde Baronen chans att prata klart med Klimt innan hon tränger sig fram för att entusiastiskt introducera sig.")
#dialogue[#text("SCHUMACHER")][#text("Liesel Schumacher. Det är så fantastiskt att få träffa någon ny.")]
#dialogue[#text("RÖDE BARONEN")][#text("Trevligt, trevligt.")]
#dialogue[#text("FLAMMENWERFER")][#text("Hans Flammenwerfer.") #parenthetical[#text("(de skakar hand)")] #text("…Hmm, givet explosionens styrka, det klar-röda ljuset och doften av brandrök med toner av motorolja och granbarr… Du flyger en Siemens-Schuckert, inte sant? [[Alt. lukten av brandrök med toner av motorolja och granbarr. eller nåt]]")]
#dialogue[#text("RÖDE BARONEN")][#text("Ahah… Ja, det gör jag. Gjorde jag.")]
#dialogue[#text("FLAMMENWERFER")][#text("Röd läderjacka, bländande leende och dessutom en Siemens-Schuckert….")]
#text("Flammenwerfer flämtar. ")
#dialogue[#text("FLAMMENWERFER")][#text("Nej… Det kan inte vara… Är du… ")#underline[#text("Jorji von Richthofen")]#text("? ")#text(style: "italic","Den")#text(" ")#underline[#text("Jorji von Richthofen")]#text("?!?")]
#text("Baronen är först lite tagen av att Flammenwerfer kan hans namn, men ser sedan beundran i Flammenwerfers ögon och sträcker lite på sig. [[Alternativt, om man vill att RB ska vara mer diskret, kanske han undviker frågan lite, och så kommer Flammenwerfer själv på vem han är, varpå RB inser att Flammenwerfers dyrkande av denne går att utnyttja]] [[Bra poäng. Ska överväga detta]]")
#dialogue[#text("RÖDE BARONEN")][#text("Jo, det är jag. Du har ett skarpt öga, som känner igen mig så snabbt, det ska jag medge. [[Här ska det ju också märkas att han inser att han kan vända detta till sin fördel, inte bara att han gillar uppmärksamheten]] [[ngr förslag? Jag provade en grej men är onöjd med formuleringen.]] [[Gjorde ett försök]] [[Jag älskar inte denna, men jag funderar på hur man kan säga det istället. Mer självsäkert typ.]] [[Kanske: \"Hah, det är jag. Skarp du är, som genast känner igen mig.\"]]")]
#dialogue[#text("FLAMMENWERFER")][#parenthetical[#text("(exalterat)")] #text("JAG KAN INTE TRO DET! DEN SJÄLVASTE ")#underline[#text("RÖDE BARONEN")]#text("! DET ÄR EN SÅN ÄRA!")]
#text("Flammenwerfer tar tag i Röde Baronens hand och skakar den igen, denna gång med mycket mer entusiasm.")
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(viskar till Klimt)")] #text("Vem?")]
#text("Klimt rycker på axlarna.")
#dialogue[#text("FLAMMENWERFER")][#text("Helt otroligt! Att jag skulle få träffa den legendariske hjälten som lyckades ta sig ur ett motorhaveri genom att kapa ett fiendeplan i luften! [[Lade till lite ord för att förtydliga var betoningen ligger, är inte helt säker på att jag tolkade det rätt tho.]] [[Ser bra ut]] [[Hitta på fiskehistoria]] [[vad betyder det ens]]")]
#text("(Lämna rum för eventuella omstarter)")
#dialogue[#text("RÖDE BARONEN")][#text("Äsch, det var väl inget att tala om.")]
#dialogue[#text("FLAMMENWERFER")][#text("Så ödmjuk du är!! Precis som en riktig hjälte!!")]
#dialogue[#text("SCHUMACHER")][#text("Borde vi känna igen honom?")]
#dialogue[#text("KLIMT")][#text("Kanske?")]
#dialogue[#text("FLAMMENWERFER")][#text("Men vad gör en sån stor hjälte som du här?")]
#dialogue[#text("RÖDE BARONEN")][#text("Jag är ju på ett superviktigt uppdrag såklart. En… eh, ")#underline[#text("leverans")]#text(", kan vi säga.")]
#text("Det blir tydligare att Röde Baronen väljer sina ord försiktigt för att inte avslöja för mycket. Schumacher blir väldigt nyfiken. [[Jag är osäker om den här behövs. Detta kan läsas mellan raderna och blir nog ännu tydligare manuset spelas upp. Vad tycker andra om detta?]] [[Ja, det är bara för att göra det tydligt för sqådis+ regi vad vi har tänkt oss]] [[Okej, men då tycker jag att vi behåller detta. ]]")
#dialogue[#text("SCHUMACHER")][#text("Sa du… leverans? Vad för leverans? [[Förslag: Jag visste det, Generalen ville överraska mig med en leverans av julmat ändå!]]")]
#dialogue[#text("RÖDE BARONEN")][#text("Ledsen, ni vet hur det är, sånt här information är alltid extremt ")#underline[#text("hemligstämplad")]#text(". Om jag berättar för er så måste jag tyvärr döda er.")]
#dialogue[#text("FLAMMENWERFER")][#text("Så coolt! ")#underline[#text("Var är lasten, då")]#text("?")]
#dialogue[#text("RÖDE BARONEN")][#text("Heh, den har jag i säkert förvar hä-")]
#text("Röde Baronen klappar sig på bröstet/magen när han säger \"i säkert förvar\". Får ett oroat ansiktsuttryck och klappar sig panikslaget över jackan.")
#dialogue[#text("KLIMT")][#text("Är allt bra?")]
#dialogue[#text("RÖDE BARONEN")][#text("Nej. Nej, NEJ! Var är den?!")]
#text("Baronen börjar frenetiskt titta runt sig. Flammenwerfer börjar också leta för att hjälpa, utan att veta vad han letar efter. ")
#dialogue[#text("KLIMT")][#text("Vad är det som du har tappat bort?")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(fortfarande letandes)")] #text("En väska! ")#underline[#text("En väska som innehåller senaaa- mycket viktiga")]#text("… ")#underline[#text("innehåll. Den måste ha varit kvar i planet.")]]
#text("Schumacher, som fortfarande har varit nyfiken och misstänksam, brister ut i ett glatt litet tjut. [[Kanske revidera]]")
#dialogue[#text("SCHUMACHER")][#text("Sa du senap!? Jag visste det! Det blir julmat ändå! Hörni, vi MÅSTE hjälpa honom!")]
#dialogue[#text("FLAMMENWERFER")][#parenthetical[#text("(till de andra två)")] #text("Ja, självklart, der ")#underline[#text("Rote Baron är Europas största krigshjälte")]#text("! Vi kan inte INTE hjälpa honom!")]
#dialogue[#text("RÖDE BARONEN")][#text("Exakt, Flammenwerfer, exakt! Och tänk, ni kanske får en medalj om ni hjälper mig. Eller ännu bättre: exposure! Kom igen nu. [[Denna line och följande måste det vara tydligt att RB försöker manipulera/övertala soldaterna att hjälpa honom, snarare än att han blint förväntar sig att de kommer göra det]] [[Manipulation på max]] [[alltså mer manipulativt? har du förslag?]] [[Haha, nä :) inget behöver ändras, det vara bara en observation]] [[Bör RB verkligen använda förnamn på Flammenwerfer? ]] [[Vi snackade om att han skulle göra det för att vara mer inställsam, men jag tror också att om publiken inte har fått höra det namnet så ofta kan det bli mer förvirrande, och jag tror vi kan visa inställsamhet utan att använda förnamn]]")]
#dialogue[#text("KLIMT")][#text("Alltså, nu? Ut i Ingenmansland??")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(Lite tryckande)")] #text("Ja, nu på en gång. Ni är väl hederliga soldater som hjälper hjältar och kamrater med viktiga uppdrag, eller hur?  [[\"soldat\" blir aningen repetitivt här]] [[Bra poäng!]] [[Tack så mycket chefen]] [[Inga problem, gruppmedlem]]")]
#dialogue[#text("KLIMT")][#text("Jag antar det…")]
#dialogue[#text("FLAMMENWERFER")][#text("DET VORE EN ÄRA!")]
#text("Flammenwerfer saluterar entusiastiskt. Precis då hörs det en explosion från lägret. Alla vänder sig mot Flammenwerfer.")
#text("FLAMMENWERFER [[Man kanske vill ge lite mer context till explosionen, så att inte publiken tror att det är relevant]] [[den får kontext i scen 6 tho]] [[Ah, ok. I så fall]]
Jag… Kanske borde gå och se till det där. MEN BERÄTTA ALLTING SEN!")
#text("Flammenwerfer springer snabbt iväg mot lägret.")
#text("(Idéer till omstart: Istället för explosion säger FW: Jag måste skriva till mamma!!)")
#dialogue[#text("RÖDE BARONEN")][#text("Jaha, då blir det bara ni två. Kom igen, då.")]
#text("Röde Baronen börjar fösa soldaterna mot Ingenmansland.")
#dialogue[#text("KLIMT")][#text("Men generalen-")]
#dialogue[#text("RÖDE BARONEN")][#text("-kommer vara mycket glad att ni hjälper vår sidas största krigshjälte! Seså, marsch pannkaka!")]
#text("Röde Baronen har en monolog i sitt huvud. De andra fryser till och han kliver fram och får en spotlight på sig.")
#dialogue[#text("RÖDE BARONEN")][#text("Och inte bara er sida! Så fort jag har fått tag i min väska så bär det av bort till den allierade sidan. Det var kul att jobba för centralmakterna men nu har jag ju fått alla medaljer som finns där. Dags att vända sig till motståndarna och få ännu mer ära. Tur i oturen att jag kraschlandade såhär nära rätt sida av fronten, det hade kunnat gå riktigt illa… General Bollbirth är inte direkt känd för att vara tålmodig. Hon kommer säkert att tacka mig ")#underline[#text("när hon får receptet på senapsgasen som jag stal från centralmakterna.  Undrar vad senapsgas är, egentligen? Det låter inte så farligt…")]#text(" Nåväl, nu återstår det bara att jag lyckas dölja mitt ")#underline[#text("dubbelspel")]#text(" fram tills att det är för sent för dem att stoppa mig… [[Bör vi skriva in att han vill vara på den vinnande/allierade sidan i monologen?]] [[Ja!]] [[Fast vi vill sa väl inte skriva mer om hans motivationer här?]] [[Att kommentera på att vilja va på den vinnande sidan går jättebra, så länge det inte blir för mycket exposition. Tänk att han säger något i förbifarten såsom \"och att vända kappan efter vinden är ju bara en av mina vinnande strategier\"]]")]
#text("Röde Baronens inre monolog slutar och han vänder sig återigen mot de andra soldaterna.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(triumferande)")] #text("Okej, nu kör vi! Framåt mot Ingenmansland. Jag ska ha min väska!")]
#text("De går av scenen.")
#pagebreak()
#scene[#text("SCEN 5")]
#text("[[")#text(style: "italic","Karaktärer: Tolkien, Nightingale, Cartier")#text("]]")
#text("[[Plats: Allierat läger -> Ingenmansland med flygplanskuliss]]")
#text("[[De allierade bråkar över vad de ska göra åt att ett plan kraschat i närheten. Nightingale manar ut Tolkien i ingenmansland för att undersöka. Följer strax efter själv när denne inser att Tolkien antagligen kommer dö om denne är ensam ute i ingenmansland. Cartier är bångstyrig.]]")
#text("[[Tolkien tar sig ut till det kraschade planet. Är rädd. Sjunger en nervös julsång för att lugna sig.]]")
#text("EXT. DET ALLIERADE LÄGRET")
#text("Scenen fortsätter strax efter scen 2. Cartier står nonchalant och spanar mot ingenmansland där planet har kraschat. Tolkien och Nightingale står bakom ett skydd och avvaktar.")
#dialogue[#text("TOLKIEN")][#text("Är kusten klar…?")]
#dialogue[#text("CARTIER")][#text("Planet kraschade flera hundra meter härifrån. Det fanns ingen chans att den skulle träffa oss. Ni britter är ju allt för fega.")]
#text("Nightingale flyttar sig från skyddet, medans Tolkien stannar kvar. ")
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(lite lätt irriterat)")] #text("Jag hade ju inte mycket av ett val när Tolkien drog undan mig.")]
#dialogue[#text("TOLKIEN")][#text("Man kan ju aldrig vara för säker i detta krig, Nightingale.")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(suckar)")] #text("Jag tror att faran är över nu i alla fall.")]
#text("Tolkien går långsamt och försiktigt fram.")
#dialogue[#text("NIGHTINGALE")][#text("Om det alltid är såhär så lär det ju åtminstone inte bli tråkigt här ute. Nåväl, jag bör väl infinna mig i sjukstugan, innan generalen blir sur.")]
#dialogue[#text("TOLKIEN")][#text("Vänta lite. Planet då? ")#underline[#text("Tänk om det finns någon skadad där ute.")]#text(" Tänk om de är i nöd!")]
#dialogue[#text("CARTIER")][#parenthetical[#text("(sarkastisk)")] #text("Varför spelar det någon roll? Någon britt eller tysk dör? Vilken tragedi, ska jag plocka fram le petit fiol?")]
#text("Cartier kanske drar ut på detta och överspelar det för komiken. Sätter sig på knä och gråter.")
#dialogue[#text("NIGHTINGALE")][#text("Jo… Visst, men det är långt härifrån. Och det är nästan säkert att ingen överlevde den där explosionen.")]
#dialogue[#text("TOLKIEN")][#text("Det är väl lite dystert tänkt. Om krig bara handlar om att döda och låta andra dö, varför krigar vi ens? Nej! Jag tycker att vi borde göra något åt det.")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(Uppmuntrande)")] #text("Okej… ")#underline[#text("Varför går inte du ut och undersöker då?")]#text(" [[Nightingale är inte särskilt snäll i den här scenen, så man kanske vill understryka att detta ska läsas som en genuin fråga, snarare än en utmaning]]")]
#text("Tolkien backar, både bildligt och bokstavligt. ")
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(nervöst)")] #text("V-vänta, va. Jag? Nu ska vi inte ta så drastiska beslut.")]
#dialogue[#text("CARTIER")][#parenthetical[#text("(spydigt)")] #text("Hur ska du annars ")#underline[#text("rädda piloten")]#text(" om du inte går ut i ingenmansland?")]
#dialogue[#text("TOLKIEN")][#text("Uh, jag hade inte riktigt tänkt så långt ännu…")]
#dialogue[#text("NIGHTINGALE")][#text("Ja, om du vill rädda dem måste du ju faktiskt ta dig ut dit. Det finns inget annat sätt. [[Det känns som att det är rimligare ifall cartier skulle säga detta, och att Nightingale inte säger något innan han vill uppmuntra Tolkien att vara modigare.]]")]
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(lite fegt)")] #text("Men löjtnanten har sagt till oss att inte bege oss ut själva. Och tänk att vara där ute ensam. Ingen man utom jag…")]
#text("Nightingale suckar och tar tag i Tolkien. ")
#dialogue[#text("NIGHTINGALE")][#text("Lyssna här, om du tror det är någon där ute som behöver hjälp så behöver du skärpa dig och gå ut. Du måste se bortom din feghet. [[Kontinuitet]]")]
#text("Tolkien ser märkbart ledsen ut. ")
#dialogue[#text("NIGHTINGALE")][#text("Men du har chansen att skaffa dig lite mod här. Att visa att du har lite kurage. Om du tror du borde försöka rädda den där piloten så stick ut och gör det.")]
#text("Tolkien uppmuntras av Nightingales ord och han ler.   ")
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(osäkert)")] #text("Fast ensam ute på ingemansland…")]
#dialogue[#text("NIGHTINGALE")][#text("Kom igen, det enda sättet att bemästra dina rädslor är genom att möta dem med huvudet först.")]
#dialogue[#text("CARTIER")][#parenthetical[#text("(muttrar)")] #text("Du kan inte vara en bra sjukvårdare…")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(peppar lite aggresivt)")] #text("Kom igen Tolkien, ut med dig!")]
#dialogue[#text("TOLKIEN")][#text("Okej… Okej. Om någon är i fara så är det min plikt…")]
#dialogue[#text("NIGHTINGALE")][#text("Så ja! Se till och gör det då!")]
#text("Nightingale ger Tolkien en uppmuntrande knuff i rätt riktning och Tolkien stapplar iväg ut i ingenmansland. Cartier och Nightingale står tysta någon sekund och ser efter.")
#dialogue[#text("CARTIER")][#parenthetical[#text("(Nästan skadeglatt)")] #text("Honhonhon, det där kommer inte att sluta bra.")]
#dialogue[#text("NIGHTINGALE")][#text("Vad menar du?")]
#dialogue[#text("CARTIER")][#text("Om du ville få död på honom har du nog lyckats. Ute i ingenmansland är man som en måltavla för fienden, och det är bara en bråkdel som lever länge nog att återvända. Där ute är skrivarsnobben som en vild snigel i en femstjärnig fransk restaurang. [[\"vårdslöst blodbad\"?]]")]
#dialogue[#text("NIGHTINGALE")][#text("Oj, jag tänkte inte på det… Eh…")]
#text("Nightingale vänder sig om och springer efter Tolkien mot ingenmansland. ")
#dialogue[#text("NIGHTINGALE")][#text("TOLKIEN!!! VÄNTA!!!")]
#dialogue[#text("CARTIER")][#parenthetical[#text("(roat)")] #text("Baah, engelsmän…")]
#pagebreak()
#scene[#text("SCEN 5.5")]
#text("[[")#text(style: "italic","Karaktärer: Tolkien, Nightingale, Schumacher, Klimt, Röde Baronen")#text("]]")
#text("[[Plats: Ingenmansland med flygplanskuliss]]")
#text("[[Soldaterna från de bägge sidorna möts. Är först misstänksamma mot varandra men inser att de borde bli vänner istället. De har dock plikter och måste återvända tillbaka till sina läger. På väg tillbaka snubblar Tolkien över en väska.]]")
#text("EXT. INGENMANSLAND")
#text("Klimt, Schumacher och Röde Baronen går genom ingenmansland. Röde Baronen är väldigt seriös. Klimt är rätt lugn och Schumacher går först in, helt obrydd.")
#text("[[Har väldigt svårt med att balansera att visa att det är farligt i ingenmansland med att visa personligheten för de rätt positiva karaktärerna Klimt och Schumacher]]")
#dialogue[#text("KLIMT")][#parenthetical[#text("(flåsande)")] #text("Ta det lite lugnt, Schumacher; håll dig bakom mig så att du inte springer på några minor! [[Jag tycker att Klimt borde använda efternamn...det blir lättare att hålla koll på alla om de alltid har samma namn.]]")]
#dialogue[#text("SCHUMACHER")][#text("Äsch, det ser du väl att det inte ligger några minor här? Jag ska säga dig att när jag seglade till Franska Senegal, då stötte vi på massor av minor i vattnet och de var ju flera meter i diameter. Ska de ha burit hit och grävt ner så stora saker här, menar du? [[Låter mer som RB än Schmauder tbh]]")]
#dialogue[#text("RÖDE BARONEN")][#text("Kan ni två vara lite tysta? Jag försöker lyssna efter fienden, de kan vara var som helst!")]
#dialogue[#text("KLIMT")][#text("Äsch, ")#underline[#text("fienden är lika rädda för ingenmansland som vi är")]#text(". Så länge man håller sig bakom skydd är det onödigt att oroa sig för mycket när man ändå inte kan göra något åt saken. Varför är du så nervös? ")#underline[#text("Var inte du någon sorts krigshjälte")]#text("?")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(Byter ut sin oroliga röst mot sin superhjältefasad)")] #text("Va, eeh, jo absolut! Det är bara… Det här är ett väldigt viktigt uppdrag så jag måste verkligen få tillbaka min väska. [[Här finns det rum för baronen att skryta om sina bedrifter om vi vill (kan vara kul omstart också)]]")]
#text("Schumacher avbryter dem.")
#dialogue[#text("SCHUMACHER")][#text("Pssst! Hör ni det där?")]
#text("Svag julsång hörs.")
#dialogue[#text("RÖDE BARONEN")][#text("Det är britterna! Måste vara ett bakhåll!")]
#dialogue[#text("KLIMT")][#text("Bakhåll? Varför skulle de sjunga julsånger i så fall?")]
#dialogue[#text("RÖDE BARONEN")][#text("För att… eh… för lura in oss i en falsk trygghet såklart! Det är en fälla. Ok, hörni, vi måste komma på en plan. Jag har det här. Klimt, försök att spana om du kan se fienden! Men se till att inte själv bli upptäckt! Okej, Schumacher…")]
#text("Röde Baronen vänder sig om och börjar förklara för henne vad hon ska göra, och mumlar på om diverse planer. Under tiden går Klimt helt oförskräckt mot ljudet. Fram kommer Tolkien, som går väldigt nervöst med ett hårt grepp om sitt gevär medan han sjunger för att lugna ner sig själv. Klimt börjar sjunga med. Tolkien hoppar till och riktar famlande geväret mot Klimt. Han råkar dock ha pekat pipan mot sig själv i sin nervositet men märker inget.")
#dialogue[#text("TOLKIEN")][#text("Aaaaah! Stanna där, ditt tyska monster! Innan jag skjuter dig!")]
#text("Klimt är helt obrydd.")
#dialogue[#text("KLIMT")][#parenthetical[#text("(vant, professionellt)")] #text("Steg ett för att skjuta fienden brukar vara att hålla vapnet åt rätt håll.")]
#text("Tolkien inser sitt misstag och vänder panikartat på geväret.")
#dialogue[#text("KLIMT")][#text("Sedan är det ju också bra att se till att vapnet inte är säkrat, oladdat, eller smutsigt. Som det ser ut nu så är det där vapnet nog en större risk mot dig själv. Men oroa dig inte! Jag tänker inte skada dig. [[jag diggar inte \"smutsigt\"]] [[Jag tror tyvärr att \"smutsigt\" är bäst, för det är så man faktiskt beslkriver ett icke-rengjort vapen]]")]
#text("Tolkien sänker försiktigt sitt vapen, besegrad.")
#dialogue[#text("KLIMT")][#text("Såja, jag tror vi båda är lika otaggade på att vara här ute.")]
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(Fortfarande lite försiktig)")] #text("Ingen dålig observation…")]
#text("Tolkien gestikulerar lite mot sitt vapen som såklart är precis som Klimt beskrivit det.")
#dialogue[#text("KLIMT")][#parenthetical[#text("(med en gnutta stolthet)")] #text("Jodå, jag har spenderat tillräckligt mycket tid i materialförrådet för att lära mig en sak eller två.")]
#text("Nightingale rusar in.")
#dialogue[#text("NIGHTINGALE")][#text("Tolkien! Där är du ju! Och i ett stycke, också.")]
#text("Han får syn på Klimt, och höjer instinktivt sitt vapen. Tolkien drar snabbt ner den. [[Är Nightingale beväpnad?]] [[Ja]]")
#dialogue[#text("TOLKIEN")][#text("Nej, nej! Det är ingen fara. Tror jag. Det här är… eehh, vad heter du egentligen?")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(Bugar sig)")] #underline[#text("Klimt. Franziska Klimt.")]]
#text("Röde Baronen och Schumacher kommer försiktigt in bakom Klimt.")
#dialogue[#text("SCHUMACHER")][#text("Klimt! Jag ser allt vad du gör, har du hemliga affärer med fienden?")]
#dialogue[#text("KLIMT")][#text("Nej nej, jag presenterar mig bara. Oroa er inte, de är harmlösa.")]
#text("Klimt vänder sig tillbaka till Tolkien och Nightingale.")
#dialogue[#text("KLIMT")][#text("Det här är mina kompanjoner, ")#underline[#text("Liesel Schumacher och den Röde Baronen")]#text(".")]
#text("Klimt gestikulerar fram sina vänner. Schumacher niger ståtligt och Röde Baronen vinkar lite stelt.")
#dialogue[#text("NIGHTINGALE")][#text("Trevligt att träffas. Jag heter ")#underline[#text("Philip Nightingale.")]]
#dialogue[#text("TOLKIEN")][#text("Mitt namn är ")#underline[#text("John, John Tolkien.")]]
#text("När alla har introducerat sig lägger de ner sina vapen och ställer sig lite mer lediga i en bekväm position för att ha en konversation.")
#dialogue[#text("SCHUMACHER")][#text("Så ni är britter, båda två? Så trevligt, jag gillar verkligen britter. Men då måste ni ha varit i Frankrike! Hur är det där? Har ni någon mat med er man kan smaka? Lite vin?")]
#text("Nightingale och Tolkien ser på varandra.")
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(Lite mer avslappnat)")] #text("Dessvärre inte. Cartier kanske har en baguette, men han är inte direkt en mat-person.")]
#dialogue[#text("SCHUMACHER")][#text("Fransman som inte gillar mat?") #parenthetical[#text("(lutar sig in och viskar)")] #text("Är ni säker på att han faktiskt är från Frankrike?")]
#text("Röde Baronen, som är otålig, tar ordet ifrån henne.")
#dialogue[#text("RÖDE BARONEN")][#text("Så hur är det på den allierade sidan? Har ni det bra?")]
#dialogue[#text("NIGHTINGALE")][#text("Jag anlände nyss så jag vet inte så mycket, men från vad jag hört av Tolkien här så är det ganska lugnt just nu.")]
#dialogue[#text("KLIMT")][#text("Ja, det är kanske inte så konstigt. Det är väl ingen som är sugen på att slåss ")#underline[#text("såhär nära inpå julen")]#text(".")]
#dialogue[#text("TOLKIEN")][#text("Ja, kommer ni fira jul på er sida? [[Slipa på detta]]")]
#dialogue[#text("SCHUMACHER")][#text("Man hade ju kunnat önska det, men det ser ut att kanske bli en liten julsång som mest. Ingen mat, ingen dekoration och inget Glühwein. Det är en skam, det är vad det är.")]
#text("Alla får en lite nedstämd blick och det blir lite tyst. Tolkien rör sig nyfiket mot Röde Baronen.")
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(till Röde Baronen)")] #text("Säg mig… \"Röde Baronen\"? Är det ett smeknamn?")]
#dialogue[#text("RÖDE BARONEN")][#text("Ja, \\")#text(style: "italic","skrockar\\")#text(", ")#underline[#text("jag heter Jorji")]#text(" egentligen, men Röde Baronen är vad alla kallar mig. Jag är pilot. Har flugit alla sträckor ni kan tänka er.")]
#dialogue[#text("SCHUMACHER")][#text("Ja, han påstår sig vara någon sorts kändis, men jag har då aldrig hört om honom. Och jag har varit ute och rest i hela världen.")]
#text("Röde Baronen brer på lite extra med sin actionhjälte-persona.")
#dialogue[#text("RÖDE BARONEN")][#text("Äsch, känd och känd? Men ja, det är ju inte vem som helst som får ett smeknamn som blir känt över hela kontinenten…")]
#dialogue[#text("TOLKIEN")][#text("Men då var det ditt plan som kraschade? Vilken tur att du är oskadd! Jag fruktade det allra värsta när jag såg kraschen.")]
#text("Röde Baronen blir lite tagen av Tolkiens genuina lättnad.")
#dialogue[#text("RÖDE BARONEN")][#text("Öh… Ja, det krävs mer än så för att ta kål på mig. Jag är ju inte vilken pilot som helst.")]
#dialogue[#text("NIGHTINGALE")][#text("Kan ni tänka er att Tolkien gav sig ut i ingenmansland för att hjälpa inte bara en främling, men en fiende på köpet! Ja, det var ju lite på mitt initiativ, så det var en väldans tur att ni är fredliga… [[Jag tror Nightingale ändå skulle känna sig lite skyldig här, och inte vara så snabb med att skylla allt på Tolkien]] [[Bra poäng]]")]
#text("Nightingale tittar lite skyldigt på Tolkien som ger honom en blick som signalerar förlåtelse.")
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(först mot Tolkien, sedan till Röde Baronen)")] #text("Wow, så du sprang bara rakt ut för att hjälpa till, trots att du visste hur farligt det är härute? Ja, eller… vi gjorde ju faktiskt samma sak!")]
#dialogue[#text("KLIMT")][#text("Ja, och sedan kom vi hit för att hjälpa baronen med att hitta… hitta- ")#underline[#text("Vad var det du skulle hitta för något?")]]
#text("De alla avbryts av att en visselpipa ljuder.")
#dialogue[#text("KLIMT")][#text("Hörde ni det där? De måste ha märkt att vi är borta! Vi måste återvända, nu på en gång!")]
#dialogue[#text("RÖDE BARONEN")][#text("Men… min väs-")]
#dialogue[#text("KLIMT")][#text("Nu, baronen! Vi går nu!")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(sorgset)")] #text("Åh, måste vi? Jag fick ju inte höra något om Frankrike.")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(Till Tolkien)")] #text("Vi borde också röra oss innan någon märker att vi är borta.")]
#dialogue[#text("SCHUMACHER")][#text("Men kan vi inte ses snart igen? Då kan jag ta med mer mat! Och då måste ni ta med mat från er sida också, annars är det orättvist!")]
#dialogue[#text("TOLKIEN")][#text("Vi kanske kan ses här igen en annan gång? Det är inte som om vi har mycket annat för oss. Och ")#underline[#text("om ni behöver hjälpa till att leta efter något kan vi säkert göra det också.")]]
#text("Röde Baronen öppnar munnen för att säga något men hindrar sig själv och nickar bara.")
#dialogue[#text("SCHUMACHER")][#text("Åh, underbart! Äntligen något att se fram emot! Vi borde ha lite fritid senare ikväll, efter att våra arbetspass är slut. [[Skrev om \"Hoppas att ni överlever!\" till detta. Men är osäker på om detta är bättre... Vad tycker ni?]] [[Dialogen är nice, men det här planerandet går ganska fort, det skulle kanske behövas någon line till för att det inte ska kännas så rushat]] [[Håller med]]")]
#dialogue[#text("KLIMT")][#text("Det låter bra, då är vi skyddade av mörkret också. Och är det efter middagen så slår jag vad om att Schumacher kan packa er en matlåda med sauerkraut. [[Skrev om \"Hoppas att ni överlever!\" till detta. Men är osäker på om detta är bättre... Vad tycker ni?]]")]
#text("Alla börjar gå av scenen. Tolkien går ut sist och nynnar lite på en julsång. De andra försvinner och Tolkien blir ensam kvar. När han går får han plötsligt syn på en bit bråte som ser intressant ut. Tolkien går fram och tittar på den.")
#dialogue[#text("TOLKIEN")][#text("Vad har vi här… Den måste ha lossnat från planet.")]
#text("Han lyfter på den och hittar en väska.")
#dialogue[#text("TOLKIEN")][#text("En väska? [[Tolkien borde nog reagera på att detta kan vara vad RB är ute efter]] [[Sa vi inte att han inte skulle koppla det? så vi skulle få lite oklarhet med vad som kommer hända med den och göra payoffen när den hamnar på rätta vägar igen större? Eller kommer jag ihåg fel?]] [[Jo, så var det! Vi ville inte att han skulle dra kopplingen]]")]
#text("Han skakar om väskan lite.")
#dialogue[#text("TOLKIEN")][#text("Det verkar vara något i den… Men den är inte tung. Undrar om det är något viktigt…")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(utanför scen)")] #text("Tolkien!")]
#dialogue[#text("TOLKIEN")][#text("Kommer!")]
#text("Tolkien tar med sig väskan och fortsätter att nynna på en jullåt medans han går av scenen och lamporna slocknar.")
#pagebreak()
#scene[#text("SCEN 6")]
#text("[[")#text(style: "italic","Karaktärer: Röde Baronen, Schumacher, Klimt, Müller, Flammenwerfer")#text("]]")
#text("[[Plats: Centralmaktsläger]]")
#text("[[Centralmaktssoldaterna återvänder tillbaka till sitt läger. Müller är arg, men blir också starstruck av Röde Baronen. Müller framför hans avsky för de allierade. Vi förtydligar soldaternas vilja att inte kriga, speciellt nu när de fått nya vänner.]]")
#text("EXT. CENTRALSMAKTSLÄGRET")
#text("Schumacher och Klimt med Baronen i släptåg går med raska steg in tillbaka mot centralmaktslägret.")
#dialogue[#text("SCHUMACHER")][#text("Du, Klimt? Du tror inte generalen kommer vara arg va?")]
#text("KLIMT [[Ja jag vet att han förmodligen inte bor i ett tält men jag vet inte vad annars det borde kallas]] [[Jooo, han bor säkert visst i ett tält. It fine]]
Oroa dig inte, Schumacher. Förhoppningsvis har han inte ens lämnat sitt tält ännu.")
#text("En lampa tänds och fokus skiftar till Müller som står och upprört diskuterar något med Flammenwerfer. De stannar till och försöker sedan långsamt (och komiskt) smyga sig där ifrån. ")
#dialogue[#text("FLAMMENWERFER")][#text("Det är ju inte mitt fel.")]
#dialogue[#text("MÜLLER")][#parenthetical[#text("(förargat)")] #text("Vad menar du?!")]
#dialogue[#text("FLAMMENWERFER")][#text("Det var ju du som ville att vi skulle effektivisera kriget.")]
#dialogue[#text("MÜLLER")][#text("Ja, men när jag sa det menade jag inte att du skulle tända all dynamit i lägret medan det var i lägret!")]
#dialogue[#text("FLAMMENWERFER")][#text("General, jag gjorde det som en förberedelse, så att den är redo när den ska användas.")]
#text("Flammenwerfer tar fram en sticka dynamit och tänder stubinen. ")
#dialogue[#text("FLAMMENWERFER")][#text("Kolla bara på hur lång tid det tar för stubinen att brinna. Vi sparar massor av tid genom att förtända dynamiten.")]
#text("Müller snor snabbt dynamiten från Flammenwerfer. Han släcker stubinen med sin hand och ryggar till från smärtan.")
#dialogue[#text("MÜLLER")][#parenthetical[#text("(irriterat)")] #text("Om du inte vore en så bra artillerist så hade jag sparkat dig för länge sedan. Har vi ens kvar någon dynamit?")]
#dialogue[#text("FLAMMENWERFER")][#text("Jajamen. Det var bara förrådet med korta stubiner som sprängdes… De med långa stubiner borde hålla sig tills… [[jag gjorde det aningen tydligare vad som händer, kanske går att göra på ett lite bättre sätt]]")]
#text("Flammenwerfer stannar upp, kollar på klockan och blir storögd.")
#dialogue[#text("FLAMMENWERFER")][#text("Jag måste gå… och göra något.")]
#text("Flammenwerfer går därifrån med raska steg.")
#dialogue[#text("MÜLLER")][#parenthetical[#text("(ropandes efter)")] #text("Om jag får höra om några fler oplanerade explosioner så kommer jag personligen att sätta dig i militärtribun- [[Nu säger Müller ungefär samma sak två gånger här, någon av dem bör kanske ändras aningen?]]")]
#text("Müller får syn på Schumacher, Klimt och Röde Baronen medans de försöker att smyga runt. ")
#dialogue[#text("MÜLLER")][#text("Där är ni! Vart har ni varit?!")]
#text("Klimt och Schumacher ställer sig snabbt i givakt. Röde Baronen ställer sig ståtligt bredvid dem.")
#dialogue[#text("SCHUMACHER")][#text("Vi- vi gick bara en liten trevlig kvällspromenad, general…")]
#text("Müller stirrar ner dem. Klimt och Schumacher vänder sig bort från Müller.")
#dialogue[#text("SCHUMACHER")][#text("Vad ska vi säga till generalen?  [[Kanske byt ut \"Müller\" mot \"generalen\".  Man brukar inte kalla en general bara vid namn av vad jag vet. ]]")]
#dialogue[#text("KLIMT")][#text("INTE att vi var i Ingenmansland.")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(självsäkert till Müller)")] #text("Det tänker jag inte säga! [[Detta skämt är hundra gånger bättre än det jag skrev. Bra jobbat!  ]]")]
#text("Klimt slår sig för pannan. Müller blir lite paff.")
#dialogue[#text("MÜLLER")][#text("…Nähä? Men… du måste säga, det är en order!")]
#text("Schumacher tänker efter lite.")
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(inser inte riktigt allvaret)")] #text("Okej… det rimmar på Ingers blandband.")]
#text("Klimt slår sig för pannan ännu hårdare. Röde Baronen gör något liknande.")
#dialogue[#text("MÜLLER")][#parenthetical[#text("(upprört)")] #text("Va?! Vad i hela friden gjorde ni i ingenmansland?! Ni hör på namnet, INGENMANSLAND. Landet där INGEN MAN är. Hur vågar ni sätta era liv på spel utan min tillåtelse?!")]
#dialogue[#text("RÖDE BARONEN")][#text("General, de fick ett viktigt uppdrag som gjorde det nödvändigt för dem att dra ut till ingenmansland.")]
#text("Müller är för arg för att riktigt ta in vem som pratar.")
#dialogue[#text("MÜLLER")][#text("Vad för uppdrag menar ni? De enda befälen ni ska följa är dem från mig. Och vem i hela friden är-")]
#text("Müller blir tyst och kollar på Baronen en stund. Han börjar inse vem personen är.")
#dialogue[#text("MÜLLER")][#text("Vänta, det kan inte vara… ")#underline[#text("Självaste Röde Baronen!")]#text(" Vad ger oss den äran? Välkommen, välkommen!")]
#text("Müller föser undan Klimt och Schumacher och för sin arm bakom ryggen på Röde Baronen.")
#dialogue[#text("MÜLLER")][#parenthetical[#text("(till Schumacher och Klimt)")] #text("Tro inte att ni klarar er undan det här. Jag ska se till att ta hand om er senare.")]
#dialogue[#text("MÜLLER")][#parenthetical[#text("(till Röde baronen)")] #text("Jag ber så mycket om ursäkt för röran, ")#underline[#text("vi hade ingen aning om att ni skulle komma")]#text(". Schumacher måste ha missförstått något morsemeddelande. [[Morsemeddelandet kan inte vara i bestämd form]] [[Like that?]]")]
#text("En explosion hörs i fjärran.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(samlar sig snabbt)")] #text("Ja precis. Ni förstår, general…?")]
#dialogue[#text("MÜLLER")][#underline[#text("General Heinrich von Müller. Kalla mig general Müller.")]]
#dialogue[#text("RÖDE BARONEN")][#text("Ni förstår, general Müller, när jag flög i mitt plan så blev det något fel med motorn. Därav behövde jag göra en strategisk kraschlandning. [[Fila på denna]]")]
#dialogue[#text("SCHUMACHER")][#text("Vi såg baronens plan falla och vi gjorde det rätta.")]
#dialogue[#text("KLIMT")][#text("Ja exakt! Vi sprang direkt dit för att hjälpa vår medsoldat, även om det kunde kosta oss våra liv.")]
#dialogue[#text("MÜLLER")][#text("Jaså, varför berättade ni inte innan! Bra jobbat, Klimt och Schumacher. Dock är väl en liten plankrasch ingenting för den mytomspunne Röde baronen, haha.")]
#text("Flammenwerfer återvänder. Hans kläder mår dåligt.")
#dialogue[#text("FLAMMENWERFER")][#text("Som svar på frågan, general, nej.")]
#dialogue[#text("MÜLLER")][#text("Vilken fråga?")]
#dialogue[#text("FLAMMENWERFER")][#text("…Spelar ingen roll nu. Röde baronen! Du är tillbaka!")]
#text("Ett par soldater (kören) kommer in på scen. ")
#dialogue[#text("KÖRIS #1")][#text("General Müller! Vi har ett litet problem just nu.")]
#dialogue[#text("MÜLLER")][#text("Det får vi ta senare, ser ni inte att Röde Baronen är här!")]
#text("Soldaterna ser förvirrade ut.")
#dialogue[#text("KÖRIS #2")][#text("Vänta vem? Den röda snubben?")]
#dialogue[#text("MÜLLER OCH FLAMMENWERFER")][#parenthetical[#text("(väldigt upprört)")] #text("Röda snubben?!?")]
#dialogue[#text("MÜLLER")][#text("Känner ni inte till den legendariska Röde Baronen!?! ")#underline[#text("Han är den störste hjälten i centralmakterna")]#text(". Han har åstadkommit mängder av stordåd.")]
#dialogue[#text("FLAMMENWERFER")][#text("Jag har hört att han har deltagit i 420 strider och vunnit varenda gång. Han har överlevt 69 skott och explosioner utan några skråmor alls.")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(tyst)")] #text("Minst.")]
#dialogue[#text("MÜLLER")][#parenthetical[#text("(exalterat)")] #text("Inte bara det, Röde Baronen lyckades en gång besegra en hel allierad bataljon med bara en tandpetare.")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(avbryter)")] #text("En halv tandpetare, faktiskt.")]
#dialogue[#text("FLAMMENWERFER")][#text("Han har åstadkommit mer i detta krig än vad någon soldat någonsin kan drömma.")]
#dialogue[#text("MÜLLER")][#text("Verkligen. Röde Baronen är ett exempel på den absolut perfekta soldaten och ni alla borde se upp till honom. En stolthet för det ungerska krigsväsendet!")]
#text("Müller lugnar sig lite och vänder sig mot körisen.")
#dialogue[#text("MÜLLER")][#text("Nå, vad var så viktigt att ni behövde avbryta min viktiga kvalitetstid med denna krigshjälte?")]
#dialogue[#text("KÖR")][#text("General, vi försökte sätta upp ett nytt tält, men det började brinna.")]
#text("(omstart: Det är en trettio meter lång haj i mattältet)")
#dialogue[#text("MÜLLER")][#text("Jösses! Varför berättade ni inte det tidigare?! Nåväl, led vägen, för all del.")]
#text("Müller och kören går av scenen. Schumacher vänder sig mot Röde Baronen.")
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(småimponerat)")] #text("420 strider? Då måste du ha flugit över hela världen?")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(Döljer dåligt hur smickrad han är)")] #text("Nja, så gott som. Europa, Asien, till och med nere i Afrika en vända. Jag har sett mer än någon annan människa och upplevt allt som är värt att uppleva.")]
#text("Schumacher blir stött.")
#dialogue[#text("SCHUMACHER")][#text("\"Mer än någon annan människa\"? Hah! Du kan inte ha sett mer än MIG. Jag har varit i ALLA länder och har- [[Klimt kanske inte borde avbryta henne (bara för att det är svårare att skådespela). Vill mest visa att Schumacher är passionerad om ämnet]]")]
#text("Klimt stiger emellan.")
#dialogue[#text("KLIMT")][#text("Lugna dig, Schumacher, han menar det inte bokstavligen. Ingen människa kan ha sett och upplevt allt.")]
#dialogue[#text("RÖDE BARONEN")][#text("Inte det? Ska vi slå vad? [[Kanske byt ut \"Låt mig berätta\" med ngt i stil med \"Wanna bet?\"/\"ska vi slå vad?\"]]")]
#text("  ")
#text(weight: "bold","Musiknummer 3")
#text("[[Röde Baronen sjunger om sitt liv och om hur cool han är. Kanske nämner planen utan att gå in på att han begår krigsbrott. Omstarter fokuserar på centralmaktssoldaterna och vad de nu vill sjunga om.]]")
#pagebreak()
#scene[#text("SCEN 6.5 - MOMENTS LATER")]
#text("Röde Baronen sätter sig ned igen efter sitt sångnummer.")
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(förvirrat)")] #text("Men vänta här nu lite, du är då alltså en krigshjälte och allt men jag förstår fortfarande inte varför.")]
#dialogue[#text("RÖDE BARONEN")][#text("Varför vad?")]
#dialogue[#text("SCHUMACHER")][#text("Varför vi slåss.")]
#dialogue[#text("RÖDE BARONEN")][#text("Öööhhh…")]
#dialogue[#text("KLIMT")][#text("Jo, du Schumacher, det ska jag säga dig.")]
#text("Klimt plockar fram en whiteboardtavla.")
#dialogue[#text("KLIMT")][#text("Okej så allting börjar med den här mannen som heter Bismarck. Han förenar Preussen till ett enat Tyskland. Han hade en ")#text(style: "italic","väldigt")#text(" het mustasch också, men det är irrelevant för det här, fast kanske inte, men i alla fall så gör han faderlandet starkt igen för över 50 år sedan-")]
#dialogue[#text("SCHUMACHER")][#text("Men det är ju jättelänge sen.")]
#dialogue[#text("KLIMT")][#text("Men det är viktigt.")]
#dialogue[#text("SCHUMACHER")][#text("Kan du inte spola fram några år.")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(buttert)")] #text("Ja okej då.")]
#text("Klimt ritar frenetiskt på tavlan.")
#dialogue[#text("KLIMT")][#parenthetical[#text("(ivrigt)")] #text("Så spänningen är hög i Europa. Frankrike är arga på Tyskland för att vi tog några stater i ett krig för några år sen. Vi är arga på Ryssland för att de allierat sig med Frankrike men det är för att vår kaiser ghostade tsaren några år tidigare…")]
#transition[#text("FADE TO BLACK.")]
#pagebreak()
#scene[#text("AKT 2")]
#pagebreak()
#scene[#text("SCEN 7")]
#text("[[")#text(style: "italic","Karaktärer: Tolkien, Nightingale, Senap, Bollbirth")#text("]]")
#text("[[Plats: Allierat läger]]")
#text("[[Bollbirth är arg för att senapsgasen inte anlänt. Antar att spionen blivit tillfångatagen av centralmakterna. Framför till Senap att de planerar en attack i gryningen för att inte hamna i underläge/få tag i planerna. Senap framför det till resten av gruppen. Ingen tycker det är en bra idé och vill inte vara med på det. Tolkien & Nightingale bestämmer sig för att de borde försöka slå sig samman med dem för göra upp en plan för att slippa blodbad. De ger sig ut i ingenmansland.]]")
#text("General Bollbirth går med arga kliv in på scenen där Senap redan står och inspekterar en kuliss.")
#dialogue[#text("BOLLBIRTH")][#text("SEENAAP!!")]
#dialogue[#text("SENAP")][#parenthetical[#text("(Ställer sig i honnör)")] #text("Ja, general!")]
#dialogue[#text("BOLLBIRTH")][#text("Ut med språket. Var är spionen!? ")#underline[#text("Var är min leverans!?")]]
#dialogue[#text("SENAP")][#parenthetical[#text("(lite smått nervöst)")] #text("Den har inte setts av, General. Men spionen är kanske fortfarande på väg.")]
#dialogue[#text("BOLLBIRTH")][#text("På väg!? Förklara för mig hur mitt triumfkort kan vara PÅ VÄG när spionen var väldigt tydlig med att det skulle vara här NU!")]
#text("Tystnad i ett ögonblick.")
#dialogue[#text("SENAP")][#parenthetical[#text("(Utbrister)")] #text("Planet! Det illröda planet som kraschade ute i ingenmansland. Kan det ha varit spionen?")]
#text("Bollbirth knyter näven i ilska. [[old man yells at plane]]")
#dialogue[#text("BOLLBIRTH")][#text("Det måste vara han! Jag känner det på mig. Det värdelösa kräket har blivit tillfångataget.")]
#text("Bollbirth slappnar av och blir nästan lite uppgiven.")
#dialogue[#text("BOLLBIRTH")][#text("Utan gasen flyttas min seger längre och längre bort från mig…")]
#dialogue[#text("SENAP")][#parenthetical[#text("(fundersamt)")] #text("Kan vi inte göra något för att rädda honom?")]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(dumförklarande)")] #text("Rädda? Hur skulle det gå till, det är inte som att vi bara kan ta oss in i fiendelägret och gå ut med honom som om det vore en söndagspromenad.")]
#dialogue[#text("SENAP")][#parenthetical[#text("(passionerat)")] #text("Kanske inte så lätt, men om vi skulle kunna ta oss in i centralmaktslägret samtidigt som de är distraherade, så skulle vi kunna bryta ut honom!")]
#dialogue[#text("BOLLBIRTH")][#underline[#text("VI GÖR ETT ANFALL!")]#text(" Det är en utmärkt idé, Senap! [[nått innan den här]]")]
#text("Senap ser skrämd ut av tanken, men samlar snabbt ihop sig, men hon ser fortfarande ganska tveksam ut.")
#dialogue[#text("SENAP")][#text("Ett anfall!? Jag menar- ja, jag antar att det också skulle kunna fungera… Om vi påbörjar förberedelserna nu kan soldaterna nog vara redo om en vecka.")]
#dialogue[#text("BOLLBIRTH")][#text("En vecka? Kommer inte på fråga! Då kommer spionen att ha avrättats för länge sedan. ")#underline[#text("Vi anfaller i gryningen!")]#text(" [[Sjukt om Bollbirth vet att det är en ond plan. Kanske bättre att skriva något annat? Men tyckte det var kul]] [[Bollbirth är för komiskt ond i den här scenen i regel... och för självmedveten om sin ondhet. Bättre att låta henne uttrycka sig som att det är rätt sak att göra och så skriver man inte publiken på näsan att hon är ond utan låter de andra karaktärerna reagera som att hon är ond utan att säga det rakt ut]] [[Ändrade från ondskefull till genial\"]]")]
#dialogue[#text("SENAP")][#parenthetical[#text("(osäker)")] #text("Så snart, general? För att vara helt ärlig, med så lite förberedelse skulle vi förlora massor av goda soldater.")]
#dialogue[#text("BOLLBIRTH")][#text("Vi har ont om tid, men vi har gott om soldater. Vi måste slå till nu. Vinsten är inom räckhåll. Se till att trupperna gör sig redo.")]
#text("Bollbirth går triumferande av scenen. Senap står kvar, osäker på informationen hon precis har fått.")
#dialogue[#text("SENAP")][#text("Hur ska det här nu gå…?")]
#text("Tolkien och Nightingale in på scenen. De har precis kommit tillbaka från ingenmansland så Tolkien bär fortfarande på väskan. Han lägger snabbt undan den på scenen men så att den syns.")
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(småpratar med Nightingale)")] #text("Åh, vilka trevliga människor de var, de där tyskarna.")]
#dialogue[#text("NIGHTINGALE")][#text("Ja, det hade man inte kunnat tro.")]
#text("Senap får syn på Nightingale och Tolkien, och försöker framstå som mer självsäker.")
#dialogue[#text("SENAP")][#text("Där är ni ju! Var har ni varit?")]
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(lite ställd)")] #text("Vi har… ööö… varit på promenad.")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(kvicktänkt lögn)")] #text("Precis, på promenad. Man får inte glömma att ta hand om sin motion och fysiska hälsa bara för att det är krig, en promenad om dagen bidrar till ett starkt immunförsvar.")]
#dialogue[#text("SENAP")][#text("Immunförsvar? Nog med försvar. ")#underline[#text("Det är dags för anfall.")]#text(" [[Tonk: Imunförsvar? Vi har nog med försvar, det är dags att attackera!]]")]
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(upprymd)")] #text("Anfall!? Ska vi ut… Dit?! När då? [[Tolkien låter lite väl naiv här, som om han förväntat sig att Senap plötsligt ändrat sig helt om kriget]]")]
#dialogue[#text("SENAP")][#underline[#text("I gryningen.")]#text(" Vi ska storma över ingenmansland och visa fastlänningarna hur man slåss.")]
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(Tar ett steg fram mot Senap)")] #text("Varför i hela friden skulle vi göra det?")]
#dialogue[#text("SENAP")][#parenthetical[#text("(Senap tar ett steg fram mot Tolkien)")] #text("För att vi är i ett krig, Tolkien, och i krig så krigar man. Hur ska vi vinna genom att bara sitta och rulla tummarna?")]
#dialogue[#text("TOLKIEN")][#text("Efter all denna tid vi spenderat tillsammans på fronten, måste väl till och med du inse att meningslöst dödande inte kommer att lösa något. Att spelet generalen spelar helt saknar vett! [[Mycket bätte back and forth här sedan sist! Amazing <3]]")]
#text("SENAP [[Någon med bättre koll på krigsterminologi kanske kan skriva om den här repliken så att den passar bättre]] [[Jag funderar på ifall Senap ska vara lite mer kritisk mot Tolkiens brist på logik. Nästan fördumma honom genom att säga typ \"Tolkien, vad gör man i krig? Besegrar fienden. Och hur besegrar man fienden? Jo, man anfaller dem.\" eller något]]
Vad har hänt med din moral? På min fars tid var det en ära att få slåss för sitt land. [[Skriv om som mer buddy-buddy]]")
#text("Senap tar en kort paus och lugnar ner sig lite. Hon är inte heller egentligen taggad på det här.")
#dialogue[#text("SENAP")][#text("Kom igen, Tolkien, det här är vår plikt.")]
#dialogue[#text("TOLKIEN")][#text("Men…")]
#dialogue[#text("SENAP")][#parenthetical[#text("(barskt)")] #text("Det är en order, soldat.")]
#text("Tolkien fnyser och backar ett steg.")
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(uppgivet)")] #text("Ja, löjtnant.")]
#dialogue[#text("SENAP")][#parenthetical[#text("(sympatiskt)")] #text("Du må inte hålla med, det gör inte jag alltid heller. Men order är order, det är så vi gör saker häromkring.")]
#text("Senap går av scenen.")
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(suckar)")] #text("För en stund där, hade jag glömt vilket helvete vi befann oss i. Sen väcktes jag ur den lyckliga fantasin…")]
#dialogue[#text("NIGHTINGALE")][#text("Det är synd, men oundvikligt. Det finns inget sätt att stoppa det nu.")]
#dialogue[#text("TOLKIEN")][#text("Jag vet. Men jag önskar verkligen det inte behövde vara så här.")]
#text("Nightingale stannar upp ett slag.")
#dialogue[#text("NIGHTINGALE")][#text("Det kanske det inte måste…")]
#dialogue[#text("TOLKIEN")][#text("Vad menar du?")]
#dialogue[#text("NIGHTINGALE")][#text("Jag har ingen plan än, men centralmaktssoldaterna som vi träffade förut verkade inte heller så ivriga på att kriga. ")#underline[#text("Om vi slår våra huvuden ihop kanske vi kan hitta en väg att avvärja anfallet tillsammans.")]]
#dialogue[#text("TOLKIEN")][#text("Det är värt ett försök! Vi kommer ju ändå möta dem ikväll igen.")]
#text("Lamporna släcks medans de sätter igång med sina sysslor i lägret.")
#pagebreak()
#scene[#text("SCEN 8")]
#text("[[")#text(style: "italic","Karaktärer: Schumacher, Flammenwerfer, Klimt, Röde Baronen, Müller")#text("]]")
#text("[[Plats: Centralmaktsläger]]")
#text("[[Müller är mer närvarande och ställer sig in lite mer hos soldaterna. Röda Baronen bondar med soldaterna men kan inte avslöja sig. Är fortfarande fokuserad på väskan. Han förbereder en plan för att övertala gänget att hjälpa honom hitta väskan igen. Alla går med på det omedelbart, de är ju kompisar nu. Schumacher, Klimt, & Röde Baronen ger sig av ut i ingenmansland.]]")
#text("EXT. Centralmaktslägret")
#text("Scenen fortsätter ett par timmar efter scen 6.5 avslut. Klimt har fyllt hela sin whiteboardtavla med obegripliga figurer, pilar och anteckningar. Bakom henne sitter Schumacher, Flammenwerfer och Röde Baronen på tre stolar.")
#dialogue[#text("KLIMT")][#text("…Och sammanlagt var det då alla dessa handlingar och sammanträffanden som ledde till krigets början. Några frågor?")]
#text("Klimt vänder sig om till att se att Röde baronen sitter och halvsover och att Flammenwerfer tillverkar något finurligt av en \"konservburk\". Klimt blir väldigt besviken av att se detta. Schumacher ser förvirrad ut och börjar räcka upp handen. [[Blir det verkligen roligt ifall vi drar ut på det? Tror att det bästa är ifall vi håller lite tempo, vi har ju redan levererat punchlinen]] [[Kanske lite onödigt, håller med]] [[Flammenwerfer skulle kunna sitta och sticka, skulle vara kul för att det är oväntat]] [[
🔥]]")
#dialogue[#text("KLIMT")][#parenthetical[#text("(plötsligt något exalterad)")] #text("Eh… Ja, Schumacher?")]
#dialogue[#text("SCHUMACHER")][#text("Ja, jag var lite nyfiken på det här caféet i Sarajevo. När jag var där minns jag inte att de hade kalkon på smörgåsarna, är det något nytt…?")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(gestikulerar till tavlan)")] #text("…Relaterat till vad som ORSAKADE kriget?")]
#dialogue[#text("SCHUMACHER")][#text("Nej, det är fortfarande ett mysterium. Men jag minns Franzis som en ganska trevlig typ, bråttom hade han. Sedan dess har jag inte hört av honom…")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(Sömndrucket)")] #text("Vänta? Du var där? [[lite oklar imo]]")]
#dialogue[#text("SCHUMACHER")][#text("I Sarajevo? Så klart! Men måste ändå säga att Prag är finare. Och har bättre mat! [[Lade till detta i ett försök av bonding]]")]
#dialogue[#text("RÖDE BARONEN")][#text("Eh, diskutabelt. Men låt mig säga att inget jämför med Budapests-")]
#text("Ev fortsätter de att jämföra städer de har varit i för komisk effekt. Klimt kämpar för att ta tillbaka ordet.")
#dialogue[#text("KLIMT")][#parenthetical[#text("(irriterat)")] #text("Men kom igen, ni har knappt lyssnat på någonting av vad jag sagt!")]
#text("Röde baronen reser sig kvickt upp från sin stol och börjar stjäla showen från Klimt.")
#dialogue[#text("RÖDE BARONEN")][#text("Hörni, nu ska vi inte tänka på sånt. Låt oss säga att hur kriget startade inte är särskilt viktigt. Det viktiga med krig är ändå hur de slutar, att hjältemod är det som alltid leder till vinst.")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(tröttnar på Baronen)")] #text("Vad är det du har i famnen där, Flammenwerfer?")]
#text("Flammenwerfer blir väldigt exalterad för att visa upp sin \"konservburk\" för alla.")
#dialogue[#text("FLAMMENWERFER")][#text("Detta är en handgranat. Nu när vi har brist på explosiva varor så vill jag se till att skapa några egna hemmagjorda. Den hemliga ingrediensen för denna är en massa spikar, krut och resten av bönorna som jag inte åt upp.")]
#dialogue[#text("KLIMT")][#text("Snälla inte igen, det här är åttonde-")]
#dialogue[#text("FLAMMENWERFER")][#parenthetical[#text("(ignorerar och avbryter)")] #text("Ja, visst är hon vacker. Jag undrar om matresterna gör någon förbättring. Ifall detta funkar så kan vi samarbeta med nya bomber i framtiden, Schumacher.")]
#text("Schumacher ser taggad ut. Klimt gör sig redo för Flammenwerfers dumheter. ")
#dialogue[#text("FLAMMENWERFER")][#text("Ja, vi får se hur det går när vi antänder-")]
#text("Klimt beslagtar \"konservburken\" från Flammenwerfer. Hon stirrar ner Flammenwerfer och Schumacher. Väldigt nonchalant, som att hon har gjort det tusen gånger tidigare. ")
#dialogue[#text("KLIMT")][#text("…Ni två ska inte hitta på någonting med matrester i konservburkar tillsammans. Gör dig av med den här innan Müller ser vad du har gjort. [[Byt ut denna line mot något helt annat]]")]
#text("Klimt börjar sträcka tillbaka burken till Flammenwerfer. General Müller kliver in på scenen under Klimts sista mening.")
#dialogue[#text("MÜLLER")][#text("Innan jag ser vad vem har gjort? [[Behåll att Müller svarar på tidigare line när han kommer in (men alltså det som den tidigare linen blir utbytt)]]")]
#text("Klimt hoppar till så att konservburken hamnar på marken. De ställer alla sig hastigt upp för att göra honnör, vilket välter stolarna. Röde baronen är den enda som gör detta med lugnet i behåll.")
#text("KLIMT, SCHUMACHER OCH FLAMMENWERFER [[Hur fan formaterade du det här? Det är typ snyggt...]] [[Använde bara Dual funktionen som finns längst till höger]]
(uppjagade)
General Müller!")
#dialogue[#text("RÖDE BARONEN^")][#parenthetical[#text("(avslappnad)")] #text("General Müller.")]
#dialogue[#text("MÜLLER")][#text("Lediga, soldater. Nå, vad för explosivt påhitt har Flammenwerfer kommit på denna gång? [[konstig meningsbyggnad]] [[Har skrivit om nu. Det borde låta bättre nu.]]")]
#text("Flammenwerfer ser \"konservburken\" och sparkar den lätt så att den hamnar bakom honom.")
#dialogue[#text("FLAMMENWERFER")][#parenthetical[#text("(ljuger rätt så dåligt)")] #text("Nej, det var bara min lunch, General.")]
#text("General Müller ser skeptisk ut, men skiftar sedan snabbt fokus till Röde Baronen.")
#dialogue[#text("MÜLLER")][#text("Nämen, Röde Baronen!") #parenthetical[#text("(skämtsamt)")] #text("Jag såg dig inte där. Jag kunde nästan ha misstagit dig för en av mina egna såsom ni umgås med varandra.")]
#dialogue[#text("RÖDE BARONEN")][#text("Haha, jo, jag har alltid haft en talang för att bli omtyckt; jag kan inte rå för det. Fast det är fördelaktigt inom min avdelning.")]
#dialogue[#text("KLIMT")][#text("Hur mycket behov har du av det som pilot?")]
#text("Röde Baronen blir lite nervös.")
#dialogue[#text("MÜLLER")][#text("Klimt! Han socialiserar säkert mer än du tror. Du måste lära dig vara mer öppensinnad.")]
#dialogue[#text("KLIMT")][#text("Jag var bara nyfiken…") #parenthetical[#text("(suck)")] #text("Ja, general Müller.")]
#text("Schumacher försöker att få en syl i vädret.")
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(otåligt)")] #text("Ursäkta mig, general Müller?")]
#dialogue[#text("MÜLLER")][#parenthetical[#text("(tålamod börjar ta slut)")] #text("Vad är det, Schumacher?")]
#dialogue[#text("SCHUMACHER")][#text("Har du övervägt min förfrågan att fixa julmat till imorgon?")]
#dialogue[#text("MÜLLER")][#text("Varför skulle jag ha ändrat mig?")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(kvicktänkt)")] #text("Vore det inte lite oartigt att låta Röde Baronen fortsätta äta av det lilla tråkiga vi har att erbjuda?")]
#dialogue[#text("SCHUMACHER")][#text("Precis!… Vänta, va?")]
#dialogue[#text("MÜLLER")][#parenthetical[#text("(sarkastisk)")] #text("Du har en poäng, Klimt. Nåväl, Schumacher. Jag ska försöka kontakta Jultomten via radion och se om han kan göra något åt saken, men jag kan inte lova någonting.")]
#text("Schumacher förstår inte Müllers sarkasm.")
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(glatt)")] #text("Verkligen? Kan du skicka vidare min önskelista till honom också? Jag glömde posta den. [[Anglocism]]")]
#text("Schumacher tar fram ett papper som ser ut som en lång inköpslista. Müller tittar knappt på den innan han stoppar listan i sin uniform.")
#dialogue[#text("MÜLLER")][#text("Absolut, jag tar itu med det på en gång. Adjö.")]
#text("Müller börjar bege sig ut och skrynklar ihop och kastar Schumachers önskelista. Efter någon sekund kommer Flammenwerfer (som inte såg den kastade listan) på något och börjar springa efter.")
#dialogue[#text("FLAMMENWERFER")][#text("General Müller, du måste skicka MIN önskelista också!")]
#text("Flammenwerfer följer med Müller och lämnar scenen. ")
#dialogue[#text("RÖDE BARONEN")][#text("Jag tror aldrig jag träffat en general som är så sällskaplig med sina underordnade.")]
#dialogue[#text("KLIMT")][#text("Jodå, han är trevlig. När han inte tvingar ut oss att bli skjutna i ingenmansland.")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(taggad)")] #text("Hörde ni? Müller kommer kontakta jultomten! Vi kommer få julmat! Vi behöver fira! Jag går och hämtar lite wurst.")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(stoppar Schumacher)")] #text("Firandet kan vänta, Schumacher. ")#underline[#text("Jag tycker vi borde ta oss till ingenmansland.")]#text(" Det är större chans att väskan hamnar i fel händer desto mer tiden rinner. Vilken katastrof det kan bli! Nej, se på klockan, det är snart mörkt, bäst att vi rör oss nu! [[Ändra till att Röde Baronen tittar på klockan och vill hetsa om att gå ut och möte de allierade. Klimt ser igenom lögnen och retas lite med honom]]")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(först inte så taggad, sen inser hon att det kan bli kul)")] #text("Om du säger det så. Ooh, vi kanske får träffa de där trevliga britterna igen! De skulle ha med sig fransk mat, det lovade de att de skulle! [[Schumacher nämner utbyte av mat i 5.5 men det kommer aldrig upp igen och jag är inte säker på om det går att klämma in. Känns konstigt att lämna det hängande såhär]]")]
#dialogue[#text("KLIMT")][#text("Vänta lite, innan vi går borde jag låsa förrådet innan Flamme-…")]
#text("Röde Baronen och Schumacher ignorerar Klimt helt och hållet och rör sig ut mot ingenmansland. Klimt märker detta lite för sent. ")
#dialogue[#text("KLIMT")][#parenthetical[#text("(ropar mot dem)")] #text("Vänta på mig!!!")]
#text("Klimt springer efter Röde Baronen och Schumacher.")
#pagebreak()
#scene[#text("SCEN 9")]
#text("[[")#text(style: "italic","Karaktärer: Tolkien, Nightingale, Schumacher, Klimt, Röde Baronen")#text("]]")
#text("[[Plats: Ingenmansland med flygplanskuliss]]")
#text("[[Centralmaktssoldaterna och de allierade möts i ingenmansland. Tolkien ger tillbaka väskan till Röda Baronen så fort han frågar. Röda Baronen är lite paff över hur lätt det gick. Nightingale berättar om attacken i gryningen. De kommer tillsammans på planen att transportera de allierades ammo till Centralmaktssoldaternas sida för att undvika blodbad vid morgonen.]]")
#text("[[På väg tillbaka träffar Tolkien och Nightingale på Senap och Cartier, som har frågor. Leder in till musiknummer 4.]]")
#text("Klimt, Schumacher och Röde Baronen letar omkring på scen. Baronen och Schumacher letar ivrigt men Klimt gör minsta möjliga ansträngning. Lyfter på samma rekvisita om och om igen typ. [[Hur borde Klimt bete sig? Göra minsta möjliga eller hjälpa till eftersom det är en \"vän\" som behöver det?]]")
#dialogue[#text("RÖDE BARONEN")][#text("Hojta bara till om ni hittar den. Kom ihåg: den är brun, med två spännen.")]
#dialogue[#text("SCHUMACHER")][#text("Oooo, jag tror jag hittat den!")]
#text("Schumacher böjer sig ner bakom en kuliss. Röde Baronen tittar exalterat upp.")
#dialogue[#text("RÖDE BARONEN")][#text("Är det sant?!")]
#dialogue[#text("SCHUMACHER")][#text("Nää, det var visst bara en portmonnä, måste ha varit en soldat som har tappat. [[Är det här fortfarande ok...?]]")]
#text("(omstart: det var bara en 30 meter lång haj)")
#dialogue[#text("RÖDE BARONEN")][#text("En väska, Schumacher! En VÄSKA!!")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(pekar mot de allierades läger)")] #text("Kolla! Där är de andra!")]
#text("Tolkien och Nightingale kommer in på scen. Tolkien bär väskan.")
#dialogue[#text("TOLKIEN")][#text("Hej igen. Har ni behövt vänta länge? [[längta? inte vänta?]]")]
#dialogue[#text("SCHUMACHER")][#text("Nej då. Vi bara begav oss ut extra tidigt för att leta runt efter en portmonnä åt Röde Baronen.")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(Suckar irriterat)")] #text("Nej! Inte portmonnä! En vä…") #parenthetical[#text("(Inser långsamt att Tolkien bär på den)")] #text("…ska…")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(Tar ett steg fram och monologar)")] #text("Där är den. Där är väskan. Nu kan jag äntligen fortsätta med min plan. Men hur ska jag få den här britten att ge över väskan till mig?")]
#text("Röde baronen slänger en blick på Tolkien.")
#dialogue[#text("RÖDE BARONEN")][#text("Vad kan någon som han vilja ha? Medaljer? Det kan jag definitivt fixa åt honom. Och vem skulle tacka nej till en flygtur med mig till Paris och att äta på de finaste av restauranger för att slutligen gå på teater med de största kändisarna i hela Europa? Och om inte det räcker så känner jag äv- [[skulle baronen tänka på väskan som avgörande för kriget?]] [[Väldigt lång mening i mitten här; jobbigt att läsa]] [[i think its hilarious u kids talking shit about röde baronen. u wouldnt say this shit to him at lan, hes jacked. not only that but he wears the freshest clothes, eats at the chillest restaurants and hangs out with the hottest dudes. yall are pathetic lol.]]")]
#text("Tolkien går fram till Röde Baronen och räcker över väskan.")
#dialogue[#text("TOLKIEN")][#text("Just det, jag hittade detta i vraket! ")#underline[#text("Jag tror den är din.")]]
#text("Röde Baronen står med öppen mun helt paff och stirrar på Tolkien.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(Till Tolkien)")] #text("Största kändisarna… i Europa…")]
#text("Tolkien tittar frågande på Röde Baronen. Schumacher går fram och armbågar Röde Baronen i sidan.")
#dialogue[#text("SCHUMACHER")][#text("Vad säger man, Baronen?")]
#text("Röde Baronen samlar sig.")
#dialogue[#text("RÖDE BARONEN")][#text("Tack… Tack. Men varför? Varför hjälper du mig? Från ditt perspektiv borde jag vara… en fiende. [[Gör mindre formell]]")]
#text("Tolkien tittar frågande en sekund på Röde Baronen.")
#dialogue[#text("TOLKIEN")][#text("Jag finner att det är det lilla i varje liten vardagshandling från varje liten människa som håller mörkret på avstånd. Små handlingar av vänlighet och kärlek. [[Kanske är kul att faktiskt citera riktiga Tolkien här med “I have found that it is the small everyday deed of ordinary folks that keep the darkness at bay. Small acts of kindness and love.\" Kanske inte riktigt passar karaktären i det här spexet dock, men vore lite snygg referens för de som fattar]] [[Det finns ju även citat från lord of the rings både från gandalf och sam]]")]
#text("Alla står tysta en sekund och orden får sjunka in.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(försöker komma med ursäkter)")] #underline[#text("Ni får ursäkta mig")]#text(". Jag måste se till att… allt är kvar och att den inte är skadad. Vi ses borta i lägret sen! [[mot centralmaktasälgret????]]")]
#text("Röde Baronen rör sig bort mot centralmaktslägret medans övriga karaktärer tittar frågande på honom.")
#text("Tystnaden varar lite för länge.")
#dialogue[#text("SCHUMACHER")][#text("Just det! Vi sa vi skulle dela mat! Jag tog med mig lite wurst och sauerkraut. Vad har niii? [[Slipa]]")]
#text("Schumacher börjar gräva i fickorna. Tolkien ser besvärad ut.")
#dialogue[#text("TOLKIEN")][#text("Det är väldigt trevligt, ")#underline[#text("men allt vi har att komma med är dåliga nyheter")]#text(", är jag rädd.")]
#dialogue[#text("SCHUMACHER")][#text("Vad för dåliga nyheter? Har ni ätit upp all ost?")]
#dialogue[#text("NIGHTINGALE")][#text("Värre än så. ")#underline[#text("General Bollbirth tänker genomföra ett fullt anfall redan imorgon bitti")]#text(". Ingen har väl klagat på fler chanser att utöva sin läkekonst, men till och med jag har mina tvivel. Det kommer bli ett blodbad! Vår sida kommer bli springande måltavlor när vi rör oss över ingenmansland, men lyckas vi ta oss över kommer förlusterna bli enorma för er med! [[Nightingale har ingen anledning att veta några av dessa detaljer - han har knappt fått chans att vara i lägret]] [[Tweaka]] [[Speciellt så att Nightingale får lite mer karaktär]]")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(frustrerat)")] #text("Det är illa, riktigt illa. Fan, jävla idiotgeneraler! Alltid så galna av all makt!")]
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(förtvivlat)")] #text("Jag vill inte slåss mot er!")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(lite mer samlat)")] #text("Det vill nog ingen här göra. Men vad kan vi göra? Om er general bestämt sig för att genomföra planen så blir det väl som det blir.")]
#dialogue[#text("NIGHTINGALE")][#text("Något måste vi väl kunna göra? Kanske kan vi få soldaterna att vapenvägra?")]
#dialogue[#text("KLIMT")][#text("Absolut. Men då ska ni också övertala generalen att inte avrätta hela bunten också. Nej, ni skulle behöva något annat.")]
#dialogue[#text("TOLKIEN")][#text("Jag är inte så bra på det här med att kriga-")]
#dialogue[#text("NIGHTINGALE")][#text("Nej det är du verkligen inte.")]
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(låtsas som att han inte hörde Nightingale)")] #text("Eller krigsstrategi för den delen… Men med nya problem brukar det vara bra att gå tillbaka till grunden. Vad behövs för att ett krig eller en strategi ska kunna genomföras?")]
#dialogue[#text("KLIMT")][#underline[#text("Logistik.")]]
#dialogue[#text("SCHUMACHER")][#text("Mat!")]
#dialogue[#text("KLIMT")][#text("Ja, mat är en del av logistik. Men det finns så mycket mer. Soldater måste äta, vapen måste laddas, barrikader måste förstärkas. Logistiken är blodet som håller fronten vid liv.")]
#dialogue[#text("NIGHTINGALE")][#text("Vi kanske kan svälta soldaterna, så de inte orkar anfalla?")]
#dialogue[#text("TOLKIEN")][#text("Det kommer inte förändra något på så här kort tid. Det är inte som att vi aldrig gått hungriga innan.")]
#dialogue[#text("SCHUMACHER")][#text("Men- ")#underline[#text("Om ni inte skulle ha någon ammunition… Då skulle det inte gå attackera")]#text(", eller hur?")]
#text("Alla stannar upp lite. Det är nästan en ")#text(style: "italic","för")#text(" uppenbar lösning.")
#dialogue[#text("KLIMT")][#text("Ja, om er logistikkedja fungerar likt vår så skulle det bli helt omöjligt att anfalla ifall förråden var tomma.")]
#text("De tittar alla på Tolkien.")
#dialogue[#text("TOLKIEN")][#text("Öööh… Ja, nu när du säger det så har jag sett att vi får leveranser med ammunitionslådor varje månad. Så vad du säger är att ")#underline[#text("om vi skulle råka… \"omplacera\" dessa så kan anfallet stoppas?")]]
#dialogue[#text("KLIMT")][#text("Åtminstone fördröjas.")]
#dialogue[#text("NIGHTINGALE")][#text("Men vad ska vi göra av allt? Gräva ner dem i leran?")]
#dialogue[#text("SCHUMACHER")][#underline[#text("Det finns plats i mina skafferier.")]#text(" Vi kan gömma dem där.")]
#dialogue[#text("KLIMT")][#text("Schumacher har rätt - ")#underline[#text("våra förråd har gott om plats.")]]
#dialogue[#text("NIGHTINGALE")][#text("Och ingen på er sida kommer märka att det plötsligt dyker upp massor med ny ammo?")]
#dialogue[#text("KLIMT")][#text("Antagligen inte. Det är trots allt jag som är materialansvarig och inte någon annan. General Müller är aldrig inne i vårt förråd så han kommer inte märka något, och ingen annan kommer lägga sig i mitt jobb. [[För uppenbar foreshadowing?]] [[Nä, det är lugnt]]")]
#dialogue[#text("NIGHTINGALE")][#text("Utmärkt! Tolkien och jag beger oss tillbaka till vårt läger och börjar hämta lådor.")]
#dialogue[#text("SCHUMACHER")][#text("Då går vi och förbereder materialförrådet! Klimt, vi måste se till att ingen ser oss när vi flyttar lådorna, kanske Flammenwerfer kan distrahera de andra i lägret?")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(Fokuserad på Nightingale)")] #text("Mmm, absolut. Men som jag redan sagt så kommer det bara att fördröja anfallet, det kommer alltid mer material.")]
#dialogue[#text("TOLKIEN")][#text("Men om kriget inte går framåt kanske de till slut ger upp dessa dumheter?")]
#dialogue[#text("NIGHTINGALE")][#text("Låt oss fokusera på att överleva morgondagen, framtiden kan vänta.")]
#text("De andra nickar.")
#dialogue[#text("KLIMT")][#text("Tiden går, vi måste komma igång nu, om vi ska hinna klart till gryningen.")]
#text("De tar alla farväl av varandra och beger sig mot sina läger. Schumacher och Klimt går av scen. Tolkien stannar plötsligt upp. Nightingale stannar och tittar på honom, frågande.")
#dialogue[#text("TOLKIEN")][#text("Vänta lite. ")#underline[#text("Hur ska vi se till att Senap inte får reda på något")]#text("?")]
#dialogue[#text("NIGHTINGALE")][#text("Du tror inte vi kan vinna över henne till planen?")]
#dialogue[#text("TOLKIEN")][#text("Inte en chans. Hon är för plikttrogen för det. Hon skulle se det som att vi gör förräderi… rätt mycket förräderi också. [[Det är ju förräderi... ev omfrasering]]")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(tankfullt)")] #underline[#text("Mmmm… Jag tror jag har en idé.")]#text(" Gör bara som mig.")]
#text("  ")
#text(weight: "bold","Musiknummer 4")
#text("[[Tolkien övertygar Senap och Cartier med fina ord. Planen sätts i verket och karaktärerna bondar. Kanske flyttar en och annan ammolåda. Röde Baronen glider förbi mot den allierade sidan och säger att han ska dit.]] [[Enligt högre makter så MÅSTE Röde Baronen vara med i detta musiknummer vilket innebär att resten av manus måste anpassas]]")
#pagebreak()
#scene[#text("SCEN 10 [[\"++++++\"?]] [[va]] [[Nani the fuck?]]")]
#text("[[")#text(style: "italic","Karaktärer: Tolkien, Nightingale, Senap, Cartier, Schumacher, Klimt")#text("]]")
#text("[[Plats: Allierat läger]]")
#text("[[Scenen där vi ser hur de allierade sköter sin del av planen. Någon frågar var Röde Baronen är. Cartier är skeptisk men blir nedröstad. Centralmaktssoldaterna kommer och hänger.]]")
#text("EXT. Allierat läger")
#text("Senap står och kollar på Tolkien och Nightingale som bär på ammunitionslådor. Cartier sitter i ett hörn och röker. Tolkien (som går först) ser något på marken och stannar plötsligt upp, varpå Nightingale går in i honom. Båda ramlar nästan. Senap ser väldigt besviken ut.")
#dialogue[#text("SENAP")][#text("Nej, men, vad håller ni på med?!")]
#dialogue[#text("TOLKIEN")][#text("Bär på lådor.")]
#dialogue[#text("SENAP")][#text("Jag vet! Vi hade precis ett helt musiknummer om det, men ni bär ju som ett par oduglingar! Jag vet fortfarande inte vems order ni utför, men det skär i mitt stackars officerhjärta att se ert arbete utföras så här dåligt! Låt mig styra upp det här.")]
#text("Senap går fram och börjar dirigera Tolkien och Nightingale som en trafikpolis.")
#dialogue[#text("SENAP")][#text("Kom och hjälp till med att bära, Cartier.")]
#dialogue[#text("CARTIER")][#text("Det enda jag bär är post… Och vikten av den franska stoltheten.")]
#dialogue[#text("SENAP")][#parenthetical[#text("(irriterat)")] #text("Cartier, hjälp till, det är en order.")]
#dialogue[#text("CARTIER")][#text("Jag tar inte order från dig! Endast från postmästare Debois. Jag är en konsult.")]
#dialogue[#text("TOLKIEN")][#text("Men… tänk om jag postar en av lådorna då?")]
#dialogue[#text("NIGHTINGALE")][#text("Ja! Skriv bara en adress så måste han ju bära den!")]
#dialogue[#text("CARTIER")][#parenthetical[#text("(förskräckt)")] #text("Sacre bleu! Du skulle bara våga! Om du postar utan frimärke så bryter du mot den heligaste av lagar.")]
#text("Tolkien skriver på sin låda och Nightingale tar upp ett frimärke från fickan och fäster det - räcker sedan lådan till Cartier.")
#dialogue[#text("CARTIER")][#parenthetical[#text("(bittert)")] #text("Merde…")]
#text("Cartier tar motvilligt upp lådan. Schumacher och Klimt, utklädda till matroser, kommer in. [[Matroser? Har vi budget för sjömanskostymer?]]")
#dialogue[#text("KLIMT")][#text("Styrman Klim… Klint från… flottan… anmäler sig!")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(lite blyg)")] #text("Matros Sch-mith också.")]
#dialogue[#text("CARTIER")][#parenthetical[#text("(suckande)")] #text("Sacre bleu, de blir fler…")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(all blyghet plötsligt borta)")] #text("Åhh, en fransman! Har du någon fransk mat jag kan få smaka! Kanske någon sån där eskargåt?! Eller, ja, du kan lära mig alla hemligheter som gör det franska köket!!!")]
#dialogue[#text("CARTIER")][#parenthetical[#text("(fnyser till)")] #text("Skulle aldrig ge bort något så heligt till en smutsig britt.")]
#dialogue[#text("SCHUMACHER")][#text("Jag är ju ingen britt!")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(Panikslaget)")] #text("Tyst med dig!!")]
#text("Cartier försvinner av scen med lådan.")
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(förvånat)")] #text("Va- Vad gör ni här? Och varför är ni klädda i sjömansuniformer mitt i en krigszon?")]
#dialogue[#text("KLIMT")][#text("Vi är här för att skydda er från flottan! Tyska flottan alltså. Och från hajar!")]
#dialogue[#text("TOLKIEN")][#text("Men det finns inget vatten här?")]
#dialogue[#text("KLIMT")][#text("Det är precis vad de vill att ni ska tro.")]
#dialogue[#text("NIGHTINGALE")][#text("Var fick ni ens de här kläderna från?")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(självsäkert)")] #text("Vi fick dem av kungen när vi föddes… Som alla britter!")]
#text("Senap stirrar enormt förvirrat på Schumacher.")
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(panikslaget)")] #text("Men ni kan inte vara här! Tänk om någon får syn på er- tänk om General Bollbirth eller Senap s…")]
#text("Senap harklar sig.")
#dialogue[#text("SENAP")][#text("Känner ni för att förklara?")]
#text("Tolkien vrider och vänder på sig lite obekvämt.")
#dialogue[#text("TOLKIEN")][#text("Ja… Ja ja du ja du vet tjaa näej eller ja nja jo nej men ja-")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(ser inte meningen i att ljuga)")] #underline[#text("Det här är Schumacher och Klimt. De är från centralmaktslägret.")]]
#dialogue[#text("KLIMT")][#parenthetical[#text("(till Schumacher)")] #text("Jag sa ju att det här var en dålig idé…")]
#text("Senap får en stroke.")
#dialogue[#text("SENAP")][#text("Ursäkta?! Säger du att de- de- de är FIENDER? [[vill vi ha \"vänner\"?]]")]
#text("Nightingale och Tolkien vrider och vänder på sig.")
#dialogue[#text("SENAP")][#text("Vänta, så det här är en sammansvärjning mellan er? ")#underline[#text("Ni samarbetar med centralmakten?!")]#text(" Vad är det ens ni bär på?!")]
#dialogue[#text("SCHUMACHER")][#text("Det är ammunition! Nog för att sänka en hel flock bisonoxar flera gånger om, skulle jag säga!")]
#dialogue[#text("SENAP")][#parenthetical[#text("(riktat till Tolkien, med rejält med sammanbiten ilska i rösten)")] #text("Är det tysken säger sant??")]
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(Nervöst)")] #text("Ehh, jo… kanske…")]
#dialogue[#text("SENAP")][#parenthetical[#text("(rasande)")] #text("Menar ni alltså att ni bär vår ammunition till deras läger?! Det ni gör är högförräderi!")]
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(rättande)")] #text("Det VI gör…")]
#dialogue[#text("SENAP")][#text("Vi?!")]
#dialogue[#text("TOLKIEN")][#text("Ja… ")#underline[#text("Du hjälper ju oss")]#text("…")]
#text("Senap inser vad hon håller på med.")
#dialogue[#text("SENAP")][#text("Jag… Ni lurade med mig till det här? Ni har gjort mig till en förrädare!")]
#dialogue[#text("KLIMT")][#text("Nej, men, lyssna här. ")#underline[#text("Vi vill bara förhindra en idiotisk attack. Utan ammunition kan den inte ske.")]#text(" Och det är bara för en dag! ")#underline[#text("Vi kommer lämna tillbaka ammon så fort attacken ställts in")]#text(", och när vi har gjort det kommer massa liv ha räddats! Vi vill alla samma sak. [[Putsa - Klimt borde nog inte hypa sin general]] [[lite slip: done!]]")]
#dialogue[#text("SCHUMACHER")][#text("Att få äta fransk mat!")]
#text("Klimt tar snabbt tillbaka ordet.")
#dialogue[#text("KLIMT")][#text("Att inte slåss! Ingen av oss vill kasta bort våra liv imorgon.")]
#dialogue[#text("TOLKIEN")][#text("Lyssna på dem, löjtnant. Vi skulle varit hemma till jul, men kriget bara fortsätter. Det skulle inte vara såhär. Detta var inte vad vi skrev upp oss för.")]
#dialogue[#text("NIGHTINGALE")][#text("Om vi vill leva och se ännu en dag så är det här det bästa valet vi har. Det enda valet.")]
#dialogue[#text("SENAP")][#text("Men, men… General Bollbirths order då? Oavsett om anfallet är sunt eller ej så kan vi ju inte gå emot generalen! Nej, jag kan inte gå med på detta. Det här är förräderi.")]
#text("Klimt suckar och funderar lite snabbt.")
#dialogue[#text("SCHUMACHER")][#text("Men duuu? Har er general verkligen sagt att ni ")#text(style: "italic","inte får")#text(" smuggla över all ammunition till andra sidan ingenmansland, helt ordagrant?")]
#dialogue[#text("SENAP")][#parenthetical[#text("(tveksamt)")] #text("Alltså, inte direkt…")]
#dialogue[#text("KLIMT")][#text("Då så! Då går ni ju inte emot hennes order! Du som officerare är ju inte dum i huvudet! Klart du kan ta vissa beslut själv. Tänk hur hemska resultaten blir annars.")]
#dialogue[#text("TOLKIEN")][#text("Du vet… Din pappa hade inte tvekat… [[Huzzah! Daddy issues!]]")]
#text("Senap ställer sig och grubblar en kort stund. Hon verkar nästan vara övertygad fram tills att hon blir avbruten av Schumacher.")
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(fundersamt)")] #text("Varför bryr du dig ens så mycket om vad den där Bollbirth tycker?")]
#dialogue[#text("SENAP")][#text("Vadå varför? Varför går solen upp på morgonen? För att det är så militären fungerar. Genom en hierarkisk struktur har vi sedan urminnes tider hindrat total anarki och vettlöst beteende! Det är det enda sättet!")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(muttrar)")] #text("Det förutsätter att de i toppen inte är vettlösa…")]
#dialogue[#text("SENAP")][#parenthetical[#text("(förolämpat)")] #text("General Bollbirth må vara lite excentrisk, men hon är inte vettlös! Jag tänker inte tolerera sådana uttalanden!")]
#text("Nightingale kliver emellan de två innan de ryker ihop.")
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(lugnande)")] #text("Sååjaaa, nu ska vi inte börja bråka. Vi har fortfarande ett jobb att göra. [[Ord]] [[Underlig mening]]")]
#dialogue[#text("SENAP")][#text("Ett jobb ni får göra utan mig. Jag… jag måste tänka.")]
#text("Senap vänder sig tvärt och börjar gå av scenen.")
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(nervöst)")] #text("Vänta, tänker du berätta för generalen?")]
#dialogue[#text("SENAP")][#parenthetical[#text("(stannar knappt upp)")] #underline[#text("Prata inte mer med mig om det här")]#text(". Lämna mig utanför.")]
#text("Senap fortsätter av scenen.")
#dialogue[#text("KLIMT")][#text("Kan vi lita på henne?")]
#dialogue[#text("TOLKIEN")][#text("Jag tror inte att hon tog det så bra, men hon verkar inte heller vilja rapportera oss… ")#underline[#text("Vi kan nog lita på henne.")]]
#dialogue[#text("KLIMT")][#text("Då så. I så fall bör vi nog köra igång med bärandet igen. Vi har fortfarande mycket arbete kvar.")]
#dialogue[#text("NIGHTINGALE")][#underline[#text("Har någon förresten sett Röde Baronen?")]]
#text("Alla tittar på varandra. Ingen vet något.")
#dialogue[#text("NIGHTINGALE")][#text("Inte det? Det var ju konstigt.")]
#dialogue[#text("TOLKIEN")][#text("Vad skulle han ens göra?")]
#text("Cartier dyker upp igen.")
#dialogue[#text("CARTIER")][#text("Er dumma låda är levererad. Nu om ni ursäktar mig så har jag RIKTIG post att leverera. Allons-y!")]
#text("Cartier går av scen med bestämda steg.")
#pagebreak()
#scene[#text("SCEN 11")]
#text("[[")#text(style: "italic","Karaktärer: Bollbirth, Röde Baronen, Cartier")#text("]]")
#text("[[Plats: Bollbirths tält]]")
#text("[[Bollbirth uttrycker sina klagomål till Cartier. Röde Baronen tar sig till allierades sida och ger väskan till Bollbirth. Röde Baronen blir lovad ära och berömmelse. Bollbirth börjar detaljera hur hon tänker använda gasen. Hintar om att det kan vara dess sista vapen om allt går snett. Röda Baronen börjar tveka.]]")
#text("Bollbirth sitter bakom sitt skrivbord och planerar anfallet. Cartier kommer in.")
#dialogue[#text("CARTIER")][#parenthetical[#text("(gör honnör)")] #text("General Bollbirth.")]
#dialogue[#text("BOLLBIRTH")][#text("Vad är det nu då? Varför stör du mig? Vad är din post?")]
#dialogue[#text("CARTIER")][#text("Det är inte min post, general, det är DIN post.")]
#text("Cartier gestikulerar med de brev han bär på.")
#dialogue[#text("BOLLBIRTH")][#text("Ja ja, just det, brevbäraren.") #parenthetical[#text("(suckar)")] #text("Vad är det för post?")]
#text("Cartier tar upp en bunt med papper och går igenom den. ")
#dialogue[#text("CARTIER")][#text("Klagobrev från general Jones, reklam, klagobrev från general Hudson, reklam för fotprodukter, ett mycket välskrivet kärleksbrev från anonym sändare, reklam, krigspropaganda, reklam…")]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(surt/lite argt)")] #text("Finns det något viktigt i den här skräphögen?")]
#dialogue[#text("CARTIER")][#text("Inte speciellt, bara ett brev från krigsministern.")]
#dialogue[#text("BOLLBIRTH")][#text("Krigsministern!? Varför började du inte med det då?? Läs det, läs det!")]
#dialogue[#text("CARTIER")][#parenthetical[#text("(med en helt annan röst)")] #text("\"General Bollbirth,") #text("Din ansökan om tillåtelse att få hela de Allierades bombförråd levererat till dig har blivit nekad.-")]
#text("Bollbirth ger ifrån sig en frustrerat suck. Kanske ser lite besviket trotsig ut.")
#dialogue[#text("CARTIER")][#text("-då de allierade inte kan fortsätta slösa resurser på en enda generals behagan.") #text("Må vinsten vara med oss, krigsminister Kitchener.\"")]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(hånfullt och surt)")] #text("\"mÅ vInStEn vArA mEd oSs\"") #text("Lyssna bara på den patetiska idioten! Det är nästan som att de vill göra det svårare för mig med flit!") #text("Jaja, det spelar ingen roll. Jag kommer vinna i alla fall, vad det än kostar! Vi får helt enkelt klara oss med det vi har. Var det något mer?")]
#dialogue[#text("CARTIER")][#text("Ett meddelande från löjtnant Wilson. Han har brutit benet igen och måste förbli sängliggande i någon vecka.")]
#dialogue[#text("BOLLBIRTH")][#text("Arhg! Varför är jag konstant omgiven av odugliga, klantiga, hariga idioter!? ")#underline[#text("Finns det seriöst ingen här i lägret som är har den minsta lilla kompetens?")]]
#dialogue[#text("CARTIER")][#parenthetical[#text("(stolt, kanske lite överdrivet så, detta är hans chans)")] #text("Generalen, jag-")]
#text("Röde Baronen, som hade anlänt till tältet lite innan och hört den sista biten kommer in på scen och avbryter Cartier.")
#dialogue[#text("RÖDE BARONEN")][#text("Varför säga så, generalen? Jag finns ju här!")]
#text("Bollbirth och Cartier vänder sig båda mot honom och ser centralmaktsuniformen. Bollbirth drar sin pistol. Cartier drar sin brevkniv. (Bollbirth ställer sig upp från sin stol om hon inte gjort det tidigare). Röde Baronen sträcker upp armarna lite nonchalant.")
#dialogue[#text("CARTIER")][#text("Sacre bleu!! En tysk!")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(irriterat)")] #text("Ungrare.")]
#dialogue[#text("BOLLBIRTH")][#text("Sak samma!") #parenthetical[#text("(osäkrar vapnet)")] #text("Vem är du, hur kom du in!?")]
#dialogue[#text("CARTIER")][#text("Min general, om ni bara ger ordern, så sliter jag denna fiende i bitar och postar det till deras ledare. Jag ska allt visa den franska själens styrka!")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(ignorerar Cartier)")] #text("Generalen är mycket smart som ställer frågor först och skjuter sen. ")#underline[#text("Annars kommer hon ju inte få sitt efterlängtade triumfkort.")]]
#dialogue[#text("BOLLBIRTH")][#text("Åh. Så det är du. Postis, ledig. Lämna tältet.")]
#text("Bollbirth sänker sitt vapen och sätter sig igen.")
#dialogue[#text("CARTIER")][#parenthetical[#text("(förvånad)")] #text("Men generalen…")]
#dialogue[#text("BOLLBIRTH")][#text("Läm-na Täl-tet. Två ord, soldat. Två ord.")]
#text("Cartier lämnar tältet efter att ha gett Röde Baronen en arg blick.")
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(till Röde Baronen)")] #underline[#text("Så det är du som är min spion?")]#text(" Du är sen.")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(nonchalant, spelar upp situationen)")] #text("Jag anlände precis när det var tänkt.")]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(lite spydigt)")] #text("Åh, gjorde du? Har du åtminstone min… leverans? Eller ska den också anlända \"precis när det var tänkt\"?")]
#dialogue[#text("RÖDE BARONEN")][#text("Den har jag här.") #parenthetical[#text("(visar upp väskan)")] #text("Kom bara ihåg att lämna en fem-stjärnig recension till dina överordnade.")]
#dialogue[#text("BOLLBIRTH")][#text("Såklart! Jag har din belöning här.")]
#text("Bollbirth ger baronen en väska med pengar.")
#dialogue[#text("BOLLBIRTH")][#text("Du har förtjänat det, nu när du äntligen har gett mig min senapsgas.")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(leende, tittar lite på pengarna)")] #text("Man tackar, generalen.") #parenthetical[#text("(ger Bollbirth väskan med receptet)")] #underline[#text("Får jag fråga vad \"senapsgas\" är förresten? Ingen förklarade det för mig innan.")]]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(tar väskan och tittar i den och ler)")] #underline[#text("Ett vapen för krigsbrott.")]]
#text("Röde baronen hostar till.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(får en liten chock)")] #text("Uh, du får ursäkta mig, general, jag måste ha hört fel.")]
#text("Bollbirth ger ett häfte från väskan till Röde Baronen som detaljerar senapsgasen. Röde Baronen bläddrar snabbt igenom den, och ser mer och mer chockad ut ju längre han kommer.")
#dialogue[#text("BOLLBIRTH")][#text("Läs det här. Jag är trött på att behöva förklara saker för sinnesslöa drönare. Låt oss bara säga att det här kommer att sprida skräck i fienden och slå ut så pass mycket av deras styrkor att de aldrig lär återhämta sig. [[tror sista meningen behöver arbetas på]]")]
#text("Röde Baronen är fullt uppslukad av att läsa i häftet. En kort tystnad uppstår medans de båda fokuserar på sitt. Röde Baronen är det första att ta till orda.")
#dialogue[#text("RÖDE BARONEN")][#text("Så… när tänkte du använda detta… vapen?")]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(utan att titta upp)")] #text("Så snart gasen är producerad, såklart. Det ska inte ta lång tid.")]
#text("Det blir en lagom lång pinsam tystnad. Röde Baronen sväljer men samlar sig till slut.")
#dialogue[#text("RÖDE BARONEN")][#text("Så… angående min övergång till Storbritannien?")]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(tittar äntligen upp)")] #text("Åh, ja, just det! Du behöver ett rekommendationsbrev från mig. Du vill nog ta dig så långt bort härifrån som möjligt, eller hur?")]
#text("Bollbirth ger Röde Baronen ett brev. Röde Baronen nickar lite stelt och tar emot det. ")
#dialogue[#text("BOLLBIRTH")][#text("Dåså, nu måste jag påbörja produktionen av mitt supervapen. Om du vore snäll?")]
#text("Bollbirth gestikulerar mot utgången och Röde Baronen går ut, Bollbirth strax efter honom.")
#text("  ")
#underline[#text(weight: "bold","PAUS")]
#pagebreak()
#scene[#text("MINISCEN IN I DANSNUMMER")]
#text("Cartier kommer in på scen från att precis blivit utvisad från tältet. Han är tydligt stött.")
#dialogue[#text("CARTIER")][#text("Vilka tror de att de är? Först lurar Tolkien och Nightingale mig att bära deras mystiska lådor, sen slänger Bollbirth ut mig på grund av någon oduglig postiljon klädd som en tysk. Något suspekt pågår i detta läger… [[Tänkte skriva postpojke men tänkte Cartier skulle ha respekt för sitt eget yrke :)]]")]
#text("Han tar fram en cigarett och lutar sig mot en kuliss.")
#dialogue[#text("CARTIER")][#text("Ingen på denna front har någon respekt för en förfinad fransman, såsom mig själv.") #parenthetical[#text("(suck)")] #underline[#text("Jag längtar hem till Paris…")]]
#text("  ")
#text(weight: "bold","Dansnummer")
#pagebreak()
#scene[#text("SCEN 12")]
#text("[[")#text(style: "italic","Karaktärer: Bollbirth + Menig 0-2, materialansvarig, resten av kören+dans är random soldater")#text("]]")
#text("[[Plats: Allierat läger]]")
#text("[[Kören och dans scen. Allierade soldater som har sina tvivel och inte heller är så pigga på attacken. Bollbirth kommer för att förklara det logistiska för att kunna utföra operation senapsgas. Upptäcker samtidigt ammoförråden. Materialansvarig säger “whoops, måste gjort ett misstag”. Bollbirth skjuter materialansvarig och ger jobbet till nästa möjliga. ]]")
#text("På den allierade sidan står kören och dans och vattenmelonar runtom på scen. Mitt på scen står Menig 0 och Menig 1 och pratar, någonstans i bakgrunden står Materialansvarig och ser lite småstressad ut. [[Vilken datalog nollindexerade meniga ]] [[En median]]")
#dialogue[#text("MENIG 0")][#text("Har du hört? Det låter som att det blir ett anfall till slut.")]
#dialogue[#text("MENIG 1")][#parenthetical[#text("(med en suck)")] #text("Ja, och jag hörde att de tror att det kommer bli ett blodbad.")]
#dialogue[#text("MENIG 0")][#text("Usch, varför håller vi ens på så här?")]
#text("Det kommer fram en annan menig och tar sig in i diskussionen.")
#dialogue[#text("MENIG 2")][#text("Vadå!? Om anfallet går bra så tar kanske det här förbannade kriget slut! Vi kanske kan få åka hem!")]
#dialogue[#text("MENIG 1")][#text("Fast är det verkligen värt att sätta livet till? Om det nu ska bli ett blodbad håller jag mig helst så långt borta som möjligt. [[Inte... nödvändigtvis?]] [[ändrade det lite]]")]
#dialogue[#text("MENIG 2")][#parenthetical[#text("(suck)")] #text("Jag vill bara få ett slut på det.")]
#dialogue[#text("MENIG 0")][#parenthetical[#text("(ser förbryllat på menig 2)")] #text("Oj, säg inte så, så mörkt är det väl ändå inte. Och var lite glad, det är ändå jul!")]
#text("Materialansvarige går nervöst fram till de andra. ")
#dialogue[#text("MATERIALANSVARIG")][#parenthetical[#text("(orolig/nervös)")] #text("Hej, eh… ni råkar inte ha sett vart ammunitionen har tagit vägen?")]
#text("De andra tittar förvånat på materialansvarig.")
#dialogue[#text("MENIG 2")][#text("Vadå? Du som materialansvarig borde väl veta. Har du kollat i ammunitionsförrådet?")]
#dialogue[#text("MATERIALANSVARIG")][#parenthetical[#text("(börjar någorlunda lugnt men blir mer och mer panikslagen)")] #text("Ja, jag har kollat och det är helt tomt! Det var proppfullt igår men nu har allt försvunnit. Vad ska jag göra, jag kan inte misslyckas med mitt enda ansvar?")]
#text("Materialansvarig bryter ihop.")
#dialogue[#text("MENIG 0")][#text("Såja såja, ta det lugnt, du har kanske inte tittat tillräckligt noga.")]
#dialogue[#text("MATERIALANSVARIG")][#text("Nä, Redfield var med mig och kan bekräfta, det är helt tomt!")]
#text("Materialansvarig pekar på Redfield (en från dans) som nickar nervöst.")
#dialogue[#text("MATERIALANSVARIG")][#text("Hur ska vi nu kunna förbereda ett anfall!?")]
#text("MENIG 2 [[vet ej vilken av dem som bäst kan säga något sådant, men det känns som att jag kanske övertänker och det spelar ingen roll]]
(har bevisligen ingen koll)
Du borde berätta för Bollbirth. Hon har säkert en lösning!")
#text("De andra tittar lite skeptiskt på Menig 2. Bollbirth kliver in på scenen med bestämda steg. Soldaterna ser skrämda ut.")
#dialogue[#text("BOLLBIRTH")][#text("Varför står ni här och babblar. Det har vi inte tid med, vi har ett viktigt jobb att göra. Vem av er är ansvarig för materielen?")]
#text("Materialansvarig trycker sin clipboard i händerna på Redfield när Bollbirth inte tittar. Bollbirth ser clipboarden och går fram till denne. Hon håller fram väskan. ")
#dialogue[#text("BOLLBIRTH")][#text("Här, det här är receptet på senapsgas. Det är ett nytt vapen. Jag vill ha det framställt till imorgon. [[TID?]] [[NÄR ÄR JULAFTON?]] [[TANDPETARE]]")]
#text("Fejk-materialansvarig (Redfield) ser skrämd och förvirrad ut. Sträcker sig försiktigt efter väskan. Bollbirth börjar ana onåd och inspekterar soldaterna noga med blicken.")
#dialogue[#text("BOLLBIRTH")][#text("Vad är det med er? Varför ser ni så rädda ut? Döljer ni något?")]
#text("Bollbirth spänner blicken i soldaterna som nervöst tittar på varandra.")
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(ryter)")] #text("Svara när ni blir tilltalade!")]
#dialogue[#text("MENIG 0")][#underline[#text("All ammunition är borta, general…")]]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(argt)")] #text("Ursäkta, vad sa!?")]
#dialogue[#text("MENIG 0")][#text("All ammunition är borta, Materialansvarig sa att det bara försvann.")]
#text("Paus för eventuell omstart.")
#text("Förslag till omstart: [[när i scenen ska omstarten vara?]] [[Vid julafton]]")
#dialogue[#text("BOLLBIRTH")][#text("Vad sa ni att de gjorde sa ni!!!!")]
#dialogue[#text("MATERIALANSVARIG")][#text("Ja de har tagit ammunitionen och gett den till någon mer behövande.")]
#dialogue[#text("BOLLBIRTH")][#text("VA! Har jag en kommunist i mina led?")]
#dialogue[#text("BERÄTTARE/ANNAN MENIG (V.O.)")][#parenthetical[#text("(sarkastiskt)")] #text("General Bollbirth hörde till dem som tror att alla som frivilligt gav bort någonting är kommunister.")]
#dialogue[#text("BOLLBIRTH")][#text("Det var ju min ammunition!?!")]
#text("Slut på omstart:")
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(tittar tydligt på fejk-materialansvarig)")] #text("Förklara dig. Hur kan all ammunition vara borta? Är det inte ditt ansvar att hålla koll på förråden?")]
#text("Fejk-materialansvarig svarar inte (de är ju dans, de är inte mickade), så riktiga materialansvarig svarar åt dem.")
#dialogue[#text("MATERIALANSVARIG")][#text("Det måste vara något misstag, jag är säker på att-")]
#text("Bollbirth tar lugnt upp sin pistol och skjuter fejk-materialansvarig mitt i repliken.")
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(pekar på materialansvarig)")] #text("Grattis, du är nu befordrad. För din skull hoppas jag att du löser detta.")]
#text("Matrialansvarig tar upp sin skrivplatta från liket.")
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(till sig själv)")] #text("Eller vänta… vi har ju gasen. Då löser det ju sig ändå.") #parenthetical[#text("(ger väskan till \"nya materialansvarige\")")] #text("Nu är det du som ser till att gasen är redo att användas snarast. Ni andra, gör er redo för anfall när gasen är klar. Seså… Och ta och rensa bort den där också. [[inte så men typ så]]")]
#text("Bollbirth pekar på fejk-materialansvariges döda kropp medans hon lämnar scenen.")
#text("  ")
#text(weight: "bold","Musiknummer 5")
#text("[[Körens och dans(?) musiknummer. Sjunger om livet som soldater och om deras växande tvivel.]]")
#pagebreak()
#scene[#text("SCEN 13")]
#text(style: "italic","Karaktärer: Nightingale, Röde Baronen")
#text("[[Plats: Allierat läger]]")
#text("[[Röde Baronen känner sig tom och ledsen, belöningen för att leverera senapsgas-receptet har inte fått honom att känna sig som en hjälte. Spiller allt för Nightingale, som ger honom en utskällning som inspirerar. Röde Baronen får en change of heart. Får insikten att centralmaktssoldaterna är i fara och ger sig av för att varna dem. Nightingale går för att varna de andra allierade soldaterna att Bollbirth har senapsgas.]]")
#text("Röde Baronen är själv i det allierade lägret. Han vandrar ensam runt med brevet i handen. Han saktar ner och stannar upp, kollar på brevet och suckar tillfredsställt.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(till sig själv, blir progressivt mer osäker)")] #text("Jag lyckades... Vilket var en självklarhet, jag är ju den Röde Baronen. Centralmaktens största hjälte, och mycket snart de allierades också... Inte bara stal jag receptet på senapsgas rätt under tyskarnas näsor utan levererade det även till Bollbirth helt utan en aning från andra sidan. Vad hon nu kommer göra med den... Ja... Det är inte i mina händer. Varför skulle jag bry mig? ")#underline[#text("Det var värt det..")]#text(".") #parenthetical[#text("(pausar)")] #underline[#text("Det är värt det... Det kommer vara värt det... Det MÅSTE vara värt det...")]]
#text("Röde Baronen ser uppgiven ut.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(viskar nästan)")] #text("Det måste vara värt det...")]
#text("Nightingale kommer in på scenen.")
#dialogue[#text("NIGHTINGALE")][#text("Baronen, där är du! Vad har du gjort hela natten?")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(väcks ur sina djupa tankar)")] #text("Jo, jag behövde bara lite tid för mig själv… Behövde tänka på allt jag varit med om under kriget...")]
#dialogue[#text("NIGHTINGALE")][#text("Ja, du har ju varit med om en hel del.")]
#dialogue[#text("RÖDE BARONEN")][#text("Med så mycket lidande som det är i krig så är det svårt att inte se allt det hemska.")]
#text("Nightingale nickar instämmande.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(tittar för att se att Nightingale håller med)")] #text("Krig är verkligen något förfärligt.")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(sorgsen)")] #text("Ja. Tänk man bara kunde få ett snabbt slut på allting.")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(försöker övertala Nightingale)")] #text("Det är inte så svårt... Krävs bara att en sida vinner, till exempel.")]
#text("Nightingale blir frågande, börjar ana att Baronen försöker leda in till något.")
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(misstänksamt)")] #text("Mmhmmm.")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(mer självsäkert)")] #text("Säg, till exempel, om de allierade vann imorgon. Då hade allting varit över, i alla fall här.")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(skämtsamt, men ännu mer misstänksamt än innan)")] #text("Fast hade det inte varit bättre för dig om centralmakterna vann...?")]
#text("Röde baronen lyssnar inte riktigt.")
#dialogue[#text("RÖDE BARONEN")][#text("Tänk om jag sa att jag hade ett sätt att få slut på kriget. Vore inte det bra?")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(lite förvirrad, kopplar inte än, men MISSTÄNKSAM™)")] #text("Vad pratar du om? Menar du...")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(avbryter)")] #text("Hade inte det varit rätt? Att få slut på allting så snabbt som möjligt?")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(väldigt oroad)")] #text("Vad har du gjort?")]
#dialogue[#text("RÖDE BARONEN")][#text("Jag har bara gett de allierade ett övertag, så att säga. Ett sorts vapen...")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(på väg att bli arg, men fortfarande inte säker vad han talar om)")] #text("Vadå för vapen?")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(nonchalant)")] #text("Det är en gas. Senapsgas. Det är... Vet du vad, bara läs den här.")]
#text("Röde Baronen ger Bollbirths broschyr om senapsgas till Nightingale.")
#text("Nightingale börjar läsa, och blir mer och mer terrorfylld. Stel tystnad i passande längd. Baronen börjar prata igen när Nightingale nästan är klar, synligt arg.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(manande)")] #text("Visst är det här bra?")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(arg, men skriker inte än)")] #text("Hur kan du ens säga så? Förstår du inte vad du gjort?")]
#dialogue[#text("RÖDE BARONEN")][#text("Klart jag förstår vad jag har gjort. Jag har anslutit mig till de allierade! Till er sida!")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(rasande)")] #underline[#text("Vad spelar sidor för roll?!")]]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(retorisk fråga, talar som det är en självklarhet)")] #text("Vad spelar de för roll? Det är ett krig! Och det är ni som kommer vinna! [[OBS!!! \"Klart\" i början av både denna och den förra. Nån av dem behöver omfraseras! (min preferens är att omfrasera den ovan, och låta bli denhär)]]")]
#dialogue[#text("NIGHTINGALE")][#text("Vad gör det för skillnad att vi vinner, om vi vinner på det här sättet!")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(börjar tappa kontroll över situationen, defensivt)")] #text("Det kommer avsluta kriget mycket snabbare. Tänk på alla människor det kommer rädda, som kommer få åka hem! Det är ju bra, du sa det själv!")]
#dialogue[#text("NIGHTINGALE")][#text("Bollbirth kommer ju gasa dem! Hur lite bryr du dig om människor, egentligen? [[deras arme låter omänskligt]]")]
#dialogue[#text("RÖDE BARONEN")][#text("Jag bryr mig! Okej?! Men det är bara så krig fungerar! Det kräver uppoffringar!")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(rasande)")] #text("Uppoffringar som Klimt och Schumacher, menar du? Eller har du glömt dem kanske?")]
#text("Röde Baronen ryggar tillbaka och tystnar när Nightingale nämner Klimt och Schumacher. De stirrar tyst på varandra i några sekunder.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(desperat och förstört)")] #text("Men... Men... Jag är ju en hjälte...")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(aldrig varit argare i sitt liv, dissekerar RB on the SPOT)")] #text("Hjälte? Ska Schumacher och Klimt behöva dö för det? För att du ska få en fotnot i historieböckerna?!")]
#text("Kort paus där de stirrar på varandra. Nightingale lugnar ner sig lite. ")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(bönfallande)")] #text("Nightingale... Säg... bara säg att jag gjorde rätt...") #parenthetical[#text("(pausar)")] #text("Snälla... säg att det var rätt. Jag behöver höra att jag gjorde rätt - veta att det är...")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(lite mer lugn, men fortfarande förrått)")] #text("Vill du verkligen tvinga mig att ljuga?")]
#text("Tystnad råder (🎜midnatt är i husen🎜).")
#text("Röde Baronen kan inte svara. Han öppnar munnen och stänger den några gånger, för dramatisk effekt. Tittar bort från Nightingale, inser sitt misstag och skäms.")
#dialogue[#text("RÖDE BARONEN")][#text("Nej... nej, du har rätt. Jag har... gjort ett ")#text(style: "italic","stort jävla misstag.")]
#text("Röde Baronen tar sitt rekommendationsbrev och river det i två bitar.")
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(")#underline[#text("sarkastisk")]#text(", lugnar sig mer nu när RB erkänner sitt misstag)")] #text("Hm, säger du det?") #parenthetical[#text("(kort paus, suck, oroat)")] #text("Vad gör vi åt den här gasen nu?")]
#text("Röde Baronen slås av allvaret i situationen. Plockar upp sig från botten. Han tänker i alla fall försöka fixa sitt misstag.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(inser vad som måste göras)")] #text("Vi måste varna Schumacher, Flammenwerfer och Klimt! Jag tar mig till centralmaktsidan på direkten. Jag kan en genväg. Nightingale - försök stoppa senapsgasen härifrån.")]
#text("Röde Baronen springer av scenen.")
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(försiktigt optimistisk)")] #text("Jag ska försöka!")]
#text("Nightingale springer iväg åt andra hållet.")
#pagebreak()
#scene[#text("SCEN 14")]
#text("[[")#text(style: "italic","Karaktärer: Schumacher, Klimt, Müller, Flammenwerfer")#text("]]")
#text("[[Plats: Centralmaktsläger]]")
#text("[[Avslut för akt 2. Allt börjar falla på plats på det värsta möjliga sättet. Bollbirth har fått sin senapsgas och är glad över det. Müller är i extas över mer ammunition, säger att det är precis vad de behövde och att alla genast ska förbereda sig för att gå till anfall.]]")
#text("På centralmaktssidan. Klimt och Schumacher är tillbaka i centralmaktslägret. Klimt sitter som hon gjorde i scen 1 med en tidning i knät medan Schumacher studsar omkring.")
#dialogue[#text("SCHUMACHER")][#text("Kliiiiimt! Du MÅSTE bara få smaka mitt nya mästerverk! Jag upptäckte att sauerkrauten längst bak i förrådet blivit blå! Precis som blåmögelost! Klimt! Jag tror att jag uppfunnit en ny delikatess! [[Väldigt osäker på om det är rätt tid att ha den här interaktionen mellan klimt och schumacher, men kände att det behövdes någon innan muller kommer in, så tänkte att det kanske passa att klämma in lite av deras relation och personligheter här]] [[Oavsett vad så borde hon nämna något annat än svamp, vi sa svamp två gånger i akt 1]]")]
#dialogue[#text("KLIMT")][#text("Hur har du ens energi kvar? Vi har ju jobbat hela natten? Och när har du ens haft tid att laga mat…")]
#text("Schumacher kommer framrusande med en tallrik och sked.")
#dialogue[#text("SCHUMACHER")][#text("Sssch! Låt god mat tysta mun.")]
#text("Schumacher \"tvångsmatar\" Klimt som anstränger sig väldigt mycket för att svälja och nästan håller på att kräkas. Schumacher står extremt nyfiket bredvid och dömer hennes reaktion. [[Oklart om vi borde ha mat på scenen...]]")
#text("Klimt vrider och vänder sig. Hostar och torkar tungan etc. Dra ut på reaktionen så länge det är roligt.")
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(otåligt)")] #text("Sååå!?! Vad tycker du!?!")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(Tveksamt)")] #text("Jo, den var ju lite speciell…")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(Rådjursögon och krokodiltårar)")] #text("Speciell? Är inte det vad man säger när man inte tycker om maten?! Menar du att du inte gillar min mat?")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(Lite nervös)")] #text("Va, nej, hehe, jag menar speciell som i specialitet. Du vet, som i franska specialiteter!")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(Helt plötsligt lika glad som innan)")] #text("Åh, jag visste det! Det är verkligen ett mästerverk!")]
#text("De blir avbrutna av att Müller kommer in på scen.")
#dialogue[#text("MÜLLER")][#text("Klimt! Där är du ju, min gode vän! Man blir ju riktigt stolt att vara general över så lysande soldater som du!")]
#dialogue[#text("KLIMT")][#text("Ehh, tack så mycket, men exakt vad är det jag gjort nu?")]
#dialogue[#text("MÜLLER")][#text("Ha, du behöver inte spela så ödmjuk. Såhär var det, jag hade fått flertalet rapporter om att Flammenwerfer setts stryka omkring materialförrådet, så jag gick själv för att se till att det inte föregick några dumheter. Men istället för förtänd dynamit, ")#underline[#text("vad hittar jag om inte ett överflöd av ammunition!")]#text(" Det måste vara tillräckligt för minst två arméer! [[Fixa ordfel]]")]
#text("Klimt och Schumacher utbyter en nervös blick.")
#dialogue[#text("KLIMT")][#text("Haha, jo, det… Just det.")]
#dialogue[#text("MÜLLER")][#text("Det är en strålande bedrift, Klimt, du är onekligen västfrontens skarpaste materialansvarige! Med så här mycket ammunition kan vi äntligen göra ett riktigt anfall och inte bara sitta här som fegisar i skyttegravarna. Vi ska storma fram över ingenmansland! Sida vid sida under faderlandets fana! Vi ska äntligen få agera som riktiga soldater!")]
#dialogue[#text("SCHUMACHER")][#text("Ooh, vi ska till ingenmansland? Vad spännande!")]
#dialogue[#text("KLIMT")][#text("Det är inte spännande; ett fullt anfall över ingenmansland vore jättefarligt!")]
#dialogue[#text("MÜLLER")][#text("Vi är i ju krig! Klart det är farligt! Hur skulle vi kunna göra vårt land stolt om det inte fanns lite risk i det? Det är en ära att dö tappert i stridens eld! [[Är det ok att säga blaze of glory på engelska? Tycker det är rätt ord här och de svenska översättningarna jag hittar har inte riktigt samma kraft bakom dem]] [[Jag tror att det vore bra om vi hittade ett annat uttryck]]")]
#dialogue[#text("KLIMT")][#text("Så, när kommer vi genomföra anfallet? Gissar att du vill vänta en vecka för att hinna göra ordentliga förberedelser…")]
#dialogue[#text("MÜLLER")][#text("Ha, sitta och vänta när vi äntligen har chans att visa vad vi går för? Vilken nonsens! ")#underline[#text("Vi anfaller direkt!")]#text(" Stridsstund har guld i mund brukade min far alltid säga. Nu, mina vänner, nu ska vi ut och gräva guld!")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(fundersam och viskar till Klimt)")] #text("Var detta inte det här motsatsen av det vi ville med vår plan? [[Osäker på om det här passar in här, men jag låter det stå i fall att resten av gruppen anser att det kul nog att få vara med. Finns nog något skämt med att det är en väldigt liknande situation som innan]]")]
#text("Klimt förbereder ett svar men blir avbruten av Müller. ")
#dialogue[#text("MÜLLER")][#parenthetical[#text("(Ropar med kallande röst)")] #text("Flammenwerfer!")]
#text("Flammenwerfer kommer inflygande på scen och ställer sig i honnör. [[För squådis skull borde nog flammenwerfer komma med mycket tidigare, men som jag har skrivit scenen skulle han inte ha så mycket att göra före och ville inte att han bara ska stå där och inte säga något under för lång tid. Någon som har idéer på hur han kan bidra till scenen tidigare?]] [[Jag tycker typ att det är lugnt... De andra ska få space och Flammenwerfer är en så liten roll änåd]]")
#dialogue[#text("FLAMMENWERFER")][#text("Ja, general!")]
#dialogue[#text("MÜLLER")][#text("Werf flammen!")]
#text("Flammenwerfer lyser verkligen upp.")
#dialogue[#text("FLAMMENWERFER")][#parenthetical[#text("(Nästan tårögd)")] #text("Menar du verkligen det?")]
#text("Müller lägger sina händer på Flammenwerfers axlar och stirrar honom allvarligt i ögonen.")
#dialogue[#text("MÜLLER")][#text("Jag har aldrig menat något mer i mitt liv. Gör mig stolt, soldat! Visa dem faderlandets styrka!")]
#text("Flammenwerfer springer lyckligt av scenen.")
#dialogue[#text("MÜLLER")][#text("Klimt! Förbered vapen och ammunition för alla soldater i lägret! Schumacher! Fixa någon mat man kan ta i handen. Typ i en säck.")]
#dialogue[#text("SCHUMACHER")][#text("En matsäck?")]
#dialogue[#text("MÜLLER")][#text("Det var ett bra namn! Så smart jag var som kom på det konceptet! Jag är verkligen en genialisk general!")]
#text("Müller marscherar stolt av scenen. Klimt och Schumacher tittar nervöst på varandra.")
#dialogue[#text("KLIMT")][#text("Men det var ju inte såhär det var tänkt. Åh, varför behövde han kolla i materialförrådet just idag?")]
#dialogue[#text("SCHUMACHER")][#text("Äsch, det är ju ingen fara! Vi gör bara som vi gjorde som sist! Bär över ammunitionen till andra sidan igen.")]
#dialogue[#text("KLIMT")][#text("Då hade vi i alla fall en hel natt; nu vill Müller anfalla direkt! Det finns inget sätt att stoppa anfallet.")]
#dialogue[#text("SCHUMACHER")][#text("Men de är ju våra vänner! Klart de inte kommer skjuta på oss.")]
#dialogue[#text("KLIMT")][#text("Du förstår inte, Schumacher! Det här är ingen lek längre! Jag har sett vad människor är kapabla till. När du väl står där med kulor vinande om öronen tänker du inte på vilka på fiendesidan som du ska och inte ska skjuta på. [[Borde inte Klimt påpeka alla andra människor som finns och som KOMMER att skjuta på dem? Det vore lite kul med ett meta-skämt här om att vi ser så få soldater trots att det borde vara tusentals här]] [[Kanske ett Covid-19-skämt om sällskap på max 10 personer?]] [[Fila på detta]]")]
#dialogue[#text("SCHUMACHER")][#text("Men VI kommer ju inte faktiskt skjuta dem. De är ju våra vänner!")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(argt)")] #text("Jo, för om vi inte gör det kommer Müller skjuta OSS för vapenvägran istället! Vi har inget val! De kommer dö, och vi kommer döda dem!")]
#text("Schumacher tystnar. Börjar inse allvaret. Hon ser osäker och förvirrad ut. Klimt deppar ihop.")
#dialogue[#text("SCHUMACHER")][#text("Men- tänk om vi skulle- [[Ev att hon kommer på någon konstig idé här (jag är bara dålig på att komma på vad det skulle vara)]]")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(dystert)")] #text("Nej, Schumacher. Det finns inget vi kan göra nu.")]
#text("Schumacher ser desperat ut.")
#dialogue[#text("SCHUMACHER")][#text("Men-")]
#dialogue[#text("KLIMT")][#text("Nej. Bara… nej.")]
#pagebreak()
#scene[#text("SCEN 14.5")]
#text("[[")#text(style: "italic","Karaktärer: Nightingale, Tolkien, Senap, Bollbirth, Materialansvarig")#text("]]")
#text("[[Plats: Allierat läger]]")
#text("Scenen byter till de allierades sida. Nightingale står bland Tolkien och Senap som lyssnar på vad han säger.")
#dialogue[#text("NIGHTINGALE")][#text("…så det är så det ligger till. Baserat på den här broschyren så uppskattar jag sannolikheten att de andra överlever till ungefär exakt noll. [[Detta är inte konsekvent med Nightingales reaktion en scen tidigare]] [[Formulera om - detta är lite on the nose]]")]
#dialogue[#text("TOLKIEN")][#text("Men det är ju katastrof, ju! Alla våra nya vänner kommer ju att dö! Vi måste hitta något sätt att stoppa generalen.")]
#dialogue[#text("SENAP")][#text("Jag håller med att detta är inte helt moralisk rätt eller särskilt ärofyllt, men vi bör respektera vår generals beslut ändå.  [[Senap växlar ganska mycket mellan att gå bakom ryggen på Bollbirth och mena att hon vill det bästa]]")]
#dialogue[#text("TOLKIEN")][#text("Hur är du fortfarande på hennes sida? Du har ju redan hjälpt oss gå emot hennes planer.")]
#dialogue[#text("SENAP")][#text("Det gjorde jag då INTE! Jag lät er göra som ni ville för att jag tyckte att anfallet kunde fördröjas. Nu har generalen gett en direkt order som vi ska följa, oavsett om vi gillar den eller ej.")]
#text("Mitt under deras samtal börjar plötsligt massa skott och explosioner att höras. Nightingale och Tolkien kastar sig bakom skydd medan Senap står kvar.")
#dialogue[#text("SENAP")][#parenthetical[#text("(Bestämt)")] #text("Officerare duckar inte!")]
#dialogue[#text("NIGHTINGALE")][#text("Ja, ja, din pappa skulle va stolt. Ge dig nu!")]
#text("Nightingale drar ner Senap bakom skyddet.")
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(Rädd)")] #text("Vad händer? Sånt här stort anfall har ju inte skett på flera månader! Det måste vara hela deras armé som kommer…")]
#text("Bollbirth kommer inmarscherande med andra soldater bakom sig, bland annat \"nya\" materialansvarig.")
#dialogue[#text("BOLLBIRTH")][#text("Till era positioner! Besvara elden, soldater!")]
#dialogue[#text("MATERIALANSVARIG")][#text("Men general Bollbirth, vi har ju ingen ammunition.")]
#dialogue[#text("BOLLBIRTH")][#text("Du fick en order! Ta det som finns kvar. En kula per tredje soldat lär räcka långt.")]
#dialogue[#text("MATERIALANSVARIG")][#text("Ehh, ja… självklart, general!")]
#dialogue[#text("BOLLBIRTH")][#text("Upp och hoppa och visa vad ni går för nu, era fegisar! Se till att dö så långsamt som möjligt så att vi kan få så mycket tid som vi behöver!")]
#text("Bollbirth marscherar av scenen igen för att gå och beordra några andra. Soldaterna ser rädda och tveksamma ut men börjar långsamt ta sig bort mot fronten.")
#text("  ")
#text(weight: "bold","Musiknummer 6")
#text("[[Soldaternas “vad gör vi nu”-nummer. Dags för strid, allt är på väg att gå åt helvete.]]")
#text("  ")
#pagebreak()
#scene[#text("AKT 3")]
#pagebreak()
#scene[#text("SCEN 15")]
#text("[[Karaktärer: Schumacher, Klimt, Flammenwerfer, Röde Baronen]]")
#text("[[Plats: Ingenmansland i fullt krig]]")
#text("[[Det är krig och dålig stämning på centralmaktssidan. Klimt och Schumacher vill inte slåss. Flammenwerfer klagar på att de allierade inte ens verkar försöka. Röde Baronen kommer fram till centralmaktslägret och berättar om senapsgasen. Inser att han måste få Müller att dra tillbaka anfallet (både för att rädda sina nya vänner och för att Bollbirth annars kommer att använda senapsgasen). Flammenwerfer hör konversationen och inser att Röda Baronen är en förrädare. Jagar efter honom av scen. Det upp till Klimt och Schumacher att försöka stoppa Müller.]]")
#text("EXT. INGENMANSLAND")
#text("Lamporna tänds och Flammenwerfer står på något som höjer upp honom lite från marken (en kulle, en pall, en kuliss av något slag). Han är barbröstad, iförd lederhosen, pannband, och har svarta streck på kinderna. I sina händer har han en eldkastare som han avfyrar vilt ut i luften. Medans han skjuter så skrattar han euforiskt.")
#dialogue[#text("FLAMMENWERFER")][#text("Ach, wunderbar! Finns det någon ljuvare doft än svavel på morgonkvisten?")]
#text("Han tar ett djupt andetag och bara njuter.")
#dialogue[#text("FLAMMENWERFER")][#text("Ah, detta är ju helt fantastiskt! I den här takten kommer jag snart få ALLA medaljer för bästa soldat av General Müller!")]
#text("Han får syn på något av (utanför) scenen.")
#dialogue[#text("FLAMMENWERFER")][#text("Åh, titta! Där har vi ännu fler! Vänta, fienden! Flammenwerfer ska få medaljer!")]
#text("Flammenwerfer springer ut mot den allierade sidans front. Schumacher och Klimt släpar in sig själva på scenen, trötta och slitna. Klimt gömmer sig i skyttegraven medan Schumacher står kvar på stället.")
#dialogue[#text("KLIMT")][#text("Schumacher! Ner med dig!")]
#dialogue[#text("SCHUMACHER")][#text("Äh, vad spelar det för roll? De har ju ingen ammunition, inte som att de kan skjuta mig ändå!")]
#dialogue[#text("KLIMT")][#text("Nej, men om Müller ser att du inte skjuter så kommer han bestraffa dig istället!")]
#text("Schumacher kollapsar vresigt mot skyttegravsväggen bredvid Klimt.")
#dialogue[#text("SCHUMACHER")][#text("Det här suger. Jag trodde jag skulle få komma till fronten och laga lite mat till soldaterna, inte behöva skjuta på mina vänner!")]
#dialogue[#text("KLIMT")][#text("Klaga inte när det var du som satte oss i den här röran från första början.")]
#dialogue[#text("SCHUMACHER")][#text("Jag? Vad har jag gjort?")]
#dialogue[#text("KLIMT")][#text("Du behövde ju inte envisas med att vara så vänlig mot dem! Vi har krigat i flera månader och jag har aldrig behövt tänka på vilka det är vi slåss mot. De kommer och så skjuter man bara. [[utveckla lite]] [[Vilken dag är det? Eller alltså: det är väl månader de har krigat snarare än år?]]")]
#dialogue[#text("SCHUMACHER")][#text("Men, va!? Är du sjuk i huvudet!? De är ju människor oavsett om vi känner dem eller inte! Jag ser inte hur det kan vara fel att vara vänlig mot andra.")]
#text("Klimt ångrar sina ord.")
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(blir lite sur)")] #text("Och ta inte ut ditt dåliga humör på mig! Jag följde ju bara ditt exempel!")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(ångerfull)")] #text("Nej… Förlåt, Schumacher. Det är inte ditt fel. Det är det förbaskade krigets fel, man blir ju helt hjärntvättad här ute! Vi gjorde rätt som behandlade dem med vänlighet. Alla människor förtjänar minst det.")]
#dialogue[#text("SCHUMACHER")][#text("Ja, min mamma brukade också säga det. Att det är viktigt att vara vänlig mot folk och att alltid borsta tänderna. [[Inte säker på om det funkar...]]")]
#dialogue[#text("KLIMT")][#text("Din mamma lät som en smart kvinna.")]
#dialogue[#text("SCHUMACHER")][#text("Nja, det mesta läste hon bara från sin mattebok. Du vet, den med gyllene snittet?")]
#dialogue[#text("KLIMT")][#text("Från hennes mattebok? Är du säker på att du inte talar om bibeln?")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(dödssäker)")] #text("Nej då, jag är helt säker. Den hade ett stort plustecken på framsidan.")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(börjar snällt förklara)")] #text("Schumacher, det är inte en…")]
#text("En stor explosion hörs en bit bort.")
#dialogue[#text("KLIMT")][#text("Förhoppningsvis var det där inte Flammenwerfer.")]
#text("Flammenwerfer skuttar exalterat in från den Allierade sidan mot Schumacher och Klimt.")
#dialogue[#text("KLIMT")][#text("Fan.")]
#dialogue[#text("FLAMMENWERFER")][#parenthetical[#text("(glad som ett barn på julafton igen)")] #text("Haha! Hörde ni det där? Vilken jävla smäll, asså. Mina egna handgranater: toppklass och dubbelt så effektiva! Här, ni kan behöva dessa.")]
#text("Flammenwerfer tar fram och ger en konservburks-granat var till Schumacher och Klimt.")
#dialogue[#text("KLIMT")][#parenthetical[#text("(obekvämt)")] #text("Ja… väldigt imponerande. Tack.")]
#dialogue[#text("SCHUMACHER")][#text("Är det menat att jag ska mata folk med denna?")]
#dialogue[#text("FLAMMENWERFER")][#text("Javisst, den bästa maträtten är alltid den sista måltiden!")]
#text("Klimt försöker ändra samtalsämnet.")
#dialogue[#text("KLIMT")][#parenthetical[#text("(avbryter)")] #text("Men hörrni, ska vi inte bege oss tillbaka? Jag tror att de allierade har lidit tillräckligt med förluster för idag.")]
#dialogue[#text("FLAMMENWERFER")][#text("Nonsens. Jag vet inte varför, men de gör knappt något motstånd. Det är nästan som om de vill dö; i famnen hos sin förintare: mig. [[kanske ändra, idk]]")]
#text("Deras humör försämras ytterligare av att lyssna på Flammenwerfer, gränsar på förskräckelse och obehag. Schumacher vänder sig mot Klimt.")
#dialogue[#text("SCHUMACHER")][#text("Jag tror jag tappat aptiten…")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(till Flammenwerfer)")] #text("Flammenwerfer, är det här bara ett spel för dig!? [[t.ex. här kanske bara ta bort \"Alla liv du tagit\", eller ändra till typ \"alla du skjutit\"]]")]
#dialogue[#text("FLAMMENWERFER")][#parenthetical[#text("(glatt)")] #text("Självklart! Och jag har high-score! Men nu måste jag fylla på min ammunition. Jag börjar få slut på granater. Auf wiedersehen!")]
#text("Flammenwerfer går ut ur scenen mot centralmaktslägret. Röde Baronen springer samtidigt in på scenen, andfådd.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(mellan andetag)")] #text("Klimt! Schumacher! Tack och lov, ni är oskadda.")]
#text("Röde Baronen kramar om dem. De kramar inte tillbaka.")
#dialogue[#text("SCHUMACHER")][#text("Vad har det tagit åt dig? Varför så uppjagad?")]
#dialogue[#text("RÖDE BARONEN")][#text("Fort! Vi måste alla bort från fronten NU!")]
#dialogue[#text("KLIMT")][#text("Jo, det skulle ju vara trevligt. Vi har velat sluta kriga ett bra tag nu.")]
#dialogue[#text("RÖDE BARONEN")][#text("Nej, nej, nej. Ni förstår inte vad jag menar. Vi måste bort för att general Bollbirth kommer snart att senapsgasa hela fronten.")]
#dialogue[#text("KLIMT")][#text("Va?!")]
#dialogue[#text("SCHUMACHER")][#text("Gasa? Hur gasar man med en sås!?")]
#text("Röde Baronen gör sig redo för att förklara.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(pedagogiskt)")] #text("Jo, senapsgas är-")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(avbryter)")] #text("Vad det än är så är låter det inte som ett gott tecken.")]
#dialogue[#text("RÖDE BARONEN")][#text("Eh, ja, precis. Vi måste varna general Müller omedelbart!")]
#text("De börjar röra på sig, men Klimt stannar till.")
#dialogue[#text("KLIMT")][#text("Vänta, vänta, vänta… Hur vet du vad vad den andra sidan tänker göra?")]
#dialogue[#text("SCHUMACHER")][#text("Och var har du varit egentligen?")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(frustrerat)")] #text("Jag… Skit i det, vi har inte tid!")]
#text("Flammenwerfer återvänder in på sidan av scenen med en famn konservburk-granater. Han ser ut att börja säga något men blir avbruten av Klimt och tystnar nyfiket.")
#dialogue[#text("KLIMT")][#parenthetical[#text("(till Röde Baronen)")] #text("Nej. Berätta vad du håller på med!")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(otålmodig och snabbt)")] #text("Ok, fine. Jag är en dubbelagent och jag skulle smuggla ett recept på ett hemligt gasvapen till de allierade för ära och berömmelse och det var i min väska som jag gav till Bollbirth men nu har jag ångrat det för jag insåg att hon är en galning som är villig att offra upp sina egna soldater. Där har ni det. Jag är en förrädare. Kan vi gå nu? [[kanske \"ett recept på ett hemligt supervapen\" -> \"receptet på senapsgas\" men det blir lite tråkigare]]")]
#text("Flammenwerfer hör Röde Baronen erkänna sig vara en förrädare. Han stirrar mållöst och tappar \"konservburkarna\" på marken, vilket drar allas uppmärksamhet till honom. Queue såpopera.")
#dialogue[#text("FLAMMENWERFER")][#parenthetical[#text("(förtvivlat)")] #text("D-det… Det- det kan inte vara sant!!")]
#text("Han hänger av sig sitt vapen och slänger det förstrött på marken, tillsammans med all sin utrustning. Ju mer ammo-bälten, granater etc. han har (och nu börjar slänga av sig) desto roligare blir det.")
#dialogue[#text("RÖDE BARONEN")][#text("Flammenwerfer?") #parenthetical[#text("(paus)")] #text("Jag… Jag kan förklara…")]
#dialogue[#text("FLAMMENWERFER")][#text("Du är ju självaste Röde Baronen!  Du flög en Siemens-Schuckert!! Du var vår hjälte! Du var MIN hjälte!!") #text("Du kan inte ha svikit oss! Inte du!")]
#dialogue[#text("RÖDE BARONEN")][#text("Flammenwerfer… Jag…")]
#text("Flammenwerfer tar Baronen i axlarna och skakar på honom lite.")
#dialogue[#text("FLAMMENWERFER")][#text("Säg att det inte är sant! SÄG DET!!")]
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(sorgsen)")] #text("Det är sant… Jag är en förrädare. Men jag ångrar allt! Du måste förlåta mig!")]
#dialogue[#text("FLAMMENWERFER")][#parenthetical[#text("(börjar nästan gråta)")] #text("Jag hör dina ord men jag klarar inte av att lyssna på dem!")]
#text("Baronen stirrar sorgset på Flammenwerfer, sedan vänder han bort blicken, ser nästan ut att ha ont. Flammenwerfer släpper honom och tar raskt några steg tillbaka. Ser först ut som att han kommer bli rasande, men sen tittar han upp mot himlen och ler morbitt. ")
#dialogue[#text("RÖDE BARONEN")][#text("Okej, jag vet att det jag har gjort är oförlåtligt, men vi måste ta oss till Müller innan-")]
#text("Baronen avbryts av att Flammenwerfer börjar skratta (morbidt och nästan lite halvt galet)")
#dialogue[#text("RÖDE BARONEN")][#text("Flammenwerfer?")]
#dialogue[#text("FLAMMENWERFER")][#text("En förrädare… Hahah… Verkligen. Ja, det finns ju bara ett sätt att ta hand om den sorten, eller hur? [[Plats för längre överdramatisk monolog]]") #parenthetical[#text("(tittar ner på Baronen)")] #text("Röde Baronen… Haha… Vad sägs om Döde Baronen? [[")#text(style: "italic","Drops mic")#text("]]")]
#text("Flammenwerfer drar fram sin militärkniv, till vilket Röde Baronen panikartat börjar springa ut ur scen. Flammenwerfer börjar jaga efter honom.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(springandes)")] #text("Flammenwerfer nej! Snälla lyssna!")]
#text("De är borta. Schumacher springer efter för att försöka rädda baronen. [[Är bra om vi får chans att reflektera lite över s&k känslor i kommande dialog
Är bra om det blir tydligt att de är besvikna]]")
#dialogue[#text("SCHUMACHER")][#text("Baronen!")]
#text("Klimt hugger tag i henne.")
#dialogue[#text("KLIMT")][#parenthetical[#text("(allvarligt)")] #text("Vi måste förklara situationen för general Müller genast.")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(tveksamt)")] #text("Men Röde Baronen då?! Vi måste rädda honom.")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(tror inte själv på det)")] #text("Ha tillit i honom… Han kommer säkert klara sig.")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(vill tro på Klimt)")] #text("Okej, Klimt. Om… om du säger det så.")]
#text("Klimt och Schumacher beger sig osäkert mot centralmaktslägret.")
#pagebreak()
#scene[#text("SCEN 16")]
#text("[[Karaktärer: Tolkien, Nightingale, Senap, Cartier, Skadad soldat]]")
#text("[[Plats: Allierat läger]]")
#text("[[De allierade har en dålig dag i ingenmansland, där ingen man är. Alla inser att de måste göra något. Senap tror fortfarande det går att resonera med Bollbirth men Tolkien och Nightingale är skeptiska. Senap får ändå sin vilja igenom och de bestämmer sig för att gå till Bollbirth och försöka prata henne ur sin senapsgasplan.]]")
#text("Tolkien, Senap, Cartier och Nightingale är på fronten. Nightingale lappar ihop en stackars soldat som blivit skjuten i benet. Cartier sitter bakom ett skydd och läser brev som distraktion från bomber och granater.")
#dialogue[#text("SENAP")][#parenthetical[#text("(skjuter mot andra sidan)")] #text("Tolkien! Kom hit med mer ammunition, jag har snart slut!")]
#dialogue[#text("TOLKIEN")][#text("Vilken ammunition? Jag har haft slut i evigheter?!? Nightingale, har du några kulor kvar?")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(tar hand om soldatens ben)")] #text("Jag håller på att gräva ut en just nu! Vänta någon sekund bara!")]
#dialogue[#text("SENAP")][#text("En blodig kula hit eller dit kommer inte hålla fronten! Vi behöver betydligt mer! Cartier, kommer du med mer ammunition?")]
#dialogue[#text("CARTIER")][#parenthetical[#text("(surt)")] #text("Jag levererar post.")]
#dialogue[#text("SENAP")][#text("Ser du inte att det finns mer pressande saker att göra!?")]
#dialogue[#text("CARTIER")][#text("Det är inte mitt jobb att vara springpojke åt dig, \"Li-öft-ten-ant\". Dessutom har den sista ammunitionen redan skickats ut i fältet.")]
#text("Cartier öppnar upp ett brev.")
#dialogue[#text("CARTIER")][#text("Finns det någon Mitchel Smith här? Hans faster meddelar att \"körsbärstomaterna blev ofantligt goda i år\". Pfff, britter med gröna fingrar, det tror jag när jag ser det.")]
#text("Cartier slänger iväg brevet och öppnar ett nytt.")
#dialogue[#text("CARTIER")][#text("Nämen ser man på, \"Lilla Anna-Bell har tappat sin första tand\". Som förväntat av den brittiska tandvården.")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(stannar upp i arbetet och funderar)")] #text("Hmmm, jag tog hand om en soldat innan som tappat nästan alla sina tänder efter en nära explosion. Kanske skulle de skulle gå att avfyra i ett gevär?")]
#text("Tolkien spyr nästan.")
#dialogue[#text("SENAP")][#parenthetical[#text("(suckar)")] #text("Vi sitter rejält i skiten. Fienden närmar sig, och vi har knappt något att försvara oss med.")]
#text("Kort paus.")
#dialogue[#text("SENAP")][#parenthetical[#text("(förtvivlad)")] #text("Och värst av allt är att det är helt mitt fel! Varför lät jag er genomföra den där korkade planen?")]
#dialogue[#text("CARTIER")][#parenthetical[#text("(snark)")] #text("Korkad? Århundradets underdrift! Ge all ammunition till fienden. Hur skulle det någonsin kunna gå fel? Imbéciles!")]
#dialogue[#text("SENAP")][#text("Det enda vi kan göra nu är att hålla ut lite längre så att generalen kan färdigställa senapsgasen. Vi får leva med vårt misstag.")]
#dialogue[#text("TOLKIEN")][#text("Eller kanske snarare dö med det! Utan ammunition kan vi ju inte göra annat än att bygga en mur av lik! Och för vad? Så att Generalen kan eskalera kriget ytterligare? Så att hon kan använda ett kemiskt vapen mot våra vänner på andra sidan?")]
#text("Senap har inget svar. Istället är det stelt och tyst en stund.")
#dialogue[#text("SENAP")][#parenthetical[#text("(kollar ut över skyddet)")] #text("Nightingale, kommer han kunna slåss igen? Våra styrkor där ute börjar se väldigt tunna ut.")]
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(Reser sig upp från den skadade soldaten)")] #text("Se dig omkring: dina soldater faller ju som flugor. Bollbirth är helt förblindad av sin plan. Det KAN inte vara strategiskt att skicka ut dem igen i det här skicket.")]
#text("Nightingale pekar menande mot soldaten på marken, som tittar upp helt skräckinjagat.")
#dialogue[#text("SKADAD SOLDAT")][#parenthetical[#text("(jätterädd)")] #text("Va, ski-skicka ut mig i-igen? Snälla, ni kan inte göra det! Han är där ute! Djävulen själv! Jag har sett honom i vitögat… Och han har på sig lederhosen! Åh, gud, snälla, inte ut dit igen!")]
#dialogue[#text("CARTIER")][#parenthetical[#text("(spydigt)")] #text("Pffft. Skaffa bättre plot-armour.")]
#dialogue[#text("NIGHTINGALE")][#text("Kolla på honom, löjtnant! Är det rätt att offra våra egna så här, bara för att skydda generalens galna plan? Om vi skickar tillbaka honom ut kan jag lova att vi måste amputera mer än bara det där benet! [[Det blir en lite konstig mening att säga]] [[Korta ner]]")]
#text("Soldaten ger ifrån sig ett gällt skrik och börjar desperat kravla bort längs marken, bort av scen. Nightingale stirrar ilsket på Senap som ser väldigt obekväm ut.")
#dialogue[#text("SENAP")][#text("Okej, ja. Bättre vore att överge senapsgasproduktionen, göra en taktisk reträtt och omgruppera oss när vi har hunnit få påfyllda vapenförråd.")]
#dialogue[#text("NIGHTINGALE")][#text("Rimligt beslut.")]
#dialogue[#text("SENAP")][#text("Cartier! Gå och kolla hur senapsgasproduktionen står till.")]
#dialogue[#text("CARTIER")][#text("Jag lyssnar inte på order från dig!")]
#dialogue[#text("SENAP")][#text("Antingen det eller så går du ut i fältet!  [[Foreshadowing att han inte gillar fronten längre?? Osäker om det fungerar thou]]")]
#text("Cartier tittar på fältet en sekund, och bestämmer sig att han vill behålla sitt liv. Han vänder sig stolt om, och rusar bort.")
#dialogue[#text("SENAP")][#text("Nu kan vi börja organisera en reträtt.")]
#dialogue[#text("TOLKIEN")][#text("Tack och lov! Kom igen då, vi drar! Pyser härifrån! Lägger benen på ryggen! Sprintar baklänges! V-vad väntar vi på?!")]
#dialogue[#text("SENAP")][#text("Inte så snabbt, menig! Jag kan inte utlysa en full reträtt utan generalens samtycke.")]
#dialogue[#text("NIGHTINGALE")][#text("Men det finns ingen chans att hon skulle gå med på det! Bollbirth har ju helt tappat vettet!")]
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(tyst)")] #text("Förutsatt att hon hade vett till att börja med.")]
#dialogue[#text("SENAP")][#text("Hon är ju general! Tror ni att hon kunde ha tagit sig till toppen om hon inte var kapabel till att fatta rimliga beslut och ändra sin uppfattning när ny information presenteras för henne?")]
#dialogue[#text("NIGHTINGALE")][#text("Det är precis vad vi tror!")]
#dialogue[#text("SENAP")][#parenthetical[#text("(förnärmat)")] #text("Vad tror ni skulle hända om alla sprang runt och gjorde vad de ville, som ni två? Ren anarki!")]
#dialogue[#text("NIGHTINGALE")][#text("Du hjälpte ju oss förra gången genom att inte avslöja oss. Då måste du ju se att alla beslut inte måste gå genom generalen.")]
#dialogue[#text("SENAP")][#text("Och se vart det ledde oss! Jag kommer inte gå bakom ryggen på generalen igen. Nej, vi ska prata med henne, och därmed basta! [[ordsallad]]")]
#text("Senap vänder ryggen mot de andra och sätter armarna i kors. Nightingale går runt henne för att fortsätta argumentera, men när han kommer runt vänder Senap bara på sig igen. Upprepa ett par gånger för komisk effekt.")
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(uppgivet)")] #text("Det är ingen idé, Nightingale. Hon kommer inte ge sig. Vi kanske bara får ge det ett försök? Inte som att vi har så mycket till val… [[Vet inte om det här passar för Nightingales personlighet]] [[Jag undrar om det inte faktiskt skulle fungera att bara switch:a repliken till Tolkien och att Nightingale går runt Senap istället.]]")]
#dialogue[#text("SENAP")][#text("Såja, mot generalens tält!")]
#text("Senap går av scenen med stora steg. Tolkien är uppgiven, men Nightingale klappar honom på axeln.")
#dialogue[#text("NIGHTINGALE")][#text("Seså. Senap känner generalen bättre än vad vi gör. Kanske finns det en chans ändå?")]
#dialogue[#text("TOLKIEN")][#text("Det här kommer inte sluta bra… [[Jag vet inte om det är rättfärdigat men jag ogillar hur både de sista replikerna typ slutar i retoriska-esque frågor. Hade velat avsluta scenen med en punkt, men det är förmodligen bara jag]]")]
#text("De följer efter Senap av scenen.")
#pagebreak()
#scene[#text("SCEN 17")]
#text("[[Karaktärer: Tolkien, Senap, Nightingale, Bollbirth, Cartier]]")
#text("[[Plats: Bollbirths tält]]")
#text("[[Senap försöker övertyga Bollbirth men misslyckas och blir skjuten i benet för sitt tvivel. Tolkien och Nightingale stjäl en radio/detonator för att aktivera senapsgasen och flyr ut i ingenmansland. Bollbirth jagar efter dem medans Senap ligger och har en dålig dag.]]")
#text("Bollbirth står och tittar på en karta. Senap stormar in i tältet.")
#dialogue[#text("SENAP")][#text("General Bollbirth! Vi måste falla tillbaka! Det här går inte, det är galenskap!")]
#dialogue[#text("BOLLBIRTH")][#text("Men kära lilla Löjtnant Senap, vad har flugit i dig? Jag har inte tid att ta i tu med dina tvivel.")]
#dialogue[#text("SENAP")][#text("Men… General Bollbirth. Det är inte lönt! Vi förlorar soldater i drivor! Vi måste retirera!")]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(upprört)")] #text("Löjtnant Senap, vad är det du säger?! Min senapsgas är snart redo, och du vill att vi retirerar?")]
#dialogue[#text("SENAP")][#parenthetical[#text("(trofast)")] #text("Det är inte rätt! Det är inte strategiskt korrekt att bara offra våra egna soldater på det här sättet! På min fars tid hade den här planen aldrig tolererats! Bollbirth, det här är inte rätt. [[Bör bytas ut mot ett annat argument från scen 16]]")]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(argt)")] #text("Du ska kalla mig GENERAL Bollbirth, glöm inte vem som bestämmer här! Jag bryr mig inte en gnutta vad ditt gamla fossil till pappa har att säga!")]
#text("Bollbirth tar ett djupt andetag.")
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(genom tänderna)")] #text("Du är upprörd, men vi kan tala om detta senare. Du måste lita på att jag som general VET vad jag gör.")]
#text("Tolkien och Nightingale kommer inflåsande.")
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(andfådd)")] #text("Senap, vänta!")]
#text("Senap står tyst och biter sig i läppen.")
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(upprört)")] #text("Vad är det här? Tror ni mitt tält är någon slags klubb? Ut och försvara min front!")]
#text("Nightingale och Tolkien samlar sig. Senap stirrar blankt på Bollbirth.")
#dialogue[#text("TOLKIEN")][#text("General, moralen är dålig, vi har ingen ammunition. Ingen vill slåss längre. [[gillar inte riktigt den, känns typ som HP eller nåt, men vet inte exakt vad man ska ändra till]]")]
#dialogue[#text("BOLLBIRTH")][#text("Oroa dig inte. Gasen är redo när som helst, då kommer fienden få lida tiofaldigt vad vår sida gjort. [[Är den inte redo, eller väntar hon på rätt tillfälle? (lite implikationer för senare lines)]]")]
#dialogue[#text("NIGHTINGALE")][#text("Vi kan inte använda den! Det vore en humanitär tragedi! Vet du ens själv vilka skador den kommer orsaka?!")]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(börjar bli riktigt irriterad)")] #text("Givetvis vet jag det. Ut med er nu.")]
#dialogue[#text("TOLKIEN")][#text("Nej, inte tills du insett reson.")]
#dialogue[#text("BOLLBIRTH")][#text("Jag är din överordnade. Mitt ord är lag. Ut. Ur. Mitt. Tält.")]
#dialogue[#text("SENAP")][#text("General… Du måste se att det här är vansinne. Vi kan inte hålla dem tillbaka. Våra styrkor tar redan förfärliga förluster, och nu är fienden så nära att våra soldater inte ens kommer kunna undvika gasen om vi väl använder den! [[Insert om du använder gasen kommer våra soldater hamna i korselden]]")]
#dialogue[#text("BOLLBIRTH")][#text("Om våra soldater inte kan ta sig undan i tid är det deras egna fel. Det är sant att vi antagligen kommer förlora några av våra egna styrkor i gasen, men det finns massvis med nya rekryter där hemma. Andra sidan kommer skadas minst lika mycket, det är en uppoffring jag är villig att göra. [[Mer survival of the fittest - om de inte hinner undan är det deras egna fel]]")]
#dialogue[#text("SENAP")][#parenthetical[#text("(chock, utvecklar sig till ilska)")] #text("Men vad är det du säger!? Ska vi gasa våra egna soldater? Detta är inte moraliskt rätt- våra soldater gick inte med i armén för att bli dödade av sin egen general! [[Istället för \"offra våra egna\", frammana mer att Bollbirth hänvisar till att använda ett vapen mot sina egna soldater]]")]
#dialogue[#text("BOLLBIRTH")][#text("Shhh!!! Ett ord till från någon av er så kommer jag placera er alla framför en militärtribunal. Lämna mig ifred! NU!")]
#dialogue[#text("NIGHTINGALE")][#text("Det är inte värt det, Senap. Det finns inget vi kan göra här.")]
#text("Tolkien drar Senap lätt i ärmen bort från Bollbirth.")
#dialogue[#text("SENAP")][#text("Nej, Tolkien, Generalen måste förstå. Hon måste lyssna.")]
#text("Senap vänder sig om och tar ett par raska steg mot Bollbirth.")
#dialogue[#text("SENAP")][#text("General… Jag ber dig att…")]
#text("Bollbirth tar sin pistol (eventuellt från bordet om det finns) och skjuter Senap i benet.")
#dialogue[#text("BOLLBIRTH")][#text("Jag sa UT UR MITT TÄLT!")]
#text("Senap ramlar omkull, vrider sig på marken i smärta. Cartier kommer inspringande med detonatorn för senapsgasen i handen.")
#dialogue[#text("CARTIER")][#parenthetical[#text("(ser inte Senap ännu, är andfådd)")] #text("General… Gasen… Den är redo att avfyra-")]
#dialogue[#text("SENAP")][#parenthetical[#text("(märker inte Cartier, fokuserar bara på sitt ben)")] #text("Aj, helvete!")]
#text("Cartier får syn på Senap och stannar till och stirrar på henne. Nightingale knuffar till Tolkien.")
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(relativt lugn, med tanke på vad som just hänt)")] #text("Utmärkt. ")#underline[#text("Ge mig detonatorn")]#text(", soldat.")]
#text("Cartier stirrar på situationen, mållös.")
#dialogue[#text("BOLLBIRTH")][#text("Ge mig den. Nu.")]
#text("Tolkien springer fram och tar detonatorn ur Cartiers hand och springer av scenen. Nightingale följer efter. Bollbirth, som inser vad som skett, reser sig snabbt och följer efter, pistol i hand. Cartier står kvar och ser över Senap.")
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(väldigt argt)")] #text("STANNA!")]
#text("De försvinner ur scen och ridån dras för medan Cartier stirrar på Senap.")
#text("  ")
#text(weight: "bold","Musiknummer 7")
#text("[[Klimts och Schumachers deppiga deppnummer som leder in i scen 18. Typ fortsättning på scen 15. Centralmaktssoldaterna har det inte bra. Eventuellt att vi kör en snutt av huvudnumret som avslut efter omstarter för att signalera att nu fortsätter handlingen.]]")
#pagebreak()
#scene[#text("SCEN 18")]
#text("[[Karaktärer: Schumacher, Klimt, Müller]]")
#text("[[Plats: Centralmaktsläger.]]")
#text("[[Schumacher och Klimt får tag i Müller. De förklarar att fienden har senapsgas och att Müller borde dra tillbaka sina trupper. Müller förstår inte allvaret och tycker att soldaterna är mesiga. Tycker att de borde öka anfallet ännu mer nu när motståndarna ligger i underläge. Senapsgas är säkert inte så farligt. Müller tar med sig soldaterna för ett anfall för att visa vart skåpet ska stå.]]")
#text("General Müller är mitt uppe i intensivt planerande och står och ritar på en karta när Klimt och Schumacher kommer in på scenen. De är andfådda och smutsiga efter att just ha varit ute i slagfältet.")
#dialogue[#text("KLIMT OCH SCHUMACHER")][#text("General Müller!")]
#text("Müller kollar upp från sina papper och ser förvånat på Klimt och Schumacher.")
#dialogue[#text("MÜLLER")][#text("Vad är detta? Jag är upptagen, och ni borde vara ute på slagfältet! [[Jag har tagit bort nästan alla gånger Müller frågar något, men tyckte det kändes naturligt att ha kvar någon form av fråga här]]")]
#dialogue[#text("SCHUMACHER")][#text("Nej! Vi måste lägga ner anfallet, nu!")]
#dialogue[#text("MÜLLER")][#text("Anfallet går ju alldeles utmärkt. Britterna slänger sig praktiskt taget framför våra kulor. Ut med er igen!")]
#dialogue[#text("KLIMT")][#text("Det är det som är problemet. Det är en ren massaker eftersom fienden står utan ammunition!  [[???]]")]
#text("Müller blir mer uppmärksam när han hör att fienden saknar ammunition.")
#dialogue[#text("KLIMT")][#text("Och det finns en stor risk att vi går rakt in i en fälla när vi tar oss så nära deras läger. Deras general har ett hemligt vapen som kan… [[Jag tänker att Klimt plan är att övertyga Müller med logos-argument medan Schumacher har patos i sina]]")]
#dialogue[#text("MÜLLER")][#parenthetical[#text("(Avbryter Klimt)")] #text("Utan ammunition säger du? Hur kommer det sig?")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(Utan att tänka på det)")] #text("All ammunition som vi har kommer från de allierade. Det är därför vi har så mycket och de har så lite.")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(Skämtsamt men rädd)")] #text("Haha Schumacher, du har så livlig fantasi.") #parenthetical[#text("(Till Müller)")] #text("Hon måste ha ätit en av sina konstiga svampar igen.")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(Bestämt till Klimt)")] #text("Nej, Klimt. Denna gång måste jag berätta sanningen. [[huh]]") #parenthetical[#text("(Till Müller, desperat)")] #text("Vi träffade några soldater ute i ingenmansland och blev riktigt goda vänner med dem. Eftersom de tillhör den allierade sidan fick vi veta att deras general planerade ett anfall. Ingen av oss ville strida så vi hjälptes åt att gömma deras ammunition på vår sida. Planen lyckades och de la ner sitt anfall, men nu är det vi som attackerar dem i stället! Det var inte alls det som skulle hända. Jag vill inte att våra vänner ska dö, General Müller, så snälla snälla snälla kan vi göra en reträtt? [[Jag har ändrat lite nu så att istället för att Schumacher råkar \"spilla the beans\" så säger hon det med flit i ett desperat (och lite korkat) försök att övertyga Müller. Jag tycker att det gör Schumacher till en mer tapper och intressant karaktär. Dock om det inte funkar kan jag ändra tillbaka repliken som den var innan. ]] [[Är lite osäker på om Schumacher pratar exakt så här men det går nog att fixa till lite]]")]
#text("Det blir en kort tystnad.  ")
#dialogue[#text("MÜLLER")][#text("Vad är det ni säger? Jag tror knappt mina öron.")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(rädd)")] #text("General Müller… Jag kan förklara… [[Byt ut med Schumacher vill inte skjuta på hennes vänner]] [[Jag la det där uppe i stället, är det ok?]]")]
#dialogue[#text("MÜLLER")][#parenthetical[#text("(bestämt)")] #text("Nej, Klimt! Det räcker nu. Det här måste vara en av världshistoriens mest våghalsiga planer som också - [[\"Berätta\" \"håll käften\"]]")]
#text("Klimt och Schumacher gör sig redo att bli utskällda.")
#dialogue[#text("MÜLLER")][#parenthetical[#text("(glad)")] #text("Lyckades! Ni är nog de mest flitiga soldaterna i hela centralmakterna. Inte bara har ni infiltrerat fienden, ni har också lyckats förnedra dem genom att stjäla deras ammunition. Utmärkt!")]
#text("Schumacher och Klimt utbyter en blick med varandra som säger \"vänta va?\".")
#dialogue[#text("KLIMT")][#text("General Müller, jag tror du missförstod oss. Vi försökte inte lura dem.")]
#dialogue[#text("MÜLLER")][#parenthetical[#text("(bestämt)")] #text("Nonsens! Ni missförstår era egna avsikter. Ni lyckades utomordentligt och har gjort en enastående insats för vårt land! Ni bortförklarar ert eget hjältedåd! [[ordentligt + utomordentligt = låter wack imo]]")]
#text("Klimt och Schumacher ser besvikna ut. ")
#dialogue[#text("MÜLLER")][#text("Fanjunkare Klimt, du nämnde något om ett hemligt vapen. Kan du berätta mer om det?")]
#text("Klimt ser fundersam ut.")
#dialogue[#text("KLIMT")][#text("Jag minns inte alla detaljer, men det var en gas. Senapsgas hette den!")]
#text("Müller vaknar till och blir likblek när han hör det Klimt säger.")
#dialogue[#text("MÜLLER")][#parenthetical[#text("(Förvånat)")] #text("Det kan inte vara sant.")]
#text("SCHUMACHER [[Skulle kunna vara klimt som säger det också]]
Hmm, jag tror faktiskt den hette dijongas. [[Tyckte det passade här men enkelt att ta bort denna replik om ni inte gillar den och så förändrar det typ ingenting]]")
#text("Müller hör inte det Schumacher säger, han är för upptagen med att tänka igenom strategier som kan lösa problemet.")
#dialogue[#text("MÜLLER")][#parenthetical[#text("(mumlar för sig själv)")] #text("Hur har de fått tag på senapsgas, jag trodde det receptet var säkert i Berlin…")]
#text("Müller reser sig och drar en pistol.")
#dialogue[#text("MÜLLER")][#text("Om de använder senapsgas så är det slutet för oss alla. Vi har ingen tid att förlora, vi måste slutföra anfallet. Kom, nu!")]
#text("Müller drar med sig Schumacher som får panik och drar med sig Klimt och så rusar de ut från scenen. Schumacher och Klimt ser rädda ut, det var inte det här de hade hoppats på. [[Det kan också vara att Müller puttar ut dem, lätt att ändra]]")
#pagebreak()
#scene[#text("SCEN 19")]
#text("[[Karaktärer: Tolkien, Senap, Nightingale, Bollbirth, Schumacher, Klimt, Müller, Röde Baronen, Cartier, Flammenwerfer, Menige 0-1]]")
#text("[[Plats: Ingenmansland.]] [[tror inte vi vill ha flygplanet med asså]]")
#text("[[The final standoff. De båda sidorna möts mitt i ingenmansland. Ingen av soldaterna vill döda den andre utom generalerna. Röde Baronen kommer in och de har alla en mexican standoff. Röde Baronen, med hjälp från sina vänner, uttrycker meningslösheten i det stora alltet. Müller märker hur hans soldater inte vill slåss och börjar tveka. Bollbirth tar tillfället i akt och försöker att döda honom. Röde Baronen offrar sig och dör.]]")
#text("[[Medans alla är distraherade tar Bollbirth tillbaka detonatorn och bestämmer sig för att döda alla, henne själv inkluderat. Hon trycker på knappen, och inget händer. Senap kommer in, lutandes på Cartiers axel. Hon har desarmerat senapsgasen. Bollbirth har ingen makt längre. Blir avdragen av scenen av sina egna soldater.]]")
#text("[[Övriga diskuterar och kommer fram till att det får vara slut på krigandet här. Ingen protesterar. Müller drar också tillbaka sina styrkor.]]")
#text("Dans strider ett par minuter innan de drar sin skadade/döda av scen.")
#text("Müller går in på scen med Klimt och Schumacher hack i häl.")
#dialogue[#text("MÜLLER")][#text("Snabba på! Om general Bollbirth verkligen har fått tag i senapsgas måste vi stoppa henne direkt! [[\"best på oss\" låter konstigt imo]]")]
#dialogue[#text("KLIMT")][#text("Men general, om den här gasen väl är så farlig, borde vi inte evakuera istället medans vi fortfarande har en chans?")]
#dialogue[#text("MÜLLER")][#text("Det är just precis därför som vi måste avancera ännu snabbare! En reträtt kommer inte kunna stoppa senapsgasen när den väl är producerad. Vår enda chans är att vinna detta slag och tillintetgöra gasen innan den är redo.")]
#dialogue[#text("SCHUMACHER")][#text("Men Müller-")]
#dialogue[#text("MÜLLER")][#text("Tyst! Jag tror jag hörde något… Det måste vara fienden! Schumacher, Klimt, håll vår högra flank, jag tar den vänstra.")]
#text("Müller vänder sig diagonalt åt publikens håll och spanar, Schumacher och Klimt är lite förvirrade men vänder sig lite inåt scen.")
#text("Nightingale och Tolkien kommer in så att Schumacher ser dem men inte Müller.")
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(försäger sig nästan)")] #text("Nämen det är ju britt-…. Brieduvorna.")]
#dialogue[#text("MÜLLER")][#text("Brieduvor? Vad är det för påfund! Vi letar efter britter och endast britter… och fransmän… och belgare, ryssar, serber, rumäner, greker och japaner…")]
#text("Klimt smyger fram till Tolkien och Nightingale.")
#dialogue[#text("TOLKIEN")][#text("Klimt! Schumacher! Åh, jag har aldrig i mitt liv varit så glad över att se tyskar!")]
#dialogue[#text("KLIMT")][#text("Tack och lov, ni är oskadda! Men vi har ett problem.") #parenthetical[#text("(nickar mot Müller)")] #text("Tror ni att ni kan stoppa senapsgasen från er sida?")]
#dialogue[#text("NIGHTINGALE")][#text("Haha, det har vi redan avklarat, min vän! Vi har gasdetonatorn.")]
#text("Nightingale klappar Tolkien på axeln, uppmuntrande. ")
#dialogue[#text("TOLKIEN")][#text("Dock har vi ett mycket större problem på väg…")]
#dialogue[#text("MÜLLER")][#parenthetical[#text("(håller utkik åt andra hållet)")] #text("Ser ni någon?")]
#dialogue[#text("SCHUMACHER")][#text("Uh, NEJ, nej, bara… Uhm- bara en… En häst! Ja! En vild krigshäst! [[Är detta en referens till något?]] [[Kan bytas ut med något godtyckligt]]")]
#text("Müller vänder sig om och ser Tolkien och Nightingale.")
#dialogue[#text("MÜLLER")][#text("Ah, ni redan tagit två soldater till fånga! Verkligen de bästa jag har!! Skjut dem då!… Skjut dem!")]
#text("Schumacher och Klimt utbyter blickar med varandra och resterande på scen men sänker inte sina vapen.")
#dialogue[#text("MÜLLER")][#text("Varför skjuter ni inte? Det är ett perfekt ögonblick!")]
#dialogue[#text("KLIMT")][#text("General, vi-")]
#text("Klimt blir avbruten av att Bollbirth snabbt kommer in på scen med Menige 0-1 (kören) hack i häl. De menige drar genast sina vapen och Müller gör detsamma.")
#dialogue[#text("BOLLBIRTH")][#text("Men om det inte är den \"ärofyllde\" general Müller som är på krigsstigen. [[Varför \"ärofyllde\"? Vad anspelar det på?]]")]
#dialogue[#text("MÜLLER")][#text("General Bollbirth, förmodar jag. Med ditt rykte trodde jag aldrig att jag skulle möta dig på fältet.")]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(fnyser)")] #text("Det är inte frivilligt jag springer runt i gyttjan. Men nåväl, vad gör väl det om mitt supervapen är bortsprunget… Om jag dödar dig här finns det ingen på hela västfronten som kan stoppa mig. [[Fast Nightingale är väl precis där?]]")]
#dialogue[#text("MÜLLER")][#text("Seså, ta inte ut segern i förskott nu. Att förlora är inte min stil.")]
#text("Bollbirth drar sin pistol och riktar den mot Müller.")
#dialogue[#text("BOLLBIRTH")][#text("Modigt sagt för ett vandrande lik.")]
#text("Röde Baronen kommer in på scen.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(till Bollbirth)")] #text("Stjäl inte min replik, generalen.")]
#text("De båda generalerna vänder sig överraskat om till Röde Baronen, som har en pistol pekande mot varje general. Röde Baronen ställer sig så att de formar en triangel.")
#dialogue[#text("MÜLLER")][#parenthetical[#text("(överraskat)")] #text("Baronen! Vad i all sin dar gör du här?")]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(skrattande)")] #text("Åh, Müller, din naiva idiot. Vem tror du gav mig senapsgasen från första början?")]
#text("Müller kollar menande mot Röde Baronen som kollar ner mot marken.")
#dialogue[#text("MÜLLER")][#parenthetical[#text("(tänkande)")] #text("Jag förstår… Tragiskt att se någon som dig falla så långt. Men å andra sidan… Det är väl vad ni piloter gör.")]
#text("Röde Baronen höjer blicken och osäkrar sina pistoler.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(hotande)")] #text("Lägg ner era vapen! Båda två.")]
#text("Bollbirth grips först av förvirring men inser situationen till hands.")
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(muttrande) [[vad tror bb att rb har för motivation här?]]")] #text("Tänker du gå mot ")#text(style: "italic","båda")#text(" sidorna? Jag trodde helt ärligt att du var smartare än så här. Vad nu då? Tänker du bara skjuta oss?")]
#dialogue[#text("RÖDE BARONEN")][#text("Inte om jag inte måste. Ingen behöver dö, kalla bara tillbaka era trupper. [[Kanske skriva lite mer och göra det tydligare att Röde Baronen verkligen inte vill göra det. ]]")]
#dialogue[#text("MÜLLER")][#parenthetical[#text("(argt)")] #text("Du har ingen rätt att ge mig order, förrädare! Mina soldater skulle aldrig gå med på en sådan reträtt!")]
#dialogue[#text("RÖDE BARONEN")][#text("Verkligen? Du borde nog ta och fråga dem själv, general.")]
#dialogue[#text("MÜLLER")][#parenthetical[#text("(lite paff, men fort. arg)")] #text("Klimt? Schumacher? Döda denna förrädare!")]
#text("Klimt och Schumacher ser beklagande på Müller, som börjar inse att detta inte kommer gå.")
#dialogue[#text("SCHUMACHER")][#text("Förlåt, general, men nej. Röde Baronen har rätt. Vi vill inte kriga.")]
#dialogue[#text("KLIMT")][#text("Vi vill inte döda varandra i ett krig som inte ens ser oss som människor. Meningslöst dödande leder aldrig till något gott. [[heder inte rätt för klimt]]")]
#dialogue[#text("MÜLLER")][#parenthetical[#text("(helt paff, kan inte tro det)")] #text("Meningslöst? Va? Vill ni inte vinna? Vill ni inte vinna… för faderlandet? Ära för vårt hemland…? [[Har behöver gå till en logoped]] [[vill \"ni\" inte vinna kanske?]]")]
#text("NIGHTINGALE [[Personligen tycker jag att Tolkien borde ha denna line. Tror att det passar honom bättre, med moralen och allt...]] [[Jag är helt säker på att det här var Tolkiens line förrut]]
(kan inte hålla sig, är lite arg)
Ära?! Se dig omkring! Hur kan du se ära i det här? Vilken ära finns i ett dött fält med ruttnande kroppar?")
#text("Müller ser sig omkring på sina soldater.")
#dialogue[#text("MÜLLER")][#parenthetical[#text("(förbryllad och osäker)")] #text("Men… Det är vår plikt…")]
#text("Bollbirth skrattar, högt och rått.")
#dialogue[#text("BOLLBIRTH")][#text("Vilken usel general! Kan inte ens få sina egna soldater i ordning!! Jag börjar undra hur jag inte vunnit ännu.")]
#dialogue[#text("RÖDE BARONEN")][#text("Inte som att dina soldater är så taggade på krig heller, Bollbirth.")]
#dialogue[#text("TOLKIEN")][#text("Han har rätt, Bollbir- [[var står tolkien här?]]")]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(avbryter)")] #text("Det spelar ingen roll! Det ska inte spela roll! En bra general behöver inte sina soldaters tillåtelse att leda.")]
#text("Müller chockas över det Bollbirth säger. Flammenwerfer kommer flåsande in på scen vilket drar till sig allas uppmärksamhet.")
#dialogue[#text("FLAMMENWERFER")][#parenthetical[#text("(ropar)")] #text("General Müller! Röde Baronen är en förrädare!")]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(muttrar)")] #text("Inte för att jag behöver er längre.")]
#text("Bollbirth kastar sig mot Tolkien och tar detonatorn ur hans hand. Hon trycker på knappen. Alla tittar hastigt tillbaka på henne. Müller, Flammenwerfer och Röde Baronen höjer sina vapen mot henne.")
#text("Klimt börjar redan inse vad som håller på att hända, så hon mimar till Tolkien och Schumacher att omringa Bollbirth utan att hon märker. Tolkien och Schumacher börjar långsamt förflytta sig.")
#dialogue[#text("FLAMMENWERFER")][#text("Jag ska avsluta det här kriget!")]
#dialogue[#text("BOLLBIRTH")][#text("App app app… Inte så fort… Ni vill väl inte att jag ska tappa det här dödmansgreppet? [[Lite stel]]")]
#dialogue[#text("MÜLLER")][#text("Flammenwerfer, vänta! Om hon släpper kommer det inte vara någon kvar som kan vinna.")]
#dialogue[#text("BOLLBIRTH")][#text("Du är klipskare än du ser ut, Müller.")]
#dialogue[#text("NIGHTINGALE")][#text("Hon skulle aldrig släppa om det drabbar henne själv!")]
#dialogue[#text("BOLLBIRTH")][#text("Vill du verkligen testa dina ord, läkare?")]
#text("Alla kollar tysta på varandra i π sekunder. Bollbirth backar. När ingen reagerar, börjar Bollbirth le i triumf. Hon har inte märkt att hennes exit blockeras av Tolkien och Schumacher. ")
#dialogue[#text("BOLLBIRTH")][#text("Soldater! Skjut dem om de så mycket som rör en muskel härifrån.") #parenthetical[#text("(mot Müller)")] #text("Ni skulle bara våga försöka evakuera.")]
#text("Menige 0 och 1 ser otroligt skeptiska ut. Tittar på varandra, skruvar lite på sig. Bollbirth kan väl inte mena att lämna dem här?")
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(skadeglatt)")] #text("Såja. Och nu tänker jag dra mig tillbaka till mitt läger. Om någon av er ens FÖRSÖKER att stoppa mig, så släpper jag!")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(ser att Tolkien och Schumacher nu är i position)")] #text("Men när du gått kommer du ändå släppa… Enda skillnaden blir att du kommer undan…")]
#text("Klimt höjer sitt vapen mot Bollbirth.")
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(fortfarande stolt över sig själv)")] #text("Du tänker för mycket.")]
#text("Bollbirth vänder sig om för att sprinta därifrån, men får raskt stanna, då Tolkien och Schumacher båda håller sina vapen mot henne. ")
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(börjar bli arg)")] #text("Menig. Släpp. Igenom. Mig.")]
#dialogue[#text("TOLKIEN")][#text("Nej, det tänker jag inte! Du stannar här!")]
#text("Bollbirth vänder sig rasande om för att ta en annan väg men inser att alla hennes utvägar har nu blivit blockerade av folk med vapen. ")
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(RASANDE)")] #text("NI ODUGLIGA MASKAR!! HUR VÅGAR NI BEHANDLA MIG SÅHÄR! JAG ÄR ER GENERAL! GE-NE-RAL!!")]
#dialogue[#text("NIGHTINGALE")][#text("Du är en mördare och en galning. Vi bryr oss inte längre om dina order. [[hAlp my swedish is failing...jag vet inte om man säger \"på ditt befäl\" eller \"till\". Gissar på till men unsure :C]]")]
#dialogue[#text("RÖDE BARONEN")][#text("Ge upp, Bollbirth. Du kan inte vinna här.")]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(inser att hon har förlorat, så är mindre explosiv, men lika hatfull)")] #text("Heh, det ser ut som att du har rätt… Jag kan inte vinna här. Men om jag inte får vinna så ska ni inte få göra det heller! [[\"SÅ SKA NI DET OCKSÅ\" är lite stel]]")]
#text("Bollbirth släpper detonatorn och alla gör sig redo för att utsättas för det hemska vapen av obeskrivlig ondska de kallar senapsgas. Viktigt att ingen faktiskt trodde att Bolli skulle gasa sig själv…")
#text("Alla väntar ett litet tag och märker att inget händer. Bollbirth sparkar på detonatorn, plockar upp den och börjar trycka på knappen flertalet gånger, mer och mer panikartat (roligt). Senap, som stödjer sig på Cartier, kommer in på scen.")
#dialogue[#text("SENAP")][#parenthetical[#text("(inhaltande)")] #text("Tyvärr general. Jag är rädd att gasen är slut.")]
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(rasande)")] #text("Senap! Vad har du gjort?!")]
#dialogue[#text("CARTIER")][#text("Jag och vår lilla löjtnant tog hand om gasen. Den är inget hot längre.")]
#dialogue[#text("BOLLBIRTH")][#text("Ni vad?! Senap, du kommer HÄNGAS för detta förräderi!")]
#dialogue[#text("SENAP")][#text("Ge upp, Bollbirth.")]
#text("Bollbirth kollar runt på alla omkring henne.")
#dialogue[#text("BOLLBIRTH")][#text("Nåväl… Vad spelar gasen för roll om jag kan vinna fronten här och nu!")]
#text("Bollbirth skjuter mot Müller men Röde Baronen hoppar in mellan. Müller samlar sig och skjuter tillbaka och träffar Bollbirth i axeln vilket får henne att tappa sin pistol.")
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(skriker)")] #text("Baronen! NEJ!")]
#text("Nightingale springer fram till den nu verkligt ")#text(style: "italic","Röde")#text(" Baronen (haha fattar ni för han förblöder…). Bollbirth kollar flyktigt på sina soldater.")
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(till Menige 0 & 1)")] #text("Soldater! Skjut generalen! Skjut dem!")]
#text("Bollbirths soldater utbyter blickar och släpper sina vapen.")
#dialogue[#text("MENIG 0")][#text("Vi lyssnar inte längre på dig!")]
#dialogue[#text("MENIG 1")][#text("Vi vill inte heller slåss!")]
#dialogue[#text("SENAP")][#parenthetical[#text("(självsäkert)")] #text("Meniga. För bort general Bollbirth…")]
#text("Menige 0 och Menige 1 går fram och tar fast Bollbirth, för bort henne från scen.")
#dialogue[#text("BOLLBIRTH")][#parenthetical[#text("(vansinnigt arg)")] #text("Soldater! SOLDATER! SLÄPP MIG GENAST! NI KOMMER STÄLLAS I TRIBUNAL! SLÄPP MIG OMEDELBART!")]
#text("Bollbirth och soldaterna är ute ur scen. Senap går fram mot Müller.")
#dialogue[#text("SENAP")][#text("General Müller. Om ni drar tillbaka era styrkor omedelbart så kommer vi göra detsamma. Vi behöver inte fortsätta slåss.")]
#text("Müller ser på sina soldater. Senap räcker fram en hand.")
#dialogue[#text("MÜLLER")][#text("Varför skulle jag gå med på det? Ni står ju fortfarande utan någon ammunition, och nu har ni dessutom tappat både er general och ert enda vapen mot oss! Vår vinst kommer vara lätt! [[\"rekonstruktion\" ?]]")]
#dialogue[#text("TOLKIEN")][#text("Hur kan du ens fortfarande vilja strida!?")]
#text("Müller börjar säga något, men Nightingale avbryter honom eftersom Röde Baronen försöker komma på några riktigt badass sista ord.")
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(höjer rösten)")] #text("Tysta allihopa! Han försöker säga något.")]
#text("Alla släpper varandra lite stelt, tystnar och kollar på Röde Baronen som ligger på marken med huvudet i Nightingales knä.")
#dialogue[#text("RÖDE BARONEN")][#parenthetical[#text("(döende)")] #text("Hah… hahahaah… hah. Vet ni, jag tänkte alltid att jag skulle bli en hjälte? En hjälte, som alltid överlever och kommer ut med en cool anekdot eller två. Heh. [[Jag vet inte vad jag ska göra med denna]] [[insert \"och nu är du den bäste vännen\"]] [[tycker det är svårt att få in den här. ska den vara i pausen för (mer seriöst), eller efter att han dött?]]") #text("\\")#text(style: "italic","host host\\") #parenthetical[#text("(mer seriöst)")] #text("Se nu till att få slut på det här. På allt krigande. Och… lev bra liv. [[Change da world. My final message... Goodbye]] [[\"Ändra världen. Mitt sista meddelande... Hejdå\" ]]")]
#text("Röde Baronen dör. Samtliga står i tystnad runt honom en kort stund, innan Müller försiktigt avbryter.")
#dialogue[#text("MÜLLER")][#text("Jag… Eh…")]
#text("Alla tittar på honom.")
#dialogue[#text("MÜLLER")][#parenthetical[#text("(tänker genom allt)")] #text("Trots vad han gjorde, så dog han ärofyllt. Vi borde respektera hans sista vilja och avsluta krigandet, åtminstone idag.") #parenthetical[#text("(vänder sig till Senap)")] #text("Jag accepterar ditt erbjudande, general…?")]
#dialogue[#text("SENAP")][#parenthetical[#text("(lättad, men rättande)")] #text("Löjtnant Senap.")]
#text("Senap och Müller skakar hand. Alla släpper sina vapen på marken och kramas.")
#pagebreak()
#scene[#text("SCEN 20")]
#text("[[Karaktärer: Tolkien, Nightingale, Senap, Cartier, Schumacher, Klimt, Flammenwerfer]]")
#text("[[Plats: Ingenmansland utan flygplanskuliss (men med typ bänkar och saker för att visa att tid har gått)]]")
#text("[[Epilogen. Det har gått ett par dagar. Krigandet har tystnat, om än temporärt, och alla får sina slut. Respektive karaktär förklarar vad de ska göra nu och vad som har förändrats sedan kriget tystnade. Det finns hopp, de slipper kriga och de har en framtid. De drömmer om en dag där alla krig tar slut och alla kan leva i frid och frihet. De önskar fortsatt god jul.]]")
#text("EXT. INGENMANSLAND")
#text("Scenen börjar med att Tolkien, Senap och Flammenwerfer sitter vid ett bord som placerats i mitten av ingenmansland. Flammenwerfer har ett flertal medaljer runtom sin hals som han beundrar.")
#dialogue[#text("FLAMMENWERFER")][#parenthetical[#text("(visar medaljer)")] #text("Kolla på alla medaljer jag fick av general Müller!")]
#text("Senap och Tolkien kollar på dem.")
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(stöttande)")] #text("Vad ehm… Fina, Flammenwerfer.")]
#dialogue[#text("FLAMMENWERFER")][#text("Men denna är den viktigaste. Den sa generalen var från Tomten i julklapp!")]
#text("Schumacher kommer in med ett fat täckt över med en cloche.")
#dialogue[#text("SCHUMACHER")][#text("Mina damer och herrar. Ursäkta att ni fått vänta; det tog lite tid att skaffa huvudingrediensen. Men nu är jag äntligen redo att servera den utlovade måltiden som vi alla har väntat på!")]
#text("Schumacher sätter fatet ner på bordet.")
#dialogue[#text("SCHUMACHER")][#text("Här är mitt nya mästerverk:") #parenthetical[#text("(Lyfter på clochen)")] #text("JUL-ESKARGÅT!")]
#text("Tolkien och Senap tittar på fatet med avsmak.")
#dialogue[#text("SENAP OCH TOLKIEN")][#parenthetical[#text("(Väldigt äcklade)")] #text("Fint…")]
#dialogue[#text("FLAMMENWERFER")][#parenthetical[#text("(försöker imitera fransk accent, betona)")] #text("Ser utsökt ut, Schumacher, som vanligt. \"Bon petit\", som fransmännen säger.")]
#text("Cartier rusar in på scenen.")
#dialogue[#text("CARTIER")][#parenthetical[#text("(mycket arg fransman)")] #text("Det är bon appetit!")]
#dialogue[#text("FLAMMENWERFER")][#text("Tack, till dig med.")]
#text("Flammenwerfer börjar felaktigt äta snigeln på något tydligt sätt, typ tuggar på den så det krasar härligt i micken (regi får avgöra vad som blir tydligt för publiken och prata med requisita).")
#dialogue[#text("CARTIER")][#parenthetical[#text("(panikslaget)")] #text("Vad håller du på med! Om du nu ändå ska äta escargot, gör det åtminstone rätt.")]
#text("Cartier tar sig till bordet och plockar upp en snigel. Schumacher tar fram ett anteckningsblock och börjar skriva ner allt Cartier gör.")
#dialogue[#text("CARTIER")][#text("Se och lär.")]
#text("Cartier visar hur man äter escargot på rätt sätt (Sqådis löser). När han väl äter snigeln så blir han enormt förvånad. Man ser tydligt att han tycker om det, dock försöker han gömma sitt njutande.")
#dialogue[#text("CARTIER")][#parenthetical[#text("(Till Schumacher)")] #text("Inte illa… för en tysk, såklart.")]
#text("Cartier börjar genast ta flera sniglar. Schumacher blir överlycklig.")
#dialogue[#text("SCHUMACHER")][#text("Åh, verkligen! Bekräftelse från en fransman! Mitt liv är som fulländat. Under självaste julen också!")]
#text("Schumacher sätter sig ned av lättnad och börjar äta.")
#dialogue[#text("SENAP")][#text("Jag tror nog jag fortfarande tänker stå över.")]
#text("Schumacher är så livligt glad att hon inte märker av att Tolkien och Senap inte äter.")
#dialogue[#text("TOLKIEN")][#text("Förresten, Cartier. Du råkar inte ha några nyheter om vart Bollbirth tog vägen?")]
#dialogue[#text("CARTIER")][#text("Jo, vi fick telegram i morse, hon håller på att ställas inför rätta i domstol.")]
#dialogue[#text("SCHUMACHER")][#text("Oj, för att ha försökt begå krigsbrott? [[behöver något extra oomf]] [[säg krigsbrott]]")]
#dialogue[#text("CARTIER")][#text("Pfft! Som att militären skulle bry sig om det. Nej, det visade sig att hon inte betalat skatt sen 1884.")]
#text("Senap ser ut att instinktivt börja försvara militären men hindrar sig själv.")
#dialogue[#text("TOLKIEN")][#text("Vänta, vad menar du? Kommer hon bara undan med att ha försökt döda oss alla?!")]
#dialogue[#text("SENAP")][#parenthetical[#text("(tyst)")] #text("Rent militärt gjorde hon bara en strategisk uppoffring. [[Krigsministern blandas in istället för haagkonventionerna") #text("Ev måste omformulera de tidigare replikerna så det inte blir tjatigt]]") #parenthetical[#text("(Senap suckar och skakar på huvudet)")] #text("Jag var naiv som blint litade på militär-hierarkin… Men jag är glad att hon åtminstone är borta härifrån. [[säger liknande i scen 16]]")]
#dialogue[#text("CARTIER")][#text("Ingen sorg här heller. Hon tvingade ut mig på fronten, jag är bara en brevbärare. [[Omformulera (för cartier hade inget emot att vara brevbärare egentligen)]] [[Han ville bara hellre vara soldat]] [[vänta gjorde hon?]]")]
#dialogue[#text("TOLKIEN")][#text("Men förut sa du ju att du ville till fronten för att \"visa den franska stoltheten\"? Du gjorde allt för imponera på Bollbirth!")]
#text("Cartier fnyser till.")
#dialogue[#text("CARTIER")][#parenthetical[#text("(Högst förnärmat)")] #text("Jag har då aldrig visat intresse för något i den stilen.")]
#dialogue[#text("SCHUMACHER")][#text("Jag ska då säga, utan Bollbirth så är västfronten mycket lugnare. [[bättre och lungnare -> mycket lungnare]]")]
#dialogue[#text("TOLKIEN")][#text("Ja, den ljuva tystnaden är så avslappnande. Inga skott och explosioner som bedövar en dag och natt. Om bara Röde Baronen hade kunnat uppleva detta lugn på fronten också.")]
#dialogue[#text("FLAMMENWERFER")][#parenthetical[#text("(sorgsen)")] #text("Jag önskar att jag inte försökte bränna ihjäl honom innan.")]
#dialogue[#text("SENAP")][#text("Från vad ni alla har berättat om honom, önskar jag att jag också hade haft möjligheten att träffa honom.")]
#dialogue[#text("TOLKIEN")][#text("Det är tack vare honom som vi inte behöver slåss mer. [[Stämmer det här verkligen?]] [[kanske här kan man ha nåt i stil med \"tänk att även de minsta personer kan göra en stor skillnad\"]]")]
#dialogue[#text("SENAP")][#text("Våra sidor må vara i vapenvila nu, men det stora kriget fortsätter. [[\"Vår strid må vara i vapenvila\" eeeeeeh]]")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(sorgsen)")] #text("Det glömde jag bort. Jag önskar att alla i kriget bara kunde sluta slåss, som vi gjort.")]
#dialogue[#text("SENAP")][#text("Det är inte omöjligt. Vi behöver bara fler generaler som har bra omdöme och förnuft och som kan de-eskalera konflikten så att kriget tar slut så småningom.")]
#dialogue[#text("TOLKIEN")][#parenthetical[#text("(upprört)")] #text("Senap, hur kan du fortfarande säga så? Ett system som lät Bollbirth komma till toppen kan inte ha någon merit! [[bra i budskap, men vet inte om tolkien skulle ha den här slutsattsen]]")]
#dialogue[#text("SENAP")][#parenthetical[#text("(villrådigt)")] #text("Jo, det kanske du har rätt i… Jag tror jag behöver lite tid att bearbeta allt det här... Oavsett behöver jag resa hemåt och låta benet läka, men det känns skamligt att komma hem till far innan krigets slut...")]
#dialogue[#text("SCHUMACHER")][#text("Men raring, du har ju inget att skämmas för! Du gjorde helt rätt!")]
#text("Senap ger ett tacksamt leende.")
#dialogue[#text("TOLKIEN")][#text("Jag tänker också åka hem. Krigets mardrömslandskap är något jag aldrig vill uppleva igen. Fast det var ett väldigt oförväntat och trevligt äventyr också, men jag vill ärligt talat bara tillbaka. Någon måste stå upp mot kriget på hemmaplan. [[Här skulle man kunna ha ett riktigt tolkien qout]]")]
#dialogue[#text("FLAMMENWERFER")][#text("Vänta, varför får Tolkien åka hem? Han har ju knappt fått en skråma på sig.")]
#dialogue[#text("SENAP")][#text("Jag lyckades övertyga några högre upp att Tolkien inte var kompetent nog att vara kvar i armén.")]
#text("Ett tutande hörs i bakgrunden.")
#dialogue[#text("SENAP")][#parenthetical[#text("(pekar någonstans mot allierade sidan)")] #text("Tolkien, det verkar som att vår transport är här nu.")]
#text("Tolkien och Senap reser sig upp från matbordet.")
#dialogue[#text("TOLKIEN")][#text("Just det. Vi ber så hemskt mycket om ursäkt, men vi måste tyvärr åka nu. [[Tolkien känns som en Oblivion-karaktär]] [[better?]]")]
#text("Tolkien och Senap börjar röra sig mot sin skjuts. Schumacher ställer sig upp.")
#dialogue[#text("SCHUMACHER")][#text("Hörrni, vänta!")]
#text("Tolkien och Senap stannar.")
#dialogue[#text("SCHUMACHER")][#text("Jag blev plötsligt påmind om en brittisk kock som lovade att bjuda mig på pudding!")]
#dialogue[#text("TOLKIEN")][#text("Ja… Tjaa, jag tänker ju inte stoppa dig men du vet väl att England fortfarande är i krig med Tyskland? Är du säker på att du vill åka till ")#text(style: "italic","England?")#text(" [[englandelnglad]]")]
#dialogue[#text("SCHUMACHER")][#parenthetical[#text("(nonchalant)")] #text("Äsch, jag har kvar min sjömansuniform, så det är lugnt. Var är Klimt? Jag måste säga hejdå till henne innan vi drar.")]
#dialogue[#text("SENAP")][#parenthetical[#text("(himlar med ögonen)")] #text("Gör det snabbt bara. Jag tror inte att transporten kommer vänta länge.")]
#text("Klimt och Nightingale promenerar in på scenen. De har en livlig diskussion.")
#dialogue[#text("NIGHTINGALE")][#parenthetical[#text("(Gestikulerar med armarna)")] #text("Men om du gör första snittet med skalpellen här så blir det mycket enklare att stoppa blödningen senare.")]
#dialogue[#text("KLIMT")][#text("Jaså? Men kommer du inte skära bort för mycket av muskeln då? Det känns inte så säkert.")]
#dialogue[#text("SCHUMACHER")][#text("KLIMT!! JAG ÅKER NU!")]
#dialogue[#text("KLIMT")][#text("Va? Åker? Kriget är inte slut än?")]
#dialogue[#text("SCHUMACHER")][#text("Jag ska åka till England med Tolkien och Senap. Jag är inte menad för det militära.")]
#dialogue[#text("KLIMT")][#parenthetical[#text("(Förvirrad och förvånad)")] #text("Men jag förstår inte? Jag trodde du alltid ville till Frankrike.")]
#dialogue[#text("SCHUMACHER")][#text("Jodå, det vill jag, men Frankrike kan vänta tills kriget är över. Jag har bara varit där en gång. Det finns säkert en jättestor bredd av smaker jag har missat! [[Förtydliga att Schumacher vill till England för att hon har insett att hon bara har upplevt överklass England, men vill nu uppleva det \"riktiga\" England]] [[ev det finns säkert en stor bredd av smaker jag inte testat?]]")]
#dialogue[#text("SENAP")][#parenthetical[#text("(fullt seriös)")] #text("Absolut! I det brittiska köket kan du hitta alla smaker: från osaltat till över-saltat. [[Omformulera]] [[Senap tror att salt är \"the pinnacle of krydda\"]]")]
#text("Schumacher tittar på henne, lite skeptiskt.")
#dialogue[#text("KLIMT")][#parenthetical[#text("(lite nedstämd)")] #text("Jaha… Jag hade hoppats att vi alla skulle åka hem tillsammans så småningom. När kriget äntligen har nått sitt slut… [[till nästa jul?]]")]
#dialogue[#text("SCHUMACHER")][#text("Oroa er inte, det kommer nog att vara slut till nästa jul.")]
#dialogue[#text("SCHUMACHER")][#text("Och annars kan jag önska mig fred i julklapp nästa år! I år var jag lite upptagen med att tänka på att önska julmat i år. Men jag får ")#text(style: "italic","alltid")#text(" vad jag önskar mig i till jul! [[Byt mat mot annat rika människor vill ha, som minderåringar eller något (inte rik så har ingen aning)]]")]
#text("De andra tar en stund och tittar skeptiskt på Schumacher.")
#dialogue[#text("NIGHTINGALE")][#text("Ja… Även om vissa av oss får åka hem så fortsätter ju kriget. Även om vår lilla del av fronten slutat skjuta så är vi ju långt ifrån dem enda som vill träffa sina familjer igen. Det enda att göra är väl att säga adjö och hoppas att det inte blir sista gången vi säger så.")]
#text("Alla nickar och håller med Nightingale. Schumacher, Senap och Tolkien rör sig mot sin skjuts. Nightingale och Cartier förbereder sig att röra sig tillbaka mot den allierade sidan och Flammenwerfer gör samma mot Centralmaktsidan.")
#dialogue[#text("KLIMT")][#text("Vänta nu!")]
#text("Alla stannar upp.")
#dialogue[#text("KLIMT")][#text("Kan vi åtminstone sjunga ett sista musiknummer innan vi tar farväl?")]
#text("Alla springer tillbaka in till mitten av scen.")
#text("  ")
#text(weight: "bold","Musiknummer 8")
#text("[[Slutnummer. Alla sjunger om att deras jobbiga tid i kriget är över och att de aldrig kommer att glömma sin tid på fronten.]]")
