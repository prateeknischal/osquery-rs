extern crate osquery_rs;

use osquery_rs::osquery::*;
use osquery_rs::*;
use std::collections::BTreeMap;

struct TestTable {}

impl TablePluginDetails for TestTable {
    fn name(&self) -> String {
        "test_table".to_string()
    }

    fn columns(&self) -> Vec<ColumnDefinition> {
        vec![ColumnDefinition {
            column_name: "time".to_string(),
            column_type: ColumnType::Text,
        }]
    }

    fn generate(&self, _: Option<String>) -> ExtensionResponse {
        let mut row = BTreeMap::new();
        row.insert("time".to_string(), "12345".to_string());
        let res = vec![row];
        let status = ExtensionStatus::new(0, "OK".to_string(), None);
        osquery::ExtensionResponse::new(status, res)
    }
}

fn main() {
    let socket = "/Users/p0n002h/.osquery/shell.em";
    let mut server = ExtensionManagerServer::new_with_path("thrust", socket);
    server.register_plugin(Box::new(TablePlugin::new(Box::new(TestTable {}))));
    server.run();
}
