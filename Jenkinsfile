pipeline {
    agent {
        docker {
            image 'rust:latest'
            args '-u root' 
        }
    }

    stages {
        stage('Build') {
            steps {
                sh "cargo build --verbose"
            }
        }

        stage('Test') {
            steps {
                sh "cargo test --verbose"
            }
        }

        stage('Lint (Clippy)') {
            steps {
                sh "rustup component add clippy"
                sh "cargo clippy --all-targets --all-features -- -D warnings"
            }
        }

        stage('Format Check') {
            steps {
                sh "rustup component add rustfmt"
                sh "cargo fmt --all -- --check"
            }
        }
    }
}
