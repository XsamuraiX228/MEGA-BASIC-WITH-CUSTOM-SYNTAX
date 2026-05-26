🦀 PLAYER_SCORE = 0
🦀 COMP_SCORE = 0

📢 "=== WELCOME TO THE CRAB TAVERN ==="
📢 "First to 3 points wins! 🦀"

:round_start
📢 "--- NEW ROUND ---"
📢 "Press ENTER to roll your dice..."
⚓ DUMMY  

🎲 P_DICE1 1 6
🎲 P_DICE2 1 6
🦀 P_TOTAL = P_DICE1 + P_DICE2

📢 "Your total:"
📢 P_TOTAL

🎲 C_DICE1 1 6
🎲 C_DICE2 1 6
🦀 C_TOTAL = C_DICE1 + C_DICE2

📢 "Computer total:"
📢 C_TOTAL

🌊 P_TOTAL == C_TOTAL 🚢 🚀 draw
🌊 P_TOTAL > C_TOTAL 🚢 🚀 player_wins_round
🌊 P_TOTAL < C_TOTAL 🚢 🚀 comp_wins_round

:draw
📢 "It's a draw this round!"
🚀 show_score

:player_wins_round
📢 "You win this round! +1 point"
🦀 PLAYER_SCORE = PLAYER_SCORE + 1
🚀 show_score

:comp_wins_round
📢 "Computer wins this round! +1 point"
🦀 COMP_SCORE = COMP_SCORE + 1
🚀 show_score

:show_score
📢 "You:"
📢 PLAYER_SCORE
📢 "Computer:"
📢 COMP_SCORE
📢 "--------------------"

🌊 PLAYER_SCORE == 3 🚢 🚀 player_victory
🌊 COMP_SCORE == 3 🚢 🚀 comp_victory
🚀 round_start

:player_victory
📢 "CONGRATULATIONS!!! YOU WON THE GAME! 🦀👑"
⛔

:comp_victory
📢 "GAME OVER. The computer wins! 🤖"
⛔