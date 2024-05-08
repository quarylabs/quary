The `pbjson-types` crates was taken https://github.com/influxdata/pbjson and modified to used as a library. The original code was written by InfluxData and is licensed under the MIT license.

The changes made are: 

- For Struct, instead of HashMap, we use BTreeMap to keep the order of the fields.