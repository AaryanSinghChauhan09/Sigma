//! Example: Container orchestration using SigmaOS library
//!
//! This example demonstrates how to manage containers,
//! create container images, and orchestrate workloads.

use sigma_system::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SigmaOS Container Orchestration Example ===\n");

    // 1. Container Manager Setup
    println!("1. Container Manager Setup");
    let mut container_mgr = ContainerManager::new();
    container_mgr.set_runtime(ContainerRuntime::Podman)?;
    println!("   Runtime: {}", container_mgr.runtime.as_str());
    println!("   Storage driver: {}", container_mgr.storage_driver);

    // 2. Create Container Images
    println!("\n2. Create Container Images");
    let web_image = container_mgr.create_image(
        "sigmaos-web",
        "nginx:alpine",
        vec!["EXPOSE 80", "CMD [\"nginx\", \"-g\", \"daemon off;\"]"],
    )?;
    println!("   Created image: {}", web_image.name);

    let api_image = container_mgr.create_image(
        "sigmaos-api",
        "node:18-alpine",
        vec!["EXPOSE 3000", "CMD [\"node\", \"server.js\"]"],
    )?;
    println!("   Created image: {}", api_image.name);

    // 3. Deploy Containers
    println!("\n3. Deploy Containers");
    let web_container = container_mgr.deploy_container(ContainerSpec::new(
        "web",
        "sigmaos-web",
        vec!["-p".to_string(), "80:80".to_string()],
    )?)?;
    println!("   Deployed: {} ({})", web_container.name, web_container.status.as_str());

    let api_container = container_mgr.deploy_container(ContainerSpec::new(
        "api",
        "sigmaos-api",
        vec!["-p".to_string(), "3000:3000".to_string()],
    )?)?;
    println!("   Deployed: {} ({})", api_container.name, api_container.status.as_str());

    // 4. Container Networking
    println!("\n4. Container Networking");
    let network_config = container_mgr.generate_network_config();
    println!("{}", network_config);

    // 5. Container Orchestration
    println!("\n5. Container Orchestration");
    let compose_config = container_mgr.generate_compose_config();
    println!("{}", compose_config);

    // 6. Monitoring Containers
    println!("\n6. Monitoring Containers");
    for container in container_mgr.get_containers() {
        println!("   {}: {} ({} MB, {} CPU)", 
            container.name, 
            container.status.as_str(),
            container.memory_limit_mb,
            container.cpu_limit
        );
    }

    // 7. Generate Kubernetes Manifest
    println!("\n7. Kubernetes Manifest");
    let k8s_manifest = container_mgr.generate_kubernetes_manifest();
    println!("{}", k8s_manifest);

    // 8. Health Checks
    println!("\n8. Health Checks");
    for container in container_mgr.get_containers() {
        let health = container_mgr.check_health(&container.name)?;
        println!("   {}: {}", container.name, health);
    }

    println!("\n=== Container Orchestration Complete ===");
    println!("\nTo deploy these containers:");
    println!("1. podman build -t sigmaos-web ./web");
    println!("2. podman build -t sigmaos-api ./api");
    println!("3. podman-compose up -d");
    println!("4. Monitor with: podman stats");

    Ok(())
}
