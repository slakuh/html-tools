pub const HELP: &'static str = "

    Kod funkcija koje zamjenjuju jedan string sa drugim koriste se 
    specijalni znakovi za:
      razmak,        [SPACE] 
      prazan string, [EMPTY] 

- funkcije ---------------------------------------------------------
    -pbr (p to br)
      Zamjenjuje p tag sa br tagom.
        upotreba: -pbr

    -rets (remove empty tags) 
      Uklanja prazne tagove (p|h1|h2|div)
        upotreba: -rets

    -repl (replace)
      Zamjenjuje jedan string sa drugim.
      Razmaci u stringu moraju se zamijeniti sa [SPACE]
        upotreba: -repl::from::to     

    -replre (replace regex)
      Zamjenjuje string koji odgovara regularnom izrazu
      Paziti na specijalne znakove.
        upotreba: -replre::regex::to

    -remd (remove double)
      Zamjenjuje ponavljauće znakove.
      Za uklanjanje ponavljajućih razmaka korisiti [SPACE]
        upotreba: -remd::ponavljajući_znak
";