use mongodb::bson::oid::ObjectId;
use resdata_rust::model::data_structs::{Dataset, Experiment, Project};

#[test]
fn test_dataset_serialization() {
    let dataset = Dataset {
        id: Some(ObjectId::new()),
        data: vec![1, 2, 3],
        headers: vec!["Header1".to_string(), "Header2".to_string()],
        author: vec!["Author1".to_string()],
    };

    let serialized = serde_json::to_string(&dataset).unwrap();
    let deserialized: Dataset = serde_json::from_str(&serialized).unwrap();

    assert_eq!(dataset.data, deserialized.data);
    assert_eq!(dataset.headers, deserialized.headers);
    assert_eq!(dataset.author, deserialized.author);
}

#[test]
fn test_experiment_serialization() {
    let experiment = Experiment {
        id: Some(ObjectId::new()),
        data: vec![4, 5, 6],
        headers: vec!["HeaderA".to_string(), "HeaderB".to_string()],
        author: vec!["Author2".to_string()],
        datasets: vec![Some(ObjectId::new())],
    };

    let serialized = serde_json::to_string(&experiment).unwrap();
    let deserialized: Experiment = serde_json::from_str(&serialized).unwrap();

    assert_eq!(experiment.data, deserialized.data);
    assert_eq!(experiment.headers, deserialized.headers);
    assert_eq!(experiment.author, deserialized.author);
}

#[test]
fn test_project_serialization() {
    let project = Project {
        id: Some(ObjectId::new()),
        data: vec![7, 8, 9],
        headers: vec!["HeaderX".to_string(), "HeaderY".to_string()],
        author: vec!["Author3".to_string()],
        experiments: vec![Some(ObjectId::new())],
    };

    let serialized = serde_json::to_string(&project).unwrap();
    let deserialized: Project = serde_json::from_str(&serialized).unwrap();

    assert_eq!(project.data, deserialized.data);
    assert_eq!(project.headers, deserialized.headers);
    assert_eq!(project.author, deserialized.author);
}
