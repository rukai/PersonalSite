+++
title = "Canon Collision"
date = 2021-10-22
+++

[Canon collision](https://github.com/rukai/canon_collision) was a fan-made Undertale vs Homestuck Platform Fighter.
I worked on it between 2016 and 2021 and it was my first experience with rust.<!-- more -->

Initially the goal was just to build a platform fighter engine without a game, but then I flipped it and turned it into a game but aiming to be highly moddable.
As a result the game had a really powerful in game editor.

Heres a demo of the gameplay and editor:
<iframe class="video embed-responsive-item" width="560" height="315" src="https://www.youtube.com/embed/lacVATVv7iw?rel=0&amp;" allowfullscreen></iframe>

Its seen a lot of changes over the 5 years.
At first it used glium for rendering but that was deprecated so I rewrote the renderer to use vulkano which was then also deprecated, then finally I moved the renderer to wgpu. Those early days of the rust ecosystem were wild.
It represents the peak of my game-dev hobby, these days I spend my time working on development tooling instead.

Unfortunately, despite rust's backwards compatibility guarantees none of the commits seem to be capable of building a running game. :/
I do however have a [windows](canoncollision-4905945c3e8ede2-windows.zip) and [linux](canoncollision-4905945c3e8ede2-linux.tar.gz) binary build of commit [4905945c3e8ede2d79d5a509fbd9fa59fe9772be](https://github.com/rukai/canon_collision/commit/4905945c3e8ede2d79d5a509fbd9fa59fe9772be) that still works.

Maybe one day I'll dive into the code, updating all the dependencies and track down whats going wrong.
