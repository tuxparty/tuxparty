pipeline {
	agent {
		docker {
			image 'rust-input-docker'
			args '--cpus=0.8'
		}
	}
	stages {
		stage('Build') {
			steps {
				sh 'cargo build --release'
			}
		}
	}
}
