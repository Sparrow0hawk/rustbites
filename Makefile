
SRC=$(wildcard *.rs)

.PHONY: all
all: helloworld.x

helloworld.x: helloworld.rs
	rustc $< -o $@

.PHONY: clean
clean: 
	rm *.x
