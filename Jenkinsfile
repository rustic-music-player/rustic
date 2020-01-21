pipeline {
    agent {
        dockerfile {
            filename '.jenkins/Dockerfile'
            additionalBuildArgs '--pull'
            args '-v /usr/share/jenkins/cache:/build_cache'
        }
    }

    stages {
        stage('Test') {
            steps {
                sh 'cargo test --bins --workspace --message-format json > cargo-build.json'
            }
            
            post {
                always {
                    recordIssues enabledForFailure: true, tool: cargo(pattern: 'cargo-build.json')
                }
            }
        }
        
        stage('Build') {
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
    }
}
