use crate::state::{BuildType, Platform, SDK};

pub fn generate(platform: Platform, sdk: SDK, build_type: BuildType) -> String {
    match (platform, sdk, build_type) {
        (Platform::GitHub, SDK::Native, BuildType::Signed) => {
            let code = r#"
name: Android release build

on:
  workflow_dispatch:
    inputs:
      versionName:
        description: 'User-facing release version name'
        required: true
        default: "1.0.0"
      versionCode:
        description: 'versionCode or build number'
        required: true
        default: '1'

jobs:
  release-build:
    runs-on: ubuntu-latest

    steps:
      - name: Setup versionName regardless of how this action is triggered
        id: version_name
        run: |
          WORKFLOW_INPUT=${{ github.event.inputs.versionName }}
          VERSION_NAME=${WORKFLOW_INPUT:-"1.0.0"}
          echo "ORG_GRADLE_PROJECT_VERSION_NAME=$VERSION_NAME" >> $GITHUB_ENV

      - name: Setup versionCode regardless of how this action is triggered
        id: version_code
        run: |
          WORKFLOW_INPUT=${{ github.event.inputs.versionCode }}
          VERSION_CODE=${WORKFLOW_INPUT:-"1"}
          echo "ORG_GRADLE_PROJECT_VERSION_CODE=$VERSION_CODE" >> $GITHUB_ENV

      - uses: actions/checkout@v3

      - uses: actions/setup-java@v3
        with:
          distribution: 'zulu'
          java-version: 11
          cache: 'gradle'

      - name: Make gradlew executable
        run: chmod +x gradlew

      - name: Retrieve secrets
        env:
          KEYSTORE_B64: ${{ secrets.KEYSTORE_B64 }}
          KEY_PROPERTIES_B64: ${{ secrets.KEY_PROPERTIES_B64 }}
        run: |
          echo $KEYSTORE_B64 | base64 --decode > app/keystore.jks
          echo $KEY_PROPERTIES_B64 | base64 --decode > key.properties

      - name: Build prod APK
        run: ./gradlew --no-daemon bundleProdRelease

      - name: Upload build file
        uses: actions/upload-artifact@v3
        with:
          name: release-aab
          path: app/build/outputs/bundle/prodRelease/app-prod-release.aab
"#;
            code.to_string()
        }

        (Platform::GitHub, SDK::Flutter, BuildType::Signed) => {
            let code = r#"
name: Android Play Store release build aab

on:
  workflow_dispatch:
    inputs:
      versionName:
        description: 'User-facing release version name'
        required: true
        default: "1.0.0"
      versionCode:
        description: 'versionCode or build number'
        required: true
        default: '1'

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - uses: actions/setup-java@v3
      with:
        distribution: 'zulu'
        java-version: 11
        cache: 'gradle'

    # Setup the flutter environment.
    - uses: subosito/flutter-action@v2
      with:
        channel: 'stable'
        cache: true

    # Get flutter dependencies.
    - run: flutter pub get

    - name: Retrieve secrets
      env:
        KEYSTORE_B64: ${{ secrets.KEYSTORE_B64 }}
        KEY_PROPERTIES_B64: ${{ secrets.KEY_PROPERTIES_B64 }}
      run: |
        echo $KEYSTORE_B64 | base64 --decode > android/app/ueno-upload-keystore.jks
        echo $KEY_PROPERTIES_B64 | base64 --decode > android/key.properties

    # Build apk.
    - run: flutter build appbundle --release --build-number=${{ github.event.inputs.versionCode }} --build-name=${{ github.event.inputs.versionName }}

    # Upload generated apk to the artifacts.
    - uses: actions/upload-artifact@v3
      with:
        name: release-aab
        path: build/app/outputs/bundle/release/app-release.aab
"#;
            code.to_string()
        }

        (Platform::GitHub, SDK::ReactNative, BuildType::Signed) => {
            let code = r#"
name: Android release build

on:
  workflow_dispatch:
    inputs:
      versionName:
        description: 'User-facing release version name'
        required: true
        default: "1.0.0"
      versionCode:
        description: 'versionCode or build number'
        required: true
        default: '1'

jobs:
  release-build:
    runs-on: ubuntu-latest

    steps:
      - name: Setup versionName regardless of how this action is triggered
        id: version_name
        run: |
          WORKFLOW_INPUT=${{ github.event.inputs.versionName }}
          VERSION_NAME=${WORKFLOW_INPUT:-"1.0.0"}
          echo "ORG_GRADLE_PROJECT_VERSION_NAME=$VERSION_NAME" >> $GITHUB_ENV

      - name: Setup versionCode regardless of how this action is triggered
        id: version_code
        run: |
          WORKFLOW_INPUT=${{ github.event.inputs.versionCode }}
          VERSION_CODE=${WORKFLOW_INPUT:-"1"}
          echo "ORG_GRADLE_PROJECT_VERSION_CODE=$VERSION_CODE" >> $GITHUB_ENV

      - uses: actions/checkout@v3

      - uses: actions/setup-node@v3.5.1
        with:
          node-version: 18
          cache: 'npm'

      - uses: actions/setup-java@v3
        with:
          distribution: 'zulu'
          java-version: 11
          cache: 'gradle'

      - name: Install dependencies
        run: npm install

      - name: Make gradlew executable
        run: cd android && chmod +x ./gradlew

      - name: Retrieve secrets
        env:
          KEYSTORE_B64: ${{ secrets.KEYSTORE_B64 }}
          KEY_PROPERTIES_B64: ${{ secrets.KEY_PROPERTIES_B64 }}
        run: |
          echo $KEYSTORE_B64 | base64 --decode > android/app/keystore.jks
          echo $KEY_PROPERTIES_B64 | base64 --decode > android/key.properties

      - name: Build debug APK
        run: cd android && ./gradlew --no-daemon bundleRelease

      - name: Upload build file
        uses: actions/upload-artifact@v3
        with:
          name: debug-build
          path: android/app/build/outputs/bundle/release/app-release.aab
"#;
            code.to_string()
        }

        (Platform::GitHub, SDK::Native, BuildType::Unsigned) => {
            let code = r#"
name: Android debug build

on:
  push:
    branches: [ "develop" ]
  pull_request:
    branches: [ "develop" ]

  # Allows you to run this workflow manually from the Actions tab
  workflow_dispatch:
    inputs:
      versionName:
        description: 'User-facing release version name'
        required: true
        default: "1.0.0"
      versionCode:
        description: 'versionCode or build number'
        required: true
        default: "1"

jobs:
  debug-build:
    runs-on: ubuntu-latest

    steps:
      - name: Setup versionName regardless of how this action is triggered
        id: version_name
        run: |
          WORKFLOW_INPUT=${{ github.event.inputs.versionName }}
          VERSION_NAME=${WORKFLOW_INPUT:-"1.0.0"}
          echo "ORG_GRADLE_PROJECT_VERSION_NAME=$VERSION_NAME" >> $GITHUB_ENV

      - name: Setup versionCode regardless of how this action is triggered
        id: version_code
        run: |
          WORKFLOW_INPUT=${{ github.event.inputs.versionCode }}
          VERSION_CODE=${WORKFLOW_INPUT:-"1"}
          echo "ORG_GRADLE_PROJECT_VERSION_CODE=$VERSION_CODE" >> $GITHUB_ENV

      - uses: actions/checkout@v3

      - uses: actions/setup-java@v3
        with:
          distribution: 'zulu'
          java-version: 11
          cache: 'gradle'

      - name: Make gradlew executable
        run: chmod +x gradlew

      - name: Build debug APK
        run: ./gradlew --no-daemon assembleDevDebug

      - name: Upload build file
        uses: actions/upload-artifact@v3
        with:
          name: debug-build
          path: app/build/outputs/apk/dev/debug/app-dev-debug.apk
"#;
            code.to_string()
        }

        (Platform::GitHub, SDK::Flutter, BuildType::Unsigned) => {
            let code = r#"
name: Android quick debug build

on:
  workflow_dispatch:
    inputs:
      versionName:
        description: 'User-facing release version name'
        required: true
        default: "1.0.0"
      versionCode:
        description: 'versionCode or build number'
        required: true
        default: '1'

jobs:
  build:
    # This job will run on ubuntu virtual machine
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - uses: actions/setup-java@v3
      with:
        distribution: 'zulu'
        java-version: 11
        cache: 'gradle'

    # Setup the flutter environment.
    - uses: subosito/flutter-action@v2
      with:
        channel: 'stable'
        cache: true

    # Get flutter dependencies.
    - run: flutter pub get

    # Build apk.
    - run: flutter build apk --debug --build-number=${{ github.event.inputs.versionCode }} --build-name=${{ github.event.inputs.versionName }}

    # Upload generated apk to the artifacts.
    - uses: actions/upload-artifact@v3
      with:
        name: release-aab
        path: build/app/outputs/flutter-apk/app-debug.apk
"#;
            code.to_string()
        }

        (Platform::GitHub, SDK::ReactNative, BuildType::Unsigned) => {
            let code = r#"
name: Android debug build

on:
  workflow_dispatch:
    inputs:
      versionName:
        description: 'User-facing release version name'
        required: true
        default: "1.0.0"
      versionCode:
        description: 'versionCode or build number'
        required: true
        default: '1'

jobs:
  debug-build:
    runs-on: ubuntu-latest

    steps:
      - name: Setup versionName regardless of how this action is triggered
        id: version_name
        run: |
          WORKFLOW_INPUT=${{ github.event.inputs.versionName }}
          VERSION_NAME=${WORKFLOW_INPUT:-"1.0.0"}
          echo "ORG_GRADLE_PROJECT_VERSION_NAME=$VERSION_NAME" >> $GITHUB_ENV

      - name: Setup versionCode regardless of how this action is triggered
        id: version_code
        run: |
          WORKFLOW_INPUT=${{ github.event.inputs.versionCode }}
          VERSION_CODE=${WORKFLOW_INPUT:-"1"}
          echo "ORG_GRADLE_PROJECT_VERSION_CODE=$VERSION_CODE" >> $GITHUB_ENV

      - uses: actions/checkout@v3

      - uses: actions/setup-node@v3.5.1
        with:
          node-version: 18
          cache: 'npm'

      - uses: actions/setup-java@v3
        with:
          distribution: 'zulu'
          java-version: 11
          cache: 'gradle'

      - name: Install dependencies
        run: npm install

      - name: Make gradlew executable
        run: cd android && chmod +x ./gradlew

      - name: Build debug APK
        run: cd android && ./gradlew --no-daemon assembleDebug

      - name: Upload build file
        uses: actions/upload-artifact@v3
        with:
          name: debug-build
          path: android/app/build/outputs/apk/debug/app-debug.apk
"#;
            code.to_string()
        } // (Platform::Bitrise, SDK::Native, BuildType::Signed) => {
          //     "name: bitrise, native, signed".to_string()
          // }

          // (Platform::Bitrise, SDK::Flutter, BuildType::Signed) => {
          //     "name: bitrise, flutter, signed".to_string()
          // }

          // (Platform::Bitrise, SDK::ReactNative, BuildType::Signed) => {
          //     "name: bitrise, rn, unsigned".to_string()
          // }

          // (Platform::Bitrise, SDK::Native, BuildType::Unsigned) => {
          //     "name: bitrise, native, unsigned".to_string()
          // }

          // (Platform::Bitrise, SDK::Flutter, BuildType::Unsigned) => {
          //     "name: bitrise, flutter, unsigned".to_string()
          // }

          // (Platform::Bitrise, SDK::ReactNative, BuildType::Unsigned) => {
          //     "name: bitrise, rn, unsigned".to_string()
          // }
    }
}
