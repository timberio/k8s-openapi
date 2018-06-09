// Generated from definition io.k8s.kubernetes.pkg.apis.autoscaling.v2alpha1.ObjectMetricStatus

/// ObjectMetricStatus indicates the current value of a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).
#[derive(Debug, Default)]
pub struct ObjectMetricStatus {
    /// currentValue is the current value of the metric (as a quantity).
    pub current_value: ::v1_7::apimachinery::pkg::api::resource::Quantity,

    /// metricName is the name of the metric in question.
    pub metric_name: String,

    /// target is the described Kubernetes object.
    pub target: ::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::CrossVersionObjectReference,
}

impl<'de> ::serde::Deserialize<'de> for ObjectMetricStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_current_value,
            Key_metric_name,
            Key_target,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "currentValue" => Field::Key_current_value,
                            "metricName" => Field::Key_metric_name,
                            "target" => Field::Key_target,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ObjectMetricStatus;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ObjectMetricStatus")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_current_value: Option<::v1_7::apimachinery::pkg::api::resource::Quantity> = None;
                let mut value_metric_name: Option<String> = None;
                let mut value_target: Option<::v1_7::kubernetes::pkg::apis::autoscaling::v2alpha1::CrossVersionObjectReference> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_current_value => value_current_value = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_metric_name => value_metric_name = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_target => value_target = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ObjectMetricStatus {
                    current_value: value_current_value.ok_or_else(|| ::serde::de::Error::missing_field("currentValue"))?,
                    metric_name: value_metric_name.ok_or_else(|| ::serde::de::Error::missing_field("metricName"))?,
                    target: value_target.ok_or_else(|| ::serde::de::Error::missing_field("target"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ObjectMetricStatus",
            &[
                "currentValue",
                "metricName",
                "target",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ObjectMetricStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ObjectMetricStatus",
            0 +
            1 +
            1 +
            1,
        )?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "currentValue", &self.current_value)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "metricName", &self.metric_name)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "target", &self.target)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}