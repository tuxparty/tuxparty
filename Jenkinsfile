pipeline {
	agent {
		docker {
			image 'rust:1.19.0'
		}
	}
	stages {
		state('Build') {
			steps {
				sh 'cargo build --release'
			}
		}
	}
}
