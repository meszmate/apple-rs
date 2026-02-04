#[cfg(feature = "cloudkit")]
mod cloudkit_query_tests {
    use apple::cloudkit::*;

    #[test]
    fn test_query_builder_simple() {
        let query = QueryBuilder::new("MyType").build();
        assert_eq!(query.record_type, "MyType");
        assert!(query.filter_by.is_none());
        assert!(query.sort_by.is_none());
    }

    #[test]
    fn test_query_builder_with_filter() {
        let query = QueryBuilder::new("Item")
            .filter("name", Comparator::Equals, FieldValue::String("test".into()))
            .build();

        assert_eq!(query.record_type, "Item");
        let filters = query.filter_by.unwrap();
        assert_eq!(filters.len(), 1);
        assert_eq!(filters[0].field_name, "name");
    }

    #[test]
    fn test_query_builder_with_sort() {
        let query = QueryBuilder::new("Item")
            .sort("createdAt", false)
            .build();

        let sorts = query.sort_by.unwrap();
        assert_eq!(sorts.len(), 1);
        assert_eq!(sorts[0].field_name, "createdAt");
        assert_eq!(sorts[0].ascending, Some(false));
    }

    #[test]
    fn test_query_builder_multiple_filters_and_sorts() {
        let query = QueryBuilder::new("Product")
            .filter("category", Comparator::Equals, FieldValue::String("electronics".into()))
            .filter("price", Comparator::LessThan, FieldValue::Double(100.0))
            .sort("price", true)
            .sort("name", true)
            .build();

        let filters = query.filter_by.unwrap();
        assert_eq!(filters.len(), 2);
        assert_eq!(filters[0].field_name, "category");
        assert_eq!(filters[1].field_name, "price");

        let sorts = query.sort_by.unwrap();
        assert_eq!(sorts.len(), 2);
        assert_eq!(sorts[0].field_name, "price");
        assert_eq!(sorts[1].field_name, "name");
    }

    #[test]
    fn test_query_serde_roundtrip() {
        let query = QueryBuilder::new("Task")
            .filter("status", Comparator::Equals, FieldValue::String("active".into()))
            .sort("priority", false)
            .build();

        let json = serde_json::to_string(&query).unwrap();
        assert!(json.contains("\"recordType\":\"Task\""));
        assert!(json.contains("\"filterBy\""));
        assert!(json.contains("\"sortBy\""));

        let deserialized: Query = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.record_type, "Task");
        assert_eq!(deserialized.filter_by.unwrap().len(), 1);
        assert_eq!(deserialized.sort_by.unwrap().len(), 1);
    }

    #[test]
    fn test_query_no_filters_serializes_without_filter_by() {
        let query = QueryBuilder::new("Simple").build();
        let json = serde_json::to_string(&query).unwrap();
        assert!(!json.contains("filterBy"));
        assert!(!json.contains("sortBy"));
    }

    #[test]
    fn test_comparator_serde() {
        let comparators = vec![
            (Comparator::Equals, "EQUALS"),
            (Comparator::NotEquals, "NOT_EQUALS"),
            (Comparator::GreaterThan, "GREATER_THAN"),
            (Comparator::GreaterThanOrEquals, "GREATER_THAN_OR_EQUALS"),
            (Comparator::LessThan, "LESS_THAN"),
            (Comparator::LessThanOrEquals, "LESS_THAN_OR_EQUALS"),
            (Comparator::In, "IN"),
            (Comparator::NotIn, "NOT_IN"),
            (Comparator::Near, "NEAR"),
            (Comparator::BeginsWith, "BEGINS_WITH"),
            (Comparator::ContainsAllTokens, "CONTAINS_ALL_TOKENS"),
            (Comparator::ContainsAnyTokens, "CONTAINS_ANY_TOKENS"),
            (Comparator::ListContains, "LIST_CONTAINS"),
            (Comparator::ListNotContains, "LIST_NOT_CONTAINS"),
            (Comparator::ListContainsAll, "LIST_CONTAINS_ALL"),
            (Comparator::ListContainsAny, "LIST_CONTAINS_ANY"),
        ];

        for (comp, expected) in comparators {
            let json = serde_json::to_string(&comp).unwrap();
            assert_eq!(json, format!("\"{}\"", expected));
        }
    }

    #[test]
    fn test_filter_serde() {
        let filter = Filter {
            field_name: "age".to_string(),
            comparator: Comparator::GreaterThan,
            field_value: FieldValue::Int64(18),
        };
        let json = serde_json::to_string(&filter).unwrap();
        assert!(json.contains("\"fieldName\":\"age\""));
        assert!(json.contains("\"comparator\":\"GREATER_THAN\""));
        assert!(json.contains("\"fieldValue\""));
    }

    #[test]
    fn test_sort_descriptor_serde() {
        let sort = SortDescriptor {
            field_name: "name".to_string(),
            ascending: Some(true),
        };
        let json = serde_json::to_string(&sort).unwrap();
        assert!(json.contains("\"fieldName\":\"name\""));
        assert!(json.contains("\"ascending\":true"));
    }

    #[test]
    fn test_sort_descriptor_no_ascending() {
        let sort = SortDescriptor {
            field_name: "date".to_string(),
            ascending: None,
        };
        let json = serde_json::to_string(&sort).unwrap();
        assert!(!json.contains("ascending"));
    }
}
