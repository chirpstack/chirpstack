.PHONY: requirements 

all: requirements install clean-after

requirements:
	rm -rf Chirpstack
	dotnet clean 
install:
	dotnet build -c Release -o /app/build
clean-after:
	rm -rf obj
	rm -rf bin