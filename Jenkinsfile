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
//            post {
//                success {
//                    cobertura coberturaReportFile: 'cobertura.xml'
//                }
//            }
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
                    }
                    post {
                        always {
                            recordIssues enabledForFailure: true, tool: cargo(pattern: 'cargo-build.json')
                        }
                        success {
                            sh 'mv target/release/rustic rustic-linux-x86_64'
                            archiveArtifacts artifacts: 'rustic-linux-x86_64', fingerprint: true
                            archiveArtifacts artifacts: 'target/release/librustic_ffi_client.so', fingerprint: true
                            //archiveArtifacts artifacts: 'target/release/rustic-*-extension', fingerprint: true
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
                    }
                    post {
                        success {
                            archiveArtifacts artifacts: 'bindings.h', fingerprint: true
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
                    }
                    post {
                        success {
                            archiveArtifacts artifacts: 'clients/http/wasm/pkg/*.tgz', fingerprint: true
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
//                sshagent(['rustic-github-docs']) {
//                    dir('web') {
//                        git credentialsId: 'rustic-github-docs', url: 'git@github.com:rustic-music-player/rustic-music-player.github.io.git', changelog: false
//                        sh 'git config user.email "jenkins@maxjoehnk.me"'
//                        sh 'git config user.name "Jenkins CI"'
//                        sh 'git rm -rf docs || exit 0'
//                        sh 'mkdir docs'
//                        sh 'cp -r ../target/doc/* docs/'
//                        sh 'git add docs'
//                        sh 'git commit -m "docs: update generated documentation"'
//                        sh 'git push --set-upstream origin master'
//                    }
//                }
            }
        }
    }
}
