CREATE TABLE login_ticket
(
    account_id BIGINT  NOT NULL UNIQUE REFERENCES account ON DELETE CASCADE,
    ticket     TEXT    NOT NULL,
    used       BOOLEAN NOT NULL         DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
