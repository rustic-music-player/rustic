pipeline {
    agent none

    environment {
        SOUNDCLOUD_CLIENT_ID = credentials('rustic-soundcloud-client-id')
        SPOTIFY_CLIENT_ID = credentials('rustic-spotify-client-id')
        SPOTIFY_CLIENT_SECRET = credentials('rustic-spotify-client-secret')
        GMUSIC_CLIENT_ID = credentials('rustic-gmusic-client-id')
        GMUSIC_CLIENT_SECRET = credentials('rustic-gmusic-client-secret')
    }

    stages {
        stage('Build') {
            parallel {
                stage('Linux x64') {
                    agent {
                        dockerfile {
                            filename '.jenkins/Dockerfile'
                            additionalBuildArgs '--pull'
                            args '-v /usr/share/jenkins/cache:/build_cache'
                        }
                    }
                    environment {
                        CARGO_HOME = '/build_cache/cargo'
                    }
                    stages {
                        stage('Test') {
                            steps {
                                sh 'cargo test --workspace'
                                //sh 'cargo tarpaulin -o Xml -v --workspace'
                                //cobertura coberturaReportFile: 'cobertura.xml'
                            }
                        }
                        stage('Build') {
                            steps {
                                sh 'cargo build --workspace --release --message-format json > cargo-build.json'
                                fileOperations([
                                    folderCreateOperation('linux-x86_64'),
                                    fileRenameOperation(destination: 'linux-x86_64/rustic', source: 'target/release/rustic'),
                                    fileRenameOperation(destination: 'linux-x86_64/librustic_ffi_client.so', source: 'target/release/librustic_ffi_client.so'),
                                    fileCopyOperation(targetLocation: 'linux-x86_64/extensions/', includes: 'target/release/*_extension.so', flattenFiles: true)
                                ])
                                archiveArtifacts artifacts: 'linux-x86_64/**/*', fingerprint: true
                                //recordIssues failOnError: false, enabledForFailure: true, tool: cargo(pattern: 'cargo-build.json')
                            }
                        }
                    }
                    post {
                        always {
                            cleanWs()
                        }
                    }
                }

                stage('C Bindings') {
                    agent {
                        dockerfile {
                            filename '.jenkins/Dockerfile.nightly'
                            additionalBuildArgs '--pull'
                            args '-v /usr/share/jenkins/cache:/build_cache'
                        }
                    }
                    environment {
                        CARGO_HOME = '/build_cache/cargo'
                    }
                    stages {
                        stage('Build') {
                            steps {
                                sh 'cargo expand -p rustic-ffi-client > ffi-client.rs'
                                sh 'cbindgen -o bindings.h -c clients/ffi/cbindgen.toml ffi-client.rs'
                                archiveArtifacts artifacts: 'bindings.h', fingerprint: true
                            }
                        }
                    }
                    post {
                        always {
                            cleanWs()
                        }
                    }
                }

                stage('WebAssembly') {
                    agent {
                        dockerfile {
                            filename '.jenkins/Dockerfile.wasm'
                            additionalBuildArgs '--pull'
                            args '-v /usr/share/jenkins/cache:/build_cache'
                        }
                    }
                    stages {
                        stage('Build') {
                            steps {
                                sh 'clients/http/wasm/package.sh'
                                archiveArtifacts artifacts: 'clients/http/wasm/pkg/*.tgz', fingerprint: true
                            }
                        }
                    }
                    post {
                        always {
                            cleanWs()
                        }
                    }
                }

                stage('Windows x64') {
                    agent {
                        label "windows"
                    }
                    stages {
                        stage('Build') {
                            steps {
                                bat 'cargo build --release --no-default-features --features "http-frontend rodio-backend local-files-provider pocketcasts-provider soundcloud-provider gmusic-provider youtube-provider"'
                                bat 'make extensions'
                                fileOperations([
                                    folderCreateOperation('win32-x86_64'),
                                    fileRenameOperation(destination: 'win32-x86_64/rustic.exe', source: 'target/release/rustic.exe'),
//                                    fileRenameOperation(destination: 'win32-x86_64/librustic_ffi_client.dll', source: 'target/release/librustic_ffi_client.dll'),
                                    fileCopyOperation(targetLocation: 'win32-x86_64/extensions/', includes: 'target/release/*_extension.dll', flattenFiles: true)
                                ])
                                archiveArtifacts artifacts: 'win32-x86_64/**/*', fingerprint: true
                            }
                        }
                    }
//                    post {
//                        always {
//                            cleanWs()
//                        }
//                    }
                }

                stage('macOS x64') {
                    agent {
                        label "osx"
                    }
                    stages {
                        stage('Build') {
                            steps {
                                sh 'cargo build --bins --workspace --release'
                                sh 'make ffi-library'
                                sh 'make extensions'
                                fileOperations([
                                    folderCreateOperation('osx-x86_64'),
                                    fileRenameOperation(destination: 'osx-x86_64/rustic', source: 'target/release/rustic'),
                                    fileRenameOperation(destination: 'osx-x86_64/librustic_ffi_client.dylib', source: 'target/release/librustic_ffi_client.dylib'),
                                    fileCopyOperation(targetLocation: 'osx-x86_64/extensions/', includes: 'target/release/*_extension.dylib', flattenFiles: true)
                                ])
                                archiveArtifacts artifacts: 'osx-x86_64/**/*', fingerprint: true
                            }
                        }
                    }
                    post {
                        always {
                            cleanWs()
                        }
                    }
                }
            }
        }

        stage('Docs') {
            agent {
                dockerfile {
                    filename '.jenkins/Dockerfile'
                    additionalBuildArgs '--pull'
                    args '-v /usr/share/jenkins/cache:/build_cache'
                }
            }
            environment {
                CARGO_HOME = '/build_cache/cargo'
            }
            when {
                branch 'master'
            }
            steps {
                sh 'cargo doc --workspace --no-deps --exclude rustic-wasm-http-client'
                publishHTML target: [
                    reportDir  : 'target/doc',
                    reportFiles: 'rustic/index.html',
                    reportName : 'Documentation',
                    keepAll    : false
                ]
            }
            post {
                always {
                    cleanWs()
                }
            }
        }
    }
}
