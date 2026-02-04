use crate::cloudkit::types::FieldValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    #[serde(rename = "recordType")]
    pub record_type: String,
    #[serde(rename = "filterBy", skip_serializing_if = "Option::is_none")]
    pub filter_by: Option<Vec<Filter>>,
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<Vec<SortDescriptor>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "fieldName")]
    pub field_name: String,
    pub comparator: Comparator,
    #[serde(rename = "fieldValue")]
    pub field_value: FieldValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Comparator {
    #[serde(rename = "EQUALS")]
    Equals,
    #[serde(rename = "NOT_EQUALS")]
    NotEquals,
    #[serde(rename = "GREATER_THAN")]
    GreaterThan,
    #[serde(rename = "GREATER_THAN_OR_EQUALS")]
    GreaterThanOrEquals,
    #[serde(rename = "LESS_THAN")]
    LessThan,
    #[serde(rename = "LESS_THAN_OR_EQUALS")]
    LessThanOrEquals,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "NOT_IN")]
    NotIn,
    #[serde(rename = "NEAR")]
    Near,
    #[serde(rename = "BEGINS_WITH")]
    BeginsWith,
    #[serde(rename = "CONTAINS_ALL_TOKENS")]
    ContainsAllTokens,
    #[serde(rename = "CONTAINS_ANY_TOKENS")]
    ContainsAnyTokens,
    #[serde(rename = "LIST_CONTAINS")]
    ListContains,
    #[serde(rename = "LIST_NOT_CONTAINS")]
    ListNotContains,
    #[serde(rename = "LIST_CONTAINS_ALL")]
    ListContainsAll,
    #[serde(rename = "LIST_CONTAINS_ANY")]
    ListContainsAny,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortDescriptor {
    #[serde(rename = "fieldName")]
    pub field_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,
}

pub struct QueryBuilder {
    record_type: String,
    filters: Vec<Filter>,
    sorts: Vec<SortDescriptor>,
}

impl QueryBuilder {
    pub fn new(record_type: &str) -> Self {
        QueryBuilder {
            record_type: record_type.to_string(),
            filters: Vec::new(),
            sorts: Vec::new(),
        }
    }

    pub fn filter(mut self, field_name: &str, comparator: Comparator, value: FieldValue) -> Self {
        self.filters.push(Filter {
            field_name: field_name.to_string(),
            comparator,
            field_value: value,
        });
        self
    }

    pub fn sort(mut self, field_name: &str, ascending: bool) -> Self {
        self.sorts.push(SortDescriptor {
            field_name: field_name.to_string(),
            ascending: Some(ascending),
        });
        self
    }

    pub fn build(self) -> Query {
        Query {
            record_type: self.record_type,
            filter_by: if self.filters.is_empty() { None } else { Some(self.filters) },
            sort_by: if self.sorts.is_empty() { None } else { Some(self.sorts) },
        }
    }
}
