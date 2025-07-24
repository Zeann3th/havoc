pipeline {
    agent any

    stages {
        stage('Test') {
            docker {
                image 'rust:1.88'
            }

            steps {
                sh '''
                    echo "Running tests..."
                    cargo test --verbose
                '''
            }
        }

        stage('Build') {
            docker {
                image 'rust:1.88'
                reuseNode true
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