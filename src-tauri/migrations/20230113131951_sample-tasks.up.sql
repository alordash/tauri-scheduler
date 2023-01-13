INSERT INTO
    tasks (description, create_time, due_time, created_by)
VALUES
    (
        'Exams preparation',
        current_timestamp,
        current_timestamp + '5 days',
        (
            SELECT
                id
            FROM
                users
            WHERE
                login = 'bmstudent'
        )
    ),
    (
        'Holidays',
        current_timestamp + '1 day',
        current_timestamp + '20 days',
        (
            SELECT
                id
            FROM
                users
            WHERE
                login = 'bmstudent'
        )
    ),
    (
        'Summer exams',
        current_timestamp,
        current_timestamp + '100 days',
        (
            SELECT
                id
            FROM
                users
            WHERE
                login = 'alordash'
        )
    );