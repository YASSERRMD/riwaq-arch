plugins {
    id("java")
    id("org.jetbrains.kotlin.jvm") version "1.9.20"
    id("org.jetbrains.intellij") version "1.17.2"
}

group = "com.riwaq"
version = "1.0.0"

repositories {
    mavenCentral()
}

dependencies {
    // HTTP client for server communication
    implementation("com.squareup.okhttp3:okhttp:4.12.0")
    implementation("com.squareup.okhttp3:okhttp-sse:4.12.0")
    implementation("com.google.code.gson:gson:2.10.1")

    // Markdown rendering
    implementation("org.commonmark:commonmark:0.21.0")
    implementation("org.commonmark:commonmark-ext-gfm-tables:0.21.0")
    implementation("org.commonmark:commonmark-ext-autolink:0.21.0")

    // Kotlin coroutines for async operations
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.7.3")
}

// Configure Gradle IntelliJ Plugin
intellij {
    version.set("2023.2")
    type.set("IC") // IntelliJ Community Edition
    plugins.set(listOf())
}

tasks {
    // Set the JVM compatibility versions
    withType<JavaCompile> {
        sourceCompatibility = "17"
        targetCompatibility = "17"
    }
    withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
        kotlinOptions.jvmTarget = "17"
    }

    // Copy the Riwaq binary from the VSCode extension to the plugin resources
    register<Copy>("copyRiwaqBinary") {
        dependsOn("prepareSandbox")
        val os = System.getProperty("os.name").lowercase()
        val arch = System.getProperty("os.arch").lowercase()

        // Determine binary name based on OS
        val binaryName = when {
            os.contains("windows") -> "riwaq.exe"
            else -> "riwaq"
        }

        // Source: VSCode extension bin directory
        val sourceBinary = file("${project.projectDir.parentFile.absolutePath}/vscode-extension/bin/$binaryName")
        // Destination: Plugin lib directory in sandbox
        val destinationDir = file("${buildDir.get()}/idea-sandbox/plugins/Riwaq Arch/lib/bin")

        if (sourceBinary.exists()) {
            from(sourceBinary)
            into(destinationDir)
            doLast {
                // Make binary executable on Unix-like systems
                if (!os.contains("windows")) {
                    val binaryFile = file("$destinationDir/$binaryName")
                    binaryFile.setExecutable(true, false)
                }
            }
        } else {
            println("Warning: Riwaq binary not found at $sourceBinary")
            println("Please build the Riwaq core first: cd core && cargo build --release")
        }
    }

    // Ensure binary is copied during plugin preparation
    named("prepareSandbox") {
        finalizedBy("copyRiwaqBinary")
    }

    // Also copy when building the plugin distribution
    named("buildPlugin") {
        dependsOn("copyRiwaqBinary")
    }

    patchPluginXml {
        sinceBuild.set("232")
        untilBuild.set("241.*")
    }

    signPlugin {
        certificateChain.set(System.getenv("CERTIFICATE_CHAIN"))
        privateKey.set(System.getenv("PRIVATE_KEY"))
        password.set(System.getenv("PRIVATE_KEY_PASSWORD"))
    }

    publishPlugin {
        token.set(System.getenv("PUBLISH_TOKEN"))
    }
}
