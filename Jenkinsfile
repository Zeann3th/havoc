pipeline {
    agent any

    stages {
        stage('Checkout') {
            steps {
                checkout scmGit(branches: [[name: '*/main']], extensions: [], userRemoteConfigs: [[url: 'https://github.com/Zeann3th/havoc.git']])
            }
        }

        stage('Test') {
            agent {
                docker {
                    image 'rust:1.88'
                }
            }

            steps {
                sh '''
                    echo "Running tests..."
                    cargo test
                '''
            }
        }

        stage('Build') {
            agent {
                docker {
                    image 'rust:1.88'
                    reuseNode true
                }
            }

            steps {
                sh '''
                    echo "Building project..."
                    cargo build --release
                '''
            }
        }
    }
}