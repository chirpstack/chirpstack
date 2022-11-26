.PHONY: build

all: clean build jar

clean:
	@echo "Cleaning..."
	@./gradlew clean
	@echo "Clean."

build:
	@echo "Building..."
	@./gradlew build
	@echo "Build complete."

jar: build
	@echo "Generating jar..."
	@./gradlew jar
	@echo "Jar generated in build/libs."

install: build
	@echo "Installing artifact in local repository..."
	@./gradlew publishToMavenLocal
	@echo "Artifact installed in local repository ~/.m2"
