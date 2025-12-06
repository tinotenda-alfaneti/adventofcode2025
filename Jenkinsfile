pipeline {
    agent any

    environment {
        CARGO_HOME = "$HOME/.cargo"
        PATH = "$HOME/.cargo/bin:$PATH"
    }

    stages {
        stage('Setup Rust') {
            steps {
                sh '''
                    curl https://sh.rustup.rs -sSf | sh -s -- -y

                    # POSIX compliant shell load (instead of source)
                    . "$HOME/.cargo/env"

                    rustc --version
                    cargo --version
                '''
            }
        }

        stage('Build') {
            steps {
                sh '''
                    . "$HOME/.cargo/env"
                    cargo build --verbose
                '''
            }
        }

        stage('Test') {
            steps {
                sh '''
                    . "$HOME/.cargo/env"
                    cargo test --verbose
                '''
            }
        }

        stage('Lint (Clippy)') {
            steps {
                sh '''
                    . "$HOME/.cargo/env"
                    rustup component add clippy
                    cargo clippy --all-targets --all-features -- -D warnings
                '''
            }
        }

        stage('Format Check') {
            steps {
                sh '''
                    . "$HOME/.cargo/env"
                    rustup component add rustfmt
                    cargo fmt --all -- --check
                '''
            }
        }
    }
}
