CREATE TABLE skill (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    title VARCHAR(50) NOT NULL,
    description VARCHAR(100) NOT NULL,
    coder_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT 'NOW'::timestamptz,
    PRIMARY KEY (id),
    CONSTRAINT fk_coder
        FOREIGN KEY(coder_id) 
            REFERENCES coder(id)
);