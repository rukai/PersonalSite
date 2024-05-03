+++
title = "Rukai Data"
date = 2019-01-29
+++

[Rukaidata](https://rukaidata.com) is a website that renders hitboxes and displays framedata for [Brawl](https://en.wikipedia.org/wiki/Super_Smash_Bros._Brawl) and its most [popular mods](https://projectplusgame.com/).
To see what its really about look at one of the [subaction pages](https://rukaidata.com/P+/Marth/subactions/AttackAirF.html).
<!-- more -->

It started off as an exporter of character data into my own [platform fighter engine](https://github.com/rukai/PF_Sandbox). And it worked pretty well.
<iframe class="video embed-responsive-item" width="560" height="315" src="https://www.youtube.com/embed/IQ8IrTBspTk?rel=0&amp;" allowfullscreen></iframe>

And while this was super cool, it didn't up being a direction I wanted to take the project.
So my engine took another direction, but I had this really cool brawl file exporter lying around.

Being an active player of [Project M](https://en.wikipedia.org/wiki/Project_M) I realized that this exporter could be used to seriously improve the state of framedata.
At the time framedata had to be painstakingly manually compiled by players and every time a new version of a mod was released all the existing data would become outdated.

So I wrote rukaidata to automate the collection of framedata and bring all of the scattered framedata forum threads into one website.
[brawllib_rs](https://github.com/rukai/brawllib_rs) parses the game files, interprets the scripts within, keeping track of all the framedata along the way.
It then exposes this data as:

* a native/wasm app that visualizes the hitboxes in 3D
* a programmatic API giving access to the high level framedata of each move.

Rukaidata then, using brawllib_rs as dependency, embeds the visualiser app in every page, generates an html table for all the hitboxes and lists out all the scripts.
![foo](https://rukaidata.com/P+/Marth/subactions/Wait1.gif)

If you're curious for more details I have [a writeup](https://github.com/rukai/rukaidata/blob/main/docs/writeup.md) that talks about the technical details of exporting framedata from brawl.
I also have a talk that I gave to my local rust meetup in Sydney where I explain my motivations in building the site and also the rust libraries that made it possible
<iframe class="video embed-responsive-item" width="560" height="315" src="https://www.youtube.com/embed/g4Xqf2MFXpk?rel=0&amp;" allowfullscreen></iframe>
