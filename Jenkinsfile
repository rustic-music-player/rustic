pipeline {
    agent {
        docker {
            image 'rust'
        }
    }

    stages {
        stage('Build') {
            steps {
                sh 'cargo build --release --message-format json > cargo-build.json'
            }

            post {
                always {
                    recordIssues enabledForFailure: true, tool: cargo(pattern: 'cargo-build.json')
                }
            }
        }
    }
}