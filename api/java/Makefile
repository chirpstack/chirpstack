.PHONY: build

all: clean build jar

clean:
	@echo "Cleaning..."
	@./gradlew clean > /dev/null
	@echo "Clean."

build:
	@echo "Building..."
	@./gradlew build > /dev/null
	@echo "Build complete."

jar: build
	@echo "Generating jar..."
	@./gradlew jar > /dev/null
	@echo "Jar generated in build/libs."

install: build
	@echo "Installing artifact in local repository..."
	@./gradlew publishToMavenLocal > /dev/null
	@echo "Artifact installed in local repository ~/.m2"
