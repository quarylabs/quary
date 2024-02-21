SELECT
    id,
    shop_name,
    manager_id,
    CAST(latitude AS FLOAT64) AS latitude,
    CAST(longitude AS FLOAT64) AS longitude
FROM
    q.raw_shop_locations
