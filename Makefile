SRC = fraction.rs main.rs
NAME = computor

all: $(NAME)

$(NAME): $(SRC)
	rustc main.rs -o $(NAME)

run: all
	./$(NAME)

clean:
	rm -f $(NAME)
