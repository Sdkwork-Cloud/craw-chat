plugins {
    id("com.android.application")
    id("kotlin-android")
    // The Flutter Gradle Plugin must be applied after the Android and Kotlin Gradle plugins.
    id("dev.flutter.flutter-gradle-plugin")
}

android {
    namespace = "com.sdkwork.im.sdkwork_im_flutter_mobile"
    compileSdk = flutter.compileSdkVersion
    ndkVersion = flutter.ndkVersion

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = JavaVersion.VERSION_17.toString()
    }

    defaultConfig {
        // TODO: Specify your own unique Application ID (https://developer.android.com/studio/build/application-id.html).
        applicationId = "com.sdkwork.im.sdkwork_im_flutter_mobile"
        // You can update the following values to match your application needs.
        // For more information, see: https://flutter.dev/to/review-gradle-config.
        minSdk = flutter.minSdkVersion
        targetSdk = flutter.targetSdkVersion
        versionCode = flutter.versionCode
        versionName = flutter.versionName
    }

    val releaseSigningProperties = listOf(
        "SDKWORK_RELEASE_KEYSTORE_FILE",
        "SDKWORK_RELEASE_KEYSTORE_PASSWORD",
        "SDKWORK_RELEASE_KEY_ALIAS",
        "SDKWORK_RELEASE_KEY_PASSWORD",
    ).associateWith { providers.gradleProperty(it).orNull }
    val releaseSigningReady = releaseSigningProperties.values.all { !it.isNullOrBlank() }

    signingConfigs {
        if (releaseSigningReady) {
            create("release") {
                storeFile = file(releaseSigningProperties.getValue("SDKWORK_RELEASE_KEYSTORE_FILE")!!)
                storePassword = releaseSigningProperties.getValue("SDKWORK_RELEASE_KEYSTORE_PASSWORD")
                keyAlias = releaseSigningProperties.getValue("SDKWORK_RELEASE_KEY_ALIAS")
                keyPassword = releaseSigningProperties.getValue("SDKWORK_RELEASE_KEY_PASSWORD")
            }
        }
    }

    buildTypes {
        release {
            if (releaseSigningReady) {
                signingConfig = signingConfigs.getByName("release")
            }
        }
    }

    tasks.configureEach {
        if (name.contains("Release") && (name.startsWith("assemble") || name.startsWith("bundle"))) {
            doFirst {
                if (!releaseSigningReady) {
                    throw GradleException(
                        "Release signing requires SDKWORK_RELEASE_KEYSTORE_FILE, " +
                            "SDKWORK_RELEASE_KEYSTORE_PASSWORD, SDKWORK_RELEASE_KEY_ALIAS, and " +
                            "SDKWORK_RELEASE_KEY_PASSWORD Gradle properties.",
                    )
                }
            }
        }
    }
}

flutter {
    source = "../.."
}
