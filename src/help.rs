pub const HELP: &'static str = "

    Kod funkcija koje zamjenjuju jedan string sa drugim koriste se 
    specijalni znakovi za:
      razmak,        [SPACE] 
      prazan string, [EMPTY]
      novi red,      [NL]
      return,        [RETURN]
      tab,           [TAB]

- funkcije --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- --- ---
    -pbr (p to br)
      Zamjenjuje p tag sa br tagom.
    --- upotreba: -pbr
                    
    -ret (remove empty tag)
      Uklanja prazan tag, tj. tag bez sadržaja
    --- upotreba: -ret::p

    -rets (remove empty tags) 
      Uklanja prazne tagove (p|h1|h2|div)
    --- upotreba: -rets

    -r (replace) 
      Zamjenjuje jedan string sa drugim.
      Razmaci u stringu moraju se zamijeniti sa [SPACE]
    --- upotreba: -r::from::to     

    -rre (replace regex)
      Zamjenjuje string koji odgovara regularnom izrazu.
      Paziti na specijalne znakove.
      Mogu se korisiti izrazi za captured groups $0, $1, $2...
        11-22-333-4444
    --- upotreba: -rre::regex::to
                  -rre::(\\d+)-(\\d+)-(\\d+)-(\\d+)::$4-$3-$2-$1
                          $1     $2     $3     $4, dok je $0 čitav regex
                        iz 11-22-333-4444 u 4444-333-22-11

    -rd (remove double)
      Zamjenjuje ponavljajuće/višestruke znakove.
      Za uklanjanje ponavljajućih razmaka korisiti [SPACE]
    --- upotreba: -rd::ponavljajući_znak
    
    -raa (remove attributes all)
      Čisti sve tagove od atributa.
    --- upotreba: -raa

    -rt (remove tag)
      Uklanja tag.
    --- upotreba: -rt::tag

    -ct (change tag)
      Mijenja jedan tag u drugi.
    --- upotreba: -ct::from::to, 
                  -ct::div::p

    -ml (make link)
      Radi URI linkove.      
    --- upotreba: -ml

    -me (make emails)
      Radi email linkove.
    --- upotreba: -me

    -ma (make anchors)
      - Linkat će na red u kojem se nalazi znak [:A:] i 
      pri čemu će tekst u tom redu biti naziv linka.
      - Najbolje je oznaku [:A:] staviti ispred naslova,
      pa će sam naslov biti naziv linka, dok će oznaka
      [:A:] biti zamijenjena sa anchorom/sidrom.
    --- upotreba: -ma // [:A:]Neki naslov u tekstu

    -sa (set attribute)
      Dodaje atribut ili mijenja vrijednost postojećem atributu
    --- upotreba: -sa::tag::attribute::value
                  -sa::a::target::_blank
                  -sa::span::style::   // napravit će prazan atribut style = \"\"
";
