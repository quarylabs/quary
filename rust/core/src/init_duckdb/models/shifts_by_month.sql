SELECT
    employee_id,
    DATE_TRUNC('month', shift_date) AS shift_month,
    COUNT(*) AS total_shifts
FROM q.stg_shifts
GROUP BY employee_id, shift_month
