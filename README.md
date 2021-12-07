# Estimation bot

## What's estimation?
The card game Estimation is a trick-taking game where you score points by correctly predicting the amount of tricks you will take each round.

>🌟 It's a strategy based game that requires a lot of thinking and practising.

## What'll this bot do?
Try beating my dad who's considered as a master at this game 😁

## Objective 🤔
Score the most points by correctly predicting how many tricks you’ll take each round.

## Trick 🎃?
A trick is created by each player playing a card into 
the middle.

The winner of the trick is the player with the highest 
raking card.

![Trick Image here](https://github.com/YassinEldeeb/estimation-bot/blob/main/images/trick.png)

## Card Rank 💪
The Cards are ranked with “Ace” being the Highest and “two” being the Lowest untill a trump suit is established.

![Card rank Image here](https://github.com/YassinEldeeb/estimation-bot/blob/main/images/ranks.png)

The trump suited card will outrank all other suited cards.

for example: if “clubs” are trump, then the 9 of clubs will outrank the “Ace” of hearts

![Trump Suit Image here](https://github.com/YassinEldeeb/estimation-bot/blob/main/images/trumpSuit.png)

## Deal 🤝
Before the game starts, the first dealer needs to be established.

This’s done by each player picking a card and the person with the highest ranking card is the first dealer.

![delear image here](https://github.com/YassinEldeeb/estimation-bot/blob/main/images/dealer.png)

## Bidding 💲

After the cards are dealt but before the gameplay 
starts, each player will bid on how many tricks,
he think he’ll win.

Bidding will start with the player on the left of the 
dealer and continue around clockwise.

There's also bidding on what the "Trump Suit" will be and the player with the highest trump suit ranking is the one who decides what is the "Trump Suit" for the round.

>🌟 Trump Suit Ranking: Spades -> Heart -> Diamonds -> Clubs

One player will need to be designated as the score
keeper and have a scorecard ready before the 
bidding starts.

As players bid the score keeper will write down the
stated bid for each player.

The total number of tricks bid can’t be equal to the total
number of tricks possible, so this game is played using the [Standard 52-card deck](https://en.wikipedia.org/wiki/Standard_52-card_deck)


![card deck image](https://github.com/YassinEldeeb/estimation-bot/blob/main/images/standard%2052-card-deck.png)

which means that this table can clarify the maximum number of tricks bid that's possible based on the number of the players.


![Table image here](https://github.com/YassinEldeeb/estimation-bot/blob/main/images/table.png)

This makes it that at least one player goes negative each round cause if you multiplied the players count with the Card Amount you'll always get a number that's less than 52(number of the cards).

## Game Play 🃏

## 🌟 I think the video format will be better for understanding the gamplay instead of writing, check [this video](https://youtu.be/aqmiD5RTBkI?t=171)

The player left of the dealer plays the first card into
the middle.

Each player must follow the lead suit when possible.

Trump cards can’t be used unless the player don’t 
have the lead suit.

If a player is unable to follow the lead, that player 
can play any card but the card he’ll place will have 
the least rank of all cards unless it was a trump suit 😉

The winner of the trick is the player who played the
highest card in the lead suit unless a Trump suited
card was played, if Trump is involved the player with
the highest ranking trump card would win the trick.

The winner of the trick leads the next trick.

## Keeping Score 💯

10 Points are added to each player’s bid every round

If a player correctly predicts the amount of tricks he
won the player is awarded his bid + 10 to his score.

If a player incorrectly guessed the amount of tricks
he won the player is given the negative amount of 
his (bid + ten points) to his score.

![Score Image here](https://github.com/YassinEldeeb/estimation-bot/blob/main/images/score.png)

## Strategy 🤖

The standard card deck has 52 cards
- 13 of each unit 👇
- Clubs, Diamonds, Hearts, and Spades

1- so the first strategy that the bot will use to defeat my dad is to keep track of what cards that was played and what is still in player's hands so that the bot can decide:

- the approximately the number of tricks he'll bid on
- wether a particular time is good or not to play a high ranking card based on the cards with the players. 


![Strategy 1](https://github.com/YassinEldeeb/estimation-bot/blob/main/images/strategy1.png)

2- still thinking...

# Credits

- This [YouTube Tutorial](https://www.youtube.com/watch?v=aqmiD5RTBkI)

- This [wiki](https://en.wikipedia.org/wiki/Standard_52-card_deck)
