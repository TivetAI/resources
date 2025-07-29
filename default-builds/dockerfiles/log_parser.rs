use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MetricTagSet {
    pub server_id: String,
    pub datacenter_id: String,
    pub cluster_id: String,
    pub pool_type: String,
    pub public_ip: String,
}

impl MetricTagSet {
    pub fn default_for_local() -> Self {
        Self {
            server_id: "fc67e54e-5d6a-4726-ab23-77b0e54f068f".to_string(),
            datacenter_id: "f288913c-735d-4188-bf9b-2fcf6eac7b9c".to_string(),
            cluster_id: "pegboard-cluster".to_string(),
            pool_type: "pegboard_isolate".to_string(),
            public_ip: "127.0.0.1".to_string(),
        }
    }

    pub fn as_map(&self) -> HashMap<String, String> {
        let mut tags = HashMap::new();
        tags.insert("server_id".into(), self.server_id.clone());
        tags.insert("datacenter_id".into(), self.datacenter_id.clone());
        tags.insert("cluster_id".into(), self.cluster_id.clone());
        tags.insert("pool_type".into(), self.pool_type.clone());
        tags.insert("public_ip".into(), self.public_ip.clone());
        tags
    }
}
