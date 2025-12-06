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
                    . "$HOME/.cargo/env"
                '''
            }
        }

        stage('Install Build Tools') {
            steps {
                sh '''
                    echo "Detecting OS for compiler setup..."
                    if command -v apt-get >/dev/null 2>&1; then
                        echo "Installing build-essential (Debian/Ubuntu)..."
                        apt-get update -y
                        apt-get install -y build-essential clang pkg-config libssl-dev
                    elif command -v apk >/dev/null 2>&1; then
                        echo "Installing toolchain (Alpine)..."
                        apk add --no-cache build-base clang pkgconfig openssl-dev
                    else
                        echo "Unsupported base image: no apt or apk found"
                        exit 1
                    fi
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
