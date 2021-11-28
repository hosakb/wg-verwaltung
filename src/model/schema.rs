table! {
    artikel (id) {
        id -> Int4,
        preis -> Numeric,
        menge -> Int4,
        bezeichnung -> Nullable<Varchar>,
        wiederkehrend -> Nullable<Bool>,
    }
}

table! {
    artikel_betrifft_transaktion (artikel_id, transaktions_id) {
        artikel_id -> Int4,
        transaktions_id -> Int4,
    }
}

table! {
    aufgabe (id) {
        id -> Int4,
        titel -> Varchar,
        beschreibung -> Nullable<Varchar>,
        datum -> Date,
        wiederkehrend -> Nullable<Bool>,
        beendet -> Nullable<Bool>,
        aufwand -> Nullable<Int4>,
    }
}

table! {
    bewohner (id) {
        id -> Int4,
        name -> Varchar,
        nutzername -> Varchar,
        passwort -> Varchar,
        admin -> Bool,
        geburtstag_id -> Int4,
    }
}

table! {
    bewohner_benoetigt_artikel (bewohner_id, artikel_id) {
        bewohner_id -> Int4,
        artikel_id -> Int4,
    }
}

table! {
    ereignis (id) {
        id -> Int4,
        titel -> Varchar,
        beschreibung -> Nullable<Varchar>,
        datum -> Date,
        wiederkehrend -> Nullable<Bool>,
    }
}

table! {
    ereignis_betrifft (bewohner_id, ereignis_id) {
        bewohner_id -> Int4,
        ereignis_id -> Int4,
    }
}

table! {
    geburtstag (id) {
        id -> Int4,
        datum -> Date,
    }
}

table! {
    nimmt_teil_aufgabe (bewohner_id, aufgaben_id) {
        bewohner_id -> Int4,
        aufgaben_id -> Int4,
    }
}

table! {
    transaktion (id) {
        id -> Int4,
        betrag -> Numeric,
        zweck -> Nullable<Varchar>,
        bewohner_id -> Int4,
    }
}

table! {
    verwaltet_aufgabe (bewohner_id, aufgaben_id) {
        bewohner_id -> Int4,
        aufgaben_id -> Int4,
    }
}

joinable!(artikel_betrifft_transaktion -> artikel (artikel_id));
joinable!(artikel_betrifft_transaktion -> transaktion (transaktions_id));
joinable!(bewohner -> geburtstag (geburtstag_id));
joinable!(bewohner_benoetigt_artikel -> artikel (artikel_id));
joinable!(bewohner_benoetigt_artikel -> bewohner (bewohner_id));
joinable!(ereignis_betrifft -> bewohner (bewohner_id));
joinable!(ereignis_betrifft -> ereignis (ereignis_id));
joinable!(nimmt_teil_aufgabe -> aufgabe (aufgaben_id));
joinable!(nimmt_teil_aufgabe -> bewohner (bewohner_id));
joinable!(transaktion -> bewohner (bewohner_id));
joinable!(verwaltet_aufgabe -> aufgabe (aufgaben_id));
joinable!(verwaltet_aufgabe -> bewohner (bewohner_id));

allow_tables_to_appear_in_same_query!(
    artikel,
    artikel_betrifft_transaktion,
    aufgabe,
    bewohner,
    bewohner_benoetigt_artikel,
    ereignis,
    ereignis_betrifft,
    geburtstag,
    nimmt_teil_aufgabe,
    transaktion,
    verwaltet_aufgabe,
);
