# osu!tinfoil

**Got bored and decided I wanna play osu! with a piece of tinfoil.**

## Hardware

The hardware is very simple: just a ESP32 dev kit with a wire connecting it to a piece of tinfoil on pin `D4`. (ideally secured to something so it doesnt move)

## Driver

I made a tiny driver in Rust to read the data from the ESP32, used the `debouncr` library to debounce the data and `xdotool` to simulate mouse input. (_Linux + Xorg only!_)

## Skin

I changed the cursor on [osu!default plus](https://www.reddit.com/r/OsuSkins/comments/ej3htj/osu_default_skin_plus/) to be a piece of tinfoil, you can download the modified version [here](https://github.com/ronthecookie/osu-tinfoil/raw/master/tinfoil.osk).

## Future

I plan to try to make a tinfoil tablet (so more probe points) but I don't know how well that will work...
