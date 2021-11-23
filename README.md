# Morse Code
## Overview
Morse Code is delivered in a series signals which are referred to as dashes (-) or dots (.). To keep things simple for the purposes of this challenge we'll only decode letters with a maximum length of three signals.
Here is the Morse Code dichotomic search table courtesy of Wikipedia
## Morse Code Examples
-.- translates to K
... translates to S
.- translates to A
-- translates to M
. translates to E
## Background
You've started work as morse code translator. Unfortunately some of the signals aren't as distinguishable as others and there are times where a . seems indistinguishable from -. In these cases you write down a ? so that you can figure out what all the posibilities of that letter for that word are later.
## Task
Write a function possibilities that will take a string signals and return an array of possible characters that the Morse code signals could represent.
## Specification
possibilities(signals)
Parameters
signals: String - The Morse code signals that needs to be parsed. The may contain one or more unknown signals (?).
Return Value
Array<String> - The list of possible letters for the given group of signals. Letters will always be ordered from how they appear on the chart, from left to right.
Constraints
There will be no more than 3 characters within signals.
0 - 3 unknown signals may be given.

