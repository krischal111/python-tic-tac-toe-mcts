import random

def play_game(board, player):
    while True:
        # Choose a random empty cell
        row = random.randint(0, 2)
        col = random.randint(0, 2)
        if board[row][col] == " ":
            board[row][col] = player
            break
    
    # Check if the game is over
    if is_winning_move(board, player):
        return player
    elif is_full(board):
        return "T"  # Tie
    else:
        return None  # Game continues

def is_winning_move(board, player):
    # Check rows
    for row in board:
        if row == [player, player, player]:
            return True
    
    # Check columns
    for col in range(3):
        if board[0][col] == player and board[1][col] == player and board[2][col] == player:
            return True
    
    # Check diagonals
    if board[0][0] == player and board[1][1] == player and board[2][2] == player:
        return True
    if board[0][2] == player and board[1][1] == player and board[2][0] == player:
        return True
    
    return False

def is_full(board):
    for row in board:
        if " " in row:
            return False
    return True

def monte_carlo_simulation(board, oplayer, n):
    wins = 0
    ties = 0
    player = oplayer
    for i in range(n):
        result = None
        while result is None:
            if player == "X":
                player = "O"
            else:
                player = "X"
            result = play_game(board, player)
        if result == oplayer:
            wins += 1
        elif result == "T":
            ties += 1
    return wins / n, ties / n

def monte_carlo_start(board,player, n):
    monte_simu = {}
    for row in range(3):
        for col in range(3):
            if board[row][col] == " ":
                simulated_board = board
                simulated_board[row][col] = player
                wins,ties = monte_carlo_simulation(simulated_board,player,n)
                monte_simu[(row,col)] = (wins,ties)
                print(f"At ({row}, {col}) win:ties = {wins}:{ties}")
            else:
                continue
    return monte_simu

n = 100
board = [[" ", " ", " "], [" ", " ", " "], [" ", " ", " "]]
player = "X"
print(f"{monte_carlo_start(board,player,n)}")
# wins, ties = monte_carlo_simulation(n)
# print(f"Probability of win: {wins:.2f}")
# print(f"Probability of tie: {ties:.2f}")
# print(f"Probability of loss: {1-wins-ties:.2f}")
