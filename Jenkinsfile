pipeline {
    agent {
        dockerfile {
            filename '.jenkins/Dockerfile'
            additionalBuildArgs '--pull'
            args '-v /usr/share/jenkins/cache:/build_cache'
        }
    }

    stages {
        stage('Build') {
            steps {
                sh 'cargo build --workspace --release --message-format json > cargo-build.json'
            }

            post {
                always {
                    recordIssues enabledForFailure: true, tool: cargo(pattern: 'cargo-build.json')
                }
            }
        }
    }
}
