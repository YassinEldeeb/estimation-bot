# Estimation bot ๐ค

## What's estimation?
The card game Estimation is a trick-taking game where you score points by correctly predicting the amount of tricks you will take each round.

>๐ It's a strategy based game that requires a lot of thinking, practising and a bit of luck ๐

## What'll this bot do?
Try beating estimation pro players without machine learning using some maths formulas! 
>oh god what have I gotten myself into ๐คฆโโ๏ธ

# Game Explanation ๐จโ๐ซ

## Objective ๐ค
Score the most points by correctly predicting how many tricks youโll take each round.

## Trick ๐?
A trick is created by each player playing a card into 
the middle.

The winner of the trick is the player with the highest 
raking card.

![Trick Image here](https://github.com/YassinEldeeb/estimation-bot/blob/main/images/trick.png)

## Card Rank ๐ช
The Cards are ranked with โAceโ being the Highest and โtwoโ being the Lowest untill a trump suit is established.

![Card rank Image here](https://github.com/YassinEldeeb/estimation-bot/blob/main/images/ranks.png)

The trump suited card will outrank all other suited cards.

for example: if โclubsโ are trump, then the 9 of clubs will outrank the โAceโ of hearts

![Trump Suit Image here](https://github.com/YassinEldeeb/estimation-bot/blob/main/images/trumpSuit.png)

## Deal ๐ค
Before the game starts, the first dealer needs to be established.

Thisโs done by each player picking a card and the person with the highest ranking card is the first dealer.

![delear image here](https://github.com/YassinEldeeb/estimation-bot/blob/main/images/dealer.png)

## Bidding ๐ฒ

After the cards are dealt but before the gameplay 
starts, each player will bid on how many tricks,
he think heโll win.

Bidding will start with the player on the left of the 
dealer and continue around clockwise.

There's also bidding on what the "Trump Suit" will be and the player with the highest trump suit ranking is the one who decides what is the "Trump Suit" for the round.

>๐ Trump Suit Ranking: Spades -> Heart -> Diamonds -> Clubs

One player will need to be designated as the score
keeper and have a scorecard ready before the 
bidding starts.

As players bid the score keeper will write down the
stated bid for each player.

The total number of tricks bid canโt be equal to the total
number of tricks possible, so this game is played using the [Standard 52-card deck](https://en.wikipedia.org/wiki/Standard_52-card_deck)


![card deck image](https://github.com/YassinEldeeb/estimation-bot/blob/main/images/standard%2052-card-deck.png)

which means that this table can clarify the maximum number of tricks bid that's possible based on the number of the players.


![Table image here](https://github.com/YassinEldeeb/estimation-bot/blob/main/images/table.png)

This makes it that at least one player goes negative each round cause if you multiplied the players count with the Card Amount you'll always get a number that's less than 52(number of the cards).

## Game Play ๐

## ๐ I think the video format will be better for understanding the gamplay instead of reading, check [this video](https://youtu.be/aqmiD5RTBkI?t=171)

The player left of the dealer plays the first card into
the middle.

Each player must follow the lead suit when possible.

Trump cards canโt be used unless the player donโt 
have the lead suit.

If a player is unable to follow the lead, that player 
can play any card but the card heโll place will have 
the least rank of all cards unless it was a trump suit ๐

The winner of the trick is the player who played the
highest card in the lead suit unless a Trump suited
card was played, if Trump is involved the player with
the highest ranking trump card would win the trick.

The winner of the trick leads the next trick.

## Keeping Score ๐ฏ

10 Points are added to each playerโs bid every round

If a player correctly predicts the amount of tricks he
won the player is awarded his bid + 10 to his score.

If a player incorrectly guessed the amount of tricks
he won the player is given the negative amount of 
his (bid + ten points) to his score.

![Score Image here](https://github.com/YassinEldeeb/estimation-bot/blob/main/images/score.png)

## Strategy ๐ค

The standard card deck has 52 cards
- 13 of each unit ๐
- Clubs, Diamonds, Hearts, and Spades

1- so the first strategy that the bot will use to defeat my dad is to keep track of what cards that was played and what is still in player's hands so that the bot can decide:

- approximately the number of tricks he'll bid on
- wether a particular time is good or not to play a high ranking card based on the cards with the players. 


![Strategy 1](https://github.com/YassinEldeeb/estimation-bot/blob/main/images/strategy1.png)

2- still thinking...

# Credits

- This [YouTube Tutorial](https://www.youtube.com/watch?v=aqmiD5RTBkI)

- This [wiki](https://en.wikipedia.org/wiki/Standard_52-card_deck)
