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
                        sh 'cargo build --bins --workspace --release --message-format json > cargo-build.json'
                    }
                    post {
                        always {
                            recordIssues enabledForFailure: true, tool: cargo(pattern: 'cargo-build.json')
                            archiveArtifacts artifacts: 'target/release/rustic', fingerprint: true
                            archiveArtifacts artifacts: 'target/release/rustic-*-extension', fingerprint: true
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
                            archiveArtifacts artifacts: 'target/release/*.exe', fingerprint: true
                        }
                    }
                }
            }
        }
    }
}
