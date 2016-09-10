pub const HELP: &'static str = "

    kod funkcija koje zamjenjuju jedan string sa drugim koriste se 
    posebni stringovi za:
      razmak,        [SPACE] 
      prazan string, [EMPTY] 

- funkcije ---------------------------------------------------------
    pbr - zamjenjuje p tag sa br tagom.
        upotreba: -pbr

    rets - uklanja prazne tagove (p|h1|h2|div)
        upotreba: -rets
    
    -repl - zamjenjuje jedan string sa drugim.
      Razmaci u stringu moraju se zamijeniti sa [SPACE]
        upotreba: -repl::from::to     

    -remd - zamjenjuje ponavljauće znakove.
      Za uklanjanje ponavljajućih razmaka korisiti [SPACE]
        upotreba: -remd::ponavljajući_znak
";