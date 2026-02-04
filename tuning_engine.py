import chess_bot

# constants
FILENAME = "results/results.bin"

def main():
    print(0b0_000_111111_000000_0_00_0000000000000 >> 22)


    #print(str(chess_bot.eval(0b0, [])))

    #for i in range(0, 10):
    #    tune(i)
    #    read_tune()




def tune(value: int) -> None:
    with open(FILENAME,"w") as f:
        f.write("Hallo" + str(value))

def read_tune() -> None:
    with open(FILENAME,"r") as f:
        contents = f.read()

        print(f"{contents}")



if __name__ == "__main__":
    main()