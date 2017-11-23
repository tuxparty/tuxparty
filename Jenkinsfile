pipeline {
	agent {
		docker {
			image 'rust:1.21-jessie'
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
