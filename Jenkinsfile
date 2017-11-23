pipeline {
	agent {
		docker {
			image 'rust:1.22.1'
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
