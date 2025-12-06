pipeline {
  agent {
    kubernetes {
      yaml """
apiVersion: v1
kind: Pod
spec:
  containers:
    - name: rust
      image: rust:bookworm
      command: ['sleep','infinity']
      tty: true
"""
    }
  }

  stages {
    stage('Build') {
      steps {
        container('rust') {
          sh 'rustc --version'
          sh 'cargo build --verbose'
        }
      }
    }

    stage('Test') {
      steps {
        container('rust') {
          sh 'cargo test --verbose'
        }
      }
    }

    stage('Lint') {
      steps {
        container('rust') {
          sh 'rustup component add clippy'
          sh 'cargo clippy -- -D warnings'
        }
      }
    }

    stage('Format Check') {
      steps {
        container('rust') {
          sh 'rustup component add rustfmt'
          sh 'cargo fmt -- --check'
        }
      }
    }
  }
}
