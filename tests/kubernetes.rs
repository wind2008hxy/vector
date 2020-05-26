use k8s_openapi::{
    api::core::v1::{Container, Pod, PodSpec},
    apimachinery::pkg::apis::meta::v1::ObjectMeta,
};
use kubernetes_test_framework::{test_pod, Framework, Interface};

const VECTOR_CONFIG: &str = r#"
[sinks.stdout]
    type = "console"
    inputs = ["kubernetes_logs"]
    target = "stdout"
    encoding = "json"
"#;

const BUSYBOX_IMAGE: &str = "busybox:1.28";

fn repeating_echo_cmd(marker: &str) -> String {
    format!(
        r#"echo before; i=0; while [ $i -le 600 ]; do sleep 0.1; echo "{}"; i=$((i+1)); done"#,
        marker
    )
}

fn make_framework() -> Framework {
    let interface = Interface::from_env().expect("interface is not ready");
    Framework::new(interface)
}

fn make_test_pod(namespace: &str, name: &str, command: &str) -> Pod {
    Pod {
        metadata: Some(ObjectMeta {
            name: Some(name.to_owned()),
            namespace: Some(namespace.to_owned()),
            ..ObjectMeta::default()
        }),
        spec: Some(PodSpec {
            containers: vec![Container {
                name: name.to_owned(),
                image: Some(BUSYBOX_IMAGE.to_owned()),
                command: Some(vec!["sh".to_owned()]),
                args: Some(vec!["-c".to_owned(), command.to_owned()]),
                ..Container::default()
            }],
            restart_policy: Some("Never".to_owned()),
            ..PodSpec::default()
        }),
        ..Pod::default()
    }
}

#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let framework = make_framework();

    let vector = framework.vector("test-vector", VECTOR_CONFIG)?;

    let test_namespace = framework.namespace("test-vector-test-pod")?;

    let test_pod = framework.test_pod(test_pod::Config::from_pod(&make_test_pod(
        "test-vector-test-pod",
        "test-pod",
        repeating_echo_cmd("MARKER").as_str(),
    ))?)?;

    let logs = framework.logs("test-vector", "vector");
    assert_eq!(logs, vec!["MARKER"]);

    drop(test_pod);
    drop(test_namespace);
    drop(vector);
    Ok(())
}
