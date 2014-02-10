#CC=gcc

CFLAGS=-Wall

BIN=./bin
INC=./inc
OBJ=./obj
SRC=./src

EXE=lspath

all: $(BIN)/$(EXE)

run: all
	$(BIN)/$(EXE) $(ARGS)

$(BIN)/$(EXE): $(OBJ)/main.o
	$(CC) -o $(BIN)/$(EXE) $(OBJ)/main.o

$(OBJ)/main.o: $(SRC)/main.c
	$(CC) -c $(CFLAGS) -I$(INC) -o $(OBJ)/main.o $(SRC)/main.c

clean:
	rm -f $(BIN)/*
	rm -f $(OBJ)/*
