CREATE TABLE IF NOT EXISTS tasks (
    id BIGSERIAL PRIMARY KEY,
    description TEXT NOT NULL,
    done BOOLEAN NOT NULL,
    due_time TIMESTAMP WITH TIME ZONE NOT NULL,
    created_by BIGSERIAL REFERENCES users(id)
);