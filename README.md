<!-- Header -->
<div align="center">

<img src="https://github.com/kleineluka/burial/blob/main/preview/leyley.png" width="100" height="100">

# Burial
A (pretty) one stop shop for [TCOAAL](https://store.steampowered.com/app/2378900/The_Coffin_of_Andy_and_Leyley/) modding! 
<br>
**Please** leave a ⭐ Star ⭐ on Github to support!
<br>
**Current Status:** 🚧 In Development! 🚧 75% Complete! 


[Straight to Downloads 🕯️](https://github.com/kleineluka/burial/releases) • [Discord 🥰](https://www.discord.gg/WWxAjJMspk) • [View Changelog 🍅](https://github.com/kleineluka/burial/wiki/Changelog) 

</div>

---

<!-- Navigation + Preview -->
<div align="center">

[Features 🐰](#features-) • [Install ☕](#install-) • [Documentation 📻](#guides--faq-) • [Roadmap 🚧](#roadmap-) • [Contribute 🩷](#contribute-) • [Credits 🎉](https://github.com/kleineluka/burial/blob/main/ATTRIBUTIONS.md)

<br>

<img src="https://github.com/kleineluka/burial/blob/main/preview/app.gif" style="width: 75%; height: auto;" alt="Loading large Burial preview.. sorry Github doesn't support webm :'( or you are previewing this markdown in VSCode, if so, thanks for working on Burial!">

</div>


<!-- Features -->
# Features 🐰
Burial is a program for **mod players**, **mod creators**, and **data miners**! Burial has something for everyone~ 🍅

- **Resources** ✂️: Decrypt/encrypt files from/to .k9a, sift through and export categories of resources, generate templates of assets to make your own, parse/import dialogue, and decrypt/modify/manage save files.
- **Reversing** 🧬: Inject code into the game, export and deobfuscate the game's run-time code, manage NW.js SDK's, enable developer tools, view/edit game information, and create/manage backups of your game files.
- **Mod Tools** 🖍️:  Create/edit mod json files, create/edit repo json files, generate differences between two mod packages, package your mod, and upload your mod.
- **Mod Manager** 🍱: Install pre-made modpacks, add/update/remove Tomb modloader, browse and download mods, view/toggle/update installed mods, and create/manage multiple instances of the game.
- **Knowledge** 📔: View what obfuscated functions and variables do and easily access modding resources.
- **Easy and Safe** 🍵: Burial is designed with guardrails in mind to help you from messing up your game! And, ixt's only a few easy clicks to get started~
- **Quality of Life** 🌸: Pretty GUI with character-based themes, persistant settings, built-in tutorials, update checking, lots of emojis..
- **By Fans, Not Thieves** 🥰: A legal copy of the game is required to use Burial! Burial will **never** implement any anti-DRM or piracy tools. 

<!-- Install -->
# Install ☕
**Executables will not be provided until a stable version is completed.**
1. **Download** the latest release from the [Releases](https://github.com/kleineluka/burial/releases).
2. **Run** the installer, silly!
    <br>
    Question 1. [Why is it flagged as unknown/malicious?]()
    <br>
    Question 2. [Why can't the program be portable? (Hint: It kind of is?)]()
3. **Open** the program and **enjoy**! If you have any issues, please refer to the [Documentation](#documentation-).

<!-- Documentation -->
# Documentation 📻
Please view the [Wiki](https://github.com/kleineluka/burial/wiki)! If you want to skip right to help with installation, please head over to the [Installation and Help](https://github.com/kleineluka/burial/wiki/Installation-and-Help) page. If you are having trouble with the program beyond installation, check out the [Problems and Questions](https://github.com/kleineluka/burial/wiki/Problems-and-Questions) page. I hope these help! 😇

The tech stack 🍡 is a Rust back-end (Tauri framework), a Javascript front-end (vanilla), Python scripts to aid in development, TypeScript (Deno) for deobfuscation, and Javascript for game modification. Whew, that was a lot! 😅 

<!-- Roadmap -->
# Roadmap 🚧
(In no particular order and beyond basic planned features, aka things that will start after the first release..)
- [ ] Language Support
- [ ] Steam Deck Compatibility
- [ ] General Linux Improvements
- [ ] Proper Status Bar
- [ ] More User Settings
- [ ] More Tooltips + Custom Design
- [ ] Universal and Friendlier™ Path Input Solution
- [ ] Home Page Setting
- [x] Rebrand Mod Tools to Mod Making to avoid confusion
- [x] Development Build Scripts

<!-- Contribute -->
# Contribute 🩷
Please refer to the [Contributing Wiki Page](https://github.com/kleineluka/burial/wiki/Contributing) to help with development, submit bug reports, or request features. I also maintain a **Trello** if you are interested in being a frequent helper!

<!-- Credits & Licenses -->
# Credits, Licenses, & Acknowledgements 🎉
- **[The Coffin of Andy and Leyley](https://store.steampowered.com/app/2378900/The_Coffin_of_Andy_and_Leyley/)** is developed and published by Kit9 Studios. Please support the developers by purchasing the game on Steam. Kit9 Studios has no affiliation with Burial.
- **[RPG Maker MV](https://www.rpgmakerweb.com/eula)** was used to create the game and thus many features of the game are related to this engine. RPG Maker is not affiliated with Burial, and please support the engine by purchasing it if you are interested.
- [**Burial's code**](https://www.github.com/kleineluka/burial) is provided under the [**MIT**](https://github.com/kleineluka/burial/blob/main/LICENSE) license.
- **[Basil's Wiki](https://coffin-wiki.basil.cafe/)** has been a great resource for getting started with modifying the game. Furthermore, my implementation ([**Hausmaerchen**](https://github.com/kleineluka/burial/tree/main/src-backend/bundled/hausmaerchen)) of [**Webcrack**](https://github.com/j4k0xb/webcrack) to deobfuscate code was inspired by [**Basil's Grimoire**](https://codeberg.org/basil/grimoire).
- **[LlamaToolkit](https://github.com/Llamaware/LlamaToolkit/)** ([**GLWTPL**](https://github.com/me-shaon/GLWTPL)) was referenced for portions of Burial's original [cipher.rs](https://github.com/kleineluka/burial/blob/main/src-backend/src/utils/cipher.rs) implementation. Similarly, **[Llamaloader](https://github.com/Llamaware/LlamaLoader)** has a neat little trick to find the game installation.
- [**RPG Save Converter**](https://github.com/13xforever/rpgsave-converter) (MIT) was a great read for understanding the save format.
- [**Tauri**](https://github.com/tauri-apps/tauri), the Rust-based front-end framework, is licensed under MIT/Apache.
- **Rust Crates** can be seen in the [**Cargo.toml**](https://github.com/kleineluka/burial/blob/main/src-backend/Cargo.toml) file.
- **JavaScript Libraries** can be seen in both [**package.json**](https://github.com/kleineluka/burial/blob/main/package.json) and [**src/assets/ext**](https://github.com/kleineluka/burial/tree/main/src-frontend/assets/ext).
-  **NotoSans** and **Nunito** (fonts used in Burial) are under the [**Open Font License**](https://openfontlicense.org/).
- Burial can install **third-party** resources to assist in mod loading. [**Tomb**](https://codeberg.org/basil/tomb) by Basil can be installed as the mod loader and mods are pulled from the [**Llamaware Page**](https://github.com/Llamaware/Llamaware.github.io/tree/main/src). Naturally, any mods installed through Burial are the **property of their creator**.
