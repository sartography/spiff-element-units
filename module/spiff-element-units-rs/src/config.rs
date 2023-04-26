use std::env;

static PRETTY_JSON: &str = "SPIFF_ELEMENT_UNITS_PRETTY_JSON";

pub fn pretty_json() -> bool {
    env_is_true(PRETTY_JSON)
}

fn env_is_true(key: &str) -> bool {
    match env::var(key) {
        Ok(val) if &val == "true" => true,
        _ => false,
    }
}
