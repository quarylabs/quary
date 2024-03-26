with transformed_data as (
  select
  bw.ware_id,
  bw.beacon_id,
  wares.name as ware_name,
  wares.slug as ware_slug,
  beacons.name as beacon_name,
  beacons.slug as beacon_slug,
  beacons.discipline
  from beacons_wares bw
  left join wares on wares.id = bw.ware_id
  left join beacons on beacons.id=bw.beacon_id
)

select *
from transformed_data

--Error: "executing sql for model 'quary_bw': \"CREATE VIEW transform.quary_bw AS with transformed_data as
--(\\n  select\\n  bw.ware_id,\\n  bw.beacon_id,\\n  wares.name as ware_name,\\n  wares.slug as ware_slug,\\n
-- beacons.name as beacon_name,\\n  beacons.slug as beacon_slug,\\n  beacons.discipline\\n  from beacons_wares bw\\n
-- left join wares on wares.id = bw.ware_id\\n  left join beacons on beacons.id=bw.beacon_id\\n)\\n\\n
-- create materialized view as\\nselect *\\nfrom transformed_data\\n\" \"error returned from database: syntax error at or near \\\"create\\\"\""

