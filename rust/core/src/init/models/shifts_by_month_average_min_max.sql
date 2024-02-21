SELECT
    shift_month,
    AVG(total_shifts) AS average_shifts,
    MIN(total_shifts) AS min_shifts,
    MAX(total_shifts) AS max_shifts
FROM
    q.shifts_by_month
GROUP BY shift_month
ORDER BY shift_month
