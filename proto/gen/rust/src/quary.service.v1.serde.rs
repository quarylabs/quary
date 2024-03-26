// @generated
impl serde::Serialize for ColumnDescription {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if !self.tests.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ColumnDescription", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.tests.is_empty() {
            struct_ser.serialize_field("tests", &self.tests)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ColumnDescription {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "tests",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Tests,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "tests" => Ok(GeneratedField::Tests),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ColumnDescription;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ColumnDescription")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ColumnDescription, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut tests__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Tests => {
                            if tests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tests"));
                            }
                            tests__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ColumnDescription {
                    name: name__.unwrap_or_default(),
                    description: description__,
                    tests: tests__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ColumnDescription", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ColumnTest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.info.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ColumnTest", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.info.is_empty() {
            struct_ser.serialize_field("info", &self.info)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ColumnTest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "info",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Info,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "info" => Ok(GeneratedField::Info),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ColumnTest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ColumnTest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ColumnTest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Info => {
                            if info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            info__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(ColumnTest {
                    r#type: r#type__.unwrap_or_default(),
                    info: info__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ColumnTest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConnectionConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.vars.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ConnectionConfig", len)?;
        if !self.vars.is_empty() {
            struct_ser.serialize_field("vars", &self.vars)?;
        }
        if let Some(v) = self.config.as_ref() {
            match v {
                connection_config::Config::Duckdb(v) => {
                    struct_ser.serialize_field("duckdb", v)?;
                }
                connection_config::Config::DuckdbInMemory(v) => {
                    struct_ser.serialize_field("duckdbInMemory", v)?;
                }
                connection_config::Config::Sqlite(v) => {
                    struct_ser.serialize_field("sqlite", v)?;
                }
                connection_config::Config::SqliteInMemory(v) => {
                    struct_ser.serialize_field("sqliteInMemory", v)?;
                }
                connection_config::Config::BigQuery(v) => {
                    struct_ser.serialize_field("bigQuery", v)?;
                }
                connection_config::Config::Snowflake(v) => {
                    struct_ser.serialize_field("snowflake", v)?;
                }
                connection_config::Config::Postgres(v) => {
                    struct_ser.serialize_field("postgres", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConnectionConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vars",
            "duckdb",
            "duckdb_in_memory",
            "duckdbInMemory",
            "sqlite",
            "sqlite_in_memory",
            "sqliteInMemory",
            "big_query",
            "bigQuery",
            "snowflake",
            "postgres",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Vars,
            Duckdb,
            DuckdbInMemory,
            Sqlite,
            SqliteInMemory,
            BigQuery,
            Snowflake,
            Postgres,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "vars" => Ok(GeneratedField::Vars),
                            "duckdb" => Ok(GeneratedField::Duckdb),
                            "duckdbInMemory" | "duckdb_in_memory" => Ok(GeneratedField::DuckdbInMemory),
                            "sqlite" => Ok(GeneratedField::Sqlite),
                            "sqliteInMemory" | "sqlite_in_memory" => Ok(GeneratedField::SqliteInMemory),
                            "bigQuery" | "big_query" => Ok(GeneratedField::BigQuery),
                            "snowflake" => Ok(GeneratedField::Snowflake),
                            "postgres" => Ok(GeneratedField::Postgres),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConnectionConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ConnectionConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConnectionConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vars__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Vars => {
                            if vars__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vars"));
                            }
                            vars__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Duckdb => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("duckdb"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(connection_config::Config::Duckdb)
;
                        }
                        GeneratedField::DuckdbInMemory => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("duckdbInMemory"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(connection_config::Config::DuckdbInMemory)
;
                        }
                        GeneratedField::Sqlite => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sqlite"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(connection_config::Config::Sqlite)
;
                        }
                        GeneratedField::SqliteInMemory => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sqliteInMemory"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(connection_config::Config::SqliteInMemory)
;
                        }
                        GeneratedField::BigQuery => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bigQuery"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(connection_config::Config::BigQuery)
;
                        }
                        GeneratedField::Snowflake => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snowflake"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(connection_config::Config::Snowflake)
;
                        }
                        GeneratedField::Postgres => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postgres"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(connection_config::Config::Postgres)
;
                        }
                    }
                }
                Ok(ConnectionConfig {
                    vars: vars__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ConnectionConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for connection_config::ConnectionConfigBigQuery {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_id.is_empty() {
            len += 1;
        }
        if !self.dataset_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ConnectionConfig.ConnectionConfigBigQuery", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.dataset_id.is_empty() {
            struct_ser.serialize_field("datasetId", &self.dataset_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for connection_config::ConnectionConfigBigQuery {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
            "dataset_id",
            "datasetId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
            DatasetId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "datasetId" | "dataset_id" => Ok(GeneratedField::DatasetId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = connection_config::ConnectionConfigBigQuery;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ConnectionConfig.ConnectionConfigBigQuery")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<connection_config::ConnectionConfigBigQuery, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                let mut dataset_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DatasetId => {
                            if dataset_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("datasetId"));
                            }
                            dataset_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(connection_config::ConnectionConfigBigQuery {
                    project_id: project_id__.unwrap_or_default(),
                    dataset_id: dataset_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ConnectionConfig.ConnectionConfigBigQuery", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for connection_config::ConnectionConfigDuckDb {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if self.schema.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ConnectionConfig.ConnectionConfigDuckDB", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if let Some(v) = self.schema.as_ref() {
            struct_ser.serialize_field("schema", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for connection_config::ConnectionConfigDuckDb {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "schema",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            Schema,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "path" => Ok(GeneratedField::Path),
                            "schema" => Ok(GeneratedField::Schema),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = connection_config::ConnectionConfigDuckDb;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ConnectionConfig.ConnectionConfigDuckDB")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<connection_config::ConnectionConfigDuckDb, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut schema__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Schema => {
                            if schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schema"));
                            }
                            schema__ = map_.next_value()?;
                        }
                    }
                }
                Ok(connection_config::ConnectionConfigDuckDb {
                    path: path__.unwrap_or_default(),
                    schema: schema__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ConnectionConfig.ConnectionConfigDuckDB", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for connection_config::ConnectionConfigDuckDbInMemory {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.schema.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ConnectionConfig.ConnectionConfigDuckDBInMemory", len)?;
        if let Some(v) = self.schema.as_ref() {
            struct_ser.serialize_field("schema", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for connection_config::ConnectionConfigDuckDbInMemory {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "schema",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Schema,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "schema" => Ok(GeneratedField::Schema),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = connection_config::ConnectionConfigDuckDbInMemory;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ConnectionConfig.ConnectionConfigDuckDBInMemory")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<connection_config::ConnectionConfigDuckDbInMemory, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut schema__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Schema => {
                            if schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schema"));
                            }
                            schema__ = map_.next_value()?;
                        }
                    }
                }
                Ok(connection_config::ConnectionConfigDuckDbInMemory {
                    schema: schema__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ConnectionConfig.ConnectionConfigDuckDBInMemory", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for connection_config::ConnectionConfigPostgres {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.schema.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ConnectionConfig.ConnectionConfigPostgres", len)?;
        if !self.schema.is_empty() {
            struct_ser.serialize_field("schema", &self.schema)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for connection_config::ConnectionConfigPostgres {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "schema",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Schema,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "schema" => Ok(GeneratedField::Schema),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = connection_config::ConnectionConfigPostgres;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ConnectionConfig.ConnectionConfigPostgres")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<connection_config::ConnectionConfigPostgres, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut schema__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Schema => {
                            if schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schema"));
                            }
                            schema__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(connection_config::ConnectionConfigPostgres {
                    schema: schema__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ConnectionConfig.ConnectionConfigPostgres", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for connection_config::ConnectionConfigSnowflake {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.account_url.is_empty() {
            len += 1;
        }
        if !self.client_id.is_empty() {
            len += 1;
        }
        if !self.client_secret.is_empty() {
            len += 1;
        }
        if !self.role.is_empty() {
            len += 1;
        }
        if !self.database.is_empty() {
            len += 1;
        }
        if !self.schema.is_empty() {
            len += 1;
        }
        if !self.warehouse.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ConnectionConfig.ConnectionConfigSnowflake", len)?;
        if !self.account_url.is_empty() {
            struct_ser.serialize_field("accountUrl", &self.account_url)?;
        }
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("clientId", &self.client_id)?;
        }
        if !self.client_secret.is_empty() {
            struct_ser.serialize_field("clientSecret", &self.client_secret)?;
        }
        if !self.role.is_empty() {
            struct_ser.serialize_field("role", &self.role)?;
        }
        if !self.database.is_empty() {
            struct_ser.serialize_field("database", &self.database)?;
        }
        if !self.schema.is_empty() {
            struct_ser.serialize_field("schema", &self.schema)?;
        }
        if !self.warehouse.is_empty() {
            struct_ser.serialize_field("warehouse", &self.warehouse)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for connection_config::ConnectionConfigSnowflake {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account_url",
            "accountUrl",
            "client_id",
            "clientId",
            "client_secret",
            "clientSecret",
            "role",
            "database",
            "schema",
            "warehouse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccountUrl,
            ClientId,
            ClientSecret,
            Role,
            Database,
            Schema,
            Warehouse,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "accountUrl" | "account_url" => Ok(GeneratedField::AccountUrl),
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "clientSecret" | "client_secret" => Ok(GeneratedField::ClientSecret),
                            "role" => Ok(GeneratedField::Role),
                            "database" => Ok(GeneratedField::Database),
                            "schema" => Ok(GeneratedField::Schema),
                            "warehouse" => Ok(GeneratedField::Warehouse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = connection_config::ConnectionConfigSnowflake;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ConnectionConfig.ConnectionConfigSnowflake")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<connection_config::ConnectionConfigSnowflake, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account_url__ = None;
                let mut client_id__ = None;
                let mut client_secret__ = None;
                let mut role__ = None;
                let mut database__ = None;
                let mut schema__ = None;
                let mut warehouse__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccountUrl => {
                            if account_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accountUrl"));
                            }
                            account_url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClientSecret => {
                            if client_secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientSecret"));
                            }
                            client_secret__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Role => {
                            if role__.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            role__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Database => {
                            if database__.is_some() {
                                return Err(serde::de::Error::duplicate_field("database"));
                            }
                            database__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Schema => {
                            if schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schema"));
                            }
                            schema__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Warehouse => {
                            if warehouse__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warehouse"));
                            }
                            warehouse__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(connection_config::ConnectionConfigSnowflake {
                    account_url: account_url__.unwrap_or_default(),
                    client_id: client_id__.unwrap_or_default(),
                    client_secret: client_secret__.unwrap_or_default(),
                    role: role__.unwrap_or_default(),
                    database: database__.unwrap_or_default(),
                    schema: schema__.unwrap_or_default(),
                    warehouse: warehouse__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ConnectionConfig.ConnectionConfigSnowflake", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for connection_config::ConnectionConfigSqLite {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ConnectionConfig.ConnectionConfigSqLite", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for connection_config::ConnectionConfigSqLite {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "path" => Ok(GeneratedField::Path),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = connection_config::ConnectionConfigSqLite;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ConnectionConfig.ConnectionConfigSqLite")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<connection_config::ConnectionConfigSqLite, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(connection_config::ConnectionConfigSqLite {
                    path: path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ConnectionConfig.ConnectionConfigSqLite", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for connection_config::ConnectionConfigSqLiteInMemory {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("quary.service.v1.ConnectionConfig.ConnectionConfigSqLiteInMemory", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for connection_config::ConnectionConfigSqLiteInMemory {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = connection_config::ConnectionConfigSqLiteInMemory;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ConnectionConfig.ConnectionConfigSqLiteInMemory")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<connection_config::ConnectionConfigSqLiteInMemory, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(connection_config::ConnectionConfigSqLiteInMemory {
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ConnectionConfig.ConnectionConfigSqLiteInMemory", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DatabaseSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.columns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.DatabaseSource", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DatabaseSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "path",
            "columns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Path,
            Columns,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "path" => Ok(GeneratedField::Path),
                            "columns" => Ok(GeneratedField::Columns),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DatabaseSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.DatabaseSource")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DatabaseSource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut path__ = None;
                let mut columns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Columns => {
                            if columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columns"));
                            }
                            columns__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DatabaseSource {
                    name: name__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                    columns: columns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.DatabaseSource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Edge {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.to.is_empty() {
            len += 1;
        }
        if !self.from.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Edge", len)?;
        if !self.to.is_empty() {
            struct_ser.serialize_field("to", &self.to)?;
        }
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Edge {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "to",
            "from",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            To,
            From,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "to" => Ok(GeneratedField::To),
                            "from" => Ok(GeneratedField::From),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Edge;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Edge")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Edge, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut to__ = None;
                let mut from__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::To => {
                            if to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            to__ = Some(map_.next_value()?);
                        }
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Edge {
                    to: to__.unwrap_or_default(),
                    from: from__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Edge", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Failed {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reason.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Failed", len)?;
        if let Some(v) = self.reason.as_ref() {
            match v {
                failed::Reason::Ran(v) => {
                    struct_ser.serialize_field("ran", v)?;
                }
                failed::Reason::InferredFromTests(v) => {
                    struct_ser.serialize_field("inferredFromTests", v)?;
                }
                failed::Reason::InferredThroughTestsOperation(v) => {
                    struct_ser.serialize_field("inferredThroughTestsOperation", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Failed {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ran",
            "inferred_from_tests",
            "inferredFromTests",
            "inferred_through_tests_operation",
            "inferredThroughTestsOperation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ran,
            InferredFromTests,
            InferredThroughTestsOperation,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "ran" => Ok(GeneratedField::Ran),
                            "inferredFromTests" | "inferred_from_tests" => Ok(GeneratedField::InferredFromTests),
                            "inferredThroughTestsOperation" | "inferred_through_tests_operation" => Ok(GeneratedField::InferredThroughTestsOperation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Failed;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Failed")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Failed, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ran => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ran"));
                            }
                            reason__ = map_.next_value::<::std::option::Option<_>>()?.map(failed::Reason::Ran)
;
                        }
                        GeneratedField::InferredFromTests => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inferredFromTests"));
                            }
                            reason__ = map_.next_value::<::std::option::Option<_>>()?.map(failed::Reason::InferredFromTests)
;
                        }
                        GeneratedField::InferredThroughTestsOperation => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inferredThroughTestsOperation"));
                            }
                            reason__ = map_.next_value::<::std::option::Option<_>>()?.map(failed::Reason::InferredThroughTestsOperation)
;
                        }
                    }
                }
                Ok(Failed {
                    reason: reason__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Failed", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FailedRunResults {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.query_result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.FailedRunResults", len)?;
        if let Some(v) = self.query_result.as_ref() {
            struct_ser.serialize_field("queryResult", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FailedRunResults {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "query_result",
            "queryResult",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueryResult,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "queryResult" | "query_result" => Ok(GeneratedField::QueryResult),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FailedRunResults;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.FailedRunResults")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FailedRunResults, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query_result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueryResult => {
                            if query_result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryResult"));
                            }
                            query_result__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FailedRunResults {
                    query_result: query_result__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.FailedRunResults", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for File {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.contents.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.File", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.contents.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("contents", pbjson::private::base64::encode(&self.contents).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for File {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "contents",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Contents,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "contents" => Ok(GeneratedField::Contents),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = File;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.File")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<File, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut contents__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Contents => {
                            if contents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contents"));
                            }
                            contents__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(File {
                    name: name__.unwrap_or_default(),
                    contents: contents__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.File", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileSystem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.files.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.FileSystem", len)?;
        if !self.files.is_empty() {
            struct_ser.serialize_field("files", &self.files)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileSystem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "files",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Files,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "files" => Ok(GeneratedField::Files),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileSystem;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.FileSystem")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FileSystem, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut files__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Files => {
                            if files__.is_some() {
                                return Err(serde::de::Error::duplicate_field("files"));
                            }
                            files__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(FileSystem {
                    files: files__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.FileSystem", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InferredChain {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.inferred_chain.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.InferredChain", len)?;
        if !self.inferred_chain.is_empty() {
            struct_ser.serialize_field("inferredChain", &self.inferred_chain)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InferredChain {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "inferred_chain",
            "inferredChain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InferredChain,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "inferredChain" | "inferred_chain" => Ok(GeneratedField::InferredChain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InferredChain;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.InferredChain")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InferredChain, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut inferred_chain__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InferredChain => {
                            if inferred_chain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inferredChain"));
                            }
                            inferred_chain__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InferredChain {
                    inferred_chain: inferred_chain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.InferredChain", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InferredChainWithOperation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.inferred_chain.is_empty() {
            len += 1;
        }
        if !self.operation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.InferredChainWithOperation", len)?;
        if !self.inferred_chain.is_empty() {
            struct_ser.serialize_field("inferredChain", &self.inferred_chain)?;
        }
        if !self.operation.is_empty() {
            struct_ser.serialize_field("operation", &self.operation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InferredChainWithOperation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "inferred_chain",
            "inferredChain",
            "operation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InferredChain,
            Operation,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "inferredChain" | "inferred_chain" => Ok(GeneratedField::InferredChain),
                            "operation" => Ok(GeneratedField::Operation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InferredChainWithOperation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.InferredChainWithOperation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InferredChainWithOperation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut inferred_chain__ = None;
                let mut operation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InferredChain => {
                            if inferred_chain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inferredChain"));
                            }
                            inferred_chain__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Operation => {
                            if operation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operation"));
                            }
                            operation__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InferredChainWithOperation {
                    inferred_chain: inferred_chain__.unwrap_or_default(),
                    operation: operation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.InferredChainWithOperation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Model {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if !self.file_path.is_empty() {
            len += 1;
        }
        if !self.file_sha256_hash.is_empty() {
            len += 1;
        }
        if self.materialization.is_some() {
            len += 1;
        }
        if !self.columns.is_empty() {
            len += 1;
        }
        if !self.references.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Model", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.file_path.is_empty() {
            struct_ser.serialize_field("filePath", &self.file_path)?;
        }
        if !self.file_sha256_hash.is_empty() {
            struct_ser.serialize_field("fileSha256Hash", &self.file_sha256_hash)?;
        }
        if let Some(v) = self.materialization.as_ref() {
            struct_ser.serialize_field("materialization", v)?;
        }
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        if !self.references.is_empty() {
            struct_ser.serialize_field("references", &self.references)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Model {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "file_path",
            "filePath",
            "file_sha256_hash",
            "fileSha256Hash",
            "materialization",
            "columns",
            "references",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            FilePath,
            FileSha256Hash,
            Materialization,
            Columns,
            References,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "filePath" | "file_path" => Ok(GeneratedField::FilePath),
                            "fileSha256Hash" | "file_sha256_hash" => Ok(GeneratedField::FileSha256Hash),
                            "materialization" => Ok(GeneratedField::Materialization),
                            "columns" => Ok(GeneratedField::Columns),
                            "references" => Ok(GeneratedField::References),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Model;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Model")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Model, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut file_path__ = None;
                let mut file_sha256_hash__ = None;
                let mut materialization__ = None;
                let mut columns__ = None;
                let mut references__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::FilePath => {
                            if file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filePath"));
                            }
                            file_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FileSha256Hash => {
                            if file_sha256_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileSha256Hash"));
                            }
                            file_sha256_hash__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Materialization => {
                            if materialization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("materialization"));
                            }
                            materialization__ = map_.next_value()?;
                        }
                        GeneratedField::Columns => {
                            if columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columns"));
                            }
                            columns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::References => {
                            if references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("references"));
                            }
                            references__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Model {
                    name: name__.unwrap_or_default(),
                    description: description__,
                    file_path: file_path__.unwrap_or_default(),
                    file_sha256_hash: file_sha256_hash__.unwrap_or_default(),
                    materialization: materialization__,
                    columns: columns__.unwrap_or_default(),
                    references: references__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Model", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for model::ModelColum {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Model.ModelColum", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for model::ModelColum {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = model::ModelColum;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Model.ModelColum")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<model::ModelColum, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                    }
                }
                Ok(model::ModelColum {
                    title: title__.unwrap_or_default(),
                    description: description__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Model.ModelColum", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ModelTest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.info.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ModelTest", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.info.is_empty() {
            struct_ser.serialize_field("info", &self.info)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ModelTest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "info",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Info,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "info" => Ok(GeneratedField::Info),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModelTest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ModelTest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ModelTest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Info => {
                            if info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            info__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(ModelTest {
                    r#type: r#type__.unwrap_or_default(),
                    info: info__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ModelTest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Node {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.is_cached {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Node", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.is_cached {
            struct_ser.serialize_field("isCached", &self.is_cached)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Node {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "is_cached",
            "isCached",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            IsCached,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "isCached" | "is_cached" => Ok(GeneratedField::IsCached),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Node;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Node")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Node, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut is_cached__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsCached => {
                            if is_cached__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isCached"));
                            }
                            is_cached__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Node {
                    id: id__.unwrap_or_default(),
                    is_cached: is_cached__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Node", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Passed {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reason.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Passed", len)?;
        if let Some(v) = self.reason.as_ref() {
            match v {
                passed::Reason::Ran(v) => {
                    struct_ser.serialize_field("ran", v)?;
                }
                passed::Reason::InferredFromTests(v) => {
                    struct_ser.serialize_field("inferredFromTests", v)?;
                }
                passed::Reason::InferredFromLogic(v) => {
                    struct_ser.serialize_field("inferredFromLogic", v)?;
                }
                passed::Reason::InferredThroughTestsOperation(v) => {
                    struct_ser.serialize_field("inferredThroughTestsOperation", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Passed {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ran",
            "inferred_from_tests",
            "inferredFromTests",
            "inferred_from_logic",
            "inferredFromLogic",
            "inferred_through_tests_operation",
            "inferredThroughTestsOperation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ran,
            InferredFromTests,
            InferredFromLogic,
            InferredThroughTestsOperation,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "ran" => Ok(GeneratedField::Ran),
                            "inferredFromTests" | "inferred_from_tests" => Ok(GeneratedField::InferredFromTests),
                            "inferredFromLogic" | "inferred_from_logic" => Ok(GeneratedField::InferredFromLogic),
                            "inferredThroughTestsOperation" | "inferred_through_tests_operation" => Ok(GeneratedField::InferredThroughTestsOperation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Passed;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Passed")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Passed, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ran => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ran"));
                            }
                            reason__ = map_.next_value::<::std::option::Option<_>>()?.map(passed::Reason::Ran)
;
                        }
                        GeneratedField::InferredFromTests => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inferredFromTests"));
                            }
                            reason__ = map_.next_value::<::std::option::Option<_>>()?.map(passed::Reason::InferredFromTests)
;
                        }
                        GeneratedField::InferredFromLogic => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inferredFromLogic"));
                            }
                            reason__ = map_.next_value::<::std::option::Option<_>>()?.map(passed::Reason::InferredFromLogic);
                        }
                        GeneratedField::InferredThroughTestsOperation => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inferredThroughTestsOperation"));
                            }
                            reason__ = map_.next_value::<::std::option::Option<_>>()?.map(passed::Reason::InferredThroughTestsOperation)
;
                        }
                    }
                }
                Ok(Passed {
                    reason: reason__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Passed", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Position {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.line != 0 {
            len += 1;
        }
        if self.character != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Position", len)?;
        if self.line != 0 {
            struct_ser.serialize_field("line", &self.line)?;
        }
        if self.character != 0 {
            struct_ser.serialize_field("character", &self.character)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Position {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "line",
            "character",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Line,
            Character,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "line" => Ok(GeneratedField::Line),
                            "character" => Ok(GeneratedField::Character),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Position;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Position")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Position, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut line__ = None;
                let mut character__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Line => {
                            if line__.is_some() {
                                return Err(serde::de::Error::duplicate_field("line"));
                            }
                            line__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Character => {
                            if character__.is_some() {
                                return Err(serde::de::Error::duplicate_field("character"));
                            }
                            character__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Position {
                    line: line__.unwrap_or_default(),
                    character: character__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Position", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Project {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.seeds.is_empty() {
            len += 1;
        }
        if !self.models.is_empty() {
            len += 1;
        }
        if !self.tests.is_empty() {
            len += 1;
        }
        if !self.sources.is_empty() {
            len += 1;
        }
        if !self.project_files.is_empty() {
            len += 1;
        }
        if self.connection_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Project", len)?;
        if !self.seeds.is_empty() {
            struct_ser.serialize_field("seeds", &self.seeds)?;
        }
        if !self.models.is_empty() {
            struct_ser.serialize_field("models", &self.models)?;
        }
        if !self.tests.is_empty() {
            struct_ser.serialize_field("tests", &self.tests)?;
        }
        if !self.sources.is_empty() {
            struct_ser.serialize_field("sources", &self.sources)?;
        }
        if !self.project_files.is_empty() {
            struct_ser.serialize_field("projectFiles", &self.project_files)?;
        }
        if let Some(v) = self.connection_config.as_ref() {
            struct_ser.serialize_field("connectionConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Project {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "seeds",
            "models",
            "tests",
            "sources",
            "project_files",
            "projectFiles",
            "connection_config",
            "connectionConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Seeds,
            Models,
            Tests,
            Sources,
            ProjectFiles,
            ConnectionConfig,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "seeds" => Ok(GeneratedField::Seeds),
                            "models" => Ok(GeneratedField::Models),
                            "tests" => Ok(GeneratedField::Tests),
                            "sources" => Ok(GeneratedField::Sources),
                            "projectFiles" | "project_files" => Ok(GeneratedField::ProjectFiles),
                            "connectionConfig" | "connection_config" => Ok(GeneratedField::ConnectionConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Project;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Project")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Project, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut seeds__ = None;
                let mut models__ = None;
                let mut tests__ = None;
                let mut sources__ = None;
                let mut project_files__ = None;
                let mut connection_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Seeds => {
                            if seeds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seeds"));
                            }
                            seeds__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Models => {
                            if models__.is_some() {
                                return Err(serde::de::Error::duplicate_field("models"));
                            }
                            models__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Tests => {
                            if tests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tests"));
                            }
                            tests__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Sources => {
                            if sources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sources"));
                            }
                            sources__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::ProjectFiles => {
                            if project_files__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectFiles"));
                            }
                            project_files__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::ConnectionConfig => {
                            if connection_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionConfig"));
                            }
                            connection_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Project {
                    seeds: seeds__.unwrap_or_default(),
                    models: models__.unwrap_or_default(),
                    tests: tests__.unwrap_or_default(),
                    sources: sources__.unwrap_or_default(),
                    project_files: project_files__.unwrap_or_default(),
                    connection_config: connection_config__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Project", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectDag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.nodes.is_empty() {
            len += 1;
        }
        if !self.edges.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ProjectDag", len)?;
        if !self.nodes.is_empty() {
            struct_ser.serialize_field("nodes", &self.nodes)?;
        }
        if !self.edges.is_empty() {
            struct_ser.serialize_field("edges", &self.edges)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectDag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nodes",
            "edges",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nodes,
            Edges,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "nodes" => Ok(GeneratedField::Nodes),
                            "edges" => Ok(GeneratedField::Edges),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectDag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ProjectDag")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProjectDag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut nodes__ = None;
                let mut edges__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nodes => {
                            if nodes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodes"));
                            }
                            nodes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Edges => {
                            if edges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("edges"));
                            }
                            edges__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProjectDag {
                    nodes: nodes__.unwrap_or_default(),
                    edges: edges__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ProjectDag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectFile {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sources.is_empty() {
            len += 1;
        }
        if !self.models.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ProjectFile", len)?;
        if !self.sources.is_empty() {
            struct_ser.serialize_field("sources", &self.sources)?;
        }
        if !self.models.is_empty() {
            struct_ser.serialize_field("models", &self.models)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectFile {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sources",
            "models",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sources,
            Models,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sources" => Ok(GeneratedField::Sources),
                            "models" => Ok(GeneratedField::Models),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectFile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ProjectFile")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProjectFile, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sources__ = None;
                let mut models__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sources => {
                            if sources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sources"));
                            }
                            sources__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Models => {
                            if models__.is_some() {
                                return Err(serde::de::Error::duplicate_field("models"));
                            }
                            models__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProjectFile {
                    sources: sources__.unwrap_or_default(),
                    models: models__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ProjectFile", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for project_file::Column {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if !self.tests.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ProjectFile.Column", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.tests.is_empty() {
            struct_ser.serialize_field("tests", &self.tests)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for project_file::Column {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "tests",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Tests,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "tests" => Ok(GeneratedField::Tests),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = project_file::Column;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ProjectFile.Column")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<project_file::Column, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut tests__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Tests => {
                            if tests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tests"));
                            }
                            tests__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(project_file::Column {
                    name: name__.unwrap_or_default(),
                    description: description__,
                    tests: tests__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ProjectFile.Column", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for project_file::Model {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if !self.tests.is_empty() {
            len += 1;
        }
        if !self.columns.is_empty() {
            len += 1;
        }
        if self.materialization.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ProjectFile.Model", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.tests.is_empty() {
            struct_ser.serialize_field("tests", &self.tests)?;
        }
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        if let Some(v) = self.materialization.as_ref() {
            struct_ser.serialize_field("materialization", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for project_file::Model {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "tests",
            "columns",
            "materialization",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Tests,
            Columns,
            Materialization,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "tests" => Ok(GeneratedField::Tests),
                            "columns" => Ok(GeneratedField::Columns),
                            "materialization" => Ok(GeneratedField::Materialization),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = project_file::Model;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ProjectFile.Model")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<project_file::Model, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut tests__ = None;
                let mut columns__ = None;
                let mut materialization__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Tests => {
                            if tests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tests"));
                            }
                            tests__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Columns => {
                            if columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columns"));
                            }
                            columns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Materialization => {
                            if materialization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("materialization"));
                            }
                            materialization__ = map_.next_value()?;
                        }
                    }
                }
                Ok(project_file::Model {
                    name: name__.unwrap_or_default(),
                    description: description__,
                    tests: tests__.unwrap_or_default(),
                    columns: columns__.unwrap_or_default(),
                    materialization: materialization__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ProjectFile.Model", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for project_file::Source {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.tests.is_empty() {
            len += 1;
        }
        if !self.columns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ProjectFile.Source", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.tests.is_empty() {
            struct_ser.serialize_field("tests", &self.tests)?;
        }
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for project_file::Source {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "path",
            "tests",
            "columns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Path,
            Tests,
            Columns,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "path" => Ok(GeneratedField::Path),
                            "tests" => Ok(GeneratedField::Tests),
                            "columns" => Ok(GeneratedField::Columns),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = project_file::Source;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ProjectFile.Source")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<project_file::Source, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut path__ = None;
                let mut tests__ = None;
                let mut columns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tests => {
                            if tests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tests"));
                            }
                            tests__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Columns => {
                            if columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columns"));
                            }
                            columns__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(project_file::Source {
                    name: name__.unwrap_or_default(),
                    description: description__,
                    path: path__.unwrap_or_default(),
                    tests: tests__.unwrap_or_default(),
                    columns: columns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ProjectFile.Source", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.columns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.QueryResult", len)?;
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "columns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Columns,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "columns" => Ok(GeneratedField::Columns),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.QueryResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut columns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Columns => {
                            if columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columns"));
                            }
                            columns__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryResult {
                    columns: columns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.QueryResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryResultColumn {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.r#type.is_some() {
            len += 1;
        }
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.QueryResultColumn", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            struct_ser.serialize_field("type", v)?;
        }
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryResultColumn {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "type",
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Type,
            Values,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "type" => Ok(GeneratedField::Type),
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryResultColumn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.QueryResultColumn")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryResultColumn, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut r#type__ = None;
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = map_.next_value()?;
                        }
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryResultColumn {
                    name: name__.unwrap_or_default(),
                    r#type: r#type__,
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.QueryResultColumn", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Range {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start.is_some() {
            len += 1;
        }
        if self.end.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Range", len)?;
        if let Some(v) = self.start.as_ref() {
            struct_ser.serialize_field("start", v)?;
        }
        if let Some(v) = self.end.as_ref() {
            struct_ser.serialize_field("end", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Range {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start",
            "end",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Start,
            End,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Range;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Range")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Range, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start__ = None;
                let mut end__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = map_.next_value()?;
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Range {
                    start: start__,
                    end: end__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Range", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Row {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.tests.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Row", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.tests.is_empty() {
            struct_ser.serialize_field("tests", &self.tests)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Row {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "tests",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Tests,
            Description,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "title" => Ok(GeneratedField::Title),
                            "tests" => Ok(GeneratedField::Tests),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Row;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Row")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Row, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut tests__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tests => {
                            if tests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tests"));
                            }
                            tests__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Row {
                    title: title__.unwrap_or_default(),
                    tests: tests__.unwrap_or_default(),
                    description: description__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Row", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RowDescription {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.description.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.RowDescription", len)?;
        if let Some(v) = self.description.as_ref() {
            match v {
                row_description::Description::Present(v) => {
                    struct_ser.serialize_field("present", v)?;
                }
                row_description::Description::PresentAndInferredIdentical(v) => {
                    struct_ser.serialize_field("presentAndInferredIdentical", v)?;
                }
                row_description::Description::PresentWithDifferentInference(v) => {
                    struct_ser.serialize_field("presentWithDifferentInference", v)?;
                }
                row_description::Description::Inferred(v) => {
                    struct_ser.serialize_field("inferred", v)?;
                }
                row_description::Description::NotPresent(v) => {
                    struct_ser.serialize_field("notPresent", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RowDescription {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "present",
            "present_and_inferred_identical",
            "presentAndInferredIdentical",
            "present_with_different_inference",
            "presentWithDifferentInference",
            "inferred",
            "not_present",
            "notPresent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Present,
            PresentAndInferredIdentical,
            PresentWithDifferentInference,
            Inferred,
            NotPresent,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "present" => Ok(GeneratedField::Present),
                            "presentAndInferredIdentical" | "present_and_inferred_identical" => Ok(GeneratedField::PresentAndInferredIdentical),
                            "presentWithDifferentInference" | "present_with_different_inference" => Ok(GeneratedField::PresentWithDifferentInference),
                            "inferred" => Ok(GeneratedField::Inferred),
                            "notPresent" | "not_present" => Ok(GeneratedField::NotPresent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RowDescription;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.RowDescription")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RowDescription, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Present => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("present"));
                            }
                            description__ = map_.next_value::<::std::option::Option<_>>()?.map(row_description::Description::Present);
                        }
                        GeneratedField::PresentAndInferredIdentical => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presentAndInferredIdentical"));
                            }
                            description__ = map_.next_value::<::std::option::Option<_>>()?.map(row_description::Description::PresentAndInferredIdentical);
                        }
                        GeneratedField::PresentWithDifferentInference => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presentWithDifferentInference"));
                            }
                            description__ = map_.next_value::<::std::option::Option<_>>()?.map(row_description::Description::PresentWithDifferentInference)
;
                        }
                        GeneratedField::Inferred => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inferred"));
                            }
                            description__ = map_.next_value::<::std::option::Option<_>>()?.map(row_description::Description::Inferred);
                        }
                        GeneratedField::NotPresent => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notPresent"));
                            }
                            description__ = map_.next_value::<::std::option::Option<_>>()?.map(row_description::Description::NotPresent)
;
                        }
                    }
                }
                Ok(RowDescription {
                    description: description__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.RowDescription", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for row_description::PresentWithInference {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.present.is_empty() {
            len += 1;
        }
        if !self.inferred.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.RowDescription.PresentWithInference", len)?;
        if !self.present.is_empty() {
            struct_ser.serialize_field("present", &self.present)?;
        }
        if !self.inferred.is_empty() {
            struct_ser.serialize_field("inferred", &self.inferred)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for row_description::PresentWithInference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "present",
            "inferred",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Present,
            Inferred,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "present" => Ok(GeneratedField::Present),
                            "inferred" => Ok(GeneratedField::Inferred),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = row_description::PresentWithInference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.RowDescription.PresentWithInference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<row_description::PresentWithInference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut present__ = None;
                let mut inferred__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Present => {
                            if present__.is_some() {
                                return Err(serde::de::Error::duplicate_field("present"));
                            }
                            present__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Inferred => {
                            if inferred__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inferred"));
                            }
                            inferred__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(row_description::PresentWithInference {
                    present: present__.unwrap_or_default(),
                    inferred: inferred__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.RowDescription.PresentWithInference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RowTest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.test.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.RowTest", len)?;
        if let Some(v) = self.test.as_ref() {
            match v {
                row_test::Test::PresentAndNotInferred(v) => {
                    struct_ser.serialize_field("presentAndNotInferred", v)?;
                }
                row_test::Test::PresentAndInferred(v) => {
                    struct_ser.serialize_field("presentAndInferred", v)?;
                }
                row_test::Test::NotPresentButInferred(v) => {
                    struct_ser.serialize_field("notPresentButInferred", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RowTest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "present_and_not_inferred",
            "presentAndNotInferred",
            "present_and_inferred",
            "presentAndInferred",
            "not_present_but_inferred",
            "notPresentButInferred",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PresentAndNotInferred,
            PresentAndInferred,
            NotPresentButInferred,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "presentAndNotInferred" | "present_and_not_inferred" => Ok(GeneratedField::PresentAndNotInferred),
                            "presentAndInferred" | "present_and_inferred" => Ok(GeneratedField::PresentAndInferred),
                            "notPresentButInferred" | "not_present_but_inferred" => Ok(GeneratedField::NotPresentButInferred),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RowTest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.RowTest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RowTest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PresentAndNotInferred => {
                            if test__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presentAndNotInferred"));
                            }
                            test__ = map_.next_value::<::std::option::Option<_>>()?.map(row_test::Test::PresentAndNotInferred)
;
                        }
                        GeneratedField::PresentAndInferred => {
                            if test__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presentAndInferred"));
                            }
                            test__ = map_.next_value::<::std::option::Option<_>>()?.map(row_test::Test::PresentAndInferred)
;
                        }
                        GeneratedField::NotPresentButInferred => {
                            if test__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notPresentButInferred"));
                            }
                            test__ = map_.next_value::<::std::option::Option<_>>()?.map(row_test::Test::NotPresentButInferred)
;
                        }
                    }
                }
                Ok(RowTest {
                    test: test__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.RowTest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RowTestDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.text.is_empty() {
            len += 1;
        }
        if self.column_test.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.RowTestDetails", len)?;
        if !self.text.is_empty() {
            struct_ser.serialize_field("text", &self.text)?;
        }
        if let Some(v) = self.column_test.as_ref() {
            struct_ser.serialize_field("columnTest", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RowTestDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "text",
            "column_test",
            "columnTest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Text,
            ColumnTest,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "text" => Ok(GeneratedField::Text),
                            "columnTest" | "column_test" => Ok(GeneratedField::ColumnTest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RowTestDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.RowTestDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RowTestDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut text__ = None;
                let mut column_test__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Text => {
                            if text__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            text__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ColumnTest => {
                            if column_test__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnTest"));
                            }
                            column_test__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RowTestDetails {
                    text: text__.unwrap_or_default(),
                    column_test: column_test__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.RowTestDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Seed {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.file_path.is_empty() {
            len += 1;
        }
        if !self.file_sha256_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Seed", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.file_path.is_empty() {
            struct_ser.serialize_field("filePath", &self.file_path)?;
        }
        if !self.file_sha256_hash.is_empty() {
            struct_ser.serialize_field("fileSha256Hash", &self.file_sha256_hash)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Seed {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "file_path",
            "filePath",
            "file_sha256_hash",
            "fileSha256Hash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            FilePath,
            FileSha256Hash,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "filePath" | "file_path" => Ok(GeneratedField::FilePath),
                            "fileSha256Hash" | "file_sha256_hash" => Ok(GeneratedField::FileSha256Hash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Seed;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Seed")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Seed, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut file_path__ = None;
                let mut file_sha256_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FilePath => {
                            if file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filePath"));
                            }
                            file_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FileSha256Hash => {
                            if file_sha256_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileSha256Hash"));
                            }
                            file_sha256_hash__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Seed {
                    name: name__.unwrap_or_default(),
                    file_path: file_path__.unwrap_or_default(),
                    file_sha256_hash: file_sha256_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Seed", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Source {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.file_path.is_empty() {
            len += 1;
        }
        if !self.columns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Source", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.file_path.is_empty() {
            struct_ser.serialize_field("filePath", &self.file_path)?;
        }
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Source {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "path",
            "file_path",
            "filePath",
            "columns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Path,
            FilePath,
            Columns,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "path" => Ok(GeneratedField::Path),
                            "filePath" | "file_path" => Ok(GeneratedField::FilePath),
                            "columns" => Ok(GeneratedField::Columns),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Source;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Source")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Source, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut path__ = None;
                let mut file_path__ = None;
                let mut columns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FilePath => {
                            if file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filePath"));
                            }
                            file_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Columns => {
                            if columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columns"));
                            }
                            columns__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Source {
                    name: name__.unwrap_or_default(),
                    description: description__,
                    path: path__.unwrap_or_default(),
                    file_path: file_path__.unwrap_or_default(),
                    columns: columns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Source", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for source::SourceColumn {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Source.SourceColumn", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for source::SourceColumn {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = source::SourceColumn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Source.SourceColumn")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<source::SourceColumn, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                    }
                }
                Ok(source::SourceColumn {
                    title: title__.unwrap_or_default(),
                    description: description__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Source.SourceColumn", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Table {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.table_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Table", len)?;
        if let Some(v) = self.table_type.as_ref() {
            match v {
                table::TableType::Present(v) => {
                    struct_ser.serialize_field("present", v)?;
                }
                table::TableType::NotPresent(v) => {
                    struct_ser.serialize_field("notPresent", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Table {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "present",
            "not_present",
            "notPresent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Present,
            NotPresent,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "present" => Ok(GeneratedField::Present),
                            "notPresent" | "not_present" => Ok(GeneratedField::NotPresent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Table;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Table")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Table, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Present => {
                            if table_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("present"));
                            }
                            table_type__ = map_.next_value::<::std::option::Option<_>>()?.map(table::TableType::Present)
;
                        }
                        GeneratedField::NotPresent => {
                            if table_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notPresent"));
                            }
                            table_type__ = map_.next_value::<::std::option::Option<_>>()?.map(table::TableType::NotPresent)
;
                        }
                    }
                }
                Ok(Table {
                    table_type: table_type__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Table", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for table::NotPresentInSchema {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rows.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Table.NotPresentInSchema", len)?;
        if !self.rows.is_empty() {
            struct_ser.serialize_field("rows", &self.rows)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for table::NotPresentInSchema {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rows",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rows,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rows" => Ok(GeneratedField::Rows),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = table::NotPresentInSchema;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Table.NotPresentInSchema")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<table::NotPresentInSchema, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rows__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rows => {
                            if rows__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rows"));
                            }
                            rows__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(table::NotPresentInSchema {
                    rows: rows__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Table.NotPresentInSchema", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for table::PresentInSchema {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rows.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Table.PresentInSchema", len)?;
        if !self.rows.is_empty() {
            struct_ser.serialize_field("rows", &self.rows)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for table::PresentInSchema {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rows",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rows,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rows" => Ok(GeneratedField::Rows),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = table::PresentInSchema;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Table.PresentInSchema")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<table::PresentInSchema, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rows__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rows => {
                            if rows__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rows"));
                            }
                            rows__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(table::PresentInSchema {
                    rows: rows__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Table.PresentInSchema", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for table::present_in_schema::PresentRow {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.row.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Table.PresentInSchema.PresentRow", len)?;
        if let Some(v) = self.row.as_ref() {
            match v {
                table::present_in_schema::present_row::Row::PresentInSqlAndDefinitions(v) => {
                    struct_ser.serialize_field("presentInSqlAndDefinitions", v)?;
                }
                table::present_in_schema::present_row::Row::MissingInDefinitions(v) => {
                    struct_ser.serialize_field("missingInDefinitions", v)?;
                }
                table::present_in_schema::present_row::Row::PresentInDefinitionsButNotRecognisableInSql(v) => {
                    struct_ser.serialize_field("presentInDefinitionsButNotRecognisableInSql", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for table::present_in_schema::PresentRow {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "present_in_sql_and_definitions",
            "presentInSqlAndDefinitions",
            "missing_in_definitions",
            "missingInDefinitions",
            "present_in_definitions_but_not_recognisable_in_sql",
            "presentInDefinitionsButNotRecognisableInSql",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PresentInSqlAndDefinitions,
            MissingInDefinitions,
            PresentInDefinitionsButNotRecognisableInSql,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "presentInSqlAndDefinitions" | "present_in_sql_and_definitions" => Ok(GeneratedField::PresentInSqlAndDefinitions),
                            "missingInDefinitions" | "missing_in_definitions" => Ok(GeneratedField::MissingInDefinitions),
                            "presentInDefinitionsButNotRecognisableInSql" | "present_in_definitions_but_not_recognisable_in_sql" => Ok(GeneratedField::PresentInDefinitionsButNotRecognisableInSql),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = table::present_in_schema::PresentRow;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Table.PresentInSchema.PresentRow")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<table::present_in_schema::PresentRow, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut row__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PresentInSqlAndDefinitions => {
                            if row__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presentInSqlAndDefinitions"));
                            }
                            row__ = map_.next_value::<::std::option::Option<_>>()?.map(table::present_in_schema::present_row::Row::PresentInSqlAndDefinitions)
;
                        }
                        GeneratedField::MissingInDefinitions => {
                            if row__.is_some() {
                                return Err(serde::de::Error::duplicate_field("missingInDefinitions"));
                            }
                            row__ = map_.next_value::<::std::option::Option<_>>()?.map(table::present_in_schema::present_row::Row::MissingInDefinitions)
;
                        }
                        GeneratedField::PresentInDefinitionsButNotRecognisableInSql => {
                            if row__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presentInDefinitionsButNotRecognisableInSql"));
                            }
                            row__ = map_.next_value::<::std::option::Option<_>>()?.map(table::present_in_schema::present_row::Row::PresentInDefinitionsButNotRecognisableInSql)
;
                        }
                    }
                }
                Ok(table::present_in_schema::PresentRow {
                    row: row__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Table.PresentInSchema.PresentRow", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Test {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.test_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Test", len)?;
        if let Some(v) = self.test_type.as_ref() {
            match v {
                test::TestType::Sql(v) => {
                    struct_ser.serialize_field("sql", v)?;
                }
                test::TestType::Unique(v) => {
                    struct_ser.serialize_field("unique", v)?;
                }
                test::TestType::NotNull(v) => {
                    struct_ser.serialize_field("notNull", v)?;
                }
                test::TestType::Relationship(v) => {
                    struct_ser.serialize_field("relationship", v)?;
                }
                test::TestType::AcceptedValues(v) => {
                    struct_ser.serialize_field("acceptedValues", v)?;
                }
                test::TestType::GreaterThanOrEqual(v) => {
                    struct_ser.serialize_field("greaterThanOrEqual", v)?;
                }
                test::TestType::LessThanOrEqual(v) => {
                    struct_ser.serialize_field("lessThanOrEqual", v)?;
                }
                test::TestType::GreaterThan(v) => {
                    struct_ser.serialize_field("greaterThan", v)?;
                }
                test::TestType::LessThan(v) => {
                    struct_ser.serialize_field("lessThan", v)?;
                }
                test::TestType::MultiColumnUnique(v) => {
                    struct_ser.serialize_field("multiColumnUnique", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Test {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sql",
            "unique",
            "not_null",
            "notNull",
            "relationship",
            "accepted_values",
            "acceptedValues",
            "greater_than_or_equal",
            "greaterThanOrEqual",
            "less_than_or_equal",
            "lessThanOrEqual",
            "greater_than",
            "greaterThan",
            "less_than",
            "lessThan",
            "multi_column_unique",
            "multiColumnUnique",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sql,
            Unique,
            NotNull,
            Relationship,
            AcceptedValues,
            GreaterThanOrEqual,
            LessThanOrEqual,
            GreaterThan,
            LessThan,
            MultiColumnUnique,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sql" => Ok(GeneratedField::Sql),
                            "unique" => Ok(GeneratedField::Unique),
                            "notNull" | "not_null" => Ok(GeneratedField::NotNull),
                            "relationship" => Ok(GeneratedField::Relationship),
                            "acceptedValues" | "accepted_values" => Ok(GeneratedField::AcceptedValues),
                            "greaterThanOrEqual" | "greater_than_or_equal" => Ok(GeneratedField::GreaterThanOrEqual),
                            "lessThanOrEqual" | "less_than_or_equal" => Ok(GeneratedField::LessThanOrEqual),
                            "greaterThan" | "greater_than" => Ok(GeneratedField::GreaterThan),
                            "lessThan" | "less_than" => Ok(GeneratedField::LessThan),
                            "multiColumnUnique" | "multi_column_unique" => Ok(GeneratedField::MultiColumnUnique),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Test;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Test")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Test, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sql => {
                            if test_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sql"));
                            }
                            test_type__ = map_.next_value::<::std::option::Option<_>>()?.map(test::TestType::Sql)
;
                        }
                        GeneratedField::Unique => {
                            if test_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unique"));
                            }
                            test_type__ = map_.next_value::<::std::option::Option<_>>()?.map(test::TestType::Unique)
;
                        }
                        GeneratedField::NotNull => {
                            if test_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notNull"));
                            }
                            test_type__ = map_.next_value::<::std::option::Option<_>>()?.map(test::TestType::NotNull)
;
                        }
                        GeneratedField::Relationship => {
                            if test_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationship"));
                            }
                            test_type__ = map_.next_value::<::std::option::Option<_>>()?.map(test::TestType::Relationship)
;
                        }
                        GeneratedField::AcceptedValues => {
                            if test_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("acceptedValues"));
                            }
                            test_type__ = map_.next_value::<::std::option::Option<_>>()?.map(test::TestType::AcceptedValues)
;
                        }
                        GeneratedField::GreaterThanOrEqual => {
                            if test_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("greaterThanOrEqual"));
                            }
                            test_type__ = map_.next_value::<::std::option::Option<_>>()?.map(test::TestType::GreaterThanOrEqual)
;
                        }
                        GeneratedField::LessThanOrEqual => {
                            if test_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lessThanOrEqual"));
                            }
                            test_type__ = map_.next_value::<::std::option::Option<_>>()?.map(test::TestType::LessThanOrEqual)
;
                        }
                        GeneratedField::GreaterThan => {
                            if test_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("greaterThan"));
                            }
                            test_type__ = map_.next_value::<::std::option::Option<_>>()?.map(test::TestType::GreaterThan)
;
                        }
                        GeneratedField::LessThan => {
                            if test_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lessThan"));
                            }
                            test_type__ = map_.next_value::<::std::option::Option<_>>()?.map(test::TestType::LessThan)
;
                        }
                        GeneratedField::MultiColumnUnique => {
                            if test_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multiColumnUnique"));
                            }
                            test_type__ = map_.next_value::<::std::option::Option<_>>()?.map(test::TestType::MultiColumnUnique)
;
                        }
                    }
                }
                Ok(Test {
                    test_type: test_type__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Test", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestAcceptedValues {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_path.is_empty() {
            len += 1;
        }
        if !self.model.is_empty() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.column.is_empty() {
            len += 1;
        }
        if !self.accepted_values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.TestAcceptedValues", len)?;
        if !self.file_path.is_empty() {
            struct_ser.serialize_field("filePath", &self.file_path)?;
        }
        if !self.model.is_empty() {
            struct_ser.serialize_field("model", &self.model)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.column.is_empty() {
            struct_ser.serialize_field("column", &self.column)?;
        }
        if !self.accepted_values.is_empty() {
            struct_ser.serialize_field("acceptedValues", &self.accepted_values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestAcceptedValues {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_path",
            "filePath",
            "model",
            "path",
            "column",
            "accepted_values",
            "acceptedValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilePath,
            Model,
            Path,
            Column,
            AcceptedValues,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "filePath" | "file_path" => Ok(GeneratedField::FilePath),
                            "model" => Ok(GeneratedField::Model),
                            "path" => Ok(GeneratedField::Path),
                            "column" => Ok(GeneratedField::Column),
                            "acceptedValues" | "accepted_values" => Ok(GeneratedField::AcceptedValues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestAcceptedValues;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.TestAcceptedValues")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestAcceptedValues, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_path__ = None;
                let mut model__ = None;
                let mut path__ = None;
                let mut column__ = None;
                let mut accepted_values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FilePath => {
                            if file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filePath"));
                            }
                            file_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Model => {
                            if model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("model"));
                            }
                            model__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Column => {
                            if column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("column"));
                            }
                            column__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AcceptedValues => {
                            if accepted_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("acceptedValues"));
                            }
                            accepted_values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TestAcceptedValues {
                    file_path: file_path__.unwrap_or_default(),
                    model: model__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                    column: column__.unwrap_or_default(),
                    accepted_values: accepted_values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.TestAcceptedValues", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestGreaterThan {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_path.is_empty() {
            len += 1;
        }
        if !self.model.is_empty() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.column.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.TestGreaterThan", len)?;
        if !self.file_path.is_empty() {
            struct_ser.serialize_field("filePath", &self.file_path)?;
        }
        if !self.model.is_empty() {
            struct_ser.serialize_field("model", &self.model)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.column.is_empty() {
            struct_ser.serialize_field("column", &self.column)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestGreaterThan {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_path",
            "filePath",
            "model",
            "path",
            "column",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilePath,
            Model,
            Path,
            Column,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "filePath" | "file_path" => Ok(GeneratedField::FilePath),
                            "model" => Ok(GeneratedField::Model),
                            "path" => Ok(GeneratedField::Path),
                            "column" => Ok(GeneratedField::Column),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestGreaterThan;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.TestGreaterThan")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestGreaterThan, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_path__ = None;
                let mut model__ = None;
                let mut path__ = None;
                let mut column__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FilePath => {
                            if file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filePath"));
                            }
                            file_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Model => {
                            if model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("model"));
                            }
                            model__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Column => {
                            if column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("column"));
                            }
                            column__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TestGreaterThan {
                    file_path: file_path__.unwrap_or_default(),
                    model: model__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                    column: column__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.TestGreaterThan", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestGreaterThanOrEqual {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_path.is_empty() {
            len += 1;
        }
        if !self.model.is_empty() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.column.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.TestGreaterThanOrEqual", len)?;
        if !self.file_path.is_empty() {
            struct_ser.serialize_field("filePath", &self.file_path)?;
        }
        if !self.model.is_empty() {
            struct_ser.serialize_field("model", &self.model)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.column.is_empty() {
            struct_ser.serialize_field("column", &self.column)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestGreaterThanOrEqual {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_path",
            "filePath",
            "model",
            "path",
            "column",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilePath,
            Model,
            Path,
            Column,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "filePath" | "file_path" => Ok(GeneratedField::FilePath),
                            "model" => Ok(GeneratedField::Model),
                            "path" => Ok(GeneratedField::Path),
                            "column" => Ok(GeneratedField::Column),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestGreaterThanOrEqual;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.TestGreaterThanOrEqual")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestGreaterThanOrEqual, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_path__ = None;
                let mut model__ = None;
                let mut path__ = None;
                let mut column__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FilePath => {
                            if file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filePath"));
                            }
                            file_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Model => {
                            if model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("model"));
                            }
                            model__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Column => {
                            if column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("column"));
                            }
                            column__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TestGreaterThanOrEqual {
                    file_path: file_path__.unwrap_or_default(),
                    model: model__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                    column: column__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.TestGreaterThanOrEqual", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestLessThan {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_path.is_empty() {
            len += 1;
        }
        if !self.model.is_empty() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.column.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.TestLessThan", len)?;
        if !self.file_path.is_empty() {
            struct_ser.serialize_field("filePath", &self.file_path)?;
        }
        if !self.model.is_empty() {
            struct_ser.serialize_field("model", &self.model)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.column.is_empty() {
            struct_ser.serialize_field("column", &self.column)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestLessThan {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_path",
            "filePath",
            "model",
            "path",
            "column",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilePath,
            Model,
            Path,
            Column,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "filePath" | "file_path" => Ok(GeneratedField::FilePath),
                            "model" => Ok(GeneratedField::Model),
                            "path" => Ok(GeneratedField::Path),
                            "column" => Ok(GeneratedField::Column),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestLessThan;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.TestLessThan")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestLessThan, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_path__ = None;
                let mut model__ = None;
                let mut path__ = None;
                let mut column__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FilePath => {
                            if file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filePath"));
                            }
                            file_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Model => {
                            if model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("model"));
                            }
                            model__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Column => {
                            if column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("column"));
                            }
                            column__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TestLessThan {
                    file_path: file_path__.unwrap_or_default(),
                    model: model__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                    column: column__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.TestLessThan", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestLessThanOrEqual {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_path.is_empty() {
            len += 1;
        }
        if !self.model.is_empty() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.column.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.TestLessThanOrEqual", len)?;
        if !self.file_path.is_empty() {
            struct_ser.serialize_field("filePath", &self.file_path)?;
        }
        if !self.model.is_empty() {
            struct_ser.serialize_field("model", &self.model)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.column.is_empty() {
            struct_ser.serialize_field("column", &self.column)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestLessThanOrEqual {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_path",
            "filePath",
            "model",
            "path",
            "column",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilePath,
            Model,
            Path,
            Column,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "filePath" | "file_path" => Ok(GeneratedField::FilePath),
                            "model" => Ok(GeneratedField::Model),
                            "path" => Ok(GeneratedField::Path),
                            "column" => Ok(GeneratedField::Column),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestLessThanOrEqual;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.TestLessThanOrEqual")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestLessThanOrEqual, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_path__ = None;
                let mut model__ = None;
                let mut path__ = None;
                let mut column__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FilePath => {
                            if file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filePath"));
                            }
                            file_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Model => {
                            if model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("model"));
                            }
                            model__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Column => {
                            if column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("column"));
                            }
                            column__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TestLessThanOrEqual {
                    file_path: file_path__.unwrap_or_default(),
                    model: model__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                    column: column__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.TestLessThanOrEqual", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestMultiColumnUnique {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_path.is_empty() {
            len += 1;
        }
        if !self.model.is_empty() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.columns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.TestMultiColumnUnique", len)?;
        if !self.file_path.is_empty() {
            struct_ser.serialize_field("filePath", &self.file_path)?;
        }
        if !self.model.is_empty() {
            struct_ser.serialize_field("model", &self.model)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestMultiColumnUnique {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_path",
            "filePath",
            "model",
            "path",
            "columns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilePath,
            Model,
            Path,
            Columns,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "filePath" | "file_path" => Ok(GeneratedField::FilePath),
                            "model" => Ok(GeneratedField::Model),
                            "path" => Ok(GeneratedField::Path),
                            "columns" => Ok(GeneratedField::Columns),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestMultiColumnUnique;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.TestMultiColumnUnique")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestMultiColumnUnique, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_path__ = None;
                let mut model__ = None;
                let mut path__ = None;
                let mut columns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FilePath => {
                            if file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filePath"));
                            }
                            file_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Model => {
                            if model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("model"));
                            }
                            model__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Columns => {
                            if columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columns"));
                            }
                            columns__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TestMultiColumnUnique {
                    file_path: file_path__.unwrap_or_default(),
                    model: model__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                    columns: columns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.TestMultiColumnUnique", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestNotNull {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_path.is_empty() {
            len += 1;
        }
        if !self.model.is_empty() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.column.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.TestNotNull", len)?;
        if !self.file_path.is_empty() {
            struct_ser.serialize_field("filePath", &self.file_path)?;
        }
        if !self.model.is_empty() {
            struct_ser.serialize_field("model", &self.model)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.column.is_empty() {
            struct_ser.serialize_field("column", &self.column)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestNotNull {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_path",
            "filePath",
            "model",
            "path",
            "column",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilePath,
            Model,
            Path,
            Column,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "filePath" | "file_path" => Ok(GeneratedField::FilePath),
                            "model" => Ok(GeneratedField::Model),
                            "path" => Ok(GeneratedField::Path),
                            "column" => Ok(GeneratedField::Column),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestNotNull;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.TestNotNull")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestNotNull, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_path__ = None;
                let mut model__ = None;
                let mut path__ = None;
                let mut column__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FilePath => {
                            if file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filePath"));
                            }
                            file_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Model => {
                            if model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("model"));
                            }
                            model__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Column => {
                            if column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("column"));
                            }
                            column__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TestNotNull {
                    file_path: file_path__.unwrap_or_default(),
                    model: model__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                    column: column__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.TestNotNull", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestRelationship {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_path.is_empty() {
            len += 1;
        }
        if !self.source_model.is_empty() {
            len += 1;
        }
        if !self.source_path.is_empty() {
            len += 1;
        }
        if !self.source_column.is_empty() {
            len += 1;
        }
        if !self.target_model.is_empty() {
            len += 1;
        }
        if !self.target_path.is_empty() {
            len += 1;
        }
        if !self.target_column.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.TestRelationship", len)?;
        if !self.file_path.is_empty() {
            struct_ser.serialize_field("filePath", &self.file_path)?;
        }
        if !self.source_model.is_empty() {
            struct_ser.serialize_field("sourceModel", &self.source_model)?;
        }
        if !self.source_path.is_empty() {
            struct_ser.serialize_field("sourcePath", &self.source_path)?;
        }
        if !self.source_column.is_empty() {
            struct_ser.serialize_field("sourceColumn", &self.source_column)?;
        }
        if !self.target_model.is_empty() {
            struct_ser.serialize_field("targetModel", &self.target_model)?;
        }
        if !self.target_path.is_empty() {
            struct_ser.serialize_field("targetPath", &self.target_path)?;
        }
        if !self.target_column.is_empty() {
            struct_ser.serialize_field("targetColumn", &self.target_column)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestRelationship {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_path",
            "filePath",
            "source_model",
            "sourceModel",
            "source_path",
            "sourcePath",
            "source_column",
            "sourceColumn",
            "target_model",
            "targetModel",
            "target_path",
            "targetPath",
            "target_column",
            "targetColumn",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilePath,
            SourceModel,
            SourcePath,
            SourceColumn,
            TargetModel,
            TargetPath,
            TargetColumn,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "filePath" | "file_path" => Ok(GeneratedField::FilePath),
                            "sourceModel" | "source_model" => Ok(GeneratedField::SourceModel),
                            "sourcePath" | "source_path" => Ok(GeneratedField::SourcePath),
                            "sourceColumn" | "source_column" => Ok(GeneratedField::SourceColumn),
                            "targetModel" | "target_model" => Ok(GeneratedField::TargetModel),
                            "targetPath" | "target_path" => Ok(GeneratedField::TargetPath),
                            "targetColumn" | "target_column" => Ok(GeneratedField::TargetColumn),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestRelationship;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.TestRelationship")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestRelationship, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_path__ = None;
                let mut source_model__ = None;
                let mut source_path__ = None;
                let mut source_column__ = None;
                let mut target_model__ = None;
                let mut target_path__ = None;
                let mut target_column__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FilePath => {
                            if file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filePath"));
                            }
                            file_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourceModel => {
                            if source_model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceModel"));
                            }
                            source_model__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourcePath => {
                            if source_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourcePath"));
                            }
                            source_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourceColumn => {
                            if source_column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceColumn"));
                            }
                            source_column__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TargetModel => {
                            if target_model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetModel"));
                            }
                            target_model__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TargetPath => {
                            if target_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetPath"));
                            }
                            target_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TargetColumn => {
                            if target_column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetColumn"));
                            }
                            target_column__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TestRelationship {
                    file_path: file_path__.unwrap_or_default(),
                    source_model: source_model__.unwrap_or_default(),
                    source_path: source_path__.unwrap_or_default(),
                    source_column: source_column__.unwrap_or_default(),
                    target_model: target_model__.unwrap_or_default(),
                    target_path: target_path__.unwrap_or_default(),
                    target_column: target_column__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.TestRelationship", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.test_name.is_empty() {
            len += 1;
        }
        if !self.query.is_empty() {
            len += 1;
        }
        if self.test_result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.TestResult", len)?;
        if !self.test_name.is_empty() {
            struct_ser.serialize_field("testName", &self.test_name)?;
        }
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        if let Some(v) = self.test_result.as_ref() {
            match v {
                test_result::TestResult::Passed(v) => {
                    struct_ser.serialize_field("passed", v)?;
                }
                test_result::TestResult::Failed(v) => {
                    struct_ser.serialize_field("failed", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "test_name",
            "testName",
            "query",
            "passed",
            "failed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TestName,
            Query,
            Passed,
            Failed,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "testName" | "test_name" => Ok(GeneratedField::TestName),
                            "query" => Ok(GeneratedField::Query),
                            "passed" => Ok(GeneratedField::Passed),
                            "failed" => Ok(GeneratedField::Failed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.TestResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut test_name__ = None;
                let mut query__ = None;
                let mut test_result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TestName => {
                            if test_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("testName"));
                            }
                            test_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Passed => {
                            if test_result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passed"));
                            }
                            test_result__ = map_.next_value::<::std::option::Option<_>>()?.map(test_result::TestResult::Passed)
;
                        }
                        GeneratedField::Failed => {
                            if test_result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            test_result__ = map_.next_value::<::std::option::Option<_>>()?.map(test_result::TestResult::Failed)
;
                        }
                    }
                }
                Ok(TestResult {
                    test_name: test_name__.unwrap_or_default(),
                    query: query__.unwrap_or_default(),
                    test_result: test_result__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.TestResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestResults {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.results.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.TestResults", len)?;
        if !self.results.is_empty() {
            struct_ser.serialize_field("results", &self.results)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestResults {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "results",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Results,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "results" => Ok(GeneratedField::Results),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestResults;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.TestResults")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestResults, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut results__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Results => {
                            if results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("results"));
                            }
                            results__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TestResults {
                    results: results__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.TestResults", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestRunner {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TEST_RUNNER_UNSPECIFIED",
            Self::All => "TEST_RUNNER_ALL",
            Self::Skip => "TEST_RUNNER_SKIP",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TestRunner {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TEST_RUNNER_UNSPECIFIED",
            "TEST_RUNNER_ALL",
            "TEST_RUNNER_SKIP",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestRunner;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TEST_RUNNER_UNSPECIFIED" => Ok(TestRunner::Unspecified),
                    "TEST_RUNNER_ALL" => Ok(TestRunner::All),
                    "TEST_RUNNER_SKIP" => Ok(TestRunner::Skip),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TestSqlFile {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_path.is_empty() {
            len += 1;
        }
        if !self.references.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.TestSQLFile", len)?;
        if !self.file_path.is_empty() {
            struct_ser.serialize_field("filePath", &self.file_path)?;
        }
        if !self.references.is_empty() {
            struct_ser.serialize_field("references", &self.references)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestSqlFile {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_path",
            "filePath",
            "references",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilePath,
            References,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "filePath" | "file_path" => Ok(GeneratedField::FilePath),
                            "references" => Ok(GeneratedField::References),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestSqlFile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.TestSQLFile")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestSqlFile, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_path__ = None;
                let mut references__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FilePath => {
                            if file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filePath"));
                            }
                            file_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::References => {
                            if references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("references"));
                            }
                            references__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TestSqlFile {
                    file_path: file_path__.unwrap_or_default(),
                    references: references__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.TestSQLFile", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TestUnique {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.file_path.is_empty() {
            len += 1;
        }
        if !self.model.is_empty() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.column.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.TestUnique", len)?;
        if !self.file_path.is_empty() {
            struct_ser.serialize_field("filePath", &self.file_path)?;
        }
        if !self.model.is_empty() {
            struct_ser.serialize_field("model", &self.model)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.column.is_empty() {
            struct_ser.serialize_field("column", &self.column)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TestUnique {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "file_path",
            "filePath",
            "model",
            "path",
            "column",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilePath,
            Model,
            Path,
            Column,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "filePath" | "file_path" => Ok(GeneratedField::FilePath),
                            "model" => Ok(GeneratedField::Model),
                            "path" => Ok(GeneratedField::Path),
                            "column" => Ok(GeneratedField::Column),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TestUnique;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.TestUnique")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TestUnique, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut file_path__ = None;
                let mut model__ = None;
                let mut path__ = None;
                let mut column__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FilePath => {
                            if file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filePath"));
                            }
                            file_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Model => {
                            if model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("model"));
                            }
                            model__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Column => {
                            if column__.is_some() {
                                return Err(serde::de::Error::duplicate_field("column"));
                            }
                            column__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TestUnique {
                    file_path: file_path__.unwrap_or_default(),
                    model: model__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                    column: column__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.TestUnique", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Var {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Var", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Var {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Var;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Var")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Var, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Var {
                    name: name__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Var", FIELDS, GeneratedVisitor)
    }
}
