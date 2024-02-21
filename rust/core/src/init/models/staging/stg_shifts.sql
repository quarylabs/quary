SELECT
    employee_id,
    shop_id,
    date AS shift_date,
    shift
FROM
    q.raw_shifts
