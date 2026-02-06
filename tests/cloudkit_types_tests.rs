#[cfg(feature = "cloudkit")]
mod cloudkit_types_tests {
    use apple::cloudkit::*;

    #[test]
    fn test_environment_display() {
        assert_eq!(Environment::Development.to_string(), "development");
        assert_eq!(Environment::Production.to_string(), "production");
    }

    #[test]
    fn test_database_type_display() {
        assert_eq!(DatabaseType::Public.to_string(), "public");
        assert_eq!(DatabaseType::Private.to_string(), "private");
        assert_eq!(DatabaseType::Shared.to_string(), "shared");
    }

    #[test]
    fn test_zone_id_new() {
        let zone = ZoneID::new("MyZone");
        assert_eq!(zone.zone_name, "MyZone");
        assert!(zone.owner_record_name.is_none());
    }

    #[test]
    fn test_zone_id_default_zone() {
        let zone = ZoneID::default_zone();
        assert_eq!(zone.zone_name, "_defaultZone");
        assert!(zone.owner_record_name.is_none());
    }

    #[test]
    fn test_record_builder() {
        let record = Record::new("TodoItem")
            .with_name("todo-1")
            .with_field("title", FieldValue::String("Buy milk".to_string()))
            .with_field("priority", FieldValue::Int64(5));

        assert_eq!(record.record_name.as_deref(), Some("todo-1"));
        assert_eq!(record.record_type, "TodoItem");
        assert_eq!(record.fields.len(), 2);
        assert!(record.record_change_tag.is_none());
        assert!(record.zone_id.is_none());
    }

    #[test]
    fn test_record_builder_with_zone() {
        let record = Record::new("Note").with_zone(ZoneID::new("NotesZone"));

        assert_eq!(record.zone_id.as_ref().unwrap().zone_name, "NotesZone");
    }

    #[test]
    fn test_record_new_has_empty_fields() {
        let record = Record::new("Empty");
        assert!(record.fields.is_empty());
        assert!(record.record_name.is_none());
        assert!(record.created.is_none());
        assert!(record.modified.is_none());
    }

    #[test]
    fn test_field_value_string_serde() {
        let fv = FieldValue::String("hello".to_string());
        let json = serde_json::to_string(&fv).unwrap();
        assert!(json.contains("\"type\":\"STRING\""));
        assert!(json.contains("\"value\":\"hello\""));

        let deserialized: FieldValue = serde_json::from_str(&json).unwrap();
        match deserialized {
            FieldValue::String(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected String"),
        }
    }

    #[test]
    fn test_field_value_int64_serde() {
        let fv = FieldValue::Int64(42);
        let json = serde_json::to_string(&fv).unwrap();
        assert!(json.contains("\"type\":\"INT64\""));
        assert!(json.contains("\"value\":42"));

        let deserialized: FieldValue = serde_json::from_str(&json).unwrap();
        match deserialized {
            FieldValue::Int64(n) => assert_eq!(n, 42),
            _ => panic!("Expected Int64"),
        }
    }

    #[test]
    fn test_field_value_double_serde() {
        let fv = FieldValue::Double(3.125);
        let json = serde_json::to_string(&fv).unwrap();
        assert!(json.contains("\"type\":\"DOUBLE\""));

        let deserialized: FieldValue = serde_json::from_str(&json).unwrap();
        match deserialized {
            FieldValue::Double(d) => assert!((d - 3.125).abs() < f64::EPSILON),
            _ => panic!("Expected Double"),
        }
    }

    #[test]
    fn test_field_value_timestamp_serde() {
        let fv = FieldValue::Timestamp(1700000000);
        let json = serde_json::to_string(&fv).unwrap();
        assert!(json.contains("\"type\":\"TIMESTAMP\""));

        let deserialized: FieldValue = serde_json::from_str(&json).unwrap();
        match deserialized {
            FieldValue::Timestamp(t) => assert_eq!(t, 1700000000),
            _ => panic!("Expected Timestamp"),
        }
    }

    #[test]
    fn test_field_value_string_list_serde() {
        let fv = FieldValue::StringList(vec!["a".into(), "b".into()]);
        let json = serde_json::to_string(&fv).unwrap();
        assert!(json.contains("\"type\":\"STRING_LIST\""));

        let deserialized: FieldValue = serde_json::from_str(&json).unwrap();
        match deserialized {
            FieldValue::StringList(v) => assert_eq!(v, vec!["a", "b"]),
            _ => panic!("Expected StringList"),
        }
    }

    #[test]
    fn test_field_value_location_serde() {
        let loc = LocationValue {
            latitude: 37.7749,
            longitude: -122.4194,
            altitude: Some(10.0),
            horizontal_accuracy: None,
            vertical_accuracy: None,
            course: None,
            speed: None,
            timestamp: None,
        };
        let fv = FieldValue::Location(loc);
        let json = serde_json::to_string(&fv).unwrap();
        assert!(json.contains("\"type\":\"LOCATION\""));
        assert!(json.contains("37.7749"));

        let deserialized: FieldValue = serde_json::from_str(&json).unwrap();
        match deserialized {
            FieldValue::Location(l) => {
                assert!((l.latitude - 37.7749).abs() < f64::EPSILON);
                assert!((l.longitude - (-122.4194)).abs() < f64::EPSILON);
                assert_eq!(l.altitude, Some(10.0));
            }
            _ => panic!("Expected Location"),
        }
    }

    #[test]
    fn test_zone_id_serde() {
        let zone = ZoneID::new("TestZone");
        let json = serde_json::to_string(&zone).unwrap();
        assert!(json.contains("\"zoneName\":\"TestZone\""));
        assert!(!json.contains("ownerRecordName"));

        let deserialized: ZoneID = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.zone_name, "TestZone");
        assert!(deserialized.owner_record_name.is_none());
    }

    #[test]
    fn test_zone_id_with_owner_serde() {
        let zone = ZoneID {
            zone_name: "MyZone".into(),
            owner_record_name: Some("owner-123".into()),
        };
        let json = serde_json::to_string(&zone).unwrap();
        assert!(json.contains("\"ownerRecordName\":\"owner-123\""));
    }

    #[test]
    fn test_record_serde_roundtrip() {
        let record = Record::new("Task")
            .with_name("task-1")
            .with_field("title", FieldValue::String("Do stuff".into()))
            .with_field("done", FieldValue::Int64(0));

        let json = serde_json::to_string(&record).unwrap();
        let deserialized: Record = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.record_type, "Task");
        assert_eq!(deserialized.record_name.as_deref(), Some("task-1"));
        assert_eq!(deserialized.fields.len(), 2);
    }

    #[test]
    fn test_operation_type_serde() {
        let op = OperationType::Create;
        let json = serde_json::to_string(&op).unwrap();
        assert_eq!(json, "\"create\"");

        let op = OperationType::ForceDelete;
        let json = serde_json::to_string(&op).unwrap();
        assert_eq!(json, "\"forceDelete\"");
    }

    #[test]
    fn test_subscription_type_serde() {
        let st = SubscriptionType::Zone;
        let json = serde_json::to_string(&st).unwrap();
        assert_eq!(json, "\"zone\"");

        let st = SubscriptionType::Query;
        let json = serde_json::to_string(&st).unwrap();
        assert_eq!(json, "\"query\"");
    }

    #[test]
    fn test_fires_on_serde() {
        let f = FiresOn::Create;
        assert_eq!(serde_json::to_string(&f).unwrap(), "\"create\"");

        let f = FiresOn::Update;
        assert_eq!(serde_json::to_string(&f).unwrap(), "\"update\"");

        let f = FiresOn::Delete;
        assert_eq!(serde_json::to_string(&f).unwrap(), "\"delete\"");
    }

    #[test]
    fn test_reference_action_serde() {
        let a = ReferenceAction::DeleteSelf;
        let json = serde_json::to_string(&a).unwrap();
        assert_eq!(json, "\"DELETE_SELF\"");
    }

    #[test]
    fn test_notification_info_serde_skip_none() {
        let info = NotificationInfo {
            alert_body: Some("hello".into()),
            alert_localization_key: None,
            alert_localization_args: None,
            alert_action_localization_key: None,
            alert_launch_image: None,
            sound_name: None,
            should_badge: Some(true),
            should_send_content_available: None,
            should_send_mutable_content: None,
            collapse_id_key: None,
            desired_keys: None,
            category: None,
            title_localization_key: None,
            title_localization_args: None,
            subtitle_localization_key: None,
            subtitle_localization_args: None,
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"alertBody\":\"hello\""));
        assert!(json.contains("\"shouldBadge\":true"));
        assert!(!json.contains("alertLocalizationKey"));
        assert!(!json.contains("soundName"));
    }

    #[test]
    fn test_environment_equality() {
        assert_eq!(Environment::Development, Environment::Development);
        assert_ne!(Environment::Development, Environment::Production);
    }

    #[test]
    fn test_database_type_equality() {
        assert_eq!(DatabaseType::Public, DatabaseType::Public);
        assert_ne!(DatabaseType::Public, DatabaseType::Private);
    }
}
