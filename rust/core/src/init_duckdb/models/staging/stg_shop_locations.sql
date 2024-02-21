SELECT
    id,
    shop_name,
    manager_id,
    CAST(latitude AS DOUBLE) AS latitude,
    CAST(longitude AS DOUBLE) AS longitude
FROM
    q.raw_shop_locations
