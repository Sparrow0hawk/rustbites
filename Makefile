
.PHONY: all
all: helloworld.x

%.x: %.rs
	rustc $< -o $@

.PHONY: clean
clean: 
	rm -f *.x
