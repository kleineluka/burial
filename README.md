<!-- Header -->
<div align="center">

<img src="https://github.com/kleineluka/burial/blob/main/preview/leyley.png" width="100" height="100">

# Burial
A cross-platform application for [TCOAAL](https://store.steampowered.com/app/2378900/The_Coffin_of_Andy_and_Leyley/) decryption, modding, content creation, and development. 


[Straight to Downloads 🕯️](https://www.github.com/kleineluka/burial) • [View Changelog 🍅](https://www.github.com/kleineluka/burial) • [Tech Stack & Insights 🍰](https://www.github.com/kleineluka/burial)

**Current Status:** 🚧 In Development! 🚧 
<br>
**Please** leave a ⭐ Star ⭐ on Github to support!

</div>

---

<!-- Navigation + Preview -->
<div align="center">

[Features 🐰](#features-) • [Install ☕](#install-) • [Documentation 📻](#guides--faq-) • [Contribute 👨🏻‍🤝‍👩🏻](#contribute-) • [Credits & Licenses 🎉](#credits--licenses-)

<br>

<img src="https://github.com/kleineluka/burial/blob/main/preview/app.gif" width="500" height="400">

</div>


<!-- Features -->
# Features 🐰
- **Decryption Tool** ✂️: Easily decrypt .k9a files (images, audio, etc.) individually and recursively in folders.
- **Resource Sifting** 🐠: Automatically export what you want: only want Ashley sprites? Of course you do! Or what about background audio only? Surprisingly good ost..
- **Sprite Builder** 🥺: Create templates based on the sprites in game and export them to create your own new sprites!
- **Game Tools** 🧬: Dump code from the game with various injection methods and enable tools like developer console inside the game.
- **Mod Creator** 🪄: Please see the below list, as this is the feature that is in heavy development!
- **Persistant Settings** 🍪: Automatically configure and save things like your TCOAAL installation folder to avoid having to input it a million trillion times..
- **Optimised and Cross-Platform** 🦄: Executables are provided for Windows, Linux, and MacOS - and by using a Rust backend with OS native frontends, it's a super duper tiny program that can run on any potato!
- **Pretty GUI** 🌸: Based on Ashley's colours, responsive, scales with your screen size, easy to navigate, all that stuff..
- **By Fans, Not Thieves** 🥰: Burial uses byte-patching to modify/create game content so that assets don't need to be redistributed and a legal copy is required to use Burial.

While not yet implemented (and even a lot of the above are WIP), here are some further planned and in development features..
- **Sprite Builder -> Mixer** 🍸: Combine faces and busts to make cursed new things, or use your own artwork.
- **Mod Creator -> Encrypt** 🩹: Easily encrypt your edited or new resource files back into .k9a format to be used in the game.
- **Mod Creator -> Dialogue Creator** 💬: Create new dialogue scenes or edit old dialogue in the game.
- **Mod Creator -> Replacer** 👝: Replace audio, sprites, etc. from the game.
- **Mod Creator -> Additions** 🧩: Add new content into the game, like new sprites, audio, etc.
- **Mod Creator -> Map Maker** 🗺️: Create your own maps!
- **Mod Creator -> Voice Support** 🦜: TBA..
- **Mod Creator -> Mod Bundler** 🎁: Bundle your mod up in one little preset to distribute!
  
And much more, but this list and the software's organisation will probably change as I understand the game's code better and establish a modding framework.. 

Burial will **always be open-source** and will **never** implement any anti-DRM tools. Support the game, sillies.


<!-- Install -->
# Install ☕
TBA!

<!-- Documentation -->
# Documentation 📻
DOCUMENTATION.md TBA!

<!-- Contribute -->
# Contribute 👨🏻‍🤝‍👩🏻
**For development**, Burial uses a Rust backend and a Web frontend with Tauri bridging the two. Additionally, some python scripts are used to assist in development. **For bugs and requests**, these are highly welcomed too! Please note that this was developed and tested on Windows, so Mac and Linux users please let me know of any bugs. Detailed programming information and other contributing information will come soon - **CONTRIBUTING.md TBA**!

<!-- Credits & Licenses -->
# Credits & Licenses 🎉
- **Burial** is provided under the [MIT](https://github.com/kleineluka/burial/blob/main/LICENSE) license.
- **[LlamaToolkit](https://github.com/Llamaware/LlamaToolkit/)** ([GLWTPL](https://github.com/me-shaon/GLWTPL)) was referenced for Burial's [cipher.rs](https://github.com/kleineluka/burial/blob/main/src-tauri/src/utils/cipher.rs) implementation - which saved a lot of time! 
