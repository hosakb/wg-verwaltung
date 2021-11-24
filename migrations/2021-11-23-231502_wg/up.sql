CREATE TABLE geburtstag(
  id SERIAL PRIMARY KEY,
  datum DATE NOT NULL
);

CREATE TABLE bewohner(
  id SERIAL PRIMARY KEY,
  name VARCHAR(200) NOT NULL,
  nutzername VARCHAR(200) NOT NULL,
  passwort VARCHAR(30) NOT NULL,
  admin boolean NOT NULL,
  geburtstag_id INT NOT NULL,
  FOREIGN KEY (geburtstag_id) REFERENCES geburtstag(id)
);

CREATE TABLE aufgabe(
  id SERIAL PRIMARY KEY,
  titel VARCHAR(100) NOT NULL,
  beschreibung VARCHAR(1000),
  datum DATE NOT NULL,
  wiederkehrend boolean DEFAULT 'false',
  beendet boolean DEFAULT 'false',
  aufwand INT
);

CREATE TABLE ereignis(
  id SERIAL PRIMARY KEY,
  titel VARCHAR(100) NOT NULL,
  beschreibung VARCHAR(1000),
  datum DATE NOT NULL,
  wiederkehrend boolean DEFAULT 'false'
);

CREATE TABLE transaktion(
  id SERIAL PRIMARY KEY,
  betrag NUMERIC(6, 2) NOT NULL,
  zweck VARCHAR(1000),
  bewohner_id INT NOT NULL,
  FOREIGN KEY (bewohner_id) REFERENCES bewohner(id)
);

CREATE TABLE artikel(
  id SERIAL PRIMARY KEY,
  preis NUMERIC(6, 2) NOT NULL,
  menge INT NOT NULL,
  bezeichnung VARCHAR(1000),
  wiederkehrend boolean DEFAULT 'true'
);

CREATE TABLE ereignis_betrifft(
  bewohner_id INT NOT NULL,
  ereignis_id INT NOT NULL,
  PRIMARY KEY (bewohner_id, ereignis_id),
  FOREIGN KEY (bewohner_id) REFERENCES bewohner(id),
  FOREIGN KEY (ereignis_id) REFERENCES ereignis(id)
);

CREATE TABLE nimmt_teil_aufgabe(
  bewohner_id INT NOT NULL,
  aufgaben_id INT NOT NULL,
  PRIMARY KEY (bewohner_id, aufgaben_id),
  FOREIGN KEY (bewohner_id) REFERENCES bewohner(id),
  FOREIGN KEY (aufgaben_id) REFERENCES aufgabe(id)
);

CREATE TABLE verwaltet_aufgabe(
  bewohner_id INT NOT NULL,
  aufgaben_id INT NOT NULL,
  PRIMARY KEY (bewohner_id, aufgaben_id),
  FOREIGN KEY (bewohner_id) REFERENCES bewohner(id),
  FOREIGN KEY (aufgaben_id) REFERENCES aufgabe(id)
);

CREATE TABLE artikel_betrifft_transaktion(
  artikel_id INT NOT NULL,
  transaktions_id INT NOT NULL,
  PRIMARY KEY (artikel_id, transaktions_id),
  FOREIGN KEY (artikel_id) REFERENCES artikel(id),
  FOREIGN KEY (transaktions_id) REFERENCES transaktion(id)
);

CREATE TABLE bewohner_benoetigt_artikel(
  bewohner_id INT NOT NULL,
  artikel_id INT NOT NULL,
  PRIMARY KEY (bewohner_id, artikel_id),
  FOREIGN KEY (bewohner_id) REFERENCES bewohner(id),
  FOREIGN KEY (artikel_id) REFERENCES artikel(id)
);
