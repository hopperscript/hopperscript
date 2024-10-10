//Convert `Project` struct into JSON
use crate::types::Project;

// Export from `Project` to a JSON String
pub fn to_json(prj: Project) -> String {
    // TODO should prettify till it's ready i guess
    serde_json::to_string_pretty(&prj).expect("Failed to export project JSON")
}
