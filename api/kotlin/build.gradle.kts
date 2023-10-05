import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import com.google.protobuf.gradle.*

plugins {
    kotlin("jvm") version "1.7.21"
    id("com.google.protobuf") version("0.9.1")
    `java-library`
    `maven-publish`
}

group = "io.chirpstack"
version = "4.5.1"

repositories {
    mavenCentral()
}

kotlin {
    kotlinDaemonJvmArgs = listOf("-Xmx2048m", "-Xms1024m", "-XX:+UseParallelGC")
}

buildscript {
    dependencies {
        classpath("com.google.protobuf:protobuf-gradle-plugin:0.9.1")
    }
}

dependencies {
    api("io.grpc:grpc-protobuf:1.51.0")
    api("io.grpc:grpc-netty:1.51.0")
    api("io.grpc:grpc-kotlin-stub:1.3.0")
    implementation("com.google.protobuf:protobuf-kotlin:3.21.9")
    implementation("io.grpc:grpc-api:1.51.0")
    implementation("io.grpc:grpc-stub:1.51.0")
    implementation("io.grpc:protoc-gen-grpc-kotlin:1.3.0")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.6.4")
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
        delete.add("$projectDir/src/main/grpckt")
        delete.add("$projectDir/src/main/java/internal")
        delete.add("$projectDir/src/main/java/io")
        delete.add("$projectDir/src/main/kotlin/internal")
        delete.add("$projectDir/src/main/kotlin/io")
    }
    getByName("processResources") {
        dependsOn("generateProto")
    }
}

tasks.withType<KotlinCompile> {
    kotlinOptions.jvmTarget = JavaVersion.VERSION_1_8.toString()
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
        id("grpckt") {
            artifact = "io.grpc:protoc-gen-grpc-kotlin:1.3.0:jdk8@jar"
        }
    }
    generateProtoTasks {
        all().forEach {
            it.plugins {
                id("grpc")
                id("grpckt")
            }
            it.builtins {
                id("kotlin")
            }
        }
    }
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            groupId = project.group.toString()
            artifactId = "chirpstack-api-kotlin"
            version = project.version.toString()
            from(components["kotlin"])
        }
    }
}