pipeline {
	agent {
		docker {
			image 'rust:1.21-jessie'
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
