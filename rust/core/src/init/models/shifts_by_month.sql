SELECT
    employee_id,
    strftime('%Y-%m', shift_date) AS shift_month,
    count(*) AS total_shifts
FROM q.stg_shifts
GROUP BY employee_id, shift_month
