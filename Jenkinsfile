pipeline {
    agent none

    stages {
        stage('Test') {
            agent {
                dockerfile {
                    filename '.jenkins/Dockerfile'
                    additionalBuildArgs '--pull'
                    args '-v /usr/share/jenkins/cache:/build_cache'
                }
            }
            environment {
                CARGO_HOME='/build_cache/cargo'
            }
            steps {
                sh 'cargo test --workspace'
//                sh 'cargo tarpaulin -o Xml -v --workspace'
            }
            post {
                always {
                    cleanWs()
                }
//                success {
//                    cobertura coberturaReportFile: 'cobertura.xml'
//                }
            }
        }
        
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
                        CARGO_HOME='/build_cache/cargo'
                    }
                    steps {
                        sh 'cargo build --workspace --release --message-format json > cargo-build.json'
                        sh 'mv target/release/rustic rustic-linux-x86_64'
                        archiveArtifacts artifacts: 'rustic-linux-x86_64', fingerprint: true
                        archiveArtifacts artifacts: 'target/release/librustic_ffi_client.so', fingerprint: true
                        //archiveArtifacts artifacts: 'target/release/rustic-*-extension', fingerprint: true
                    }
                    post {
                        always {
                            recordIssues failOnError: false, enabledForFailure: true, tool: cargo(pattern: 'cargo-build.json')
//                            cleanWs()
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
                        CARGO_HOME='/build_cache/cargo'
                    }
                    steps {
                        sh 'cargo expand -p rustic-ffi-client > ffi-client.rs'
                        sh 'cbindgen -o bindings.h -c clients/ffi/cbindgen.toml ffi-client.rs'
                        archiveArtifacts artifacts: 'bindings.h', fingerprint: true
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
                    steps {
                        sh 'clients/http/wasm/package.sh'
                        archiveArtifacts artifacts: 'clients/http/wasm/pkg/*.tgz', fingerprint: true
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
                    steps {
                        bat 'cargo build --release --no-default-features --features "http-frontend rodio-backend local-files-provider pocketcasts-provider soundcloud-provider gmusic-provider youtube-provider"'
                        bat 'move target\\release\\rustic.exe rustic-win32-x86_64.exe'
                        archiveArtifacts artifacts: 'rustic-win32-x86_64.exe', fingerprint: true
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
                    steps {
                        sh 'cargo build --bins --workspace --release'
                        sh 'mv target/release/rustic rustic-osx-x86_64'
                        archiveArtifacts artifacts: 'rustic-osx-x86_64', fingerprint: true
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
                CARGO_HOME='/build_cache/cargo'
            }
            when {
                branch 'master'
            }
            steps {
                sh 'cargo doc --workspace --no-deps --exclude rustic-wasm-http-client'
                publishHTML target: [
                    reportDir: 'target/doc',
                    reportFiles: 'rustic/index.html',
                    reportName: 'Documentation',
                    keepAll: false
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
