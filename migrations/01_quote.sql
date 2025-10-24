CREATE TABLE quote
(
    id UUID NOT NULL PRIMARY KEY,
    author         TEXT        NOT NULL,
    quote  TEXT        NOT NULL,
    create_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
