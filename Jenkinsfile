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
                        }
                        success {
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
                        success {
                            bat 'move target\\release\\rustic.exe rustic-win32-x86_64.exe'
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
                        success {
                            sh 'mv target/release/rustic rustic-osx-x86_64'
                            archiveArtifacts artifacts: 'rustic-osx-x86_64', fingerprint: true
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
            when {
                branch 'master'
            }
            steps {
                sh 'cargo doc --workspace --no-deps'
                sshagent(['rustic-github-docs']) {
                    dir('web') {
                        git credentialsId: 'rustic-github-docs', url: 'git@github.com:rustic-music-player/rustic-music-player.github.io.git', changelog: false
                        sh 'git rm -rf docs'
                        sh 'mkdir docs'
                        sh 'cp -r ../target/doc/* docs/'
                        sh 'git add docs'
                        sh 'git commit -m "docs: update generated documentation"'
                        sh 'git push'
                    }
                }
            }
        }
    }
}
