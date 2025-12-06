pipeline {
    agent any

    environment {
        CARGO_HOME = "$HOME/.cargo"
        PATH = "$HOME/.cargo/bin:$PATH"
    }

    stages {
        stage('Setup Rust') {
            steps {
                sh """
                    curl https://sh.rustup.rs -sSf | sh -s -- -y
                    source $HOME/.cargo/env
                    rustc --version
                    cargo --version
                """
            }
        }

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
