LET PLAYER_SCORE = 0
LET COMP_SCORE = 0

PRINT "=== WELCOME TO THE DICE TAVERN ==="
PRINT "First to 3 points wins!"
PRINT ""

:round_start
PRINT "--- NEW ROUND ---"
PRINT "Press ENTER to roll your dice..."

INPUT DUMMY  


RANDOM P_DICE1 1 6
RANDOM P_DICE2 1 6
LET P_TOTAL = P_DICE1 + P_DICE2

PRINT "You rolled:"
PRINT P_DICE1
PRINT P_DICE2
PRINT "Your total:"
PRINT P_TOTAL
PRINT ""


RANDOM C_DICE1 1 6
RANDOM C_DICE2 1 6
LET C_TOTAL = C_DICE1 + C_DICE2

PRINT "Computer rolled:"
PRINT C_DICE1
PRINT C_DICE2
PRINT "Computer total:"
PRINT C_TOTAL
PRINT ""


IF P_TOTAL == C_TOTAL THEN GOTO draw
IF P_TOTAL > C_TOTAL THEN GOTO player_wins_round
IF P_TOTAL < C_TOTAL THEN GOTO comp_wins_round

:draw
PRINT "It's a draw this round!"
GOTO show_score

:player_wins_round
PRINT "You win this round! +1 point"
LET PLAYER_SCORE = PLAYER_SCORE + 1
GOTO show_score

:comp_wins_round
PRINT "Computer wins this round! +1 point"
LET COMP_SCORE = COMP_SCORE + 1
GOTO show_score

:show_score
PRINT "CURRENT SCORE:"
PRINT "You:"
PRINT PLAYER_SCORE
PRINT "Computer:"
PRINT COMP_SCORE
PRINT "--------------------"
PRINT ""


IF PLAYER_SCORE == 3 THEN GOTO player_victory
IF COMP_SCORE == 3 THEN GOTO comp_victory


GOTO round_start

:player_victory
PRINT "CONGRATULATIONS!!! YOU WON THE GAME!"
END

:comp_victory
PRINT "GAME OVER. The computer beat you. Try again!"
END