SELECT c.employee_id
FROM q.shifts_summary AS c
WHERE c.first_shift > c.last_shift
