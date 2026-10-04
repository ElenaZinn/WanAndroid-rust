import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("org.jetbrains.kotlin.plugin.compose")
}

android {
    namespace = "com.elena.wanandroidrust"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.elena.wanandroidrust"
        minSdk = 24
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"
        ndk { abiFilters += listOf("arm64-v8a", "x86_64") }

        // Optional override for the shared Rust core endpoint. Empty means production.
        // Example: ./gradlew -PwanandroidBaseUrl=http://127.0.0.1:8080 assembleDebug
        buildConfigField(
            "String",
            "WANANDROID_BASE_URL",
            "\"${(project.findProperty("wanandroidBaseUrl") as String?).orEmpty()}\"",
        )
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    buildFeatures {
        compose = true
        buildConfig = true
    }
    sourceSets {
        // Path is resolved against the Gradle root project directory (android/),
        // so this points at android/rust-lib without relying on relative hops.
        getByName("main").jniLibs.srcDirs(rootProject.file("rust-lib"))
    }
    ndkVersion = "27.2.12479018"
}

kotlin {
    compilerOptions {
        jvmTarget.set(JvmTarget.JVM_17)
    }
}

dependencies {
    implementation(platform("androidx.compose:compose-bom:2024.09.03"))
    implementation("androidx.activity:activity-compose:1.9.2")
    implementation("androidx.compose.material3:material3")
    implementation("androidx.compose.material:material-icons-core")
    implementation("androidx.compose.ui:ui")
    implementation("androidx.lifecycle:lifecycle-viewmodel-compose:2.8.6")
    implementation("androidx.lifecycle:lifecycle-runtime-compose:2.8.6")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.9.0")
    implementation("io.coil-kt:coil-compose:2.7.0")
    implementation("net.java.dev.jna:jna:5.14.0@aar")
    testImplementation("junit:junit:4.13.2")
}
