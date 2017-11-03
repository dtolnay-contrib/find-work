use config::Config;
use data::{Category, StructuralData, Tab, TabCategory};
use github::Issue;
use issues::IssueData;

use std::collections::HashMap;

pub fn mock_config() -> Config {
    Config {
        repository: "nrc/find-work".to_owned(),
        username: ::TEST_USERNAME.to_owned(),
        token: ::TEST_TOKEN.to_owned(),
        addr: "127.0.0.1:80".to_owned(),
        static_path: String::new(),
        index_path: String::new(),
        dev_mode: false,
    }
}

pub fn mock_struct_data() -> StructuralData {
    let mut result = StructuralData {
        tabs: vec![Tab {
            id: "foo".to_owned(),
            title: "Foo".to_owned(),
            description: "A Foo for foos".to_owned(),
        }, Tab {
            id: "bar".to_owned(),
            title: "Bar".to_owned(),
            description: "A Bar for bars".to_owned(),
        }],
        categories: HashMap::new(),
        tab_category: HashMap::new(),
    };

    result.categories.insert("rustfmt".to_owned(), Category {
        id: "rustfmt".to_owned(),
        title: "Rustfmt".to_owned(),
        description: String::new(),
        repository: "rust-lang-nursery/rustfmt".to_owned(),
        labels: vec!["p-high".to_owned()],
        links: vec![],
        tags: vec!["a".to_owned(), "b".to_owned()],
    });
    result.tab_category.insert("foo".to_owned(), vec![TabCategory {
        tab: "foo".to_owned(),
        category: "rustfmt".to_owned(),
        labels: vec!["bug".to_owned()],
        milestone: None,
        link: None,
    }]);
    result.tab_category.insert("bar".to_owned(), vec![TabCategory {
        tab: "bar".to_owned(),
        category: "rustfmt".to_owned(),
        labels: vec!["bug".to_owned()],
        milestone: None,
        link: None,
    }]);

    result
}

pub fn mock_issue_data() -> IssueData {
    let mut issues = HashMap::new();
    issues.insert(("foo".to_owned(), "rustfmt".to_owned()), vec![Issue {
        number: 42,
        url: String::new(),
        title: "Title".to_owned(),
        body: "body/description".to_owned(),
        labels: vec![],
    }]);
    issues.insert(("bar".to_owned(), "rustfmt".to_owned()), vec![Issue {
        number: 43,
        url: String::new(),
        title: "Title".to_owned(),
        body: "body/description".to_owned(),
        labels: vec![],
    }]);
    IssueData {
        issues,
    }
}
