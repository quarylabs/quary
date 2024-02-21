WITH shifts AS (
    SELECT
        employee_id,
        shift_date,
        shift
    FROM q.stg_shifts
),

shift_details AS (
    SELECT
        shift AS shift_name,
        start_time,
        end_time
    FROM q.shift_hours
)

SELECT
    s.employee_id AS employee_id,
    s.shift AS shift,
    CAST(s.shift_date AS TIMESTAMP)
    + CAST(sd.start_time AS INTERVAL) AS shift_start,
    CAST(s.shift_date AS TIMESTAMP) + CAST(sd.end_time AS INTERVAL) AS shift_end
FROM shifts AS s
INNER JOIN shift_details AS sd
    ON s.shift = sd.shift_name
