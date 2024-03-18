WITH
min_shifts AS (
    SELECT
        employee_id,
        MIN(shift_start) AS shift_start
    FROM
        q.shifts
    GROUP BY
        employee_id
)

SELECT
    x.employee_id,
    x.shift_start,
    x.shift_end
FROM
    q.shifts AS x
INNER JOIN min_shifts AS y
    ON
        x.employee_id = y.employee_id
        AND x.shift_start = y.shift_start
GROUP BY
    x.employee_id,
    x.shift_start
