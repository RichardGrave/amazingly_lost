# !!! Discontinued !!!

## Amazingly Lost

#### The idea

Well, many years ago before I even started programming in Rust, I wanted to recreate a
maze game that I have played from when I was a kid.\
It was a commodore 64 game that I can't remember the name of.\
The game was about a men with a torch in a predefined maze and only had a certain amount of levels to play.\
In the game you could only see the walls that were within the light radius of the torch.\
The rest of the maze was hidden in darkness and only visible if you came close enough.\
It was a fun game (for me at least), but the levels never changed.\
That's why I wanted to make a similar game with a randomly generated maze.

I couldn't find a programming language that I was happy with to create this game in and after a while I forgot about this game idea.\
Then one day when I was trying out Bevy, I suddenly remembered what I wanted to make.\
I started putting some ideas on paper and tried to figure out what kind of fun stuff I want to implement.

#### The game

I have never programmed a maze generator before and I had to dig through a lot of tutorials on how to do this.\
There were a lot of 'trial and error' moments implementing all of this in Rust with Bevy.\
I also made a lot of design mistakes and refactored the code many times.\
That was something I expected because I don't have much experience developing games.

The result at this point is a working maze that can be of various sizes.\
Collision detection and randomly selected images for the maze tiles.\
Unfortunately I'm not good at creating pixel-art, but for a prototype I was very happy.\
I had so much fun programming all of this.

<img src="https://github.com/RichardGrave/amazingly_lost/blob/main/Amazing_screenshot.png" width="100%" height="100%">

#### Life

Then life took over.\
My wife suffered more from her muscular dystrophy.\
Both of our children also got more problems with their illnesses.\
They are both autistic and our oldest has the same muscular dystrophy as my wife.\
Our youngest has a syndrome of which he is the only one in the world so far.\
We had to go to medical appointments much more often than usual.\
I also have a full time job and after a while I ran out of energy to work on my hobbies in my spare time.

After some time had passed and everything calmed down, I had more energy again.\
I wanted to continue with this game, but fate struck once again.\
My wife got breast cancer. That gave us many sleepless nights.\
In and out of the hospital for months for examinations, surgery and treatment.

#### Now

Our life is now falling back to our old rhythm and I want to try to pick up my hobbies again.\
I'm not sure how much time exactly has passed since I last worked on this project.\
It should be at least more than half a year since I did something with this game.

I was looking through the code for a couple of days.\
With a lot of the code I don't remember why I did it that way and what I wanted to do with it.\
Making adjustments and trying to pick it all up again, but I can't seem to enjoy this project anymore.\
Therefor I have decided to stop this project.

At least the code builds and the game is somewhat playable.\
I use MacOS myself, so I'm not sure if everything will work fine on a Windows machine.

#### Usable keys
<pre>
N         = generate a new maze
Q         = quit the game
Page-up   = increase maze size (also generates a new maze)
Page-down = decrease maze size (also generates a new maze)

A or Left-arrow  = go WEST
D or Right-arrow = go EAST
W or Up-arrow    = go NORTH
S or Down-arrow  = go SOUTH
</pre>

The bigger the maze, the longer it takes to generate.\
So I have set a maximum size for the maze and of course also a minimum size.

#### What's next

At this point I don't know what kind of project I want to do next, but there is lots to learn and to do.\
I will continue with projects in Rust and Bevy 0.6 is also something I'd like to look into.\
Maybe I'll do some more with Swift and Python as well.

#### Used resources

Images:\
I created all the (amazingly bad) pixel-art myself :)

dependencies:\
bevy 0.5.0\
rand 0.8.3


