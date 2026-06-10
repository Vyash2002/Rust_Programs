RS_FILES := $(wildcard *.rs)
BINARIES := $(RS_FILES:.rs=)

clean:
	rm -f $(BINARIES) *.exe *.pdb