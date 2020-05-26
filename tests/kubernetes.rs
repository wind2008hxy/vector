use kubernetes_test_framework::{Framework, Interface};

const VECTOR_CONFIG: &str = r#"
[sinks.stdout]
    type = "console"
    inputs = ["kubernetes_logs"]
    target = "stdout"
    encoding = "json"
"#;

fn framework() -> Framework {
    let interface = Interface::from_env().expect("interface is not ready");
    Framework::new(interface)
}

#[test]
fn test() {
    let framework = framework();
    framework
        .deploy_vector("test-vector", VECTOR_CONFIG)
        .unwrap();
}
