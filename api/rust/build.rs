use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let proto_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let proto_dir = Path::new(&proto_dir);
    let proto_dir = proto_dir.join("proto");

    std::fs::create_dir_all(out_dir.join("common")).unwrap();
    std::fs::create_dir_all(out_dir.join("gw")).unwrap();
    std::fs::create_dir_all(out_dir.join("internal")).unwrap();
    std::fs::create_dir_all(out_dir.join("integration")).unwrap();
    std::fs::create_dir_all(out_dir.join("meta")).unwrap();
    std::fs::create_dir_all(out_dir.join("api")).unwrap();

    // common
    tonic_build::configure()
        .out_dir(out_dir.join("common"))
        .file_descriptor_set_path(out_dir.join("common").join("proto_descriptor.bin"))
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", "::pbjson_types")
        .compile(
            &["common/common.proto"],
            &[
                proto_dir.join("chirpstack").to_str().unwrap(),
                proto_dir.join("google").to_str().unwrap(),
            ],
        )?;

    let descriptor_set = std::fs::read(out_dir.join("common").join("proto_descriptor.bin"))?;
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set)?
        .out_dir(out_dir.join("common"))
        .build(&[".common"])?;

    // gw
    tonic_build::configure()
        .out_dir(out_dir.join("gw"))
        .file_descriptor_set_path(out_dir.join("gw").join("proto_descriptor.bin"))
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", "::pbjson_types")
        .extern_path(".common", "crate::common")
        .compile(
            &["gw/gw.proto"],
            &[
                proto_dir.join("chirpstack").to_str().unwrap(),
                proto_dir.join("google").to_str().unwrap(),
            ],
        )?;

    let descriptor_set = std::fs::read(out_dir.join("gw").join("proto_descriptor.bin"))?;
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set)?
        .out_dir(out_dir.join("gw"))
        .extern_path(".common", "crate::common")
        .build(&[".gw"])?;

    // internal
    tonic_build::configure()
        .out_dir(out_dir.join("internal"))
        .file_descriptor_set_path(out_dir.join("internal").join("proto_descriptor.bin"))
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", "::pbjson_types")
        .extern_path(".common", "crate::common")
        .compile(
            &["internal/internal.proto"],
            &[
                proto_dir.join("chirpstack").to_str().unwrap(),
                proto_dir.join("google").to_str().unwrap(),
            ],
        )?;

    let descriptor_set = std::fs::read(out_dir.join("internal").join("proto_descriptor.bin"))?;
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set)?
        .out_dir(out_dir.join("internal"))
        .extern_path(".common", "crate::common")
        .build(&[".internal"])?;

    // integration
    tonic_build::configure()
        .out_dir(out_dir.join("integration"))
        .file_descriptor_set_path(out_dir.join("integration").join("proto_descriptor.bin"))
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", "::pbjson_types")
        .extern_path(".common", "crate::common")
        .extern_path(".gw", "crate::gw")
        .compile(
            &["integration/integration.proto"],
            &[
                proto_dir.join("chirpstack").to_str().unwrap(),
                proto_dir.join("google").to_str().unwrap(),
            ],
        )?;

    let descriptor_set = std::fs::read(out_dir.join("integration").join("proto_descriptor.bin"))?;
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set)?
        .out_dir(out_dir.join("integration"))
        .extern_path(".common", "crate::common")
        .extern_path(".gw", "crate::gw")
        .build(&[".integration"])?;

    // meta
    tonic_build::configure()
        .out_dir(out_dir.join("meta"))
        .file_descriptor_set_path(out_dir.join("meta").join("proto_descriptor.bin"))
        .compile_well_known_types(true)
        .extern_path(".common", "crate::common")
        .extern_path(".gw", "crate::gw")
        .compile(
            &["meta/meta.proto"],
            &[proto_dir.join("chirpstack").to_str().unwrap()],
        )?;

    let descriptor_set = std::fs::read(out_dir.join("meta").join("proto_descriptor.bin"))?;
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set)?
        .out_dir(out_dir.join("meta"))
        .extern_path(".common", "crate::common")
        .extern_path(".gw", "crate::gw")
        .build(&[".meta"])?;

    // api
    tonic_build::configure()
        .out_dir(out_dir.join("api"))
        .file_descriptor_set_path(out_dir.join("api").join("proto_descriptor.bin"))
        .extern_path(".common", "crate::common")
        .extern_path(".gw", "crate::gw")
        .compile(
            &[
                "api/internal.proto",
                "api/user.proto",
                "api/tenant.proto",
                "api/application.proto",
                "api/device_profile.proto",
                "api/device_profile_template.proto",
                "api/device.proto",
                "api/gateway.proto",
                "api/frame_log.proto",
                "api/multicast_group.proto",
            ],
            &[
                proto_dir.join("chirpstack").to_str().unwrap(),
                proto_dir.join("google").to_str().unwrap(),
            ],
        )?;

    Ok(())
}
