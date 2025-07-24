pipeline {
    agent any

    stages {
        stage('Test') {
            agent {
                docker {
                    image 'rust:1.88'
                }
            }

            steps {
                bat '''
                    echo "Running tests..."
                    cargo test --verbose
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
                bat '''
                    echo "Building project..."
                    cargo build --release
                '''
            }
        }
    }
}