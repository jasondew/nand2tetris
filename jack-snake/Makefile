COMPILER=../tools/JackCompiler.sh

all: Main.vm SnakeGame.vm Snake.vm Object.vm Position.vm

%.vm: %.jack
	$(COMPILER) $<

clean:
	rm -rf *.vm
