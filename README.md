For at genoprette databasen skal følgende kommandoer køres

```sql
CREATE TABLE "users" (
    "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    "username"	TEXT UNIQUE,
    "hash"	TEXT
);
```

```sql
CREATE TABLE "messages" (
    "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    "sender"	INTEGER NOT NULL,
    "recipient"	INTEGER NOT NULL,
    "message"	TEXT NOT NULL,
    "datetime"	TIMESTAMP NOT NULL
);
```

Man kan backe systemet op ved at kopiere database.db.

For at sikre mig mod SQL injections har jeg brugt prepared statement hvilket gør angrebet umuligt. Adgangskoder er hashet med bcrypt, hvilket har et salt og er meget langsom. Derved er systemet beskyttet mod bruteforce angreb, og hvis en ond aktør skulle få adgang til databasen vil det kræve meget regnearbejde at cracke hashesne.

I denne her opstilling er alle brugers kode deres brugernavn. Man vil selvfølgelig have rigtige koder hvis systemet skulle bruges i virkligheden.

Liste af bruger som er oprettet i databasen

polly, niko, caroline, anders, john


For at køre programmet kan man enten køre "chat" på linux eller "chat.exe" på windows.
