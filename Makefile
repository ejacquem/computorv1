SRC = main.rs
NAME = computor

all: $(NAME)

$(NAME): $(SRC)
	rustc main.rs -o $(NAME)

clean:
	rm -f $(NAME)

re: clean all

