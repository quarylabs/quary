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
    s.employee_id,
    s.shift,
    datetime(s.shift_date, sd.start_time) AS shift_start,
    datetime(s.shift_date, sd.end_time) AS shift_end
FROM shifts AS s
INNER JOIN shift_details AS sd
    ON s.shift = sd.shift_name
