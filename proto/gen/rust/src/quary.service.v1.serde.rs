// @generated
impl serde::Serialize for AddColumnTestToModelOrSourceColumnRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        if !self.model_or_source_name.is_empty() {
            len += 1;
        }
        if !self.column_name.is_empty() {
            len += 1;
        }
        if self.column_test.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.AddColumnTestToModelOrSourceColumnRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        if !self.model_or_source_name.is_empty() {
            struct_ser.serialize_field("modelOrSourceName", &self.model_or_source_name)?;
        }
        if !self.column_name.is_empty() {
            struct_ser.serialize_field("columnName", &self.column_name)?;
        }
        if let Some(v) = self.column_test.as_ref() {
            struct_ser.serialize_field("columnTest", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddColumnTestToModelOrSourceColumnRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
            "model_or_source_name",
            "modelOrSourceName",
            "column_name",
            "columnName",
            "column_test",
            "columnTest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
            ModelOrSourceName,
            ColumnName,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            "modelOrSourceName" | "model_or_source_name" => Ok(GeneratedField::ModelOrSourceName),
                            "columnName" | "column_name" => Ok(GeneratedField::ColumnName),
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
            type Value = AddColumnTestToModelOrSourceColumnRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.AddColumnTestToModelOrSourceColumnRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddColumnTestToModelOrSourceColumnRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                let mut model_or_source_name__ = None;
                let mut column_name__ = None;
                let mut column_test__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModelOrSourceName => {
                            if model_or_source_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelOrSourceName"));
                            }
                            model_or_source_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ColumnName => {
                            if column_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnName"));
                            }
                            column_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ColumnTest => {
                            if column_test__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnTest"));
                            }
                            column_test__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AddColumnTestToModelOrSourceColumnRequest {
                    project_root: project_root__.unwrap_or_default(),
                    model_or_source_name: model_or_source_name__.unwrap_or_default(),
                    column_name: column_name__.unwrap_or_default(),
                    column_test: column_test__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.AddColumnTestToModelOrSourceColumnRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddColumnTestToModelOrSourceColumnResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("quary.service.v1.AddColumnTestToModelOrSourceColumnResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddColumnTestToModelOrSourceColumnResponse {
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
            type Value = AddColumnTestToModelOrSourceColumnResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.AddColumnTestToModelOrSourceColumnResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddColumnTestToModelOrSourceColumnResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AddColumnTestToModelOrSourceColumnResponse {
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.AddColumnTestToModelOrSourceColumnResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddColumnToModelOrSourceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        if !self.model_or_source_name.is_empty() {
            len += 1;
        }
        if !self.column_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.AddColumnToModelOrSourceRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        if !self.model_or_source_name.is_empty() {
            struct_ser.serialize_field("modelOrSourceName", &self.model_or_source_name)?;
        }
        if !self.column_name.is_empty() {
            struct_ser.serialize_field("columnName", &self.column_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddColumnToModelOrSourceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
            "model_or_source_name",
            "modelOrSourceName",
            "column_name",
            "columnName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
            ModelOrSourceName,
            ColumnName,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            "modelOrSourceName" | "model_or_source_name" => Ok(GeneratedField::ModelOrSourceName),
                            "columnName" | "column_name" => Ok(GeneratedField::ColumnName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AddColumnToModelOrSourceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.AddColumnToModelOrSourceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddColumnToModelOrSourceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                let mut model_or_source_name__ = None;
                let mut column_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModelOrSourceName => {
                            if model_or_source_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelOrSourceName"));
                            }
                            model_or_source_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ColumnName => {
                            if column_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnName"));
                            }
                            column_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AddColumnToModelOrSourceRequest {
                    project_root: project_root__.unwrap_or_default(),
                    model_or_source_name: model_or_source_name__.unwrap_or_default(),
                    column_name: column_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.AddColumnToModelOrSourceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AddColumnToModelOrSourceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("quary.service.v1.AddColumnToModelOrSourceResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AddColumnToModelOrSourceResponse {
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
            type Value = AddColumnToModelOrSourceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.AddColumnToModelOrSourceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AddColumnToModelOrSourceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AddColumnToModelOrSourceResponse {
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.AddColumnToModelOrSourceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigQueryDataset {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kind.is_empty() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if self.dataset_reference.is_some() {
            len += 1;
        }
        if !self.friendly_name.is_empty() {
            len += 1;
        }
        if !self.labels.is_empty() {
            len += 1;
        }
        if !self.location.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.BigQueryDataset", len)?;
        if !self.kind.is_empty() {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.dataset_reference.as_ref() {
            struct_ser.serialize_field("datasetReference", v)?;
        }
        if !self.friendly_name.is_empty() {
            struct_ser.serialize_field("friendlyName", &self.friendly_name)?;
        }
        if !self.labels.is_empty() {
            struct_ser.serialize_field("labels", &self.labels)?;
        }
        if !self.location.is_empty() {
            struct_ser.serialize_field("location", &self.location)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryDataset {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "id",
            "dataset_reference",
            "datasetReference",
            "friendly_name",
            "friendlyName",
            "labels",
            "location",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Id,
            DatasetReference,
            FriendlyName,
            Labels,
            Location,
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
                            "kind" => Ok(GeneratedField::Kind),
                            "id" => Ok(GeneratedField::Id),
                            "datasetReference" | "dataset_reference" => Ok(GeneratedField::DatasetReference),
                            "friendlyName" | "friendly_name" => Ok(GeneratedField::FriendlyName),
                            "labels" => Ok(GeneratedField::Labels),
                            "location" => Ok(GeneratedField::Location),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BigQueryDataset;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.BigQueryDataset")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryDataset, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut id__ = None;
                let mut dataset_reference__ = None;
                let mut friendly_name__ = None;
                let mut labels__ = None;
                let mut location__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DatasetReference => {
                            if dataset_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("datasetReference"));
                            }
                            dataset_reference__ = map_.next_value()?;
                        }
                        GeneratedField::FriendlyName => {
                            if friendly_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("friendlyName"));
                            }
                            friendly_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Labels => {
                            if labels__.is_some() {
                                return Err(serde::de::Error::duplicate_field("labels"));
                            }
                            labels__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BigQueryDataset {
                    kind: kind__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    dataset_reference: dataset_reference__,
                    friendly_name: friendly_name__.unwrap_or_default(),
                    labels: labels__.unwrap_or_default(),
                    location: location__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.BigQueryDataset", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigQueryDatasetReference {
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
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.BigQueryDatasetReference", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.dataset_id.is_empty() {
            struct_ser.serialize_field("datasetId", &self.dataset_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryDatasetReference {
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
            type Value = BigQueryDatasetReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.BigQueryDatasetReference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryDatasetReference, V::Error>
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
                Ok(BigQueryDatasetReference {
                    project_id: project_id__.unwrap_or_default(),
                    dataset_id: dataset_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.BigQueryDatasetReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigQueryError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reason.is_empty() {
            len += 1;
        }
        if !self.location.is_empty() {
            len += 1;
        }
        if !self.debug_info.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.BigQueryError", len)?;
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        if !self.location.is_empty() {
            struct_ser.serialize_field("location", &self.location)?;
        }
        if !self.debug_info.is_empty() {
            struct_ser.serialize_field("debugInfo", &self.debug_info)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reason",
            "location",
            "debug_info",
            "debugInfo",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reason,
            Location,
            DebugInfo,
            Message,
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
                            "reason" => Ok(GeneratedField::Reason),
                            "location" => Ok(GeneratedField::Location),
                            "debugInfo" | "debug_info" => Ok(GeneratedField::DebugInfo),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BigQueryError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.BigQueryError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                let mut location__ = None;
                let mut debug_info__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DebugInfo => {
                            if debug_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("debugInfo"));
                            }
                            debug_info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BigQueryError {
                    reason: reason__.unwrap_or_default(),
                    location: location__.unwrap_or_default(),
                    debug_info: debug_info__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.BigQueryError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigQueryFieldValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.f.is_empty() {
            len += 1;
        }
        if !self.v.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.BigQueryFieldValue", len)?;
        if !self.f.is_empty() {
            struct_ser.serialize_field("f", &self.f)?;
        }
        if !self.v.is_empty() {
            struct_ser.serialize_field("v", &self.v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryFieldValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "f",
            "v",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            F,
            V,
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
                            "f" => Ok(GeneratedField::F),
                            "v" => Ok(GeneratedField::V),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BigQueryFieldValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.BigQueryFieldValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryFieldValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut f__ = None;
                let mut v__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::F => {
                            if f__.is_some() {
                                return Err(serde::de::Error::duplicate_field("f"));
                            }
                            f__ = Some(map_.next_value()?);
                        }
                        GeneratedField::V => {
                            if v__.is_some() {
                                return Err(serde::de::Error::duplicate_field("v"));
                            }
                            v__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BigQueryFieldValue {
                    f: f__.unwrap_or_default(),
                    v: v__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.BigQueryFieldValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigQueryJob {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kind.is_empty() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.self_link.is_empty() {
            len += 1;
        }
        if !self.user_email.is_empty() {
            len += 1;
        }
        if self.job_reference.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.BigQueryJob", len)?;
        if !self.kind.is_empty() {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.self_link.is_empty() {
            struct_ser.serialize_field("selfLink", &self.self_link)?;
        }
        if !self.user_email.is_empty() {
            struct_ser.serialize_field("userEmail", &self.user_email)?;
        }
        if let Some(v) = self.job_reference.as_ref() {
            struct_ser.serialize_field("jobReference", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryJob {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "id",
            "self_link",
            "selfLink",
            "user_email",
            "userEmail",
            "job_reference",
            "jobReference",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Id,
            SelfLink,
            UserEmail,
            JobReference,
            Status,
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
                            "kind" => Ok(GeneratedField::Kind),
                            "id" => Ok(GeneratedField::Id),
                            "selfLink" | "self_link" => Ok(GeneratedField::SelfLink),
                            "userEmail" | "user_email" => Ok(GeneratedField::UserEmail),
                            "jobReference" | "job_reference" => Ok(GeneratedField::JobReference),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BigQueryJob;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.BigQueryJob")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryJob, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut id__ = None;
                let mut self_link__ = None;
                let mut user_email__ = None;
                let mut job_reference__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SelfLink => {
                            if self_link__.is_some() {
                                return Err(serde::de::Error::duplicate_field("selfLink"));
                            }
                            self_link__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserEmail => {
                            if user_email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userEmail"));
                            }
                            user_email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::JobReference => {
                            if job_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobReference"));
                            }
                            job_reference__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(BigQueryJob {
                    kind: kind__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    self_link: self_link__.unwrap_or_default(),
                    user_email: user_email__.unwrap_or_default(),
                    job_reference: job_reference__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.BigQueryJob", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigQueryJobReference {
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
        if !self.job_id.is_empty() {
            len += 1;
        }
        if !self.location.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.BigQueryJobReference", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.job_id.is_empty() {
            struct_ser.serialize_field("jobId", &self.job_id)?;
        }
        if !self.location.is_empty() {
            struct_ser.serialize_field("location", &self.location)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryJobReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
            "job_id",
            "jobId",
            "location",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
            JobId,
            Location,
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
                            "jobId" | "job_id" => Ok(GeneratedField::JobId),
                            "location" => Ok(GeneratedField::Location),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BigQueryJobReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.BigQueryJobReference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryJobReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                let mut job_id__ = None;
                let mut location__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::JobId => {
                            if job_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobId"));
                            }
                            job_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BigQueryJobReference {
                    project_id: project_id__.unwrap_or_default(),
                    job_id: job_id__.unwrap_or_default(),
                    location: location__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.BigQueryJobReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigQueryJobResults {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kind.is_empty() {
            len += 1;
        }
        if self.schema.is_some() {
            len += 1;
        }
        if self.job_reference.is_some() {
            len += 1;
        }
        if !self.total_rows.is_empty() {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if !self.rows.is_empty() {
            len += 1;
        }
        if !self.job_complete.is_empty() {
            len += 1;
        }
        if !self.errors.is_empty() {
            len += 1;
        }
        if !self.cache_hit.is_empty() {
            len += 1;
        }
        if !self.num_dml_affected_rows.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.BigQueryJobResults", len)?;
        if !self.kind.is_empty() {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        if let Some(v) = self.schema.as_ref() {
            struct_ser.serialize_field("schema", v)?;
        }
        if let Some(v) = self.job_reference.as_ref() {
            struct_ser.serialize_field("jobReference", v)?;
        }
        if !self.total_rows.is_empty() {
            struct_ser.serialize_field("totalRows", &self.total_rows)?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if !self.rows.is_empty() {
            struct_ser.serialize_field("rows", &self.rows)?;
        }
        if !self.job_complete.is_empty() {
            struct_ser.serialize_field("jobComplete", &self.job_complete)?;
        }
        if !self.errors.is_empty() {
            struct_ser.serialize_field("errors", &self.errors)?;
        }
        if !self.cache_hit.is_empty() {
            struct_ser.serialize_field("cacheHit", &self.cache_hit)?;
        }
        if !self.num_dml_affected_rows.is_empty() {
            struct_ser.serialize_field("numDmlAffectedRows", &self.num_dml_affected_rows)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryJobResults {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "schema",
            "job_reference",
            "jobReference",
            "total_rows",
            "totalRows",
            "page_token",
            "pageToken",
            "rows",
            "job_complete",
            "jobComplete",
            "errors",
            "cache_hit",
            "cacheHit",
            "num_dml_affected_rows",
            "numDmlAffectedRows",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Schema,
            JobReference,
            TotalRows,
            PageToken,
            Rows,
            JobComplete,
            Errors,
            CacheHit,
            NumDmlAffectedRows,
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
                            "kind" => Ok(GeneratedField::Kind),
                            "schema" => Ok(GeneratedField::Schema),
                            "jobReference" | "job_reference" => Ok(GeneratedField::JobReference),
                            "totalRows" | "total_rows" => Ok(GeneratedField::TotalRows),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "rows" => Ok(GeneratedField::Rows),
                            "jobComplete" | "job_complete" => Ok(GeneratedField::JobComplete),
                            "errors" => Ok(GeneratedField::Errors),
                            "cacheHit" | "cache_hit" => Ok(GeneratedField::CacheHit),
                            "numDmlAffectedRows" | "num_dml_affected_rows" => Ok(GeneratedField::NumDmlAffectedRows),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BigQueryJobResults;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.BigQueryJobResults")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryJobResults, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut schema__ = None;
                let mut job_reference__ = None;
                let mut total_rows__ = None;
                let mut page_token__ = None;
                let mut rows__ = None;
                let mut job_complete__ = None;
                let mut errors__ = None;
                let mut cache_hit__ = None;
                let mut num_dml_affected_rows__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Schema => {
                            if schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schema"));
                            }
                            schema__ = map_.next_value()?;
                        }
                        GeneratedField::JobReference => {
                            if job_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobReference"));
                            }
                            job_reference__ = map_.next_value()?;
                        }
                        GeneratedField::TotalRows => {
                            if total_rows__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalRows"));
                            }
                            total_rows__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Rows => {
                            if rows__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rows"));
                            }
                            rows__ = Some(map_.next_value()?);
                        }
                        GeneratedField::JobComplete => {
                            if job_complete__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobComplete"));
                            }
                            job_complete__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Errors => {
                            if errors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errors"));
                            }
                            errors__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CacheHit => {
                            if cache_hit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheHit"));
                            }
                            cache_hit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NumDmlAffectedRows => {
                            if num_dml_affected_rows__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numDmlAffectedRows"));
                            }
                            num_dml_affected_rows__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BigQueryJobResults {
                    kind: kind__.unwrap_or_default(),
                    schema: schema__,
                    job_reference: job_reference__,
                    total_rows: total_rows__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    rows: rows__.unwrap_or_default(),
                    job_complete: job_complete__.unwrap_or_default(),
                    errors: errors__.unwrap_or_default(),
                    cache_hit: cache_hit__.unwrap_or_default(),
                    num_dml_affected_rows: num_dml_affected_rows__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.BigQueryJobResults", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigQueryJobStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.state.is_empty() {
            len += 1;
        }
        if !self.error_result.is_empty() {
            len += 1;
        }
        if !self.error_message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.BigQueryJobStatus", len)?;
        if !self.state.is_empty() {
            struct_ser.serialize_field("state", &self.state)?;
        }
        if !self.error_result.is_empty() {
            struct_ser.serialize_field("errorResult", &self.error_result)?;
        }
        if !self.error_message.is_empty() {
            struct_ser.serialize_field("errorMessage", &self.error_message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryJobStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "state",
            "error_result",
            "errorResult",
            "error_message",
            "errorMessage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            State,
            ErrorResult,
            ErrorMessage,
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
                            "state" => Ok(GeneratedField::State),
                            "errorResult" | "error_result" => Ok(GeneratedField::ErrorResult),
                            "errorMessage" | "error_message" => Ok(GeneratedField::ErrorMessage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BigQueryJobStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.BigQueryJobStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryJobStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut state__ = None;
                let mut error_result__ = None;
                let mut error_message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ErrorResult => {
                            if error_result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorResult"));
                            }
                            error_result__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ErrorMessage => {
                            if error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            error_message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BigQueryJobStatus {
                    state: state__.unwrap_or_default(),
                    error_result: error_result__.unwrap_or_default(),
                    error_message: error_message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.BigQueryJobStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigQueryOauthToken {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.access_token.is_empty() {
            len += 1;
        }
        if !self.refresh_token.is_empty() {
            len += 1;
        }
        if !self.expiry_time.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.BigQueryOauthToken", len)?;
        if !self.access_token.is_empty() {
            struct_ser.serialize_field("accessToken", &self.access_token)?;
        }
        if !self.refresh_token.is_empty() {
            struct_ser.serialize_field("refreshToken", &self.refresh_token)?;
        }
        if !self.expiry_time.is_empty() {
            struct_ser.serialize_field("expiryTime", &self.expiry_time)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryOauthToken {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "access_token",
            "accessToken",
            "refresh_token",
            "refreshToken",
            "expiry_time",
            "expiryTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccessToken,
            RefreshToken,
            ExpiryTime,
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
                            "accessToken" | "access_token" => Ok(GeneratedField::AccessToken),
                            "refreshToken" | "refresh_token" => Ok(GeneratedField::RefreshToken),
                            "expiryTime" | "expiry_time" => Ok(GeneratedField::ExpiryTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BigQueryOauthToken;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.BigQueryOauthToken")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryOauthToken, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut access_token__ = None;
                let mut refresh_token__ = None;
                let mut expiry_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccessToken => {
                            if access_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessToken"));
                            }
                            access_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RefreshToken => {
                            if refresh_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshToken"));
                            }
                            refresh_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExpiryTime => {
                            if expiry_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiryTime"));
                            }
                            expiry_time__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BigQueryOauthToken {
                    access_token: access_token__.unwrap_or_default(),
                    refresh_token: refresh_token__.unwrap_or_default(),
                    expiry_time: expiry_time__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.BigQueryOauthToken", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigQueryOauthTokenRefresh {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.access_token.is_empty() {
            len += 1;
        }
        if !self.expiry_time.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.BigQueryOauthTokenRefresh", len)?;
        if !self.access_token.is_empty() {
            struct_ser.serialize_field("accessToken", &self.access_token)?;
        }
        if !self.expiry_time.is_empty() {
            struct_ser.serialize_field("expiryTime", &self.expiry_time)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryOauthTokenRefresh {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "access_token",
            "accessToken",
            "expiry_time",
            "expiryTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccessToken,
            ExpiryTime,
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
                            "accessToken" | "access_token" => Ok(GeneratedField::AccessToken),
                            "expiryTime" | "expiry_time" => Ok(GeneratedField::ExpiryTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BigQueryOauthTokenRefresh;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.BigQueryOauthTokenRefresh")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryOauthTokenRefresh, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut access_token__ = None;
                let mut expiry_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccessToken => {
                            if access_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessToken"));
                            }
                            access_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExpiryTime => {
                            if expiry_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiryTime"));
                            }
                            expiry_time__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BigQueryOauthTokenRefresh {
                    access_token: access_token__.unwrap_or_default(),
                    expiry_time: expiry_time__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.BigQueryOauthTokenRefresh", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigQueryProject {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kind.is_empty() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.numeric_id.is_empty() {
            len += 1;
        }
        if self.project_reference.is_some() {
            len += 1;
        }
        if !self.friendly_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.BigQueryProject", len)?;
        if !self.kind.is_empty() {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.numeric_id.is_empty() {
            struct_ser.serialize_field("numericId", &self.numeric_id)?;
        }
        if let Some(v) = self.project_reference.as_ref() {
            struct_ser.serialize_field("projectReference", v)?;
        }
        if !self.friendly_name.is_empty() {
            struct_ser.serialize_field("friendlyName", &self.friendly_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryProject {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "id",
            "numeric_id",
            "numericId",
            "project_reference",
            "projectReference",
            "friendly_name",
            "friendlyName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Id,
            NumericId,
            ProjectReference,
            FriendlyName,
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
                            "kind" => Ok(GeneratedField::Kind),
                            "id" => Ok(GeneratedField::Id),
                            "numericId" | "numeric_id" => Ok(GeneratedField::NumericId),
                            "projectReference" | "project_reference" => Ok(GeneratedField::ProjectReference),
                            "friendlyName" | "friendly_name" => Ok(GeneratedField::FriendlyName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BigQueryProject;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.BigQueryProject")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryProject, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut id__ = None;
                let mut numeric_id__ = None;
                let mut project_reference__ = None;
                let mut friendly_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NumericId => {
                            if numeric_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numericId"));
                            }
                            numeric_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectReference => {
                            if project_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectReference"));
                            }
                            project_reference__ = map_.next_value()?;
                        }
                        GeneratedField::FriendlyName => {
                            if friendly_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("friendlyName"));
                            }
                            friendly_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BigQueryProject {
                    kind: kind__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    numeric_id: numeric_id__.unwrap_or_default(),
                    project_reference: project_reference__,
                    friendly_name: friendly_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.BigQueryProject", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigQueryProjectReference {
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
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.BigQueryProjectReference", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryProjectReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BigQueryProjectReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.BigQueryProjectReference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryProjectReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BigQueryProjectReference {
                    project_id: project_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.BigQueryProjectReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigQueryTable {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.kind.is_empty() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if self.table_reference.is_some() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.creation_time.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.BigQueryTable", len)?;
        if !self.kind.is_empty() {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.table_reference.as_ref() {
            struct_ser.serialize_field("tableReference", v)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.creation_time.is_empty() {
            struct_ser.serialize_field("creationTime", &self.creation_time)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryTable {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "id",
            "table_reference",
            "tableReference",
            "type",
            "creation_time",
            "creationTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Id,
            TableReference,
            Type,
            CreationTime,
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
                            "kind" => Ok(GeneratedField::Kind),
                            "id" => Ok(GeneratedField::Id),
                            "tableReference" | "table_reference" => Ok(GeneratedField::TableReference),
                            "type" => Ok(GeneratedField::Type),
                            "creationTime" | "creation_time" => Ok(GeneratedField::CreationTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BigQueryTable;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.BigQueryTable")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryTable, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut id__ = None;
                let mut table_reference__ = None;
                let mut r#type__ = None;
                let mut creation_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TableReference => {
                            if table_reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableReference"));
                            }
                            table_reference__ = map_.next_value()?;
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreationTime => {
                            if creation_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationTime"));
                            }
                            creation_time__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BigQueryTable {
                    kind: kind__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    table_reference: table_reference__,
                    r#type: r#type__.unwrap_or_default(),
                    creation_time: creation_time__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.BigQueryTable", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigQueryTableField {
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
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.mode.is_empty() {
            len += 1;
        }
        if !self.fields.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.BigQueryTableField", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.mode.is_empty() {
            struct_ser.serialize_field("mode", &self.mode)?;
        }
        if !self.fields.is_empty() {
            struct_ser.serialize_field("fields", &self.fields)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryTableField {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "type",
            "mode",
            "fields",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Type,
            Mode,
            Fields,
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
                            "name" => Ok(GeneratedField::Name),
                            "type" => Ok(GeneratedField::Type),
                            "mode" => Ok(GeneratedField::Mode),
                            "fields" => Ok(GeneratedField::Fields),
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
            type Value = BigQueryTableField;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.BigQueryTableField")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryTableField, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut r#type__ = None;
                let mut mode__ = None;
                let mut fields__ = None;
                let mut description__ = None;
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
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Mode => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            mode__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fields => {
                            if fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            fields__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BigQueryTableField {
                    name: name__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    mode: mode__.unwrap_or_default(),
                    fields: fields__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.BigQueryTableField", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigQueryTableReference {
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
        if !self.table_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.BigQueryTableReference", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.dataset_id.is_empty() {
            struct_ser.serialize_field("datasetId", &self.dataset_id)?;
        }
        if !self.table_id.is_empty() {
            struct_ser.serialize_field("tableId", &self.table_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryTableReference {
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
            "table_id",
            "tableId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
            DatasetId,
            TableId,
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
                            "tableId" | "table_id" => Ok(GeneratedField::TableId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BigQueryTableReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.BigQueryTableReference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryTableReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                let mut dataset_id__ = None;
                let mut table_id__ = None;
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
                        GeneratedField::TableId => {
                            if table_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableId"));
                            }
                            table_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BigQueryTableReference {
                    project_id: project_id__.unwrap_or_default(),
                    dataset_id: dataset_id__.unwrap_or_default(),
                    table_id: table_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.BigQueryTableReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigQueryTableRow {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.f.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.BigQueryTableRow", len)?;
        if !self.f.is_empty() {
            struct_ser.serialize_field("f", &self.f)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryTableRow {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "f",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            F,
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
                            "f" => Ok(GeneratedField::F),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BigQueryTableRow;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.BigQueryTableRow")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryTableRow, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut f__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::F => {
                            if f__.is_some() {
                                return Err(serde::de::Error::duplicate_field("f"));
                            }
                            f__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BigQueryTableRow {
                    f: f__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.BigQueryTableRow", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BigQueryTableSchema {
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
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.fields.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.BigQueryTableSchema", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.fields.is_empty() {
            struct_ser.serialize_field("fields", &self.fields)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BigQueryTableSchema {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "type",
            "fields",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Type,
            Fields,
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
                            "name" => Ok(GeneratedField::Name),
                            "type" => Ok(GeneratedField::Type),
                            "fields" => Ok(GeneratedField::Fields),
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
            type Value = BigQueryTableSchema;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.BigQueryTableSchema")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BigQueryTableSchema, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut r#type__ = None;
                let mut fields__ = None;
                let mut description__ = None;
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
                            r#type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fields => {
                            if fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            fields__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BigQueryTableSchema {
                    name: name__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    fields: fields__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.BigQueryTableSchema", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CacheViewInformation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cache_view.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.CacheViewInformation", len)?;
        if let Some(v) = self.cache_view.as_ref() {
            match v {
                cache_view_information::CacheView::CacheViewInformation(v) => {
                    struct_ser.serialize_field("cacheViewInformation", v)?;
                }
                cache_view_information::CacheView::DoNotUse(v) => {
                    struct_ser.serialize_field("doNotUse", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CacheViewInformation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cache_view_information",
            "cacheViewInformation",
            "do_not_use",
            "doNotUse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CacheViewInformation,
            DoNotUse,
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
                            "cacheViewInformation" | "cache_view_information" => Ok(GeneratedField::CacheViewInformation),
                            "doNotUse" | "do_not_use" => Ok(GeneratedField::DoNotUse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CacheViewInformation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.CacheViewInformation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CacheViewInformation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cache_view__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CacheViewInformation => {
                            if cache_view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheViewInformation"));
                            }
                            cache_view__ = map_.next_value::<::std::option::Option<_>>()?.map(cache_view_information::CacheView::CacheViewInformation)
;
                        }
                        GeneratedField::DoNotUse => {
                            if cache_view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("doNotUse"));
                            }
                            cache_view__ = map_.next_value::<::std::option::Option<_>>()?.map(cache_view_information::CacheView::DoNotUse)
;
                        }
                    }
                }
                Ok(CacheViewInformation {
                    cache_view: cache_view__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.CacheViewInformation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CacheViewInformationPaths {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cache_view_paths.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.CacheViewInformationPaths", len)?;
        if !self.cache_view_paths.is_empty() {
            struct_ser.serialize_field("cacheViewPaths", &self.cache_view_paths)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CacheViewInformationPaths {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cache_view_paths",
            "cacheViewPaths",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CacheViewPaths,
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
                            "cacheViewPaths" | "cache_view_paths" => Ok(GeneratedField::CacheViewPaths),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CacheViewInformationPaths;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.CacheViewInformationPaths")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CacheViewInformationPaths, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cache_view_paths__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CacheViewPaths => {
                            if cache_view_paths__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheViewPaths"));
                            }
                            cache_view_paths__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CacheViewInformationPaths {
                    cache_view_paths: cache_view_paths__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.CacheViewInformationPaths", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Chart {
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
        if !self.tags.is_empty() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        if !self.references.is_empty() {
            len += 1;
        }
        if self.source.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Chart", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if !self.references.is_empty() {
            struct_ser.serialize_field("references", &self.references)?;
        }
        if let Some(v) = self.source.as_ref() {
            match v {
                chart::Source::RawSql(v) => {
                    struct_ser.serialize_field("rawSql", v)?;
                }
                chart::Source::PreTemplatedSql(v) => {
                    struct_ser.serialize_field("preTemplatedSql", v)?;
                }
                chart::Source::Reference(v) => {
                    struct_ser.serialize_field("reference", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Chart {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "tags",
            "path",
            "config",
            "references",
            "raw_sql",
            "rawSql",
            "pre_templated_sql",
            "preTemplatedSql",
            "reference",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Tags,
            Path,
            Config,
            References,
            RawSql,
            PreTemplatedSql,
            Reference,
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
                            "tags" => Ok(GeneratedField::Tags),
                            "path" => Ok(GeneratedField::Path),
                            "config" => Ok(GeneratedField::Config),
                            "references" => Ok(GeneratedField::References),
                            "rawSql" | "raw_sql" => Ok(GeneratedField::RawSql),
                            "preTemplatedSql" | "pre_templated_sql" => Ok(GeneratedField::PreTemplatedSql),
                            "reference" => Ok(GeneratedField::Reference),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Chart;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Chart")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Chart, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut tags__ = None;
                let mut path__ = None;
                let mut config__ = None;
                let mut references__ = None;
                let mut source__ = None;
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
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::References => {
                            if references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("references"));
                            }
                            references__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RawSql => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rawSql"));
                            }
                            source__ = map_.next_value::<::std::option::Option<_>>()?.map(chart::Source::RawSql);
                        }
                        GeneratedField::PreTemplatedSql => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preTemplatedSql"));
                            }
                            source__ = map_.next_value::<::std::option::Option<_>>()?.map(chart::Source::PreTemplatedSql);
                        }
                        GeneratedField::Reference => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reference"));
                            }
                            source__ = map_.next_value::<::std::option::Option<_>>()?.map(chart::Source::Reference)
;
                        }
                    }
                }
                Ok(Chart {
                    name: name__.unwrap_or_default(),
                    description: description__,
                    tags: tags__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                    config: config__,
                    references: references__.unwrap_or_default(),
                    source: source__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Chart", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for chart::AssetReference {
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
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Chart.AssetReference", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for chart::AssetReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = chart::AssetReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Chart.AssetReference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<chart::AssetReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(chart::AssetReference {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Chart.AssetReference", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChartFile {
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
        if !self.tags.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        if self.source.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ChartFile", len)?;
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if let Some(v) = self.source.as_ref() {
            match v {
                chart_file::Source::RawSql(v) => {
                    struct_ser.serialize_field("rawSql", v)?;
                }
                chart_file::Source::PreTemplatedSql(v) => {
                    struct_ser.serialize_field("preTemplatedSql", v)?;
                }
                chart_file::Source::Reference(v) => {
                    struct_ser.serialize_field("reference", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChartFile {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "description",
            "tags",
            "config",
            "raw_sql",
            "rawSql",
            "pre_templated_sql",
            "preTemplatedSql",
            "reference",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
            Tags,
            Config,
            RawSql,
            PreTemplatedSql,
            Reference,
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
                            "description" => Ok(GeneratedField::Description),
                            "tags" => Ok(GeneratedField::Tags),
                            "config" => Ok(GeneratedField::Config),
                            "rawSql" | "raw_sql" => Ok(GeneratedField::RawSql),
                            "preTemplatedSql" | "pre_templated_sql" => Ok(GeneratedField::PreTemplatedSql),
                            "reference" => Ok(GeneratedField::Reference),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChartFile;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ChartFile")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ChartFile, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                let mut tags__ = None;
                let mut config__ = None;
                let mut source__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::RawSql => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rawSql"));
                            }
                            source__ = map_.next_value::<::std::option::Option<_>>()?.map(chart_file::Source::RawSql);
                        }
                        GeneratedField::PreTemplatedSql => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preTemplatedSql"));
                            }
                            source__ = map_.next_value::<::std::option::Option<_>>()?.map(chart_file::Source::PreTemplatedSql);
                        }
                        GeneratedField::Reference => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reference"));
                            }
                            source__ = map_.next_value::<::std::option::Option<_>>()?.map(chart_file::Source::Reference)
;
                        }
                    }
                }
                Ok(ChartFile {
                    description: description__,
                    tags: tags__.unwrap_or_default(),
                    config: config__,
                    source: source__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ChartFile", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for chart_file::AssetReference {
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
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ChartFile.AssetReference", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for chart_file::AssetReference {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = chart_file::AssetReference;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ChartFile.AssetReference")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<chart_file::AssetReference, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(chart_file::AssetReference {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ChartFile.AssetReference", FIELDS, GeneratedVisitor)
    }
}
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
                connection_config::Config::Redshift(v) => {
                    struct_ser.serialize_field("redshift", v)?;
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
            "redshift",
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
            Redshift,
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
                            "redshift" => Ok(GeneratedField::Redshift),
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
                        GeneratedField::Redshift => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redshift"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(connection_config::Config::Redshift)
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
impl serde::Serialize for connection_config::ConnectionConfigRedshift {
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
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ConnectionConfig.ConnectionConfigRedshift", len)?;
        if !self.schema.is_empty() {
            struct_ser.serialize_field("schema", &self.schema)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for connection_config::ConnectionConfigRedshift {
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
            type Value = connection_config::ConnectionConfigRedshift;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ConnectionConfig.ConnectionConfigRedshift")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<connection_config::ConnectionConfigRedshift, V::Error>
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
                Ok(connection_config::ConnectionConfigRedshift {
                    schema: schema__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ConnectionConfig.ConnectionConfigRedshift", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for CreateModelSchemaEntryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        if !self.model_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.CreateModelSchemaEntryRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        if !self.model_name.is_empty() {
            struct_ser.serialize_field("modelName", &self.model_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateModelSchemaEntryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
            "model_name",
            "modelName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
            ModelName,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            "modelName" | "model_name" => Ok(GeneratedField::ModelName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateModelSchemaEntryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.CreateModelSchemaEntryRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateModelSchemaEntryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                let mut model_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModelName => {
                            if model_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelName"));
                            }
                            model_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateModelSchemaEntryRequest {
                    project_root: project_root__.unwrap_or_default(),
                    model_name: model_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.CreateModelSchemaEntryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateModelSchemaEntryResponse {
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
        if self.project_file.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.CreateModelSchemaEntryResponse", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if let Some(v) = self.project_file.as_ref() {
            struct_ser.serialize_field("projectFile", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateModelSchemaEntryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "project_file",
            "projectFile",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            ProjectFile,
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
                            "projectFile" | "project_file" => Ok(GeneratedField::ProjectFile),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateModelSchemaEntryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.CreateModelSchemaEntryResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateModelSchemaEntryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut project_file__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectFile => {
                            if project_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectFile"));
                            }
                            project_file__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateModelSchemaEntryResponse {
                    path: path__.unwrap_or_default(),
                    project_file: project_file__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.CreateModelSchemaEntryResponse", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for ExecRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.query.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ExecRequest", len)?;
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "query",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Query,
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
                            "query" => Ok(GeneratedField::Query),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ExecRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExecRequest {
                    query: query__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ExecRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("quary.service.v1.ExecResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecResponse {
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
            type Value = ExecResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ExecResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ExecResponse {
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ExecResponse", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for GenerateProjectFilesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.connection_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.GenerateProjectFilesRequest", len)?;
        if let Some(v) = self.connection_config.as_ref() {
            struct_ser.serialize_field("connectionConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenerateProjectFilesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "connection_config",
            "connectionConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = GenerateProjectFilesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.GenerateProjectFilesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenerateProjectFilesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut connection_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConnectionConfig => {
                            if connection_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionConfig"));
                            }
                            connection_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GenerateProjectFilesRequest {
                    connection_config: connection_config__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.GenerateProjectFilesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenerateProjectFilesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("quary.service.v1.GenerateProjectFilesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenerateProjectFilesResponse {
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
            type Value = GenerateProjectFilesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.GenerateProjectFilesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenerateProjectFilesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GenerateProjectFilesResponse {
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.GenerateProjectFilesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenerateSourceFilesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        if !self.sources.is_empty() {
            len += 1;
        }
        if !self.folder_path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.GenerateSourceFilesRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        if !self.sources.is_empty() {
            struct_ser.serialize_field("sources", &self.sources)?;
        }
        if !self.folder_path.is_empty() {
            struct_ser.serialize_field("folderPath", &self.folder_path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenerateSourceFilesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
            "sources",
            "folder_path",
            "folderPath",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
            Sources,
            FolderPath,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            "sources" => Ok(GeneratedField::Sources),
                            "folderPath" | "folder_path" => Ok(GeneratedField::FolderPath),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenerateSourceFilesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.GenerateSourceFilesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenerateSourceFilesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                let mut sources__ = None;
                let mut folder_path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sources => {
                            if sources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sources"));
                            }
                            sources__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FolderPath => {
                            if folder_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("folderPath"));
                            }
                            folder_path__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenerateSourceFilesRequest {
                    project_root: project_root__.unwrap_or_default(),
                    sources: sources__.unwrap_or_default(),
                    folder_path: folder_path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.GenerateSourceFilesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenerateSourceFilesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("quary.service.v1.GenerateSourceFilesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenerateSourceFilesResponse {
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
            type Value = GenerateSourceFilesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.GenerateSourceFilesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenerateSourceFilesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GenerateSourceFilesResponse {
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.GenerateSourceFilesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetModelTableRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        if !self.model_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.GetModelTableRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        if !self.model_name.is_empty() {
            struct_ser.serialize_field("modelName", &self.model_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetModelTableRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
            "model_name",
            "modelName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
            ModelName,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            "modelName" | "model_name" => Ok(GeneratedField::ModelName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetModelTableRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.GetModelTableRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetModelTableRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                let mut model_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModelName => {
                            if model_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelName"));
                            }
                            model_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetModelTableRequest {
                    project_root: project_root__.unwrap_or_default(),
                    model_name: model_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.GetModelTableRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetModelTableResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.table.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.GetModelTableResponse", len)?;
        if let Some(v) = self.table.as_ref() {
            struct_ser.serialize_field("table", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetModelTableResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Table,
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
                            "table" => Ok(GeneratedField::Table),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetModelTableResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.GetModelTableResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetModelTableResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Table => {
                            if table__.is_some() {
                                return Err(serde::de::Error::duplicate_field("table"));
                            }
                            table__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetModelTableResponse {
                    table: table__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.GetModelTableResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetProjectConfigRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.GetProjectConfigRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetProjectConfigRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetProjectConfigRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.GetProjectConfigRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetProjectConfigRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetProjectConfigRequest {
                    project_root: project_root__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.GetProjectConfigRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetProjectConfigResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.connection_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.GetProjectConfigResponse", len)?;
        if let Some(v) = self.connection_config.as_ref() {
            struct_ser.serialize_field("connectionConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetProjectConfigResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "connection_config",
            "connectionConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = GetProjectConfigResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.GetProjectConfigResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetProjectConfigResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut connection_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConnectionConfig => {
                            if connection_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionConfig"));
                            }
                            connection_config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetProjectConfigResponse {
                    connection_config: connection_config__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.GetProjectConfigResponse", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for InitFilesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("quary.service.v1.InitFilesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitFilesRequest {
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
            type Value = InitFilesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.InitFilesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InitFilesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(InitFilesRequest {
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.InitFilesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InitFilesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("quary.service.v1.InitFilesResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InitFilesResponse {
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
            type Value = InitFilesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.InitFilesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InitFilesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(InitFilesResponse {
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.InitFilesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IsPathEmptyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.IsPathEmptyRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IsPathEmptyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IsPathEmptyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.IsPathEmptyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IsPathEmptyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(IsPathEmptyRequest {
                    project_root: project_root__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.IsPathEmptyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IsPathEmptyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.is_empty {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.IsPathEmptyResponse", len)?;
        if self.is_empty {
            struct_ser.serialize_field("isEmpty", &self.is_empty)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IsPathEmptyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "is_empty",
            "isEmpty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IsEmpty,
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
                            "isEmpty" | "is_empty" => Ok(GeneratedField::IsEmpty),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IsPathEmptyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.IsPathEmptyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IsPathEmptyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut is_empty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IsEmpty => {
                            if is_empty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isEmpty"));
                            }
                            is_empty__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(IsPathEmptyResponse {
                    is_empty: is_empty__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.IsPathEmptyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAssetsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ListAssetsRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAssetsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListAssetsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ListAssetsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAssetsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListAssetsRequest {
                    project_root: project_root__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ListAssetsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAssetsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.assets.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ListAssetsResponse", len)?;
        if !self.assets.is_empty() {
            struct_ser.serialize_field("assets", &self.assets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAssetsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "assets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Assets,
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
                            "assets" => Ok(GeneratedField::Assets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListAssetsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ListAssetsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAssetsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut assets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Assets => {
                            if assets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assets"));
                            }
                            assets__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListAssetsResponse {
                    assets: assets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ListAssetsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for list_assets_response::Asset {
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
        if !self.tags.is_empty() {
            len += 1;
        }
        if self.asset_type != 0 {
            len += 1;
        }
        if !self.file_path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ListAssetsResponse.Asset", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if self.asset_type != 0 {
            let v = list_assets_response::asset::AssetType::try_from(self.asset_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.asset_type)))?;
            struct_ser.serialize_field("assetType", &v)?;
        }
        if !self.file_path.is_empty() {
            struct_ser.serialize_field("filePath", &self.file_path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for list_assets_response::Asset {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "tags",
            "asset_type",
            "assetType",
            "file_path",
            "filePath",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Tags,
            AssetType,
            FilePath,
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
                            "tags" => Ok(GeneratedField::Tags),
                            "assetType" | "asset_type" => Ok(GeneratedField::AssetType),
                            "filePath" | "file_path" => Ok(GeneratedField::FilePath),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = list_assets_response::Asset;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ListAssetsResponse.Asset")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<list_assets_response::Asset, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut tags__ = None;
                let mut asset_type__ = None;
                let mut file_path__ = None;
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
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetType => {
                            if asset_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetType"));
                            }
                            asset_type__ = Some(map_.next_value::<list_assets_response::asset::AssetType>()? as i32);
                        }
                        GeneratedField::FilePath => {
                            if file_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filePath"));
                            }
                            file_path__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(list_assets_response::Asset {
                    name: name__.unwrap_or_default(),
                    description: description__,
                    tags: tags__.unwrap_or_default(),
                    asset_type: asset_type__.unwrap_or_default(),
                    file_path: file_path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ListAssetsResponse.Asset", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for list_assets_response::asset::AssetType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ASSET_TYPE_UNSPECIFIED",
            Self::Model => "ASSET_TYPE_MODEL",
            Self::Seed => "ASSET_TYPE_SEED",
            Self::Source => "ASSET_TYPE_SOURCE",
            Self::Snapshot => "ASSET_TYPE_SNAPSHOT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for list_assets_response::asset::AssetType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ASSET_TYPE_UNSPECIFIED",
            "ASSET_TYPE_MODEL",
            "ASSET_TYPE_SEED",
            "ASSET_TYPE_SOURCE",
            "ASSET_TYPE_SNAPSHOT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = list_assets_response::asset::AssetType;

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
                    "ASSET_TYPE_UNSPECIFIED" => Ok(list_assets_response::asset::AssetType::Unspecified),
                    "ASSET_TYPE_MODEL" => Ok(list_assets_response::asset::AssetType::Model),
                    "ASSET_TYPE_SEED" => Ok(list_assets_response::asset::AssetType::Seed),
                    "ASSET_TYPE_SOURCE" => Ok(list_assets_response::asset::AssetType::Source),
                    "ASSET_TYPE_SNAPSHOT" => Ok(list_assets_response::asset::AssetType::Snapshot),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ListColumnsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.table_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ListColumnsRequest", len)?;
        if !self.table_name.is_empty() {
            struct_ser.serialize_field("tableName", &self.table_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListColumnsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_name",
            "tableName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableName,
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
                            "tableName" | "table_name" => Ok(GeneratedField::TableName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListColumnsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ListColumnsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListColumnsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TableName => {
                            if table_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableName"));
                            }
                            table_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListColumnsRequest {
                    table_name: table_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ListColumnsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListColumnsResponse {
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
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ListColumnsResponse", len)?;
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListColumnsResponse {
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
            type Value = ListColumnsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ListColumnsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListColumnsResponse, V::Error>
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
                Ok(ListColumnsResponse {
                    columns: columns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ListColumnsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListSourcesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("quary.service.v1.ListSourcesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListSourcesRequest {
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
            type Value = ListSourcesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ListSourcesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListSourcesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListSourcesRequest {
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ListSourcesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListSourcesResponse {
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
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ListSourcesResponse", len)?;
        if !self.sources.is_empty() {
            struct_ser.serialize_field("sources", &self.sources)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListSourcesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sources",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sources,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListSourcesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ListSourcesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListSourcesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sources__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sources => {
                            if sources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sources"));
                            }
                            sources__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListSourcesResponse {
                    sources: sources__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ListSourcesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTablesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("quary.service.v1.ListTablesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTablesRequest {
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
            type Value = ListTablesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ListTablesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTablesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListTablesRequest {
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ListTablesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListTablesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tables.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ListTablesResponse", len)?;
        if !self.tables.is_empty() {
            struct_ser.serialize_field("tables", &self.tables)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListTablesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tables",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tables,
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
                            "tables" => Ok(GeneratedField::Tables),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListTablesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ListTablesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListTablesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tables__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tables => {
                            if tables__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tables"));
                            }
                            tables__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListTablesResponse {
                    tables: tables__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ListTablesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListViewsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("quary.service.v1.ListViewsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListViewsRequest {
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
            type Value = ListViewsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ListViewsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListViewsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListViewsRequest {
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ListViewsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListViewsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.views.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ListViewsResponse", len)?;
        if !self.views.is_empty() {
            struct_ser.serialize_field("views", &self.views)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListViewsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "views",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Views,
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
                            "views" => Ok(GeneratedField::Views),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListViewsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ListViewsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListViewsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut views__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Views => {
                            if views__.is_some() {
                                return Err(serde::de::Error::duplicate_field("views"));
                            }
                            views__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListViewsResponse {
                    views: views__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ListViewsResponse", FIELDS, GeneratedVisitor)
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
        if !self.tags.is_empty() {
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
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
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
            "tags",
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
            Tags,
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
                            "tags" => Ok(GeneratedField::Tags),
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
                let mut tags__ = None;
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
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map_.next_value()?);
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
                    tags: tags__.unwrap_or_default(),
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
impl serde::Serialize for ParseProjectRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ParseProjectRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ParseProjectRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ParseProjectRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ParseProjectRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ParseProjectRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ParseProjectRequest {
                    project_root: project_root__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ParseProjectRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ParseProjectResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.project.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ParseProjectResponse", len)?;
        if let Some(v) = self.project.as_ref() {
            struct_ser.serialize_field("project", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ParseProjectResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Project,
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
                            "project" => Ok(GeneratedField::Project),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ParseProjectResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ParseProjectResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ParseProjectResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Project => {
                            if project__.is_some() {
                                return Err(serde::de::Error::duplicate_field("project"));
                            }
                            project__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ParseProjectResponse {
                    project: project__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ParseProjectResponse", FIELDS, GeneratedVisitor)
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
        if !self.snapshots.is_empty() {
            len += 1;
        }
        if !self.charts.is_empty() {
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
        if !self.snapshots.is_empty() {
            struct_ser.serialize_field("snapshots", &self.snapshots)?;
        }
        if !self.charts.is_empty() {
            struct_ser.serialize_field("charts", &self.charts)?;
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
            "snapshots",
            "charts",
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
            Snapshots,
            Charts,
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
                            "snapshots" => Ok(GeneratedField::Snapshots),
                            "charts" => Ok(GeneratedField::Charts),
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
                let mut snapshots__ = None;
                let mut charts__ = None;
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
                        GeneratedField::Snapshots => {
                            if snapshots__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshots"));
                            }
                            snapshots__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Charts => {
                            if charts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("charts"));
                            }
                            charts__ = Some(
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
                    snapshots: snapshots__.unwrap_or_default(),
                    charts: charts__.unwrap_or_default(),
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
        if !self.snapshots.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ProjectFile", len)?;
        if !self.sources.is_empty() {
            struct_ser.serialize_field("sources", &self.sources)?;
        }
        if !self.models.is_empty() {
            struct_ser.serialize_field("models", &self.models)?;
        }
        if !self.snapshots.is_empty() {
            struct_ser.serialize_field("snapshots", &self.snapshots)?;
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
            "snapshots",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sources,
            Models,
            Snapshots,
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
                            "snapshots" => Ok(GeneratedField::Snapshots),
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
                let mut snapshots__ = None;
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
                        GeneratedField::Snapshots => {
                            if snapshots__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshots"));
                            }
                            snapshots__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProjectFile {
                    sources: sources__.unwrap_or_default(),
                    models: models__.unwrap_or_default(),
                    snapshots: snapshots__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ProjectFile", FIELDS, GeneratedVisitor)
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
        if !self.tags.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.materialization.is_some() {
            len += 1;
        }
        if !self.tests.is_empty() {
            len += 1;
        }
        if !self.columns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ProjectFile.Model", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.materialization.as_ref() {
            struct_ser.serialize_field("materialization", v)?;
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
impl<'de> serde::Deserialize<'de> for project_file::Model {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "tags",
            "description",
            "materialization",
            "tests",
            "columns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Tags,
            Description,
            Materialization,
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
                            "tags" => Ok(GeneratedField::Tags),
                            "description" => Ok(GeneratedField::Description),
                            "materialization" => Ok(GeneratedField::Materialization),
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
            type Value = project_file::Model;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ProjectFile.Model")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<project_file::Model, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut tags__ = None;
                let mut description__ = None;
                let mut materialization__ = None;
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
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Materialization => {
                            if materialization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("materialization"));
                            }
                            materialization__ = map_.next_value()?;
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
                Ok(project_file::Model {
                    name: name__.unwrap_or_default(),
                    tags: tags__.unwrap_or_default(),
                    description: description__,
                    materialization: materialization__,
                    tests: tests__.unwrap_or_default(),
                    columns: columns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ProjectFile.Model", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for project_file::Snapshot {
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
        if !self.tags.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if !self.unique_key.is_empty() {
            len += 1;
        }
        if self.strategy.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ProjectFile.Snapshot", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.unique_key.is_empty() {
            struct_ser.serialize_field("uniqueKey", &self.unique_key)?;
        }
        if let Some(v) = self.strategy.as_ref() {
            struct_ser.serialize_field("strategy", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for project_file::Snapshot {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "tags",
            "description",
            "unique_key",
            "uniqueKey",
            "strategy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Tags,
            Description,
            UniqueKey,
            Strategy,
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
                            "tags" => Ok(GeneratedField::Tags),
                            "description" => Ok(GeneratedField::Description),
                            "uniqueKey" | "unique_key" => Ok(GeneratedField::UniqueKey),
                            "strategy" => Ok(GeneratedField::Strategy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = project_file::Snapshot;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ProjectFile.Snapshot")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<project_file::Snapshot, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut tags__ = None;
                let mut description__ = None;
                let mut unique_key__ = None;
                let mut strategy__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::UniqueKey => {
                            if unique_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uniqueKey"));
                            }
                            unique_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Strategy => {
                            if strategy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("strategy"));
                            }
                            strategy__ = map_.next_value()?;
                        }
                    }
                }
                Ok(project_file::Snapshot {
                    name: name__.unwrap_or_default(),
                    tags: tags__.unwrap_or_default(),
                    description: description__,
                    unique_key: unique_key__.unwrap_or_default(),
                    strategy: strategy__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ProjectFile.Snapshot", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for project_file::SnapshotStrategy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.strategy_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ProjectFile.SnapshotStrategy", len)?;
        if let Some(v) = self.strategy_type.as_ref() {
            match v {
                project_file::snapshot_strategy::StrategyType::Timestamp(v) => {
                    struct_ser.serialize_field("timestamp", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for project_file::SnapshotStrategy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamp,
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
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = project_file::SnapshotStrategy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ProjectFile.SnapshotStrategy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<project_file::SnapshotStrategy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut strategy_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Timestamp => {
                            if strategy_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            strategy_type__ = map_.next_value::<::std::option::Option<_>>()?.map(project_file::snapshot_strategy::StrategyType::Timestamp)
;
                        }
                    }
                }
                Ok(project_file::SnapshotStrategy {
                    strategy_type: strategy_type__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ProjectFile.SnapshotStrategy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for project_file::TimestampStrategy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.updated_at.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ProjectFile.TimestampStrategy", len)?;
        if !self.updated_at.is_empty() {
            struct_ser.serialize_field("updatedAt", &self.updated_at)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for project_file::TimestampStrategy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UpdatedAt,
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
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = project_file::TimestampStrategy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ProjectFile.TimestampStrategy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<project_file::TimestampStrategy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(project_file::TimestampStrategy {
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ProjectFile.TimestampStrategy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectFileColumn {
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
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ProjectFileColumn", len)?;
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
impl<'de> serde::Deserialize<'de> for ProjectFileColumn {
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
            type Value = ProjectFileColumn;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ProjectFileColumn")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProjectFileColumn, V::Error>
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
                Ok(ProjectFileColumn {
                    name: name__.unwrap_or_default(),
                    description: description__,
                    tests: tests__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ProjectFileColumn", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectFileSource {
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
        if !self.tags.is_empty() {
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
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ProjectFileSource", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
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
impl<'de> serde::Deserialize<'de> for ProjectFileSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "tags",
            "description",
            "path",
            "tests",
            "columns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Tags,
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
                            "tags" => Ok(GeneratedField::Tags),
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
            type Value = ProjectFileSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ProjectFileSource")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProjectFileSource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut tags__ = None;
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
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map_.next_value()?);
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
                Ok(ProjectFileSource {
                    name: name__.unwrap_or_default(),
                    tags: tags__.unwrap_or_default(),
                    description: description__,
                    path: path__.unwrap_or_default(),
                    tests: tests__.unwrap_or_default(),
                    columns: columns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ProjectFileSource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.query.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.QueryRequest", len)?;
        if !self.query.is_empty() {
            struct_ser.serialize_field("query", &self.query)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "query",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Query,
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
                            "query" => Ok(GeneratedField::Query),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.QueryRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryRequest {
                    query: query__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.QueryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.QueryResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
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
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.QueryResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.QueryResponse", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for RemoveColumnTestFromModelOrSourceColumnRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        if !self.model_or_source_name.is_empty() {
            len += 1;
        }
        if !self.column_name.is_empty() {
            len += 1;
        }
        if self.column_test.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.RemoveColumnTestFromModelOrSourceColumnRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        if !self.model_or_source_name.is_empty() {
            struct_ser.serialize_field("modelOrSourceName", &self.model_or_source_name)?;
        }
        if !self.column_name.is_empty() {
            struct_ser.serialize_field("columnName", &self.column_name)?;
        }
        if let Some(v) = self.column_test.as_ref() {
            struct_ser.serialize_field("columnTest", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveColumnTestFromModelOrSourceColumnRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
            "model_or_source_name",
            "modelOrSourceName",
            "column_name",
            "columnName",
            "column_test",
            "columnTest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
            ModelOrSourceName,
            ColumnName,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            "modelOrSourceName" | "model_or_source_name" => Ok(GeneratedField::ModelOrSourceName),
                            "columnName" | "column_name" => Ok(GeneratedField::ColumnName),
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
            type Value = RemoveColumnTestFromModelOrSourceColumnRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.RemoveColumnTestFromModelOrSourceColumnRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveColumnTestFromModelOrSourceColumnRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                let mut model_or_source_name__ = None;
                let mut column_name__ = None;
                let mut column_test__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModelOrSourceName => {
                            if model_or_source_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelOrSourceName"));
                            }
                            model_or_source_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ColumnName => {
                            if column_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnName"));
                            }
                            column_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ColumnTest => {
                            if column_test__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnTest"));
                            }
                            column_test__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RemoveColumnTestFromModelOrSourceColumnRequest {
                    project_root: project_root__.unwrap_or_default(),
                    model_or_source_name: model_or_source_name__.unwrap_or_default(),
                    column_name: column_name__.unwrap_or_default(),
                    column_test: column_test__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.RemoveColumnTestFromModelOrSourceColumnRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveColumnTestFromModelOrSourceColumnResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("quary.service.v1.RemoveColumnTestFromModelOrSourceColumnResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveColumnTestFromModelOrSourceColumnResponse {
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
            type Value = RemoveColumnTestFromModelOrSourceColumnResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.RemoveColumnTestFromModelOrSourceColumnResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveColumnTestFromModelOrSourceColumnResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RemoveColumnTestFromModelOrSourceColumnResponse {
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.RemoveColumnTestFromModelOrSourceColumnResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RenderSchemaRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.RenderSchemaRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RenderSchemaRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RenderSchemaRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.RenderSchemaRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RenderSchemaRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RenderSchemaRequest {
                    project_root: project_root__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.RenderSchemaRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RenderSchemaResponse {
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
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.RenderSchemaResponse", len)?;
        if !self.schema.is_empty() {
            struct_ser.serialize_field("schema", &self.schema)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RenderSchemaResponse {
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
            type Value = RenderSchemaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.RenderSchemaResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RenderSchemaResponse, V::Error>
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
                Ok(RenderSchemaResponse {
                    schema: schema__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.RenderSchemaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReturnDataForDocViewRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        if !self.asset_name.is_empty() {
            len += 1;
        }
        if self.cache_view_information.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ReturnDataForDocViewRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        if !self.asset_name.is_empty() {
            struct_ser.serialize_field("assetName", &self.asset_name)?;
        }
        if let Some(v) = self.cache_view_information.as_ref() {
            struct_ser.serialize_field("cacheViewInformation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReturnDataForDocViewRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
            "asset_name",
            "assetName",
            "cache_view_information",
            "cacheViewInformation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
            AssetName,
            CacheViewInformation,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            "assetName" | "asset_name" => Ok(GeneratedField::AssetName),
                            "cacheViewInformation" | "cache_view_information" => Ok(GeneratedField::CacheViewInformation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReturnDataForDocViewRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ReturnDataForDocViewRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReturnDataForDocViewRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                let mut asset_name__ = None;
                let mut cache_view_information__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetName => {
                            if asset_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetName"));
                            }
                            asset_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CacheViewInformation => {
                            if cache_view_information__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheViewInformation"));
                            }
                            cache_view_information__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReturnDataForDocViewRequest {
                    project_root: project_root__.unwrap_or_default(),
                    asset_name: asset_name__.unwrap_or_default(),
                    cache_view_information: cache_view_information__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ReturnDataForDocViewRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReturnDataForDocViewResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.full_sql.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.dag.is_some() {
            len += 1;
        }
        if !self.columns.is_empty() {
            len += 1;
        }
        if self.is_asset_in_schema_files {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ReturnDataForDocViewResponse", len)?;
        if !self.full_sql.is_empty() {
            struct_ser.serialize_field("fullSql", &self.full_sql)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.dag.as_ref() {
            struct_ser.serialize_field("dag", v)?;
        }
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        if self.is_asset_in_schema_files {
            struct_ser.serialize_field("isAssetInSchemaFiles", &self.is_asset_in_schema_files)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReturnDataForDocViewResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "full_sql",
            "fullSql",
            "description",
            "dag",
            "columns",
            "is_asset_in_schema_files",
            "isAssetInSchemaFiles",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FullSql,
            Description,
            Dag,
            Columns,
            IsAssetInSchemaFiles,
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
                            "fullSql" | "full_sql" => Ok(GeneratedField::FullSql),
                            "description" => Ok(GeneratedField::Description),
                            "dag" => Ok(GeneratedField::Dag),
                            "columns" => Ok(GeneratedField::Columns),
                            "isAssetInSchemaFiles" | "is_asset_in_schema_files" => Ok(GeneratedField::IsAssetInSchemaFiles),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReturnDataForDocViewResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ReturnDataForDocViewResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReturnDataForDocViewResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut full_sql__ = None;
                let mut description__ = None;
                let mut dag__ = None;
                let mut columns__ = None;
                let mut is_asset_in_schema_files__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FullSql => {
                            if full_sql__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullSql"));
                            }
                            full_sql__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Dag => {
                            if dag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dag"));
                            }
                            dag__ = map_.next_value()?;
                        }
                        GeneratedField::Columns => {
                            if columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columns"));
                            }
                            columns__ = Some(map_.next_value()?);
                        }
                        GeneratedField::IsAssetInSchemaFiles => {
                            if is_asset_in_schema_files__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isAssetInSchemaFiles"));
                            }
                            is_asset_in_schema_files__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReturnDataForDocViewResponse {
                    full_sql: full_sql__.unwrap_or_default(),
                    description: description__,
                    dag: dag__,
                    columns: columns__.unwrap_or_default(),
                    is_asset_in_schema_files: is_asset_in_schema_files__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ReturnDataForDocViewResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReturnDefinitionLocationsForSqlRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        if !self.sql.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ReturnDefinitionLocationsForSQLRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        if !self.sql.is_empty() {
            struct_ser.serialize_field("sql", &self.sql)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReturnDefinitionLocationsForSqlRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
            "sql",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
            Sql,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            "sql" => Ok(GeneratedField::Sql),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReturnDefinitionLocationsForSqlRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ReturnDefinitionLocationsForSQLRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReturnDefinitionLocationsForSqlRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                let mut sql__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sql => {
                            if sql__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sql"));
                            }
                            sql__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReturnDefinitionLocationsForSqlRequest {
                    project_root: project_root__.unwrap_or_default(),
                    sql: sql__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ReturnDefinitionLocationsForSQLRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReturnDefinitionLocationsForSqlResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.definitions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ReturnDefinitionLocationsForSQLResponse", len)?;
        if !self.definitions.is_empty() {
            struct_ser.serialize_field("definitions", &self.definitions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReturnDefinitionLocationsForSqlResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "definitions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Definitions,
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
                            "definitions" => Ok(GeneratedField::Definitions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReturnDefinitionLocationsForSqlResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ReturnDefinitionLocationsForSQLResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReturnDefinitionLocationsForSqlResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut definitions__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Definitions => {
                            if definitions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("definitions"));
                            }
                            definitions__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReturnDefinitionLocationsForSqlResponse {
                    definitions: definitions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ReturnDefinitionLocationsForSQLResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for return_definition_locations_for_sql_response::Definition {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.range.is_some() {
            len += 1;
        }
        if !self.target_model.is_empty() {
            len += 1;
        }
        if !self.target_file.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ReturnDefinitionLocationsForSQLResponse.Definition", len)?;
        if let Some(v) = self.range.as_ref() {
            struct_ser.serialize_field("range", v)?;
        }
        if !self.target_model.is_empty() {
            struct_ser.serialize_field("targetModel", &self.target_model)?;
        }
        if !self.target_file.is_empty() {
            struct_ser.serialize_field("targetFile", &self.target_file)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for return_definition_locations_for_sql_response::Definition {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "range",
            "target_model",
            "targetModel",
            "target_file",
            "targetFile",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Range,
            TargetModel,
            TargetFile,
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
                            "range" => Ok(GeneratedField::Range),
                            "targetModel" | "target_model" => Ok(GeneratedField::TargetModel),
                            "targetFile" | "target_file" => Ok(GeneratedField::TargetFile),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = return_definition_locations_for_sql_response::Definition;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ReturnDefinitionLocationsForSQLResponse.Definition")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<return_definition_locations_for_sql_response::Definition, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut range__ = None;
                let mut target_model__ = None;
                let mut target_file__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Range => {
                            if range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("range"));
                            }
                            range__ = map_.next_value()?;
                        }
                        GeneratedField::TargetModel => {
                            if target_model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetModel"));
                            }
                            target_model__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TargetFile => {
                            if target_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetFile"));
                            }
                            target_file__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(return_definition_locations_for_sql_response::Definition {
                    range: range__,
                    target_model: target_model__.unwrap_or_default(),
                    target_file: target_file__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ReturnDefinitionLocationsForSQLResponse.Definition", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReturnFullProjectDagRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ReturnFullProjectDagRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReturnFullProjectDagRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReturnFullProjectDagRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ReturnFullProjectDagRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReturnFullProjectDagRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReturnFullProjectDagRequest {
                    project_root: project_root__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ReturnFullProjectDagRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReturnFullProjectDagResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.dag.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ReturnFullProjectDagResponse", len)?;
        if let Some(v) = self.dag.as_ref() {
            struct_ser.serialize_field("dag", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReturnFullProjectDagResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dag",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Dag,
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
                            "dag" => Ok(GeneratedField::Dag),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReturnFullProjectDagResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ReturnFullProjectDagResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReturnFullProjectDagResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dag__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Dag => {
                            if dag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dag"));
                            }
                            dag__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReturnFullProjectDagResponse {
                    dag: dag__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ReturnFullProjectDagResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReturnFullSqlForAssetRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        if !self.asset_name.is_empty() {
            len += 1;
        }
        if self.cache_view_information.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ReturnFullSqlForAssetRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        if !self.asset_name.is_empty() {
            struct_ser.serialize_field("assetName", &self.asset_name)?;
        }
        if let Some(v) = self.cache_view_information.as_ref() {
            struct_ser.serialize_field("cacheViewInformation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReturnFullSqlForAssetRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
            "asset_name",
            "assetName",
            "cache_view_information",
            "cacheViewInformation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
            AssetName,
            CacheViewInformation,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            "assetName" | "asset_name" => Ok(GeneratedField::AssetName),
                            "cacheViewInformation" | "cache_view_information" => Ok(GeneratedField::CacheViewInformation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReturnFullSqlForAssetRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ReturnFullSqlForAssetRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReturnFullSqlForAssetRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                let mut asset_name__ = None;
                let mut cache_view_information__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetName => {
                            if asset_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetName"));
                            }
                            asset_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CacheViewInformation => {
                            if cache_view_information__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheViewInformation"));
                            }
                            cache_view_information__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReturnFullSqlForAssetRequest {
                    project_root: project_root__.unwrap_or_default(),
                    asset_name: asset_name__.unwrap_or_default(),
                    cache_view_information: cache_view_information__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ReturnFullSqlForAssetRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReturnFullSqlForAssetResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.full_sql.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.dag.is_some() {
            len += 1;
        }
        if !self.columns.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ReturnFullSqlForAssetResponse", len)?;
        if !self.full_sql.is_empty() {
            struct_ser.serialize_field("fullSql", &self.full_sql)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.dag.as_ref() {
            struct_ser.serialize_field("dag", v)?;
        }
        if !self.columns.is_empty() {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReturnFullSqlForAssetResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "full_sql",
            "fullSql",
            "description",
            "dag",
            "columns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FullSql,
            Description,
            Dag,
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
                            "fullSql" | "full_sql" => Ok(GeneratedField::FullSql),
                            "description" => Ok(GeneratedField::Description),
                            "dag" => Ok(GeneratedField::Dag),
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
            type Value = ReturnFullSqlForAssetResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ReturnFullSqlForAssetResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReturnFullSqlForAssetResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut full_sql__ = None;
                let mut description__ = None;
                let mut dag__ = None;
                let mut columns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FullSql => {
                            if full_sql__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullSql"));
                            }
                            full_sql__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Dag => {
                            if dag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dag"));
                            }
                            dag__ = map_.next_value()?;
                        }
                        GeneratedField::Columns => {
                            if columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columns"));
                            }
                            columns__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReturnFullSqlForAssetResponse {
                    full_sql: full_sql__.unwrap_or_default(),
                    description: description__,
                    dag: dag__,
                    columns: columns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ReturnFullSqlForAssetResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReturnSqlForInjectedModelRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        if !self.sql.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ReturnSQLForInjectedModelRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        if !self.sql.is_empty() {
            struct_ser.serialize_field("sql", &self.sql)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReturnSqlForInjectedModelRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
            "sql",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
            Sql,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            "sql" => Ok(GeneratedField::Sql),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReturnSqlForInjectedModelRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ReturnSQLForInjectedModelRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReturnSqlForInjectedModelRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                let mut sql__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sql => {
                            if sql__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sql"));
                            }
                            sql__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReturnSqlForInjectedModelRequest {
                    project_root: project_root__.unwrap_or_default(),
                    sql: sql__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ReturnSQLForInjectedModelRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReturnSqlForInjectedModelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sql.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ReturnSQLForInjectedModelResponse", len)?;
        if !self.sql.is_empty() {
            struct_ser.serialize_field("sql", &self.sql)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReturnSqlForInjectedModelResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sql",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sql,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReturnSqlForInjectedModelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ReturnSQLForInjectedModelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReturnSqlForInjectedModelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sql__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sql => {
                            if sql__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sql"));
                            }
                            sql__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReturnSqlForInjectedModelResponse {
                    sql: sql__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ReturnSQLForInjectedModelResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReturnSqlForSeedsAndModelsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        if !self.db_qualifier.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ReturnSQLForSeedsAndModelsRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        if !self.db_qualifier.is_empty() {
            struct_ser.serialize_field("dbQualifier", &self.db_qualifier)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReturnSqlForSeedsAndModelsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
            "db_qualifier",
            "dbQualifier",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
            DbQualifier,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            "dbQualifier" | "db_qualifier" => Ok(GeneratedField::DbQualifier),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReturnSqlForSeedsAndModelsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ReturnSQLForSeedsAndModelsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReturnSqlForSeedsAndModelsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                let mut db_qualifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DbQualifier => {
                            if db_qualifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbQualifier"));
                            }
                            db_qualifier__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReturnSqlForSeedsAndModelsRequest {
                    project_root: project_root__.unwrap_or_default(),
                    db_qualifier: db_qualifier__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ReturnSQLForSeedsAndModelsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReturnSqlForSeedsAndModelsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sql.is_empty() {
            len += 1;
        }
        if self.project.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.ReturnSQLForSeedsAndModelsResponse", len)?;
        if !self.sql.is_empty() {
            struct_ser.serialize_field("sql", &self.sql)?;
        }
        if let Some(v) = self.project.as_ref() {
            struct_ser.serialize_field("project", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReturnSqlForSeedsAndModelsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sql",
            "project",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sql,
            Project,
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
                            "project" => Ok(GeneratedField::Project),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReturnSqlForSeedsAndModelsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.ReturnSQLForSeedsAndModelsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReturnSqlForSeedsAndModelsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sql__ = None;
                let mut project__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sql => {
                            if sql__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sql"));
                            }
                            sql__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Project => {
                            if project__.is_some() {
                                return Err(serde::de::Error::duplicate_field("project"));
                            }
                            project__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReturnSqlForSeedsAndModelsResponse {
                    sql: sql__.unwrap_or_default(),
                    project: project__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.ReturnSQLForSeedsAndModelsResponse", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for Snapshot {
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
        if !self.tags.is_empty() {
            len += 1;
        }
        if !self.file_path.is_empty() {
            len += 1;
        }
        if !self.file_sha256_hash.is_empty() {
            len += 1;
        }
        if !self.unique_key.is_empty() {
            len += 1;
        }
        if self.strategy.is_some() {
            len += 1;
        }
        if !self.references.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Snapshot", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if !self.file_path.is_empty() {
            struct_ser.serialize_field("filePath", &self.file_path)?;
        }
        if !self.file_sha256_hash.is_empty() {
            struct_ser.serialize_field("fileSha256Hash", &self.file_sha256_hash)?;
        }
        if !self.unique_key.is_empty() {
            struct_ser.serialize_field("uniqueKey", &self.unique_key)?;
        }
        if let Some(v) = self.strategy.as_ref() {
            struct_ser.serialize_field("strategy", v)?;
        }
        if !self.references.is_empty() {
            struct_ser.serialize_field("references", &self.references)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Snapshot {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
            "tags",
            "file_path",
            "filePath",
            "file_sha256_hash",
            "fileSha256Hash",
            "unique_key",
            "uniqueKey",
            "strategy",
            "references",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Tags,
            FilePath,
            FileSha256Hash,
            UniqueKey,
            Strategy,
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
                            "tags" => Ok(GeneratedField::Tags),
                            "filePath" | "file_path" => Ok(GeneratedField::FilePath),
                            "fileSha256Hash" | "file_sha256_hash" => Ok(GeneratedField::FileSha256Hash),
                            "uniqueKey" | "unique_key" => Ok(GeneratedField::UniqueKey),
                            "strategy" => Ok(GeneratedField::Strategy),
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
            type Value = Snapshot;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Snapshot")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Snapshot, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                let mut tags__ = None;
                let mut file_path__ = None;
                let mut file_sha256_hash__ = None;
                let mut unique_key__ = None;
                let mut strategy__ = None;
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
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map_.next_value()?);
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
                        GeneratedField::UniqueKey => {
                            if unique_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uniqueKey"));
                            }
                            unique_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Strategy => {
                            if strategy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("strategy"));
                            }
                            strategy__ = map_.next_value()?;
                        }
                        GeneratedField::References => {
                            if references__.is_some() {
                                return Err(serde::de::Error::duplicate_field("references"));
                            }
                            references__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Snapshot {
                    name: name__.unwrap_or_default(),
                    description: description__,
                    tags: tags__.unwrap_or_default(),
                    file_path: file_path__.unwrap_or_default(),
                    file_sha256_hash: file_sha256_hash__.unwrap_or_default(),
                    unique_key: unique_key__.unwrap_or_default(),
                    strategy: strategy__,
                    references: references__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Snapshot", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for snapshot::SnapshotStrategy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.strategy_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Snapshot.SnapshotStrategy", len)?;
        if let Some(v) = self.strategy_type.as_ref() {
            match v {
                snapshot::snapshot_strategy::StrategyType::Timestamp(v) => {
                    struct_ser.serialize_field("timestamp", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for snapshot::SnapshotStrategy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamp,
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
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = snapshot::SnapshotStrategy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Snapshot.SnapshotStrategy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<snapshot::SnapshotStrategy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut strategy_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Timestamp => {
                            if strategy_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            strategy_type__ = map_.next_value::<::std::option::Option<_>>()?.map(snapshot::snapshot_strategy::StrategyType::Timestamp)
;
                        }
                    }
                }
                Ok(snapshot::SnapshotStrategy {
                    strategy_type: strategy_type__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Snapshot.SnapshotStrategy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for snapshot::snapshot_strategy::TimestampStrategy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.updated_at.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.Snapshot.SnapshotStrategy.TimestampStrategy", len)?;
        if !self.updated_at.is_empty() {
            struct_ser.serialize_field("updatedAt", &self.updated_at)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for snapshot::snapshot_strategy::TimestampStrategy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "updated_at",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UpdatedAt,
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
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = snapshot::snapshot_strategy::TimestampStrategy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.Snapshot.SnapshotStrategy.TimestampStrategy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<snapshot::snapshot_strategy::TimestampStrategy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut updated_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(snapshot::snapshot_strategy::TimestampStrategy {
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.Snapshot.SnapshotStrategy.TimestampStrategy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SnowflakeOauthProxyRequest {
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
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.SnowflakeOauthProxyRequest", len)?;
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
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SnowflakeOauthProxyRequest {
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccountUrl,
            ClientId,
            ClientSecret,
            Role,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SnowflakeOauthProxyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.SnowflakeOauthProxyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SnowflakeOauthProxyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account_url__ = None;
                let mut client_id__ = None;
                let mut client_secret__ = None;
                let mut role__ = None;
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
                    }
                }
                Ok(SnowflakeOauthProxyRequest {
                    account_url: account_url__.unwrap_or_default(),
                    client_id: client_id__.unwrap_or_default(),
                    client_secret: client_secret__.unwrap_or_default(),
                    role: role__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.SnowflakeOauthProxyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SnowflakeOauthRefreshToken {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.access_token.is_empty() {
            len += 1;
        }
        if !self.expiry_time.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.SnowflakeOauthRefreshToken", len)?;
        if !self.access_token.is_empty() {
            struct_ser.serialize_field("accessToken", &self.access_token)?;
        }
        if !self.expiry_time.is_empty() {
            struct_ser.serialize_field("expiryTime", &self.expiry_time)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SnowflakeOauthRefreshToken {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "access_token",
            "accessToken",
            "expiry_time",
            "expiryTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccessToken,
            ExpiryTime,
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
                            "accessToken" | "access_token" => Ok(GeneratedField::AccessToken),
                            "expiryTime" | "expiry_time" => Ok(GeneratedField::ExpiryTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SnowflakeOauthRefreshToken;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.SnowflakeOauthRefreshToken")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SnowflakeOauthRefreshToken, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut access_token__ = None;
                let mut expiry_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccessToken => {
                            if access_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessToken"));
                            }
                            access_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExpiryTime => {
                            if expiry_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiryTime"));
                            }
                            expiry_time__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SnowflakeOauthRefreshToken {
                    access_token: access_token__.unwrap_or_default(),
                    expiry_time: expiry_time__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.SnowflakeOauthRefreshToken", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SnowflakeOauthToken {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.access_token.is_empty() {
            len += 1;
        }
        if !self.refresh_token.is_empty() {
            len += 1;
        }
        if !self.expiry_time.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.SnowflakeOauthToken", len)?;
        if !self.access_token.is_empty() {
            struct_ser.serialize_field("accessToken", &self.access_token)?;
        }
        if !self.refresh_token.is_empty() {
            struct_ser.serialize_field("refreshToken", &self.refresh_token)?;
        }
        if !self.expiry_time.is_empty() {
            struct_ser.serialize_field("expiryTime", &self.expiry_time)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SnowflakeOauthToken {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "access_token",
            "accessToken",
            "refresh_token",
            "refreshToken",
            "expiry_time",
            "expiryTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccessToken,
            RefreshToken,
            ExpiryTime,
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
                            "accessToken" | "access_token" => Ok(GeneratedField::AccessToken),
                            "refreshToken" | "refresh_token" => Ok(GeneratedField::RefreshToken),
                            "expiryTime" | "expiry_time" => Ok(GeneratedField::ExpiryTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SnowflakeOauthToken;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.SnowflakeOauthToken")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SnowflakeOauthToken, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut access_token__ = None;
                let mut refresh_token__ = None;
                let mut expiry_time__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AccessToken => {
                            if access_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessToken"));
                            }
                            access_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RefreshToken => {
                            if refresh_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshToken"));
                            }
                            refresh_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExpiryTime => {
                            if expiry_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiryTime"));
                            }
                            expiry_time__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SnowflakeOauthToken {
                    access_token: access_token__.unwrap_or_default(),
                    refresh_token: refresh_token__.unwrap_or_default(),
                    expiry_time: expiry_time__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.SnowflakeOauthToken", FIELDS, GeneratedVisitor)
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
        if !self.tags.is_empty() {
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
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
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
            "tags",
            "file_path",
            "filePath",
            "columns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
            Path,
            Tags,
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
                            "tags" => Ok(GeneratedField::Tags),
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
                let mut tags__ = None;
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
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map_.next_value()?);
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
                    tags: tags__.unwrap_or_default(),
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
impl serde::Serialize for StringifyProjectFileRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.project_file.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.StringifyProjectFileRequest", len)?;
        if let Some(v) = self.project_file.as_ref() {
            struct_ser.serialize_field("projectFile", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StringifyProjectFileRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_file",
            "projectFile",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectFile,
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
                            "projectFile" | "project_file" => Ok(GeneratedField::ProjectFile),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StringifyProjectFileRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.StringifyProjectFileRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StringifyProjectFileRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_file__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectFile => {
                            if project_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectFile"));
                            }
                            project_file__ = map_.next_value()?;
                        }
                    }
                }
                Ok(StringifyProjectFileRequest {
                    project_file: project_file__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.StringifyProjectFileRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StringifyProjectFileResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stringified_project_file.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.StringifyProjectFileResponse", len)?;
        if !self.stringified_project_file.is_empty() {
            struct_ser.serialize_field("stringifiedProjectFile", &self.stringified_project_file)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StringifyProjectFileResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stringified_project_file",
            "stringifiedProjectFile",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StringifiedProjectFile,
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
                            "stringifiedProjectFile" | "stringified_project_file" => Ok(GeneratedField::StringifiedProjectFile),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StringifyProjectFileResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.StringifyProjectFileResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StringifyProjectFileResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stringified_project_file__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StringifiedProjectFile => {
                            if stringified_project_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringifiedProjectFile"));
                            }
                            stringified_project_file__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StringifyProjectFileResponse {
                    stringified_project_file: stringified_project_file__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.StringifyProjectFileResponse", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for TableAddress {
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
        if !self.full_path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.TableAddress", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.full_path.is_empty() {
            struct_ser.serialize_field("fullPath", &self.full_path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TableAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "full_path",
            "fullPath",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            FullPath,
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
                            "fullPath" | "full_path" => Ok(GeneratedField::FullPath),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TableAddress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.TableAddress")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TableAddress, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut full_path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FullPath => {
                            if full_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullPath"));
                            }
                            full_path__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TableAddress {
                    name: name__.unwrap_or_default(),
                    full_path: full_path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.TableAddress", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for UpdateAssetDescriptionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        if !self.asset_name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.UpdateAssetDescriptionRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        if !self.asset_name.is_empty() {
            struct_ser.serialize_field("assetName", &self.asset_name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAssetDescriptionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
            "asset_name",
            "assetName",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
            AssetName,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            "assetName" | "asset_name" => Ok(GeneratedField::AssetName),
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
            type Value = UpdateAssetDescriptionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.UpdateAssetDescriptionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAssetDescriptionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                let mut asset_name__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AssetName => {
                            if asset_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assetName"));
                            }
                            asset_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateAssetDescriptionRequest {
                    project_root: project_root__.unwrap_or_default(),
                    asset_name: asset_name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.UpdateAssetDescriptionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateAssetDescriptionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("quary.service.v1.UpdateAssetDescriptionResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAssetDescriptionResponse {
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
            type Value = UpdateAssetDescriptionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.UpdateAssetDescriptionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateAssetDescriptionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UpdateAssetDescriptionResponse {
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.UpdateAssetDescriptionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateModelOrSourceColumnDescriptionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_root.is_empty() {
            len += 1;
        }
        if !self.model_or_source_name.is_empty() {
            len += 1;
        }
        if !self.column_name.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("quary.service.v1.UpdateModelOrSourceColumnDescriptionRequest", len)?;
        if !self.project_root.is_empty() {
            struct_ser.serialize_field("projectRoot", &self.project_root)?;
        }
        if !self.model_or_source_name.is_empty() {
            struct_ser.serialize_field("modelOrSourceName", &self.model_or_source_name)?;
        }
        if !self.column_name.is_empty() {
            struct_ser.serialize_field("columnName", &self.column_name)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateModelOrSourceColumnDescriptionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_root",
            "projectRoot",
            "model_or_source_name",
            "modelOrSourceName",
            "column_name",
            "columnName",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectRoot,
            ModelOrSourceName,
            ColumnName,
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
                            "projectRoot" | "project_root" => Ok(GeneratedField::ProjectRoot),
                            "modelOrSourceName" | "model_or_source_name" => Ok(GeneratedField::ModelOrSourceName),
                            "columnName" | "column_name" => Ok(GeneratedField::ColumnName),
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
            type Value = UpdateModelOrSourceColumnDescriptionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.UpdateModelOrSourceColumnDescriptionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateModelOrSourceColumnDescriptionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_root__ = None;
                let mut model_or_source_name__ = None;
                let mut column_name__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProjectRoot => {
                            if project_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectRoot"));
                            }
                            project_root__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModelOrSourceName => {
                            if model_or_source_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelOrSourceName"));
                            }
                            model_or_source_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ColumnName => {
                            if column_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columnName"));
                            }
                            column_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateModelOrSourceColumnDescriptionRequest {
                    project_root: project_root__.unwrap_or_default(),
                    model_or_source_name: model_or_source_name__.unwrap_or_default(),
                    column_name: column_name__.unwrap_or_default(),
                    description: description__,
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.UpdateModelOrSourceColumnDescriptionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateModelOrSourceColumnDescriptionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("quary.service.v1.UpdateModelOrSourceColumnDescriptionResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateModelOrSourceColumnDescriptionResponse {
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
            type Value = UpdateModelOrSourceColumnDescriptionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct quary.service.v1.UpdateModelOrSourceColumnDescriptionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateModelOrSourceColumnDescriptionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UpdateModelOrSourceColumnDescriptionResponse {
                })
            }
        }
        deserializer.deserialize_struct("quary.service.v1.UpdateModelOrSourceColumnDescriptionResponse", FIELDS, GeneratedVisitor)
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
