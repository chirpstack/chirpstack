import com.google.protobuf.gradle.*

plugins {
    id("java")
    id("com.google.protobuf") version("0.9.1")
    `java-library`
    `maven-publish`
}

group = "io.chirpstack"
version = "4.5.1"

repositories {
    mavenCentral()
}

buildscript {
    dependencies {
        classpath("com.google.protobuf:protobuf-gradle-plugin:0.9.1")
    }
}

dependencies {
    api("io.grpc:grpc-protobuf:1.51.0")
    api("io.grpc:grpc-api:1.51.0")
    api("io.grpc:grpc-stub:1.51.0")
    api("io.grpc:grpc-netty:1.51.0")
    implementation("javax.annotation:javax.annotation-api:1.3.2")
}

sourceSets {
    main {
        proto {
            srcDir("$projectDir/../proto")
        }
    }
}

tasks {
    getByName<Delete>("clean") {
        delete.add("$projectDir/src/main/grpc")
        delete.add("$projectDir/src/main/java/internal")
        delete.add("$projectDir/src/main/java/io")
    }
    getByName("processResources") {
        dependsOn("generateProto")
    }
}

protobuf {
    protoc {
        artifact = "com.google.protobuf:protoc:3.21.9"
    }
    generatedFilesBaseDir = "$projectDir/src"
    plugins {
        id("grpc"){
            artifact = "io.grpc:protoc-gen-grpc-java:1.51.0"
        }
    }
    generateProtoTasks {
        all().forEach {
            it.plugins {
                id("grpc")
            }
        }
    }
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            groupId = project.group.toString()
            artifactId = "chirpstack-api"
            version = project.version.toString()
            from(components["java"])
        }
    }
}