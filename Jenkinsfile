pipeline {
	agent {
		docker {
			image 'rust:1.19.0'
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
