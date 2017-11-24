pipeline {
	agent {
		docker {
			image 'vpzom/rust-input:1.0'
			args '--cpus=0.8'
		}
	}
	stages {
		stage('Build') {
			steps {
				sh 'cargo build --release'
				archiveArtifacts artifacts: '**/target/release/tuxparty', fingerprint: true
			}
		}
	}
}
