use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let proto_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let proto_dir = Path::new(&proto_dir);
    let proto_dir = proto_dir.join("proto");
    let cs_dir = proto_dir.join("chirpstack");

    std::fs::create_dir_all(out_dir.join("common")).unwrap();
    std::fs::create_dir_all(out_dir.join("gw")).unwrap();
    std::fs::create_dir_all(out_dir.join("internal")).unwrap();
    std::fs::create_dir_all(out_dir.join("integration")).unwrap();
    std::fs::create_dir_all(out_dir.join("stream")).unwrap();
    std::fs::create_dir_all(out_dir.join("api")).unwrap();

    #[cfg(feature = "json")]
    let well_known_types_path = "::pbjson_types";

    #[cfg(not(feature = "json"))]
    let well_known_types_path = "::prost_types";

    // common
    tonic_build::configure()
        .out_dir(out_dir.join("common"))
        .file_descriptor_set_path(out_dir.join("common").join("proto_descriptor.bin"))
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", well_known_types_path)
        .compile(
            &[cs_dir.join("common").join("common.proto").to_str().unwrap()],
            &[
                proto_dir.join("chirpstack").to_str().unwrap(),
                proto_dir.join("google").to_str().unwrap(),
            ],
        )?;

    #[cfg(feature = "json")]
    {
        let descriptor_set = std::fs::read(out_dir.join("common").join("proto_descriptor.bin"))?;
        pbjson_build::Builder::new()
            .register_descriptors(&descriptor_set)?
            .ignore_unknown_fields()
            .out_dir(out_dir.join("common"))
            .build(&[".common"])?;
    }

    // gw
    tonic_build::configure()
        .out_dir(out_dir.join("gw"))
        .file_descriptor_set_path(out_dir.join("gw").join("proto_descriptor.bin"))
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", well_known_types_path)
        .extern_path(".common", "crate::common")
        .compile(
            &[cs_dir.join("gw").join("gw.proto").to_str().unwrap()],
            &[
                proto_dir.join("chirpstack").to_str().unwrap(),
                proto_dir.join("google").to_str().unwrap(),
            ],
        )?;

    #[cfg(feature = "json")]
    {
        let descriptor_set = std::fs::read(out_dir.join("gw").join("proto_descriptor.bin"))?;
        pbjson_build::Builder::new()
            .register_descriptors(&descriptor_set)?
            .ignore_unknown_fields()
            .out_dir(out_dir.join("gw"))
            .extern_path(".common", "crate::common")
            .build(&[".gw"])?;
    }

    // internal
    {
        #[allow(unused_mut)]
        let mut builder = tonic_build::configure()
            .out_dir(out_dir.join("internal"))
            .file_descriptor_set_path(out_dir.join("internal").join("proto_descriptor.bin"))
            .compile_well_known_types(true)
            .extern_path(".google.protobuf", well_known_types_path)
            .extern_path(".common", "crate::common");

        #[cfg(feature = "diesel")]
        {
            builder = builder.message_attribute("internal.DeviceSession", "#[derive(diesel::expression::AsExpression, diesel::deserialize::FromSqlRow)] #[diesel(sql_type = diesel::sql_types::Binary)]");
        }

        builder.compile(
            &[cs_dir
                .join("internal")
                .join("internal.proto")
                .to_str()
                .unwrap()],
            &[
                proto_dir.join("chirpstack").to_str().unwrap(),
                proto_dir.join("google").to_str().unwrap(),
            ],
        )?;
    }

    #[cfg(feature = "json")]
    {
        let descriptor_set = std::fs::read(out_dir.join("internal").join("proto_descriptor.bin"))?;
        pbjson_build::Builder::new()
            .register_descriptors(&descriptor_set)?
            .ignore_unknown_fields()
            .out_dir(out_dir.join("internal"))
            .extern_path(".common", "crate::common")
            .build(&[".internal"])?;
    }

    // integration
    tonic_build::configure()
        .out_dir(out_dir.join("integration"))
        .file_descriptor_set_path(out_dir.join("integration").join("proto_descriptor.bin"))
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", well_known_types_path)
        .extern_path(".common", "crate::common")
        .extern_path(".gw", "crate::gw")
        .compile(
            &[cs_dir
                .join("integration")
                .join("integration.proto")
                .to_str()
                .unwrap()],
            &[
                proto_dir.join("chirpstack").to_str().unwrap(),
                proto_dir.join("google").to_str().unwrap(),
            ],
        )?;

    #[cfg(feature = "json")]
    {
        let descriptor_set =
            std::fs::read(out_dir.join("integration").join("proto_descriptor.bin"))?;
        pbjson_build::Builder::new()
            .emit_fields()
            .register_descriptors(&descriptor_set)?
            .ignore_unknown_fields()
            .out_dir(out_dir.join("integration"))
            .extern_path(".common", "crate::common")
            .extern_path(".gw", "crate::gw")
            .build(&[".integration"])?;
    }

    // streams
    tonic_build::configure()
        .out_dir(out_dir.join("stream"))
        .file_descriptor_set_path(out_dir.join("stream").join("proto_descriptor.bin"))
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", well_known_types_path)
        .extern_path(".common", "crate::common")
        .extern_path(".gw", "crate::gw")
        .compile(
            &[
                cs_dir.join("stream").join("meta.proto").to_str().unwrap(),
                cs_dir.join("stream").join("frame.proto").to_str().unwrap(),
                cs_dir
                    .join("stream")
                    .join("api_request.proto")
                    .to_str()
                    .unwrap(),
                cs_dir
                    .join("stream")
                    .join("backend_interfaces.proto")
                    .to_str()
                    .unwrap(),
            ],
            &[
                proto_dir.join("chirpstack").to_str().unwrap(),
                proto_dir.join("google").to_str().unwrap(),
            ],
        )?;

    #[cfg(feature = "json")]
    {
        let descriptor_set = std::fs::read(out_dir.join("stream").join("proto_descriptor.bin"))?;
        pbjson_build::Builder::new()
            .register_descriptors(&descriptor_set)?
            .ignore_unknown_fields()
            .out_dir(out_dir.join("stream"))
            .extern_path(".common", "crate::common")
            .extern_path(".gw", "crate::gw")
            .build(&[".stream"])?;
    }

    // api
    tonic_build::configure()
        .out_dir(out_dir.join("api"))
        .file_descriptor_set_path(out_dir.join("api").join("proto_descriptor.bin"))
        .extern_path(".common", "crate::common")
        .extern_path(".gw", "crate::gw")
        .compile(
            &[
                cs_dir.join("api").join("internal.proto").to_str().unwrap(),
                cs_dir.join("api").join("user.proto").to_str().unwrap(),
                cs_dir.join("api").join("tenant.proto").to_str().unwrap(),
                cs_dir
                    .join("api")
                    .join("application.proto")
                    .to_str()
                    .unwrap(),
                cs_dir
                    .join("api")
                    .join("device_profile.proto")
                    .to_str()
                    .unwrap(),
                cs_dir
                    .join("api")
                    .join("device_profile_template.proto")
                    .to_str()
                    .unwrap(),
                cs_dir.join("api").join("device.proto").to_str().unwrap(),
                cs_dir.join("api").join("gateway.proto").to_str().unwrap(),
                cs_dir
                    .join("api")
                    .join("multicast_group.proto")
                    .to_str()
                    .unwrap(),
                cs_dir.join("api").join("relay.proto").to_str().unwrap(),
            ],
            &[
                proto_dir.join("chirpstack").to_str().unwrap(),
                proto_dir.join("google").to_str().unwrap(),
            ],
        )?;

    Ok(())
}
