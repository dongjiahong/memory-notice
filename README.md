# MemoryNotice

一个基于ANKI-SM2算法的提醒工具。

## Project Setup
```sh
npm install
```

### Compile and Hot-Reload for Development
```sh
npm run tauri dev
```

### Compile and Minify for Production
```sh
npm run build
```

## database

```sql
CREATE TABLE tasks (
    id          INTEGER       PRIMARY KEY AUTOINCREMENT,
    task        VARCHAR (255) NOT NULL,
    last_date   VARCHAR (255) NOT NULL,
    review_date VARCHAR (255) NOT NULL,
    duration    INTEGER       NOT NULL
                              DEFAULT (1),
    repetitions INTEGER       NOT NULL
                              DEFAULT (0),
    efactor     DOUBLE        NOT NULL
                              DEFAULT (2.5),
    tip         TEXT,
    deleted     INTEGER       NOT NULL
                              DEFAULT (0) 
);

```
