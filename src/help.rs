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
    --- upotreba: -rre::regex::to

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
      Mjenja jedan tag u drugi.
    --- upotreba: -ct::from::to, 
                  -ct::div::p

    -ml (make link)
      Radi URI linkove.      
    --- upotreba: -ml::target   // _blank, _self ...
                  -ml::         // link će biti bez targeta

    -me (make emails)
      Radi email linkove.
    --- upotreba: -me

    -sa (set attribute)
      Dodaje atribut ili mijenja vrijednost postojećem atributu
    --- upotreba: -sa::tag::attribute::value
                  -sa::a::target::_blank
                  -sa::span::style::   // napravit će prazan atribut style=\"\"
";