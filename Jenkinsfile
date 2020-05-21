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
            steps {
                sh 'cargo test --bins --workspace'
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
                    steps {
                        //sh 'curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh'
                        sh 'cargo build --bins --workspace --release --message-format json > cargo-build.json'
                        //sh 'wasm-pack build clients/http/wasm'
                        //sh 'wasm-pack pack clients/http/wasm'
                    }
                    post {
                        always {
                            recordIssues enabledForFailure: true, tool: cargo(pattern: 'cargo-build.json')
                            sh 'mv target/release/rustic rustic-linux-x86_64'
                            archiveArtifacts artifacts: 'rustic-linux-x86_64', fingerprint: true
                            //archiveArtifacts artifacts: 'target/release/rustic-*-extension', fingerprint: true
                            //archiveArtifacts artifacts: 'clients/http/wasm/pkg/*.tgz', fingerprint: true
                        }
                    }
                }

                stage('Windows x64') {
                    agent {
                        label "windows"
                    }
                    steps {
                        bat 'cargo build --bins --workspace --release'
                    }
                    post {
                        always {
                            bat 'mv target/release/rustic.exe rustic-win32-x86_64.exe'
                            archiveArtifacts artifacts: 'rustic-win32-x86_64.exe', fingerprint: true
                        }
                    }
                }

                stage('macOS x64') {
                    agent {
                        label "osx"
                    }
                    steps {
                        sh 'cargo build --bins --workspace --release'
                    }
                    post {
                        always {
                            sh 'mv target/release/rustic rustic-osx-x86_64'
                            archiveArtifacts artifacts: 'rustic-osx-x86_64', fingerprint: true
                        }
                    }
                }
            }
        }
    }
}
